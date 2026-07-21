use crate::prototype::Prototype;
use crate::prototype::PrototypeManager;
use bevy::prelude::*;
use serde::Deserialize;
use std::collections::{HashMap, HashSet};
use std::fs;
use toml::Value;
use walkdir::WalkDir;

#[derive(Deserialize)]
struct ProtoFile {
    #[serde(default)]
    prototypes: Vec<Value>,
}

pub(super) fn load_prototypes(mut prototype_manager: ResMut<PrototypeManager>) {
    let mut raw_protos: HashMap<String, Value> = HashMap::new();

    for entry in WalkDir::new("assets/prototypes")
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        if path.extension().map(|e| e == "toml") != Some(true) {
            continue;
        }
        let content = match fs::read_to_string(path) {
            Ok(c) => c,
            Err(e) => {
                warn!("Read error {:?}: {}", path, e);
                continue;
            }
        };

        let file: ProtoFile = match toml::from_str(&content) {
            Ok(f) => f,
            Err(e) => {
                warn!("Parse error {:?}: {}", path, e);
                continue;
            }
        };

        for proto in file.prototypes {
            let Some(id) = proto.get("id").and_then(Value::as_str) else {
                warn!("Prototype without `id` in {:?}", path);
                continue;
            };
            if raw_protos.insert(id.to_string(), proto.clone()).is_some() {
                warn!("Duplicate prototype id `{}` in {:?}", id, path);
            }
        }
    }

    // резолвим наследование: parent-поля подкладываются под собственные поля прототипа
    let mut resolved: HashMap<String, Value> = HashMap::new();
    let ids: Vec<String> = raw_protos.keys().cloned().collect();
    for id in ids {
        if resolved.contains_key(&id) {
            continue;
        }
        let mut visiting = HashSet::new();
        if let Err(e) = resolve_prototype(&id, &raw_protos, &mut resolved, &mut visiting) {
            warn!("Failed to resolve prototype `{}`: {}", id, e);
        }
    }

    for (id, value) in resolved {
        match value.try_into::<Prototype>() {
            Ok(proto) => {
                prototype_manager.prototypes.insert(id, proto);
            }
            Err(e) => warn!("Failed to deserialize prototype `{}`: {}", id, e),
        }
    }
}

//нейрослоп. если что-то сломается, то возможно это тут
/// Рекурсивно резолвит цепочку parent -> ... -> root, мержит таблицы
/// (поля ребёнка перекрывают поля родителя). Кэширует результат в `resolved`,
/// чтобы общий предок не пересчитывался для каждого потомка.
fn resolve_prototype(
    id: &str,
    raw: &HashMap<String, Value>,
    resolved: &mut HashMap<String, Value>,
    visiting: &mut HashSet<String>,
) -> Result<Value, String> {
    if let Some(v) = resolved.get(id) {
        return Ok(v.clone());
    }
    if !visiting.insert(id.to_string()) {
        return Err(format!("inheritance cycle detected at `{}`", id));
    }

    let raw_proto = raw
        .get(id)
        .ok_or_else(|| format!("unknown prototype id `{}`", id))?
        .clone();

    let merged = match raw_proto.get("parent").and_then(Value::as_str) {
        Some(parent_id) => {
            let parent_id = parent_id.to_string();
            let parent_value = resolve_prototype(&parent_id, raw, resolved, visiting)?;
            let mut base = parent_value
                .as_table()
                .cloned()
                .ok_or_else(|| format!("parent `{}` is not a table", parent_id))?;
            let child = raw_proto
                .as_table()
                .ok_or_else(|| format!("prototype `{}` is not a table", id))?;
            merge_table(&mut base, child);
            Value::Table(base)
        }
        None => raw_proto,
    };

    visiting.remove(id);
    resolved.insert(id.to_string(), merged.clone());
    Ok(merged)
}

/// Shallow merge: каждый ключ из overlay перезаписывает ключ в base.
fn merge_table(base: &mut toml::value::Table, overlay: &toml::value::Table) {
    for (k, v) in overlay {
        base.insert(k.clone(), v.clone());
    }
}

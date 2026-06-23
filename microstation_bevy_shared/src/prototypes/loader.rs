use crate::prototypes::{EntityPrototype, PrototypeManager};
use bevy::prelude::*;
use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use walkdir::WalkDir;

pub fn load_prototypes(mut manager: ResMut<PrototypeManager>) {
    let mut raw_protos: HashMap<String, Value> = HashMap::new();
    let mut success = 0;
    let mut failed = 0;

    for entry in WalkDir::new("Resources/Prototypes")
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        if path.extension().map(|e| e == "yaml" || e == "yml") != Some(true) {
            continue;
        }
        let content = match fs::read_to_string(path) {
            Ok(c) => c,
            Err(e) => {
                warn!("Read error {:?}: {}", path, e);
                failed += 1;
                continue;
            }
        };

        let options = serde_saphyr::options! { strict_booleans: true };
        match serde_saphyr::from_str_with_options::<Vec<Value>>(&content, options) {
            Ok(docs) => {
                for doc in docs {
                    if let Value::Object(ref map) = doc {
                        if let Some(Value::String(id)) = map.get("id") {
                            raw_protos.insert(id.clone(), doc);
                        }
                    }
                }
                success += 1;
            }
            Err(e) => {
                warn!("Parse error {:?}: {}", path, e);
                failed += 1;
            }
        }
    }

    info!("Prototypes parsed: {} ok, {} failed", success, failed);

    let mut resolved_cache: HashMap<String, EntityPrototype> = HashMap::new();
    let mut visiting: HashMap<String, bool> = HashMap::new();

    for id in raw_protos.keys().cloned().collect::<Vec<_>>() {
        match resolve_and_merge(&id, &raw_protos, &mut visiting) {
            Ok(merged) => match serde_json::from_value::<EntityPrototype>(merged) {
                Ok(proto) => {
                    resolved_cache.insert(id, proto);
                }
                Err(e) => error!("Deserialization failed for '{}': {}", id, e),
            },
            Err(e) => error!("Resolution failed for '{}': {}", id, e),
        }
    }

    manager.entity_prototypes = resolved_cache;
}

fn resolve_and_merge(
    id: &str,
    raw: &HashMap<String, Value>,
    visiting: &mut HashMap<String, bool>,
) -> Result<Value, String> {
    if visiting.contains_key(id) {
        return Err("Circular inheritance detected".into());
    }
    visiting.insert(id.to_string(), true);

    let base = raw
        .get(id)
        .ok_or_else(|| format!("Missing prototype '{}'", id))?;
    let merged = if let Value::Object(map) = base {
        if let Some(Value::String(parent_id)) = map.get("parent") {
            let parent_val = resolve_and_merge(parent_id, raw, visiting)?;
            merge_values(&parent_val, base)
        } else {
            base.clone()
        }
    } else {
        base.clone()
    };

    visiting.remove(id);
    Ok(merged)
}

fn merge_values(base: &Value, child: &Value) -> Value {
    match (base, child) {
        (Value::Object(b_map), Value::Object(c_map)) => {
            let mut merged = b_map.clone();
            for (k, c_val) in c_map {
                if let Some(b_val) = merged.get(k) {
                    if k == "components" {
                        merged.insert(k.clone(), merge_component_arrays(b_val, c_val));
                    } else {
                        merged.insert(k.clone(), merge_values(b_val, c_val));
                    }
                } else {
                    merged.insert(k.clone(), c_val.clone());
                }
            }
            Value::Object(merged)
        }
        _ => child.clone(),
    }
}

fn merge_component_arrays(parent: &Value, child: &Value) -> Value {
    let mut comp_map: HashMap<String, Value> = HashMap::new();
    if let Value::Array(arr) = parent {
        for comp in arr {
            if let Value::Object(m) = comp {
                if let Some(Value::String(t)) = m.get("type") {
                    comp_map.insert(t.clone(), comp.clone());
                }
            }
        }
    }
    if let Value::Array(arr) = child {
        for comp in arr {
            if let Value::Object(m) = comp {
                if let Some(Value::String(t)) = m.get("type") {
                    if let Some(existing) = comp_map.get_mut(t) {
                        *existing = merge_values(existing, comp);
                    } else {
                        comp_map.insert(t.clone(), comp.clone());
                    }
                }
            }
        }
    }
    Value::Array(comp_map.into_values().collect())
}

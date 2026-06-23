mod entity;
pub mod loader;
mod tile;

use crate::prototypes::entity::{EntityPrototype, Parent, YamlComponent};
use crate::prototypes::tile::TilePrototype;
use crate::world::Position;
use bevy::prelude::*;
use bevy_replicon::prelude::Replicated;
use std::collections::HashMap;

#[derive(Resource, Default)]
pub struct PrototypeManager {
    pub entity_prototypes: HashMap<String, EntityPrototype>,
    pub tile_prototypes: HashMap<String, TilePrototype>, //TODO other protos
}

impl PrototypeManager {
    pub fn spawn_entity(
        &self,
        id: &str,
        position: Option<Position>,
        commands: &mut Commands,
    ) -> Option<Entity> {
        //TODO переделать в Result
        let Some(proto) = self.entity_prototypes.get(id) else {
            error!("prototype {id} not found");
            return None;
        };
        let mut entity = match &proto.parent {
            None => commands.spawn_empty(),
            Some(Parent::One(p)) => {
                let e = self.spawn_entity(p, position.clone(), commands).unwrap();
                commands.entity(e)
            }
            Some(Parent::Many(ps)) => {
                todo!(); // TODO я хуй знает как это делать, придётся жеж всё к хуям перелопачивать (добавить одну внутреннюю функцию)
            }
        };
        let mut has_unknown_components = false;
        for comp in &proto.components {
            match comp {
                YamlComponent::MetaData(c) => {
                    entity.insert(c.clone());
                }
                YamlComponent::Transform(c) => {
                    entity.insert(c.clone());
                }
                YamlComponent::Sprite(c) => {
                    entity.insert(c.clone());
                }
                YamlComponent::IconSmooth(c) => {
                    entity.insert(c.clone());
                }
                YamlComponent::Item(c) => {
                    entity.insert(c.clone());
                }
                YamlComponent::Other => {
                    has_unknown_components = true;
                }
            }
        }
        if has_unknown_components {
            warn!("Prototype {id} has unknown components");
        }
        if let Some(pos) = position {
            entity.insert(pos);
        }
        entity.insert(Replicated);
        info!(
            "[Spawn] Entity {:?} components: {:?}",
            entity.id(),
            entity.id()
        );
        Some(entity.id())
    }
}

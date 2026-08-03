pub mod plugin;
use bevy::prelude::*;
use microstation_bevy_shared::{
    map::Position,
    prototype::{EntityPrototypeComponent, PrototypeId, PrototypeKind, PrototypeManager},
};

use crate::sprites::tile::TILE_SIZE;

pub fn on_prototype_added(
    trigger: On<Add, PrototypeId>,
    prototypes: Query<(&PrototypeId, &Position)>,
    prototype_registry: Res<PrototypeManager>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) -> Result<()> {
    let (PrototypeId(id), Position(pos)) = prototypes.get(trigger.entity)?;
    let prototype = prototype_registry
        .prototypes
        .get(id)
        .ok_or_else(|| BevyError::error(format!("prototype `{id}` not found").as_str()))?;

    if let Some(name) = &prototype.name {
        commands
            .entity(trigger.entity)
            .insert(Name::new(name.clone()));
    }
    let PrototypeKind::Entity(entity_proto) = &prototype.proto else {
        return Err(BevyError::error(
            format!("prototype `{id}` is not an entity").as_str(),
        ));
    };
    commands.entity(trigger.entity).insert((
        Transform::from_translation((pos.as_vec2() * TILE_SIZE).extend(1.)),
        Visibility::default(),
    ));
    for component in &entity_proto.components {
        // only client components are handled here
        match component {
            EntityPrototypeComponent::Sprite(sprite_info) => {
                commands.entity(trigger.entity).insert(Sprite {
                    image: asset_server.load(&sprite_info.path),
                    ..Default::default()
                });
            }
            _ => {}
        }
    }
    Ok(())
}

use bevy::prelude::*;
use bevy_replicon::shared::replication::Replicated;
use microstation_bevy_shared::{
    map::{Position, occupancy::BlocksMovement},
    prototype::{EntityPrototypeComponent, PrototypeId, PrototypeKind, PrototypeManager},
};

pub fn spawn_prototype(
    commands: &mut Commands,
    prototypes: &PrototypeManager,
    proto_id: &str,
    pos: IVec2,
) -> Result<Entity> {
    let proto = prototypes
        .prototypes
        .get(proto_id)
        .ok_or_else(|| BevyError::error(format!("prototype `{proto_id}` not found").as_str()))?;

    let PrototypeKind::Entity(entity_proto) = &proto.proto else {
        return Err(BevyError::error(
            format!("prototype `{proto_id}` is not an entity").as_str(),
        ));
    };

    let mut e = commands.spawn((PrototypeId(proto_id.to_string()), Position(pos), Replicated));

    for component in &entity_proto.components {
        // only server components are handled here
        match component {
            EntityPrototypeComponent::BlocksMovement => {
                e.insert(BlocksMovement);
            }
            _ => {}
        }
    }
    Ok(e.id())
}

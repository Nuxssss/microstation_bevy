use bevy::platform::collections::{HashMap, HashSet};
use bevy::prelude::*;
use microstation_bevy_shared::{
    components::{icon_smooth::*, sprite::*},
    world::Position,
};

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct SmoothSystems;

pub struct IconSmoothPlugin;

impl Plugin for IconSmoothPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SmoothQueue>();
        app.init_resource::<SmoothSpatialIndex>();
        app.add_systems(Startup, init_spatial_index);
        app.add_systems(
            Update,
            (
                update_spatial_index_add,
                update_spatial_index_move,
                update_spatial_index_remove,
                queue_dirty_entities,
                process_smooth_queue,
            )
                .chain()
                .in_set(SmoothSystems),
        );
    }
}

#[derive(Resource, Default, Debug)]
pub struct SmoothQueue {
    dirty: HashSet<Entity>,
    generation: u32,
    processed: HashMap<Entity, u32>,
}

#[derive(Resource, Default, Debug)]
pub struct SmoothSpatialIndex {
    index: HashMap<IVec2, Vec<Entity>>,
    reverse: HashMap<Entity, IVec2>,
}

fn init_spatial_index(
    entities: Query<(Entity, &Position), With<IconSmooth>>,
    mut index: ResMut<SmoothSpatialIndex>,
) {
    for (entity, pos) in &entities {
        index.index.entry(pos.0).or_default().push(entity);
        index.reverse.insert(entity, pos.0);
    }
}

fn update_spatial_index_add(
    added: Query<(Entity, &Position), Added<IconSmooth>>,
    mut index: ResMut<SmoothSpatialIndex>,
) {
    for (entity, pos) in &added {
        index.index.entry(pos.0).or_default().push(entity);
        index.reverse.insert(entity, pos.0);
    }
}

fn update_spatial_index_move(
    moved: Query<(Entity, &Position), (Changed<Position>, With<IconSmooth>)>,
    mut index: ResMut<SmoothSpatialIndex>,
) {
    for (entity, pos) in &moved {
        if let Some(old_pos) = index.reverse.remove(&entity) {
            if let Some(entities) = index.index.get_mut(&old_pos) {
                entities.retain(|&e| e != entity);
                if entities.is_empty() {
                    index.index.remove(&old_pos);
                }
            }
        }
        index.index.entry(pos.0).or_default().push(entity);
        index.reverse.insert(entity, pos.0);
    }
}

fn update_spatial_index_remove(
    mut removed: RemovedComponents<IconSmooth>,
    mut index: ResMut<SmoothSpatialIndex>,
    mut queue: ResMut<SmoothQueue>,
) {
    for entity in removed.read() {
        if let Some(old_pos) = index.reverse.remove(&entity) {
            dirty_neighbors(old_pos, &index, &mut queue);
            if let Some(entities) = index.index.get_mut(&old_pos) {
                entities.retain(|&e| e != entity);
                if entities.is_empty() {
                    index.index.remove(&old_pos);
                }
            }
        }
    }
}

fn queue_dirty_entities(
    changed_pos: Query<(Entity, &Position), (With<IconSmooth>, Changed<Position>)>,
    added_smooth: Query<(Entity, &Position), Added<IconSmooth>>,
    index: Res<SmoothSpatialIndex>,
    mut queue: ResMut<SmoothQueue>,
) {
    for (entity, pos) in &changed_pos {
        queue.dirty.insert(entity);
        dirty_neighbors(pos.0, &index, &mut queue);
    }
    for (entity, pos) in &added_smooth {
        queue.dirty.insert(entity);
        dirty_neighbors(pos.0, &index, &mut queue);
    }
}

fn dirty_neighbors(pos: IVec2, index: &SmoothSpatialIndex, queue: &mut SmoothQueue) {
    const OFFSETS: [IVec2; 8] = [
        IVec2::X,
        IVec2::NEG_X,
        IVec2::Y,
        IVec2::NEG_Y,
        IVec2::new(1, 1),
        IVec2::new(1, -1),
        IVec2::new(-1, 1),
        IVec2::new(-1, -1),
    ];
    for offset in OFFSETS {
        if let Some(entities) = index.index.get(&(pos + offset)) {
            for &entity in entities {
                queue.dirty.insert(entity);
            }
        }
    }
}

pub fn process_smooth_queue(
    mut queue: ResMut<SmoothQueue>,
    smooth_data: Query<(&Position, &IconSmooth)>,
    mut sprites: Query<&mut ComplexSprite>,
    index: Res<SmoothSpatialIndex>,
) {
    if queue.dirty.is_empty() {
        return;
    }
    queue.generation = queue.generation.wrapping_add(1);
    let r#gen = queue.generation;
    let dirty: Vec<Entity> = queue.dirty.drain().collect();
    for entity in dirty {
        if queue.processed.get(&entity) == Some(&r#gen) {
            continue;
        }
        let Ok((pos, smooth)) = smooth_data.get(entity) else {
            continue;
        };
        if !smooth.enabled {
            continue;
        }
        queue.processed.insert(entity, r#gen);
        let Ok(mut sprite) = sprites.get_mut(entity) else {
            continue;
        };
        match smooth.mode {
            IconSmoothingMode::Corners => {
                calculate_corners(smooth, pos.0, &mut sprite, &smooth_data, &index)
            }
            IconSmoothingMode::CardinalFlags => {
                calculate_cardinal(smooth, pos.0, &mut sprite, &smooth_data, &index)
            }
            IconSmoothingMode::Diagonal => {
                calculate_diagonal(smooth, pos.0, &mut sprite, &smooth_data, &index)
            }
            IconSmoothingMode::NoSprite => {}
        }
    }
}

fn is_compatible(a: &IconSmooth, b: &IconSmooth) -> bool {
    if !b.enabled {
        return false;
    }
    let Some(ref key_a) = a.smooth_key else {
        return false;
    };
    let Some(ref key_b) = b.smooth_key else {
        return false;
    };
    key_a == key_b || a.additional_keys.contains(key_b)
}

fn has_matching_neighbor(
    pos: IVec2,
    smooth: &IconSmooth,
    smooth_data: &Query<(&Position, &IconSmooth)>,
    index: &SmoothSpatialIndex,
) -> bool {
    let Some(entities) = index.index.get(&pos) else {
        return false;
    };
    for &entity in entities {
        let Ok((_, neighbor_smooth)) = smooth_data.get(entity) else {
            continue;
        };
        if is_compatible(smooth, neighbor_smooth) {
            return true;
        }
    }
    false
}

fn calculate_cardinal(
    smooth: &IconSmooth,
    pos: IVec2,
    sprite: &mut ComplexSprite,
    smooth_data: &Query<(&Position, &IconSmooth)>,
    index: &SmoothSpatialIndex,
) {
    let mut dirs = 0u8;
    if has_matching_neighbor(pos + IVec2::Y, smooth, smooth_data, index) {
        dirs |= 1;
    }
    if has_matching_neighbor(pos - IVec2::Y, smooth, smooth_data, index) {
        dirs |= 2;
    }
    if has_matching_neighbor(pos + IVec2::X, smooth, smooth_data, index) {
        dirs |= 4;
    }
    if has_matching_neighbor(pos - IVec2::X, smooth, smooth_data, index) {
        dirs |= 8;
    }
    sprite.state = Some(format!("{}{}", smooth.state_base, dirs));
}

fn calculate_diagonal(
    smooth: &IconSmooth,
    pos: IVec2,
    sprite: &mut ComplexSprite,
    smooth_data: &Query<(&Position, &IconSmooth)>,
    index: &SmoothSpatialIndex,
) {
    let matching = has_matching_neighbor(pos + IVec2::X, smooth, smooth_data, index)
        && has_matching_neighbor(pos + IVec2::new(1, -1), smooth, smooth_data, index)
        && has_matching_neighbor(pos - IVec2::Y, smooth, smooth_data, index);
    sprite.state = Some(format!(
        "{}{}",
        smooth.state_base,
        if matching { "1" } else { "0" }
    ));
}

fn calculate_corners(
    smooth: &IconSmooth,
    pos: IVec2,
    sprite: &mut ComplexSprite,
    smooth_data: &Query<(&Position, &IconSmooth)>,
    index: &SmoothSpatialIndex,
) {
    let n = has_matching_neighbor(pos + IVec2::Y, smooth, smooth_data, index);
    let s = has_matching_neighbor(pos - IVec2::Y, smooth, smooth_data, index);
    let e = has_matching_neighbor(pos + IVec2::X, smooth, smooth_data, index);
    let w = has_matching_neighbor(pos - IVec2::X, smooth, smooth_data, index);
    let ne = has_matching_neighbor(pos + IVec2::new(1, 1), smooth, smooth_data, index);
    let nw = has_matching_neighbor(pos + IVec2::new(-1, 1), smooth, smooth_data, index);
    let se = has_matching_neighbor(pos + IVec2::new(1, -1), smooth, smooth_data, index);
    let sw = has_matching_neighbor(pos + IVec2::new(-1, -1), smooth, smooth_data, index);

    let (mut c_ne, mut c_nw, mut c_se, mut c_sw) = (0u8, 0u8, 0u8, 0u8);
    const CCW: u8 = 1;
    const DIAG: u8 = 2;
    const CW: u8 = 4;

    if n {
        c_ne |= CCW;
        c_nw |= CW;
    }
    if ne {
        c_ne |= DIAG;
    }
    if e {
        c_ne |= CW;
        c_se |= CCW;
    }
    if se {
        c_se |= DIAG;
    }
    if s {
        c_se |= CW;
        c_sw |= CCW;
    }
    if sw {
        c_sw |= DIAG;
    }
    if w {
        c_sw |= CW;
        c_nw |= CCW;
    }
    if nw {
        c_nw |= DIAG;
    }

    sprite.layers = vec![
        create_corner_layer(&smooth.state_base, c_se, DirectionOffset::None),
        create_corner_layer(&smooth.state_base, c_ne, DirectionOffset::CounterClockWise),
        create_corner_layer(&smooth.state_base, c_nw, DirectionOffset::Flip),
        create_corner_layer(&smooth.state_base, c_sw, DirectionOffset::ClockWise),
    ];
}

fn create_corner_layer(base: &str, fill: u8, dir_offset: DirectionOffset) -> Layer {
    Layer {
        state: Some(format!("{}{}", base, fill)),
        visible: true,
        dir_offset,
        ..Default::default()
    }
}

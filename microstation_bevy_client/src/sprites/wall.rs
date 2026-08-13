use bevy::{platform::collections::HashMap, prelude::*};
use microstation_bevy_shared::map::Position;

const MASK_W: u8 = 0b0001;
const MASK_E: u8 = 0b0010;
const MASK_S: u8 = 0b0100;
const MASK_N: u8 = 0b1000;

// (offset, mask_for_self, mask_for_neighbor)
const NEIGHBORS: [(IVec2, u8, u8); 4] = [
    (IVec2::new(-1, 0), MASK_W, MASK_E),
    (IVec2::new(1, 0), MASK_E, MASK_W),
    (IVec2::new(0, -1), MASK_S, MASK_N),
    (IVec2::new(0, 1), MASK_N, MASK_S),
];

pub struct WallSpritePlugin;

impl Plugin for WallSpritePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<WallPositions>();
        app.init_resource::<WallCornerAtlasLayout>();
        app.add_observer(on_wall_added);
        app.add_observer(on_wall_removed);
        app.add_systems(Update, update_wall_sprites);
    }
}

#[derive(Resource, Default)]
pub struct WallPositions(HashMap<IVec2, Entity>);

#[derive(Resource)]
pub struct WallCornerAtlasLayout(Handle<TextureAtlasLayout>);

impl FromWorld for WallCornerAtlasLayout {
    fn from_world(world: &mut World) -> Self {
        let mut layouts = world.resource_mut::<Assets<TextureAtlasLayout>>();
        let layout = TextureAtlasLayout::from_grid(UVec2::new(16, 16), 2, 2, None, None);
        WallCornerAtlasLayout(layouts.add(layout))
    }
}

#[derive(Component)]
pub struct WallSprite {
    pub texture: Handle<Image>,
    pub mask: u8,
}

pub fn on_wall_added(
    trigger: On<Add, WallSprite>,
    positions: Query<&Position>,
    mut walls: Query<&mut WallSprite>,
    mut wall_positions: ResMut<WallPositions>,
) -> Result<()> {
    let pos = positions.get(trigger.entity)?.0;
    wall_positions.0.insert(pos, trigger.entity);

    for (offset, mask, opposite) in NEIGHBORS {
        if let Some(&neighbor) = wall_positions.0.get(&(pos + offset)) {
            let mut wall = walls.get_mut(trigger.entity)?;
            wall.mask |= mask;
            if let Ok(mut n) = walls.get_mut(neighbor) {
                n.mask |= opposite;
            }
        }
    }
    Ok(())
}

pub fn on_wall_removed(
    trigger: On<Remove, WallSprite>,
    positions: Query<&Position>,
    mut walls: Query<&mut WallSprite>,
    mut wall_positions: ResMut<WallPositions>,
) -> Result<()> {
    let pos = positions.get(trigger.entity)?.0;
    wall_positions.0.remove(&pos);

    for (offset, _, opposite) in NEIGHBORS {
        if let Some(&neighbor) = wall_positions.0.get(&(pos + offset)) {
            if let Ok(mut n) = walls.get_mut(neighbor) {
                n.mask &= !opposite;
            }
        }
    }
    Ok(())
}

pub fn update_wall_sprites(
    walls: Query<(Entity, &WallSprite), Changed<WallSprite>>,
    layout: Res<WallCornerAtlasLayout>,
    mut commands: Commands,
) {
    const OFFSET: f32 = 8.0;
    const QUARTER: f32 = std::f32::consts::FRAC_PI_2;

    for (entity, wall) in &walls {
        let m = wall.mask;
        // (vert_mask, horiz_mask, base_flip_x, base_flip_y, corner_offset)
        let corners = [
            (MASK_N, MASK_E, true, false, Vec2::new(OFFSET, OFFSET)),
            (MASK_N, MASK_W, false, false, Vec2::new(-OFFSET, OFFSET)),
            (MASK_S, MASK_E, true, true, Vec2::new(OFFSET, -OFFSET)),
            (MASK_S, MASK_W, false, true, Vec2::new(-OFFSET, -OFFSET)),
        ];

        commands
            .entity(entity)
            .despawn_children()
            .with_children(|parent| {
                for (vert, horiz, bx, by, off) in corners {
                    let (index, turns) = match ((m & vert != 0), (m & horiz != 0)) {
                        (false, false) => (0, 0),
                        (false, true) => (1, 0),
                        (true, false) => (1, 1),
                        (true, true) => (2, 0),
                    };
                    let (flip_x, flip_y) = if turns & 1 == 1 { (by, bx) } else { (bx, by) };

                    parent.spawn((
                        Sprite {
                            image: wall.texture.clone(),
                            texture_atlas: Some(TextureAtlas {
                                layout: layout.0.clone(),
                                index,
                            }),
                            flip_x,
                            flip_y,
                            ..default()
                        },
                        Transform {
                            translation: off.extend(0.1),
                            rotation: Quat::from_rotation_z(QUARTER * turns as f32),
                            ..default()
                        },
                        Visibility::default(),
                    ));
                }
            });
    }
}

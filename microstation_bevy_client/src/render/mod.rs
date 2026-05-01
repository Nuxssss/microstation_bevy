mod item;
mod icon_smoothing;

use crate::components::player::LocalPlayer;
use icon_smoothing::{IconSmoothPlugin, SmoothSystems};
use crate::rsi::{ErrorSprite, RsiRegistry};
use bevy::asset::Assets;
use bevy::prelude::*;
use microstation_bevy_shared::components::player::Player;
use microstation_bevy_shared::components::sprite::{ComplexSprite, DirectionOffset};
use microstation_bevy_shared::world::Position;
use std::f32::consts::{FRAC_PI_2, PI};

pub const TILE_SIZE: f32 = 32.0;

pub struct RenderPlugin;

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(IconSmoothPlugin);
        // ItemRenderPlugin удален, системы добавлены в общую цепочку ниже

        app.add_systems(Startup, spawn_camera);
        app.add_systems(
            Update,
            (
                sync_positions,          // 1. Position -> Transform (X, Y)
                sync_complex_sprite,     // 2. Слои + Z
                ApplyDeferred,           // 3. Применение команд
                apply_sprite_states,     // 4. Базовый рендер
                item::apply_item_sprites, // 5. Предмет-специфичный рендер
                sync_draw_depth,         // 6. Глубина (если нужно обновить)
                follow_local_player,     // 7. Камера
            )
                .chain()
                .after(SmoothSystems),       // Гарантия: после вычисления сглаживания
        );
        app.add_observer(on_player_added);
    }
}

pub fn sync_positions(
    mut query: Query<(&Position, &mut Transform), Or<(Added<Position>, Changed<Position>)>>,
) {
    for (pos, mut tf) in query.iter_mut() {
        tf.translation.x = pos.0.x as f32 * TILE_SIZE;
        tf.translation.y = pos.0.y as f32 * TILE_SIZE;
        tf.rotation = Quat::IDENTITY;
    }
}

pub fn sync_draw_depth(
    mut query: Query<(&ComplexSprite, &mut Transform)>,
) {
    for (cs, mut tf) in query.iter_mut() {
        tf.translation.z = cs.draw_depth.as_z();
    }
}

#[derive(Component)]
pub struct SpriteLayer {
    pub index: usize,
}

#[derive(Component, Clone, Debug)]
pub struct LayerRenderData {
    pub rsi_path: String,
    pub state: String,
    pub color: Color,
    pub visible: bool,
}

fn on_player_added(trigger: On<Add, Player>, mut commands: Commands) {
    commands.entity(trigger.entity).insert((
        Sprite {
            color: Color::srgb(0.3, 0.7, 1.0),
            custom_size: Some(Vec2::splat(TILE_SIZE)),
            ..default()
        },
        Transform::default(),
        Visibility::default(),
    ));
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn follow_local_player(
    p: Query<&Position, (With<LocalPlayer>, Changed<Position>)>,
    mut cam: Query<&mut Transform, With<Camera2d>>,
) {
    if let Ok(pos) = p.single() {
        if let Ok(mut t) = cam.single_mut() {
            t.translation.x = pos.0.x as f32 * TILE_SIZE;
            t.translation.y = pos.0.y as f32 * TILE_SIZE;
        }
    }
}

fn dir_to_quat(o: &DirectionOffset) -> Quat {
    match o {
        DirectionOffset::None => Quat::IDENTITY,
        DirectionOffset::ClockWise => Quat::from_rotation_z(-FRAC_PI_2),
        DirectionOffset::CounterClockWise => Quat::from_rotation_z(FRAC_PI_2),
        DirectionOffset::Flip => Quat::from_rotation_z(PI),
    }
}

fn sync_complex_sprite(
    mut commands: Commands,
    changed: Query<(Entity, &ComplexSprite), Or<(Added<ComplexSprite>, Changed<ComplexSprite>)>>,
    mut tf_q: Query<&mut Transform>,
) {
    for (e, cs) in &changed {
        if let Ok(mut t) = tf_q.get_mut(e) {
            t.translation.z = cs.draw_depth.as_z();
        }
        commands.entity(e).despawn_children().with_children(|p| {
            let base_rsi = cs.rsi_path.clone().unwrap_or_default();
            let base_state = cs.state.clone().unwrap_or_default();
            for (i, layer) in cs.layers.iter().enumerate() {
                let rsi = layer.rsi_path.clone().unwrap_or_else(|| base_rsi.clone());
                let state = layer.state.clone().unwrap_or_else(|| base_state.clone());
                p.spawn((
                    SpriteLayer { index: i },
                    LayerRenderData {
                        rsi_path: rsi,
                        state,
                        color: layer.color.unwrap_or(Color::WHITE),
                        visible: layer.visible,
                    },
                    Transform::from_rotation(dir_to_quat(&layer.dir_offset))
                        .with_translation(Vec3::new(0.0, 0.0, (i as f32 + 1.0) * 0.001)),
                    Visibility::Inherited,
                ));
            }
        });
    }
}

fn apply_sprite_states(
    base_q: Query<
        (Entity, &ComplexSprite),
        Or<(Added<ComplexSprite>, Changed<ComplexSprite>)>,
    >,
    layer_q: Query<
        (Entity, &LayerRenderData),
        Or<(Added<LayerRenderData>, Changed<LayerRenderData>)>,
    >,
    mut registry: ResMut<RsiRegistry>,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
    server: Res<AssetServer>,
    error: Res<ErrorSprite>,
    mut commands: Commands,
) {
    for (e, cs) in &base_q {
        if cs.layers.is_empty() {
            match &cs.rsi_path {
                Some(rsi) => {
                    let state = cs.state.clone().unwrap_or_else(|| {
                        registry.get_first_state(rsi).unwrap_or_default()
                    });
                    if state.is_empty() {
                        commands.entity(e).remove::<Sprite>().insert(Visibility::Hidden);
                        continue;
                    }
                    info!("[Render] Applying {} / {}", rsi, state);
                    assign_sprite(
                        e, rsi, &state,
                        cs.color.unwrap_or(Color::WHITE), cs.visible,
                        &mut registry, &mut layouts, &server, &error, &mut commands,
                    );
                }
                None => {
                    commands.entity(e).remove::<Sprite>().insert(Visibility::Hidden);
                }
            }
        }
    }

    for (e, ld) in &layer_q {
        let state = if ld.state.is_empty() {
            registry.get_first_state(&ld.rsi_path).unwrap_or_default()
        } else {
            ld.state.clone()
        };
        if state.is_empty() {
            commands.entity(e).remove::<Sprite>().insert(Visibility::Hidden);
            continue;
        }
        info!("[Render] Layer {} / {}", &ld.rsi_path, state);
        assign_sprite(
            e, &ld.rsi_path, &state, ld.color, ld.visible,
            &mut registry, &mut layouts, &server, &error, &mut commands,
        );
    }
}

fn assign_sprite(
    e: Entity,
    rsi: &str,
    state: &str,
    color: Color,
    visible: bool,
    registry: &mut RsiRegistry,
    layouts: &mut Assets<TextureAtlasLayout>,
    server: &AssetServer,
    error: &ErrorSprite,
    commands: &mut Commands,
) {
    if !visible || rsi.is_empty() || state.is_empty() {
        commands.entity(e).remove::<Sprite>().insert(Visibility::Hidden);
        return;
    }

    if let Some(h) = registry.get_handles(rsi, state, layouts, server) {
        commands.entity(e).insert((
            Sprite {
                image: h.image,
                texture_atlas: Some(TextureAtlas {
                    layout: h.layout,
                    index: 0,
                }),
                color,
                ..default()
            },
            Visibility::Inherited,
        ));
    } else {
        warn!("[Render] RSI not found: {rsi}/{state}");
        commands.entity(e).insert((
            Sprite {
                image: error.image.clone(),
                texture_atlas: Some(TextureAtlas {
                    layout: error.layout.clone(),
                    index: 0,
                }),
                color: Color::srgb(1.0, 0.0, 1.0),
                ..default()
            },
            Visibility::Inherited,
        ));
    }
}
use crate::rsi::{ErrorSprite, RsiRegistry};
use bevy::asset::Assets;
use bevy::prelude::*;
use microstation_bevy_shared::components::item::Item;
use microstation_bevy_shared::components::sprite::ComplexSprite;

/// Применяет текстуры к предметам.
/// Координаты (X, Y) управляются sync_positions, Z — sync_draw_depth/sync_complex_sprite.
/// Эта система меняет ТОЛЬКО Sprite и Visibility.
pub fn apply_item_sprites(
    mut commands: Commands,
    q: Query<(Entity, &ComplexSprite), With<Item>>,
    mut registry: ResMut<RsiRegistry>,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
    server: Res<AssetServer>,
    error: Res<ErrorSprite>,
) {
    for (e, cs) in &q {
        let rsi = cs.rsi_path.as_deref().unwrap_or("");
        let state = cs.state.as_deref().unwrap_or("");

        if rsi == "" || state == "" {
            commands
                .entity(e)
                .remove::<Sprite>()
                .insert(Visibility::Hidden);
            continue;
        }

        if let Some(h) = registry.get_handles(rsi, state, &mut layouts, &server) {
            commands.entity(e).insert((
                Sprite {
                    image: h.image,
                    texture_atlas: Some(TextureAtlas {
                        layout: h.layout,
                        index: 0,
                    }),
                    color: cs.color.unwrap_or(Color::WHITE),
                    ..default()
                },
                Visibility::Inherited,
            ));
        } else {
            warn!("[ITEM] RSI_FAIL: {rsi}/{state}");
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
}

use bevy::prelude::*;

#[derive(Component)]
pub struct WallSprite {
    sprite_path: String,
    mask: u8,
}

pub fn update_wall_masks(
    mut walls: Query<(&mut WallSprite, &Transform)>,
    wall_positions: Query<&Transform, With<WallSprite>>,
) {
    for (mut wall_sprite, wall_transform) in walls.iter_mut() {
        let mut mask = 0u8;

        let wall_pos = wall_transform.translation.truncate();

        // Check for neighboring walls in the four cardinal directions
        let neighbors = [
            (IVec2::new(0, 1), 1),  // North
            (IVec2::new(1, 0), 2),  // East
            (IVec2::new(0, -1), 4), // South
            (IVec2::new(-1, 0), 8), // West
        ];

        for (offset, bit) in neighbors.iter() {
            let neighbor_pos = wall_pos + offset.as_vec2();
            if wall_positions
                .iter()
                .any(|t| t.translation.truncate() == neighbor_pos)
            {
                mask |= bit;
            }
        }

        wall_sprite.mask = mask;
    }
}

// pub fn update_wall_sprites(
//     mut walls: Query<(&WallSprite, &mut Sprite)>,
//     asset_server: Res<AssetServer>,
// ) {
//     for (wall_sprite, mut sprite) in walls.iter_mut() {
//         let sprite_path = match wall_sprite.mask {};

//         sprite.image = asset_server.load(sprite_path);
//     }
// }

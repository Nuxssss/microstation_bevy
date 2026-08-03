use bevy::prelude::*;
use bevy_replicon::prelude::*;
use microstation_bevy_shared::map::tile::TileChunk;
use microstation_bevy_shared::map::{CHUNK_TILES_COUNT, tile_registry::TileRegistry};
use microstation_bevy_shared::prototype::PrototypeManager;

use crate::map::occupancy::OccupancyChunk;
use crate::prototype::spawn_prototype;

// пока карты нет, всё будет просто генерироваться

pub fn generate_test_map(
    mut commands: Commands,
    tile_registry: ResMut<TileRegistry>,
) -> Result<()> {
    // регистрируем нужные тайлы под фиксированные id
    // let floor_id = tile_registry.register("floor_steel"); // добавить метод register в TileRegistry,
    // который и добавляет запись, и шлёт diff на клиентов
    let floor_id = tile_registry.get_idx("floor_steel")?;

    const MAP_CHUNKS: i32 = 3; // 3x3 чанка для теста

    for cy in 0..MAP_CHUNKS {
        for cx in 0..MAP_CHUNKS {
            let mut tiles = [0u8; CHUNK_TILES_COUNT];
            tiles.fill(floor_id); // сплошной пол

            commands.spawn((
                TileChunk {
                    tiles,
                    crd: IVec2::new(cx, cy),
                },
                OccupancyChunk::default(),
                Name::new("Chunk"),
                Replicated,
            ));
        }
    }
    debug!("map generated");
    Ok(())
}

pub fn spawn_test_walls(mut commands: Commands, prototypes: Res<PrototypeManager>) -> Result<()> {
    let mut walls = Vec::new();

    // прямая линия
    for x in 0..5 {
        walls.push(IVec2::new(x, 2));
    }

    // L-образная
    for y in 4..8 {
        walls.push(IVec2::new(0, y));
    }
    for x in 1..4 {
        walls.push(IVec2::new(x, 7));
    }

    // коробка (замкнутый периметр)
    for x in 6..10 {
        walls.push(IVec2::new(x, 4));
        walls.push(IVec2::new(x, 8));
    }
    for y in 4..=8 {
        walls.push(IVec2::new(6, y));
        walls.push(IVec2::new(9, y));
    }

    // зигзаг
    let mut zx = 0;
    for zy in 10..15 {
        walls.push(IVec2::new(zx, zy));
        zx += if zy % 2 == 0 { 1 } else { -1 };
    }

    for pos in walls {
        spawn_prototype(&mut commands, &prototypes, "steel_wall", pos)?;
    }
    Ok(())
}

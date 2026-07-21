use bevy::prelude::*;
use bevy_replicon::prelude::*;
use microstation_bevy_shared::grid::{CHUNK_TILES_COUNT, Chunk, tile_registry::TileRegistry};

pub fn generate_test_map(mut commands: Commands, tile_registry: ResMut<TileRegistry>) {
    // регистрируем нужные тайлы под фиксированные id
    // let floor_id = tile_registry.register("floor_steel"); // добавить метод register в TileRegistry,
    // который и добавляет запись, и шлёт diff на клиентов
    let Some(floor_id) = tile_registry.id_of("floor_steel") else {
        error!("no floor found");
        return;
    };

    const MAP_CHUNKS: i32 = 3; // 3x3 чанка для теста

    for cy in 0..MAP_CHUNKS {
        for cx in 0..MAP_CHUNKS {
            let mut tiles = [0u8; CHUNK_TILES_COUNT];
            tiles.fill(floor_id); // сплошной пол

            commands.spawn((
                Chunk {
                    tiles,
                    crd: IVec2::new(cx, cy),
                },
                Replicated,
            ));
        }
    }
    debug!("map generated");
}

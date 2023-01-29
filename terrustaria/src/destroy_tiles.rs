use std::thread::sleep;
use bevy::{prelude::*, math::Vec4Swizzles};
use bevy_ecs_tilemap::prelude::*;
use crate::cursor::CursorPos;
use crate::map::WithColliders;


pub fn destroy_tile_after_click(
    mut commands: Commands,
    cursor_pos: Res<CursorPos>,
    mut tilemap_q: Query<(
        &TilemapSize,
        &TilemapGridSize,
        &TilemapType,
        &mut TileStorage,
        &Transform,
    ), With<WithColliders>>,
    mut tile_q: Query<&mut TileTextureIndex>,
    mouse: Res<Input<MouseButton>>,
) {

    for (map_size, grid_size, map_type, mut tile_storage, map_transform) in tilemap_q.iter_mut() {
        let cursor_pos: Vec3 = cursor_pos.0;
        let cursor_in_map_pos: Vec2 = {
            let cursor_pos = Vec4::from((cursor_pos, 1.0));
            let cursor_in_map_pos = map_transform.compute_matrix().inverse() * cursor_pos;
            cursor_in_map_pos.xy()
        };

        if !mouse.pressed(MouseButton::Left) {
            continue;
        }

        if let Some(tile_pos) =
            TilePos::from_world_pos(&cursor_in_map_pos, map_size, grid_size, map_type)
        {
            if let Some(tile_entity) = tile_storage.get(&tile_pos) {
                if let Ok(mut tile_texture) = tile_q.get_mut(tile_entity) {
                    if tile_texture.0 % 5 == 4 {
                        commands.entity(tile_entity).despawn_recursive();
                        tile_storage.remove(&tile_pos);
                    }
                    else {
                        tile_texture.0 += 1;
                        sleep(std::time::Duration::from_millis(100));
                    }
                }
            }
        }
    }
}

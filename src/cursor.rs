use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

pub struct CursorPlugin;
impl Plugin for CursorPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CursorPos>();
        app.init_resource::<SelectedTile>();
        app.init_resource::<SelectedTilePos>();
        app.add_systems(Update, (update_cursor_pos, hover_tile));
    }
}

#[derive(Resource, Default)]
pub struct SelectedTile(pub Option<Entity>);

#[derive(Resource)]
pub struct CursorPos(Vec2);

impl Default for CursorPos {
    fn default() -> Self {
        Self(Vec2::new(-1000.0, -1000.0))
    }
}

#[derive(Resource)]
#[derive(Default)]
pub struct SelectedTilePos(Option<Vec2>);


pub fn update_cursor_pos(
    camera_q: Query<(&GlobalTransform, &Camera)>,
    mut cursor_moved_events: EventReader<CursorMoved>,
    mut cursor_pos: ResMut<CursorPos>,
) {
    for cursor_moved in cursor_moved_events.read() {
        // To get the mouse's world position, we have to transform its window position by
        // any transforms on the camera. This is done by projecting the cursor position into
        // camera space (world space).
        for (cam_t, cam) in camera_q.iter() {
            if let Some(pos) = cam.viewport_to_world_2d(cam_t, cursor_moved.position) {
                *cursor_pos = CursorPos(pos);
            }
        }
    }
}

fn hover_tile(
    cursor_pos: Res<CursorPos>,
    mut tilemap_q: Query<(
        &TilemapSize,
        &TilemapGridSize,
        &TilemapType,
        &mut TileStorage,
        &Transform,
    )>,
    mut tile_selected: ResMut<SelectedTile>,
    mut tile_pos_selected: ResMut<SelectedTilePos>,
) {
    for (map_size, grid_size, map_type, tile_storage, map_transform) in tilemap_q.iter_mut() {
        let cursor_pos: Vec2 = cursor_pos.0;

        let cursor_in_map_pos: Vec2 = {
            // Extend the cursor_pos vec3 by 0.0 and 1.0
            let cursor_pos = Vec4::from((cursor_pos, 0.0, 1.0));
            let cursor_in_map_pos = map_transform.compute_matrix().inverse() * cursor_pos;
            cursor_in_map_pos.xy()
        };

        if let Some(tile_pos) =
            TilePos::from_world_pos(&cursor_in_map_pos, map_size, grid_size, map_type)
        {
            if let Some(tile_entity) = tile_storage.get(&tile_pos) {
                tile_pos_selected.0 = Some(tile_pos.into());
                tile_selected.0 = Some(tile_entity);
            } else {
                tile_selected.0 = None;
                tile_pos_selected.0 = None;
            }
        }
    }
}

// fn print_terrain_state(selected_tile: Res<SelectedTile>, query: Query<&Terrain>) {
//     if let Some(selected_tile) = selected_tile.0 {
//         if let Ok(terrain) = query.get(selected_tile) {
//             println!("Selected tile terrain: {:?}", terrain.terrain_type);
//         }
//     }
// }

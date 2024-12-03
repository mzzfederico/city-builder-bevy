use bevy::prelude::*;
use bevy_common_assets::json::JsonAssetPlugin;
use bevy_ecs_tilemap::prelude::*;

use crate::AppState;

pub struct GridPlugin;
impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((JsonAssetPlugin::<Level>::new(&["level.json"]),))
            .init_state::<AppState>()
            .add_systems(Startup, setup.before(create_terrain_tilemap))
            .add_systems(
                Update,
                create_terrain_tilemap.run_if(in_state(AppState::Loading)),
            );
    }
}

pub const TILE_W: u32 = 64;
pub const TILE_H: u32 = 32;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let level = asset_server.load("test.level.json");
    commands.insert_resource(CurrentLevel(level));
}

fn create_terrain_tilemap(
    mut state: ResMut<NextState<AppState>>,
    asset_server: Res<AssetServer>,
    current_level: Res<CurrentLevel>,
    levels: ResMut<Assets<Level>>,
    mut commands: Commands,
) {
    let texture_handle: Handle<Image> = asset_server.load("terrain.png");

    if let Some(level) = levels.get(current_level.0.id()) {
        let map_size = TilemapSize {
            x: level.width,
            y: level.height,
        };

        let tilemap_entity = commands.spawn_empty().id();
        let mut tile_storage = TileStorage::empty(map_size);

        let tiles = level
            .map
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| match c {
                        'G' => Terrain::create_grass(),
                        'R' => Terrain::create_water(),
                        // 'M' => Terrain::create_coast(),
                        _ => Terrain::default(),
                    })
                    .collect::<Vec<Terrain>>()
            })
            .collect::<Vec<Vec<Terrain>>>();

        for x in 0..tiles.len() {
            for y in 0..tiles[0].len() {
                let tile_pos = TilePos {
                    x: x as u32,
                    y: y as u32,
                };

                let tile_bundle = TileBundle {
                    position: tile_pos,
                    tilemap_id: TilemapId(tilemap_entity),
                    texture_index: tiles[x][y].terrain_type.into(),
                    ..Default::default()
                };

                let tile_entity = commands.spawn((tiles[x][y].clone(), tile_bundle)).id();

                tile_storage.set(&tile_pos, tile_entity);
            }
        }

        let tile_size = TilemapTileSize {
            x: TILE_W as f32,
            y: TILE_H as f32,
        };
        let grid_size: TilemapGridSize = tile_size.into();
        let map_type = TilemapType::Isometric(IsoCoordSystem::Diamond);

        commands.entity(tilemap_entity).insert(TilemapBundle {
            grid_size,
            size: map_size,
            storage: tile_storage,
            tile_size,
            texture: TilemapTexture::Single(texture_handle),
            map_type,
            transform: get_tilemap_center_transform(&map_size, &grid_size, &map_type, 0.0),
            ..Default::default()
        });

        state.set(AppState::Level);
    }
}

#[derive(serde::Deserialize, bevy::asset::Asset, bevy::reflect::TypePath, Debug)]
pub struct Level {
    pub map: String,
    pub width: u32,
    pub height: u32,
}

#[derive(Resource)]
pub struct CurrentLevel(pub Handle<Level>);

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum TerrainType {
    Grass = 0,
    Water = 1,
    // Mountain = 2,
}

impl Into<TileTextureIndex> for TerrainType {
    fn into(self) -> TileTextureIndex {
        match self {
            x => TileTextureIndex(x as u32),
        }
    }
}

#[derive(Component, Clone)]
pub struct Terrain {
    pub terrain_type: TerrainType,
    pub is_buildable: bool,
    // pub is_coast: bool,
    // pub building_entity: Option<Entity>,
    // pub vegetation_entity: Option<Entity>,
}

impl Default for Terrain {
    fn default() -> Self {
        Self {
            terrain_type: TerrainType::Grass,
            is_buildable: true,
            // is_coast: false,
            // building_entity: None,
            // vegetation_entity: None,
        }
    }
}

impl Terrain {
    pub fn create_grass() -> Self {
        Self {
            terrain_type: TerrainType::Grass,
            is_buildable: true,
            // is_coast: false,
            // building_entity: None,
            // vegetation_entity: None,
        }
    }

    //     pub fn create_coast() -> Self {
    //         Self {
    // // terrain_type: TerrainType::Grass,
    // // is_buildable: true,
    // // is_coast: true,
    // // building_entity: None,
    // // vegetation_entity: None,
    //         }
    //     }

    pub fn create_water() -> Self {
        Self {
            terrain_type: TerrainType::Water,
            is_buildable: false,
            // is_coast: false,
            // building_entity: None,
            // vegetation_entity: None,
        }
    }
}

use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

use crate::cli::Args;

pub struct GridPlugin;

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, create_terrain_tilemap);
    }
}

const MAP_SIDE_LENGTH: u32 = 32;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum TerrainType {
    Grass = 0,
    Water = 1,
    Mountain = 2,
}

#[derive(Component)]
pub struct Terrain {
    pub terrain_type: TerrainType,
    pub is_buildable: bool,
    pub is_coast: bool,
    pub building_entity: Option<Entity>,
    pub vegetation_entity: Option<Entity>,
}

impl Default for Terrain {
    fn default() -> Self {
        Self {
            terrain_type: TerrainType::Grass,
            is_buildable: true,
            is_coast: false,
            building_entity: None,
            vegetation_entity: None,
        }
    }
}

impl Terrain {
    pub fn create_grass() -> Self {
        Self {
            terrain_type: TerrainType::Grass,
            is_buildable: true,
            is_coast: false,
            building_entity: None,
            vegetation_entity: None,
        }
    }

    pub fn create_coast() -> Self {
        Self {
            terrain_type: TerrainType::Grass,
            is_buildable: true,
            is_coast: true,
            building_entity: None,
            vegetation_entity: None,
        }
    }

    pub fn create_water() -> Self {
        Self {
            terrain_type: TerrainType::Water,
            is_buildable: false,
            is_coast: false,
            building_entity: None,
            vegetation_entity: None,
        }
    }
}

fn create_terrain_tilemap(mut commands: Commands, asset_server: Res<AssetServer>, args: Res<Args>) {
    let texture_handle: Handle<Image> = asset_server.load("iso_color.png");

    let map_size = TilemapSize {
        x: args.tilemap_size.unwrap_or(MAP_SIDE_LENGTH),
        y: args.tilemap_size.unwrap_or(MAP_SIDE_LENGTH),
    };

    let tilemap_entity = commands.spawn_empty().id();
    let mut tile_storage = TileStorage::empty(map_size);

    for x in 0..map_size.x {
        for y in 0..map_size.y {
            let tile_pos = TilePos { x, y };
            let tile_bundle = TileBundle {
                position: tile_pos,
                tilemap_id: TilemapId(tilemap_entity),
                ..Default::default()
            };

            let tile_entity = commands.spawn((Terrain::create_grass(), tile_bundle)).id();

            tile_storage.set(&tile_pos, tile_entity);
        }
    }

    let tile_size = TilemapTileSize { x: 64.0, y: 32.0 };
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
}

use bevy::input::mouse::MouseButtonInput;
use bevy::prelude::*;
use bevy_ecs_tilemap::map::TilemapGridSize;
use bevy_ecs_tilemap::map::TilemapType;
use bevy_ecs_tilemap::tiles::TilePos;

use crate::cursor::SelectedTile;
use crate::grid::Level;
use crate::grid::Terrain;
use crate::grid::TILE_H;
use crate::grid::TILE_W;
use crate::AppState;

pub struct BuildingPlugin;
impl Plugin for BuildingPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<BuildingMode>();

        app.add_systems(Update, enable_building.run_if(in_state(AppState::Level)));

        app.add_systems(
            Update,
            (
                update_building_cursor,
                check_buildable_status,
                construct_building,
            )
                .run_if(in_state(BuildingMode::On)),
        );
    }
}

fn enable_building(
    keys: Res<ButtonInput<KeyCode>>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut building_mode: ResMut<NextState<BuildingMode>>,
    template_q: Query<Entity, With<BuildingTemplateMarker>>,
) {
    if keys.just_pressed(KeyCode::KeyB) {
        building_mode.set(BuildingMode::On);

        template_q.iter().for_each(|e| {
            commands.entity(e).despawn();
        });

        let texture = asset_server.load("buildings/theatre.png");

        commands.spawn((
            BuildingTemplateMarker,
            BuildingTemplateCanBuild(false),
            BuildingSize((2, 2)),
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgba(0.5, 0.5, 0.5, 0.7),
                    ..default()
                },
                texture: texture.clone(),
                transform: Transform::from_xyz(100000000., 100000000., 2.),
                ..default()
            },
        ));
    } else if keys.just_pressed(KeyCode::Escape) {
        building_mode.set(BuildingMode::Off);

        template_q.iter().for_each(|e| {
            commands.entity(e).despawn();
        });
    }
}

// #[derive(Resource)]
// pub struct SelectedBuilding(Option<Entity>);

fn update_building_cursor(
    selected_tile: Res<SelectedTile>,
    tile_q: Query<&mut TilePos>,
    tilemap_q: Query<(&TilemapType, &TilemapGridSize)>,
    current_level: Res<crate::grid::CurrentLevel>,
    levels: Res<Assets<Level>>,
    mut template_q: Query<
        (&mut Transform, &mut Sprite, &mut BuildingTemplateCanBuild),
        With<BuildingTemplateMarker>,
    >,
) {
    if selected_tile.0.as_ref().is_none() || template_q.iter_mut().count() == 0 {
        return;
    }

    let selected_tile = selected_tile.0.as_ref().unwrap();
    let current_level = levels.get(current_level.0.id()).unwrap();

    for (map_type, grid_size) in tilemap_q.iter() {
        let tile_pos = tile_q.get(*selected_tile).unwrap();
        let tile_center = tile_pos.center_in_world(grid_size, map_type).extend(1.0);
        for (mut transform, mut sprite, can_build) in template_q.iter_mut() {
            transform.translation.x =
                tile_center.x - (TILE_W * ((current_level.width / 2) - 1)) as f32;
            transform.translation.y = tile_center.y + (TILE_H / 2) as f32;
            transform.translation.z = 3.;

            if can_build.0 {
                sprite.color = Color::srgba(0.5, 1.0, 0.5, 0.7);
            } else {
                sprite.color = Color::srgba(1.0, 0.5, 0.5, 0.7);
            }
        }
    }
}

/**
* Check if a position is in a region starting at start_x, start_y with width and height
*/
fn position_is_in_region(start: &TilePos, width: u32, height: u32, pos: &TilePos) -> bool {
    pos.x >= start.x && pos.x < start.x + width && pos.y >= start.y && pos.y < start.y + height
}

fn check_buildable_status(
    resources: Res<crate::resources::GlobalResources>,
    selected_tile: Res<SelectedTile>,
    mut template_q: Query<
        (&mut BuildingTemplateCanBuild, &BuildingSize),
        With<BuildingTemplateMarker>,
    >,
    tile_q: Query<(&TilePos, &Terrain)>,
) {
    template_q
        .iter_mut()
        .for_each(|(mut can_build, building_size)| {
            if resources.gold < 100 {
                return can_build.0 = false;
            }

            if let Some(selected_tile) = selected_tile.0 {
                let (tile_pos, _tt) = tile_q.get(selected_tile).unwrap();
                let (tx, ty) = building_size.0;

                let possible_tiles: Vec<bool> = tile_q
                    .iter()
                    .filter(|(pos, _terr)| position_is_in_region(tile_pos, tx, ty, pos))
                    .map(|(_pos, terr)| terr.is_buildable)
                    .collect();

                let all_buildable = possible_tiles.iter().all(|&x| x);

                if all_buildable && (possible_tiles.len() as u32) == tx * ty {
                    return can_build.0 = true;
                } else {
                    return can_build.0 = false;
                }
            } else {
                return can_build.0 = false;
            }
        });
}

fn construct_building(
    asset_server: Res<AssetServer>,
    mut mouse: EventReader<MouseButtonInput>,
    mut commands: Commands,
    template_q: Query<Entity, With<BuildingTemplateMarker>>,
    transform_q: Query<&mut Transform, With<BuildingTemplateMarker>>,
    mut resources: ResMut<crate::resources::GlobalResources>,
    mut building_mode: ResMut<NextState<BuildingMode>>,
) {
    mouse.read().for_each(|event| {
        if event.button == MouseButton::Left && event.state.is_pressed() {
            let translation = transform_q.iter().last().unwrap();

            commands.spawn((
                Building(BuildingType::Theatre),
                BuildingSize((2, 2)),
                SpriteBundle {
                    texture: asset_server.load("buildings/theatre.png"),
                    transform: Transform::from(*translation),
                    ..default()
                },
            ));

            resources.gold -= 100;
        }

        building_mode.set(BuildingMode::Off);

        template_q.iter().for_each(|e| {
            commands.entity(e).despawn();
        });
    });
}

pub enum BuildingType {
    Theatre,
    //    Amphiteatre,
    //    Colosseum,
}

#[derive(Component)]
pub struct Building(BuildingType);
#[derive(Component)]
pub struct BuildingSize((u32, u32));

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum BuildingMode {
    #[default]
    Off,
    On,
}

#[derive(Component)]
pub struct BuildingTemplateMarker;

#[derive(Component)]
pub struct BuildingTemplateCanBuild(bool);

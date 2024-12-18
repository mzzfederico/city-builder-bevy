pub mod bundle;
pub mod components;

use bevy::input::mouse::MouseButtonInput;
use bevy::prelude::*;
use bevy_ecs_tilemap::map::TilemapGridSize;
use bevy_ecs_tilemap::map::TilemapType;
use bevy_ecs_tilemap::tiles::TilePos;
use bundle::BuildingMarkerBundle;
use components::Building;
use components::BuildingTemplateMarker;
use components::BuildingType;
use components::CanBuild;

use crate::building::bundle::BuildingBundle;
use crate::building::components::CoveringTiles;

use crate::cursor::SelectedTile;
use crate::grid::Level;
use crate::grid::Occupied;
use crate::grid::Terrain;
use crate::grid::TILE_H;
use crate::grid::TILE_W;
use crate::time::GameTimer;
use crate::time::TimeState;
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

        app.add_systems(
            FixedUpdate,
            (pay_wages).run_if(in_state(TimeState::Running)),
        );
    }
}

fn pay_wages(
    time: Res<Time>,
    mut timer: ResMut<GameTimer>,
    mut resources: ResMut<crate::resources::GlobalResources>,
    q: Query<&BuildingType, With<Building>>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        for building in q.iter() {
            resources.gold -= (building.occupation() * 30) as i32;
        }
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

        commands.spawn(BuildingMarkerBundle::theatre(asset_server));
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
        (&mut Transform, &mut Sprite, &mut CanBuild),
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
        for (mut transform, mut sprite, can_build) in &mut template_q {
            transform.translation.x =
                tile_center.x - (TILE_W * ((current_level.width / 2) - 1)) as f32;
            transform.translation.y = tile_center.y + (TILE_H / 2) as f32;
            transform.translation.z = 3.;

            if can_build.0 {
                sprite.color = BuildableColor::Green.into();
            } else {
                sprite.color = BuildableColor::Red.into();
            }
        }
    }
}

/**
* Check if a position is in a region starting at `start_x`, `start_y` with width and height
*/
fn position_is_in_region(start: &TilePos, width: u32, height: u32, pos: &TilePos) -> bool {
    pos.x >= start.x && pos.x < start.x + width && pos.y >= start.y && pos.y < start.y + height
}

fn check_buildable_status(
    resources: Res<crate::resources::GlobalResources>,
    selected_tile: Res<SelectedTile>,
    mut template_q: Query<
        (&BuildingType, &mut CanBuild, &mut CoveringTiles),
        (With<BuildingTemplateMarker>, Without<Building>),
    >,
    tile_q: Query<(Entity, &TilePos, &Terrain, &Occupied)>,
) {
    template_q
        .iter_mut()
        .for_each(|(building_type, mut can_build, mut possible_tiles)| {
            if resources.gold < building_type.cost() as i32 {
                return can_build.0 = false;
            }

            if let Some(selected_tile) = selected_tile.0 {
                let (_, tile_pos, _, _) = tile_q.get(selected_tile).unwrap();
                let (tx, ty) = building_type.size();

                possible_tiles.0 = tile_q
                    .iter()
                    .filter(|(_, pos, terr, occupied)| {
                        position_is_in_region(tile_pos, tx, ty, pos)
                            && terr.is_buildable
                            && occupied.0.is_none()
                    })
                    .map(|(entity, _, _, _)| entity)
                    .collect();

                can_build.0 = (possible_tiles.0.len() as u32) == tx * ty;
            } else {
                can_build.0 = false;
            }
        });
}

fn construct_building(
    mut mouse: EventReader<MouseButtonInput>,
    mut commands: Commands,
    mut resources: ResMut<crate::resources::GlobalResources>,
    mut building_mode: ResMut<NextState<BuildingMode>>,
    asset_server: Res<AssetServer>,
    marker_entity_q: Query<Entity, With<BuildingTemplateMarker>>,
    marker_components_q: Query<
        (&BuildingType, &Transform, &CoveringTiles, &CanBuild),
        With<BuildingTemplateMarker>,
    >,
) {
    mouse.read().for_each(|event| {
        if event.button == MouseButton::Left && event.state.is_pressed() {
            marker_components_q.iter().for_each(
                |(building_type, transform, covering_tiles, can_build)| {
                    if can_build.0 {
                        let new_building_entity = commands
                            .spawn(BuildingBundle::build(
                                *building_type,
                                transform.translation,
                                &asset_server,
                            ))
                            .id();

                        resources.gold -= building_type.cost() as i32;
                        building_mode.set(BuildingMode::Off);

                        covering_tiles.0.iter().for_each(|e| {
                            commands
                                .entity(*e)
                                .remove::<Occupied>()
                                .insert(Occupied(Some(new_building_entity)));
                        });

                        marker_entity_q.iter().for_each(|e| {
                            commands.entity(e).despawn();
                        });
                    }
                },
            );
        }
    });
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum BuildingMode {
    #[default]
    Off,
    On,
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash)]
pub enum BuildableColor {
    #[default]
    Green,
    Red,
}

impl From<BuildableColor> for Color {
    fn from(bc: BuildableColor) -> Self {
        match bc {
            BuildableColor::Green => Color::srgba(0., 0.5, 0., 0.7),
            BuildableColor::Red => Color::srgba(0.5, 0., 0., 0.7),
        }
    }
}

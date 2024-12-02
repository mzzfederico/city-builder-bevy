use bevy::ecs::observer::TriggerTargets;
use bevy::input::mouse::MouseButtonInput;
use bevy::prelude::*;
use bevy_ecs_tilemap::map::TilemapGridSize;
use bevy_ecs_tilemap::map::TilemapType;
use bevy_ecs_tilemap::tiles::TilePos;

pub struct BuildingPlugin;
impl Plugin for BuildingPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<BuildingEnabled>();
        app.init_resource::<BuildingTemplate>();

        app.add_systems(Update, enable_building);
        app.add_systems(Update, update_building_template);
        app.add_systems(Update, spawn_building_template);
        app.add_systems(Update, place_down_building);
    }
}

#[derive(Resource, Default)]
pub struct BuildingEnabled(bool);

fn enable_building(keys: Res<ButtonInput<KeyCode>>, mut building_enabled: ResMut<BuildingEnabled>) {
    if keys.just_pressed(KeyCode::KeyB) {
        building_enabled.0 = !building_enabled.0;
    } else if keys.just_pressed(KeyCode::Escape) {
        building_enabled.0 = false;
    }
}

#[derive(Resource, Default)]
pub struct BuildingTemplate(Option<Entity>);

#[derive(Component)]
pub struct BuildingTemplateMarker;

fn spawn_building_template(
    is_building_enabled: Res<BuildingEnabled>,
    asset_server: Res<AssetServer>,
    mut template: ResMut<BuildingTemplate>,
    mut commands: Commands,
) {
    if is_building_enabled.0 {
        if template.0.is_none() {
            let texture = asset_server.load("buildings/theatre.png");
            
            let entity = commands
                .spawn((
                    BuildingTemplateMarker,
                    SpriteBundle {
                        sprite: Sprite {
                            color: Color::srgba(0.0, 0.0, 0.0, 0.7),
                            ..default()
                        },
                        texture: texture.clone(),
                        transform: Transform::from_xyz(0., 0., 2.),
                        ..default()
                    }
                ))
                .id();

            template.0 = Some(entity);
        }
    } else if let Some(entity) = template.0 {
        commands.entity(entity).despawn();
        template.0 = None;
    }
}

fn place_down_building(
    asset_server: Res<AssetServer>,
    template: ResMut<BuildingTemplate>,
    mut mouse: EventReader<MouseButtonInput>,
    mut is_building_enabled: ResMut<BuildingEnabled>,
    mut commands: Commands,
    mut transform_q: Query<&mut Transform, With<BuildingTemplateMarker>>,
    mut resources: ResMut<crate::resources::GlobalResources>,
) {
    if is_building_enabled.0 & template.0.is_some() {
        mouse.read().for_each(|event| {
            if event.button == MouseButton::Left && event.state.is_pressed() {
                let template = template.0.unwrap();
                let translation = transform_q.get_mut(template).unwrap().translation;

                commands.spawn((
                    Building(BuildingType::Theatre),
                    SpriteBundle {
                        texture: asset_server.load("buildings/theatre.png"),
                        transform: Transform::from_xyz(translation.x, translation.y, 1.),
                        ..default()
                    },
                ));

                resources.gold -= 100;

                is_building_enabled.0 = false;
            }
        });
    }
}

// #[derive(Resource)]
// pub struct SelectedBuilding(Option<Entity>);
use crate::cursor::SelectedTile;
fn update_building_template(
    is_building_enabled: Res<BuildingEnabled>,
    selected_tile: Res<SelectedTile>,
    tile_q: Query<&mut TilePos>,
    tilemap_q: Query<(&TilemapType, &TilemapGridSize)>,
    mut template_q: Query<&mut Transform, With<BuildingTemplateMarker>>,
) {
    if !is_building_enabled.0 {
        return;
    }

    if let Some(selected_tile) = &selected_tile.0 {
        for (map_type, grid_size) in tilemap_q.iter() {
            let tile_pos = tile_q.get(*selected_tile).unwrap();
            let tile_center = tile_pos.center_in_world(grid_size, map_type).extend(1.0);
            
            println!("Tile center: {:?}", grid_size);

            for mut transform in template_q.iter_mut() {
                transform.translation.x = tile_center.x - 960.0;
                transform.translation.y = tile_center.y + 16.0;
                transform.translation.z = 2.;
            }
        }
    }
}

pub enum BuildingType {
    Theatre,
    Amphiteatre,
    Colosseum,
}

#[derive(Component)]
pub struct Building(BuildingType);

mod building;
mod camera;
mod cli;
mod cursor;
mod grid;
mod resources;
mod ui;

use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use clap::Parser;

use building::BuildingPlugin;
use camera::CameraPlugin;
use cli::Args;
use cursor::CursorPlugin;
use grid::GridPlugin;
use resources::ResourcesPlugin;
use ui::UiPlugin;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum AppState {
    #[default]
    Loading,
    Level,
}

fn main() {
    let args = Args::parse();

    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: String::from("Iso Diamond Example"),
                        ..Default::default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .insert_resource(args)
        .add_plugins(TilemapPlugin) // This is the plugin for the tilemap
        .add_plugins(GridPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(CursorPlugin)
        .add_plugins(BuildingPlugin)
        .add_plugins(ResourcesPlugin)
        .add_plugins(UiPlugin)
        .run();
}

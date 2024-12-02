use bevy::prelude::Resource;
use clap::Parser;

#[derive(Parser, Debug, Resource)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Map file to load
    #[arg(short, long)]
    pub map: Option<String>,
    /// Map size in tiles
    #[arg(short, long, default_value = "32")]
    pub tilemap_size: Option<u32>,
}

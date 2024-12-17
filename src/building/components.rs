use bevy::prelude::*;
use bevy_ecs_tilemap::tiles::TilePos;

#[derive(Component)]
pub struct Building;

#[derive(Component, Clone, Copy)]
pub struct BuildingSize(pub (u32, u32));

#[derive(Component, Clone, Copy)]
pub enum BuildingType {
    Theatre,
    //    Amphiteatre,
    //    Colosseum,
}

impl BuildingType {
    pub fn occupation(&self) -> u32 {
        match self {
            BuildingType::Theatre => 4,
            //            BuildingType::Amphiteatre => 8,
            //            BuildingType::Colosseum => 12,
        }
    }

    pub fn cost(&self) -> u32 {
        match self {
            BuildingType::Theatre => 10,
            //            BuildingType::Amphiteatre => 20,
            //            BuildingType::Colosseum => 30,
        }
    }

    pub fn name(&self) -> String {
        match self {
            BuildingType::Theatre => "Theatre".to_string(),
            //            BuildingType::Amphiteatre => "Amphiteatre".to_string(),
            //            BuildingType::Colosseum => "Colosseum".to_string(),
        }
    }

    pub fn size(&self) -> (u32, u32) {
        match self {
            BuildingType::Theatre => (2, 2).to_owned(),
            //            BuildingType::Amphiteatre => (4, 4),
            //            BuildingType::Colosseum => (5, 5),
        }
    }

    pub fn sprite(&self) -> String {
        match self {
            BuildingType::Theatre => "buildings/theatre.png".to_string(),
            //            BuildingType::Amphiteatre => "buildings/amphiteatre.png",
            //            BuildingType::Colosseum => "buildings/colosseum.png",
        }
    }
}

#[derive(Component, Clone)]
pub struct CoveringTiles(pub Vec<TilePos>);

#[derive(Component)]
pub struct BuildingTemplateMarker;

#[derive(Component)]
pub struct CanBuild(pub bool);

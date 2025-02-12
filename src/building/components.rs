use bevy::prelude::*;

use crate::resources::MaterialType;

#[derive(Component)]
pub struct Building;

#[derive(Component, Clone, Copy)]
pub enum BuildingType {
    Theatre,
    WheatFarm,
    House,
}

impl BuildingType {
    pub fn size(&self) -> (u32, u32) {
        match self {
            BuildingType::Theatre => (2, 2).to_owned(),
            BuildingType::WheatFarm => (3, 3).to_owned(),
            BuildingType::House => (2, 2).to_owned(),
        }
    }

    pub fn sprite(&self) -> &'static str {
        match self {
            BuildingType::Theatre => "buildings/theatre.png",
            BuildingType::WheatFarm => "buildings/amphiteatre.png",
            BuildingType::House => "buildings/theatre.png",
        }
    }

    pub fn cost(&self) -> u32 {
        match self {
            BuildingType::Theatre => 10,
            BuildingType::WheatFarm => 30,
            BuildingType::House => 5,
        }
    }

    pub fn name(&self) -> String {
        match self {
            BuildingType::Theatre => "Theatre".to_string(),
            BuildingType::WheatFarm => "Wheat Farm".to_string(),
            BuildingType::House => "House".to_string(),
            //            BuildingType::Amphiteatre => "Amphiteatre".to_string(),
            //            BuildingType::Colosseum => "Colosseum".to_string(),
        }
    }
}

#[derive(Component, Clone)]
pub struct CoveringTiles(pub Vec<Entity>);

#[derive(Component)]
pub struct BuildingTemplateMarker;

#[derive(Component)]
pub struct CanBuild(pub bool);

#[derive(Component)]
pub struct Employees {
    pub employees: u32,
    pub max_employees: u32,
}

#[derive(Component)]
pub struct Housing {
    pub capacity: u32,
    pub population: u32,
}

#[derive(Component)]
pub struct Storage {
    pub allowed_resources: Vec<MaterialType>,
    pub storage: Vec<(MaterialType, u32)>,
    pub total_capacity: u32,
}

#[derive(Component)]
pub struct Production {
    pub required: Option<MaterialType>,
    pub produced: MaterialType,
    pub production_done: u32,
    pub production_time: u32,
}

#[derive(Component)]
pub struct Consumption {
    pub resource: MaterialType,
    pub amount: u32,
    pub frequency: u32,
}

#[derive(Component)]
pub enum ServiceType {
    Entertainment,
}

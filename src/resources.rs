use bevy::prelude::*;

#[derive(Resource)]
pub struct GlobalResources {
    pub gold: i32,
    pub population: i32,
}

impl Default for GlobalResources {
    fn default() -> Self {
        Self {
            gold: 1000,
            population: 0,
        }
    }
}

pub struct ResourcesPlugin;
impl Plugin for ResourcesPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GlobalResources>();
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MaterialType {
    // Agricoltural products
    Wheat,
    Vegetables,
    Meat,
    Olives,
    Wool,
    // Prime resources
    Wood,
    Iron,
    Clay,
    // Secondary resources
    Wine,
    Cloth,
    OliveOil,
    Pottery,
}

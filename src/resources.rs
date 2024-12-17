use bevy::prelude::*;

#[derive(Resource)]
pub struct GlobalResources {
    pub gold: i32,
}

impl Default for GlobalResources {
    fn default() -> Self {
        Self { gold: 1000 }
    }
}

pub struct ResourcesPlugin;
impl Plugin for ResourcesPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GlobalResources>();
    }
}

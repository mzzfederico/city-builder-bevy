use bevy::prelude::*;

#[derive(Resource)]
pub struct GlobalResources {
    pub gold: u32,
}

impl Default for GlobalResources {
     fn default() -> Self {
        Self { gold: 10000 }
    }
}

pub struct ResourcesPlugin;
impl Plugin for ResourcesPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GlobalResources>();
    }
}


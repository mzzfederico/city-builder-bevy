use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use crate::resources::GlobalResources;

pub struct UiPlugin;
impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin);
        app.add_systems(Update, ui_example_system);
    }
}

fn ui_example_system(mut contexts: EguiContexts, mut resources: ResMut<GlobalResources>) {
    egui::Window::new("Resources").show(contexts.ctx_mut(), |ui| {
        ui.label(format!("Gold: {}", resources.gold.to_string()));
    });
}

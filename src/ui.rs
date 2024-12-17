use crate::{
    resources::GlobalResources,
    time::{GameTimer, TimeSpeed, TimeState},
};
use bevy::prelude::*;
use bevy_egui::{
    egui::{self, RichText},
    EguiContexts, EguiPlugin,
};

pub struct UiPlugin;
impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin);
        app.add_systems(Update, ui_generic_resources);
        app.add_systems(Update, ui_time_controls);
    }
}

fn ui_generic_resources(mut contexts: EguiContexts, resources: ResMut<GlobalResources>) {
    egui::Window::new("Resources").show(contexts.ctx_mut(), |ui| {
        ui.label(format!("Gold"));
        ui.label(RichText::new(resources.gold.to_string()).color(Color32::WHITE));
    });
}

use bevy_egui::egui::Color32;

fn is_enabled<T: PartialEq>(new: &T, current: &T) -> Color32 {
    if new == current {
        Color32::GREEN
    } else {
        Color32::WHITE
    }
}

fn ui_time_controls(
    mut contexts: EguiContexts,
    speed: Res<State<TimeSpeed>>,
    time_state: Res<State<TimeState>>,
    mut next_speed: ResMut<NextState<TimeSpeed>>,
    mut next_time_state: ResMut<NextState<TimeState>>,
    mut game_timer: ResMut<GameTimer>,
) {
    egui::Window::new("Time").show(contexts.ctx_mut(), |ui| {
        if ui
            .button(RichText::new("Pause").color(is_enabled(time_state.get(), &TimeState::Paused)))
            .clicked()
        {
            next_time_state.set(TimeState::Paused);
        }
        if ui
            .button(RichText::new("Normal").color(is_enabled(speed.get(), &TimeSpeed::Normal)))
            .clicked()
        {
            next_time_state.set(TimeState::Running);
            next_speed.set(TimeSpeed::Normal);
            game_timer.0 = Timer::from_seconds(7.0, TimerMode::Repeating);
        }
        if ui
            .button(RichText::new("Fast").color(is_enabled(speed.get(), &TimeSpeed::Fast)))
            .clicked()
        {
            next_time_state.set(TimeState::Running);
            next_speed.set(TimeSpeed::Fast);
            game_timer.0 = Timer::from_seconds(3.5, TimerMode::Repeating);
        }
        if ui
            .button(RichText::new("Faster").color(is_enabled(speed.get(), &TimeSpeed::Faster)))
            .clicked()
        {
            next_time_state.set(TimeState::Running);
            next_speed.set(TimeSpeed::Faster);
            game_timer.0 = Timer::from_seconds(1.75, TimerMode::Repeating);
        }
        if ui
            .button(RichText::new("Fastest").color(is_enabled(speed.get(), &TimeSpeed::Fastest)))
            .clicked()
        {
            next_time_state.set(TimeState::Running);
            next_speed.set(TimeSpeed::Fastest);
            game_timer.0 = Timer::from_seconds(0.8525, TimerMode::Repeating);
        }
    });
}

use bevy::prelude::*;

pub struct TimeControlsPlugin;
impl Plugin for TimeControlsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_state(TimeState::Running);
        app.insert_state(TimeSpeed::Normal);
        app.insert_resource(GameTimer(Timer::from_seconds(7.0, TimerMode::Repeating)));
    }
}

#[derive(Resource)]
pub struct GameTimer(pub Timer);

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
pub enum TimeState {
    Paused,
    #[default]
    Running,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
pub enum TimeSpeed {
    #[default]
    Normal,
    Fast,
    Faster,
    Fastest,
}

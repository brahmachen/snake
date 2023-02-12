use bevy::prelude::*;

pub const HEIGHT: f32 = 600.0;
pub const WIDTH: f32 = 1200.0;

#[derive(Component, Clone, Debug, Hash, PartialEq, Eq)]
pub enum AppState {
    MainMenu,
    InGame,
    GameOver,
}

#[derive(Component, Clone, Debug, Hash, PartialEq, Eq)]
pub enum GameState {
    Playing,
    Restarted,
    Quitted,
    Pause,
    Fail,
}
use bevy::prelude::*;

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
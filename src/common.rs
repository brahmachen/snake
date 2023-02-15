use bevy::prelude::*;

pub const HEIGHT: f32 = 480.0;
pub const WIDTH: f32 = 900.0;

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
}

#[derive(Debug, Resource)]
pub struct GameAudios {
    pub up: Handle<AudioSource>,
    pub down: Handle<AudioSource>,
    pub left: Handle<AudioSource>,
    pub right: Handle<AudioSource>,
    pub eat: Handle<AudioSource>,
    pub die: Handle<AudioSource>,
}

pub fn setup_game_audios(mut command: Commands, asset_server: Res<AssetServer>) {
    let game_audios = GameAudios {
        up: asset_server.load("sounds/up.wav"),
        down: asset_server.load("sounds/down.wav"),
        left: asset_server.load("sounds/left.wav"),
        right: asset_server.load("sounds/right.wav"),
        eat: asset_server.load("sounds/eat.wav"),
        die: asset_server.load("sounds/die.wav"),
    };
    command.insert_resource(game_audios);
}
use bevy::prelude::*;
// use bevy_inspector_egui::quick::WorldInspectorPlugin;

use common::*;
use menu::*;
use snake::*;

mod common;
mod menu;
mod snake;


fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(49.0/255.0, 44.0/255.0, 63.0/255.0)))
        .add_state(GameState::Quitted)
        .add_state(AppState::MainMenu)
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "snake".to_string(),
                width: WIDTH + SQUARE_SIZE,
                height: HEIGHT + SQUARE_SIZE,
                resizable: false,
                ..default()
            },
            ..default()
        }))
        // .add_plugin(WorldInspectorPlugin)
        .add_startup_system(setup)
        .add_system_set(
            SystemSet::on_enter(AppState::MainMenu)
                .with_system(setup_main_menu)
                .with_system(clear_snake)
                .with_system(clear_food)
                .with_system(setup_snake)
        )
        .add_system_set(
            SystemSet::on_exit(AppState::MainMenu)
                .with_system(despawn_screen::<OnMainMenuScreen>)
        )
        .add_system_set(SystemSet::on_update(AppState::MainMenu).with_system(click_button))
        .add_system_set(SystemSet::on_update(AppState::InGame).with_system(pause_game))

        // Game Over Menu
        .add_system_set(SystemSet::on_enter(AppState::GameOver).with_system(setup_game_over_menu))
        .add_system_set(SystemSet::on_update(AppState::GameOver).with_system(click_button))
        .add_system_set(
            SystemSet::on_exit(AppState::GameOver)
                .with_system(despawn_screen::<OnGameOverMenuScreen>)
        )

        // Game Playing
        .add_system_set(
            SystemSet::on_update(GameState::Playing)
                .with_system(generate_food)
                .with_system(move_snake)
                .with_system(contral_snake)
        )
        // Game Restarted
        .add_system_set(
            SystemSet::on_enter(GameState::Restarted)
                .with_system(clear_snake)
                .with_system(clear_food)
                .with_system(setup_snake)
        )
        .add_system_set(SystemSet::on_update(GameState::Restarted).with_system(play_game))
        
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

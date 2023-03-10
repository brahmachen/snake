use bevy::app::AppExit;
use bevy::prelude::*;

use crate::{
  common::{AppState, GameState},
};

#[derive(Component)]
pub enum MenuButtonAction {
    StartGame,
    RestartGame,
    BackToMainMenu,
    Quit,
}

#[derive(Component)]
pub struct OnMainMenuScreen;

#[derive(Component)]
pub struct OnGamePausedMenuScreen;

#[derive(Component)]
pub struct OnGameOverMenuScreen;

pub fn setup_main_menu(
  mut commands: Commands,
  asset_server: Res<AssetServer>,
) {
  commands
    .spawn((
        NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            ..default()
        },
        OnMainMenuScreen,
    ))
    .with_children(|parent| {
      parent
          .spawn(NodeBundle {
              style: Style {
                  flex_direction: FlexDirection::Column,
                  align_items: AlignItems::Center,
                  ..default()
              },
              background_color: Color::CRIMSON.into(),
              ..default()
          })
          .with_children(|parent| {
              // 标题
              parent.spawn(
                  TextBundle::from_section(
                      "Snake Main Menu",
                      TextStyle {
                          font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                          font_size: 25.0,
                          color: Color::rgb(0.9, 0.9, 0.9),
                      },
                  )
                  .with_style(Style {
                      margin: UiRect::all(Val::Px(20.0)),
                      ..default()
                  }),
              );

              // 开始按钮
              parent
                  .spawn((
                      ButtonBundle {
                          style: Style {
                              size: Size::new(Val::Px(50.0), Val::Px(30.0)),
                              margin: UiRect::all(Val::Px(10.0)),
                              justify_content: JustifyContent::Center,
                              align_items: AlignItems::Center,
                              ..default()
                          },
                          background_color: Color::rgb(0.15, 0.15, 0.15).into(),
                          ..default()
                      },
                      MenuButtonAction::StartGame,
                  ))
                  .with_children(|parent| {
                      parent.spawn(TextBundle::from_section(
                          "Start",
                          TextStyle {
                              font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                              font_size: 20.0,
                              color: Color::rgb(0.9, 0.9, 0.9),
                          },
                      ));
                  });

              // 退出按钮
              parent
                  .spawn((
                      ButtonBundle {
                          style: Style {
                              size: Size::new(Val::Px(50.0), Val::Px(30.0)),
                              margin: UiRect::all(Val::Px(10.0)),
                              justify_content: JustifyContent::Center,
                              align_items: AlignItems::Center,
                              ..default()
                          },
                          background_color: Color::rgb(0.15, 0.15, 0.15).into(),
                          ..default()
                      },
                      MenuButtonAction::Quit,
                  ))
                  .with_children(|parent| {
                      parent.spawn(TextBundle::from_section(
                          "Quit",
                          TextStyle {
                              font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                              font_size: 20.0,
                              color: Color::rgb(0.9, 0.9, 0.9),
                          },
                      ));
                  });
          });
  });
}

pub fn click_button(
  mut interaction_query: Query<
      (&Interaction, &MenuButtonAction),
      (Changed<Interaction>, With<Button>),
  >,
  mut app_state: ResMut<State<AppState>>,
  mut game_state: ResMut<State<GameState>>,
  mut exit: EventWriter<AppExit>,
) {
  for (interaction, menu_button_action) in &mut interaction_query {
      match *interaction {
          Interaction::Clicked => match menu_button_action {
              MenuButtonAction::StartGame => {
                  info!("StartGame button clicked");
                  app_state.set(AppState::InGame).unwrap();
                  game_state.set(GameState::Playing).unwrap();
              }
              MenuButtonAction::RestartGame => {
                  info!("RestartGame button clicked");
                  if app_state.current().clone() != AppState::InGame {
                      app_state.set(AppState::InGame).unwrap();
                  }
                  game_state.set(GameState::Restarted).unwrap();
              }
              MenuButtonAction::BackToMainMenu => {
                  info!("BackToMainMenu button clicked");
                  println!("{:?}", app_state.current());
                  if app_state.current().clone() != AppState::MainMenu {
                      app_state.set(AppState::MainMenu).unwrap();
                  }
                  if game_state.current().clone() != GameState::Quitted {
                      game_state.set(GameState::Quitted).unwrap();
                  }
              }
              MenuButtonAction::Quit => {
                  info!("Quit button clicked");
                  exit.send_default();
              }
          },
          _ => {}
      }
  }
}


pub fn setup_game_over_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
            OnGameOverMenuScreen,
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: Color::CRIMSON.into(),
                    ..default()
                })
                .with_children(|parent| {
                    // 标题
                    parent.spawn(
                        TextBundle::from_section(
                            "Game Over",
                            TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 25.0,
                                color: Color::rgb(0.9, 0.9, 0.9),
                            },
                        )
                        .with_style(Style {
                            margin: UiRect::all(Val::Px(20.0)),
                            ..default()
                        }),
                    );

                    // 返回主菜单按钮
                    parent
                        .spawn((
                            ButtonBundle {
                                style: Style {
                                    size: Size::new(Val::Px(90.0), Val::Px(30.0)),
                                    margin: UiRect::all(Val::Px(10.0)),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                background_color: Color::rgb(0.15, 0.15, 0.15).into(),
                                ..default()
                            },
                            MenuButtonAction::BackToMainMenu,
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "Main Menu",
                                TextStyle {
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    font_size: 20.0,
                                    color: Color::rgb(0.9, 0.9, 0.9),
                                },
                            ));
                        });

                    // 重新开始按钮
                    parent
                        .spawn((
                            ButtonBundle {
                                style: Style {
                                    size: Size::new(Val::Px(90.0), Val::Px(30.0)),
                                    margin: UiRect::all(Val::Px(10.0)),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                background_color: Color::rgb(0.15, 0.15, 0.15).into(),
                                ..default()
                            },
                            MenuButtonAction::RestartGame,
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "Restart",
                                TextStyle {
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    font_size: 20.0,
                                    color: Color::rgb(0.9, 0.9, 0.9),
                                },
                            ));
                        });
                });
        });
}



pub fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}


pub fn play_game(mut game_state: ResMut<State<GameState>>) {
    game_state.set(GameState::Playing).unwrap();
}
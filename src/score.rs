use bevy::prelude::*;

// 分数
#[derive(Resource)]
pub struct Score(pub u32);
#[derive(Component)]
pub struct Scoreboard;

// 记录
#[derive(Resource)]
pub struct Record(pub u32);
#[derive(Component)]
pub struct Recordboard;

pub fn setup_score(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // 分数
    commands
        .spawn(
            TextBundle::from_sections([
                TextSection::new(
                    "Score: ",
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 30.0,
                        color: Color::rgba(0.5, 0.5, 1.0, 0.5),
                    },
                ),
                TextSection::new(
                    "0",
                    TextStyle {
                        font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                        font_size: 30.0,
                        color: Color::rgba(1.0, 0.5, 0.5, 0.5),
                    },
                ),
            ])
            .with_style(Style {
                position_type: PositionType::Absolute,
                position: UiRect {
                    top: Val::Px(4.0),
                    left: Val::Px(10.0),
                    ..default()
                },
                
                ..default()
            }),
        )
        .insert(Scoreboard);

    //  最高记录
    commands
        .spawn(
            TextBundle::from_sections([
                TextSection::new(
                    "Record: ",
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 30.0,
                        color: Color::rgba(0.5, 0.5, 1.0, 0.5),
                    },
                ),
                TextSection::new(
                    "0",
                    TextStyle {
                        font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                        font_size: 30.0,
                        color: Color::rgba(1.0, 0.5, 0.5, 0.5),
                    },
                ),
            ])
            .with_style(Style {
                position_type: PositionType::Absolute,
                position: UiRect {
                    top: Val::Px(28.0),
                    left: Val::Px(10.0),
                    ..default()
                },
                ..default()
            }),
        )
        .insert(Recordboard);

}

pub fn update_scoreboard(score: Res<Score>, mut query: Query<&mut Text, With<Scoreboard>>) {
    let mut text = query.single_mut();
    text.sections[1].value = score.0.to_string();
}

pub fn update_recordboard(lines: Res<Record>, mut query: Query<&mut Text, With<Recordboard>>) {
    let mut text = query.single_mut();
    text.sections[1].value = lines.0.to_string();
}

pub fn clear_score(mut score: ResMut<Score>) {
    score.0 = 0;
}

use bevy::prelude::*;
use rand::prelude::*;


pub const HEIGHT: f32 = 720.0;
pub const WIDTH: f32 = 1280.0;
pub const SquareSize: f32 = 40.0;

#[derive(Component)]
struct Square {
    x: i32,
    y: i32,
}

impl Square {
    fn random() -> Self {
        let square_width: i32 = (WIDTH / SquareSize / 2.0) as i32;
        let square_height: i32 = (HEIGHT / SquareSize / 2.0) as i32;

        let mut rng = thread_rng();
        let x = rng.gen_range(-square_width..square_width);
        let y = rng.gen_range(-square_height..square_height);
        Self {
            x, y
        }
    }
    fn translation(&self) -> Vec3 {
        Vec3 { x: (self.x as f32) * SquareSize, y: (self.y as f32) * SquareSize, z: 0.0 }
    }
}

#[derive(Component)]
struct FoodTimer(Timer);

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.2, 0.2, 0.2)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "snake".to_string(),
                width: WIDTH,
                height: HEIGHT,
                resizable: false,
                ..default()
            },
            ..default()
        }))
        .add_startup_system(setup)
        .add_system(generate_food)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn(FoodTimer(Timer::from_seconds(2.0, TimerMode::Repeating)));
}

fn generate_food(
    mut commonds: Commands,
    mut query: Query<&mut FoodTimer>,
    time: Res<Time>
) {
    for mut timer in &mut query {
        if timer.0.tick(time.delta()).just_finished() {
            let square = Square::random();
            println!("x:{}, y:{}, translation:{}", square.x, square.y, square.translation());
            commonds.spawn((
                SpriteBundle {
                    transform: Transform {
                        translation: square.translation(),
                        ..default()
                    },
                    sprite: Sprite {
                        color: Color::rgb(0.8, 0.8, 0.8),
                        custom_size: Some(Vec2::new(SquareSize, SquareSize)),
                        ..default()
                    },
                    ..default()
                },
                square,
            ));
        }
    }
}

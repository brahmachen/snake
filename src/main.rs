use bevy::prelude::*;
use std::f32::consts::*;

use rand::prelude::*;


pub const HEIGHT: f32 = 720.0;
pub const WIDTH: f32 = 1280.0;
pub const SquareSize: f32 = 40.0;

#[derive(Component, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Point {
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
    fn from_direction(&mut self, direction: &Direction) -> Self {
        let mut x = self.x;
        let mut y = self.y;
        match *direction {
            Direction::Up => y += 1,
            Direction::Down => y -= 1,
            Direction::Left => x -= 1,
            Direction::Right => x += 1,
        }
        Self {
            x, y
        }
    }
    fn next_point(&mut self, direction: &Direction) {
        let point = self.from_direction(direction);
        self.x = point.x;
        self.y = point.y;
    }
}

#[derive(Component)]
struct Food(Point);

#[derive(Component)]
struct FoodTimer(Timer);

#[derive(Component)]
struct BodyIncreaseTimer(Timer);

#[derive(Component)]
struct Snake {
    move_timer: Timer,
    move_direction: Direction,
}

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
        .add_startup_system(setup_snake)
        .add_system(generate_food)
        .add_system(move_snake)
        .add_system(contral_snake)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn(FoodTimer(Timer::from_seconds(2.0, TimerMode::Repeating)));
}

fn setup_snake(mut commands: Commands) {
    let mut point = Point {
        x: 0, y: 0
    };
    let snake = Snake {
        move_timer: Timer::from_seconds(0.5, TimerMode::Repeating),
        move_direction: Direction::Up,
    };
    let translation = point.translation();

    let parent = commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: Vec3 { x: 0.0, y: 0.0, z: 0.0 },
                ..default()
            },
            ..default()
        },
        snake
    )).id();

    let children = commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: translation,
                ..default()
            },
            sprite: Sprite {
                color: Color::rgb(0.4, 0.4, 0.8),
                custom_size: Some(Vec2::new(SquareSize, SquareSize)),
                ..default()
            },
            ..default()
        },
        point
    )).id();

    commands.entity(parent).add_child(children);
}

fn generate_food(
    mut commonds: Commands,
    mut query: Query<&mut FoodTimer>,
    time: Res<Time>
) {
    for mut timer in &mut query {
        if timer.0.tick(time.delta()).just_finished() {
            let square = Food(Point::random());
            println!("x:{}, y:{}, translation:{}", square.0.x, square.0.y, square.0.translation());
            commonds.spawn((
                SpriteBundle {
                    transform: Transform {
                        translation: square.0.translation(),
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


fn move_snake(
    mut commands: Commands,
    time: Res<Time>,
    mut parents_query: Query<(Entity, &Children, &mut Snake), With<Sprite>>,
    mut transform_query: Query<&mut Transform, With<Sprite>>,
    mut point_query: Query<&mut Point, With<Sprite>>,
    mut food_query: Query<&Food>,

) {
    for (parent, children, mut snake) in &mut parents_query {
        if snake.move_timer.tick(time.delta()).just_finished() {

            let mut is_eat_food = false;
            if let Ok(mut head) = point_query.get_mut(children[0]) {
                let new_point = head.from_direction(&snake.move_direction);
                food_query.for_each(|food| {
                    if food.0.x == new_point.x && food.0.y == new_point.y {
                        is_eat_food = true;
                    }
                });

                if is_eat_food {
                    let new_child = commands.spawn((
                        SpriteBundle {
                            transform: Transform {
                                translation: new_point.translation(),
                                ..default()
                            },
                            sprite: Sprite {
                                color: Color::rgb(0.4, 0.4, 0.8),
                                custom_size: Some(Vec2::new(SquareSize, SquareSize)),
                                ..default()
                            },
                            ..default()
                        },
                        new_point
                    )).id();

                    commands.entity(parent).insert_children(0, &vec![new_child]);
                } else {
                    for child in children {
                        if let Ok(mut point) = point_query.get_mut(*child) {
                            point.next_point(&snake.move_direction);
                            let translation = point.translation();
        
                            if let Ok(mut transform) = transform_query.get_mut(*child) {
                                transform.translation.x = translation.x;
                                transform.translation.y = translation.y;
                            }
                        }
                    }
                }

            }
        }
    }
}

fn contral_snake(
    keyboard_input: Res<Input<KeyCode>>,
    mut snake_query: Query<&mut Snake, With<Sprite>>,
) {
    for mut snake in &mut snake_query {
        let direction;
        if keyboard_input.pressed(KeyCode::Up) {
            direction = Direction::Up;
        } else if keyboard_input.pressed(KeyCode::Down) {
            direction = Direction::Down;
        } else if keyboard_input.pressed(KeyCode::Left) {
            direction = Direction::Left;
        } else if keyboard_input.pressed(KeyCode::Right) {
            direction = Direction::Right;
        } else {
            return;
        }

        snake.move_direction = direction;
    }

}
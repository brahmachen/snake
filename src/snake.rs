use bevy::prelude::*;
use rand::prelude::*;

use crate::{
  common::{AppState, GameState, HEIGHT, WIDTH},
};

pub const SQUARE_SIZE: f32 = 30.0;
pub const X: i32 = (WIDTH / 2.0 / SQUARE_SIZE) as i32;
pub const Y: i32 = (HEIGHT / 2.0 / SQUARE_SIZE) as i32;

#[derive(Component, Clone, Copy)]
pub struct Point {
    x: i32,
    y: i32,
}

#[derive(PartialEq, Eq)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Point {
    fn random() -> Self {
        let square_width: i32 = (WIDTH / SQUARE_SIZE / 2.0) as i32;
        let square_height: i32 = (HEIGHT / SQUARE_SIZE / 2.0) as i32;

        let mut rng = thread_rng();
        let x = rng.gen_range(-square_width..square_width);
        let y = rng.gen_range(-square_height..square_height);
        Self {
            x, y
        }
    }
    fn translation(&self) -> Vec3 {
        Vec3 { x: (self.x as f32) * SQUARE_SIZE, y: (self.y as f32) * SQUARE_SIZE, z: 0.0 }
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
pub struct Food(Point);

#[derive(Component)]
pub struct FoodTimer(Timer);

#[derive(Component)]
pub struct BodyIncreaseTimer(Timer);

#[derive(Component)]
pub struct Snake {
    move_timer: Timer,
    move_direction: Direction,
}


pub fn setup_snake(mut commands: Commands) {
	commands.spawn(FoodTimer(Timer::from_seconds(1.0, TimerMode::Once)));

	let parent = commands.spawn((
		SpriteBundle {
			transform: Transform {
				translation: Vec3 { x: 0.0, y: 0.0, z: 0.0 },
				..default()
			},
			..default()
		},
		Snake {
			move_timer: Timer::from_seconds(0.15, TimerMode::Repeating),
			move_direction: Direction::Right,
		}
	)).id();

	let init_points:Vec<Point> = vec![
		Point { x: -5, y: 0 },
		Point { x: -6, y: 0 },
		Point { x: -7, y: 0 },
	];

	for point in init_points {
		let children = commands.spawn((
			SpriteBundle {
				transform: Transform {
					translation: point.translation(),
					..default()
				},
				sprite: Sprite {
					color: Color::rgb(0.4, 0.4, 0.8),
					custom_size: Some(Vec2::new(SQUARE_SIZE, SQUARE_SIZE)),
					..default()
				},
				..default()
			},
			point
		)).id();
		commands.entity(parent).add_child(children);
	}

}

pub fn generate_food(
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
						custom_size: Some(Vec2::new(SQUARE_SIZE, SQUARE_SIZE)),
						..default()
					},
					..default()
				},
				square,
			));
		}
	}
}


pub fn move_snake(
	mut commands: Commands,
	time: Res<Time>,
	mut parents_query: Query<(Entity, &Children, &mut Snake), With<Sprite>>,
	mut transform_query: Query<&mut Transform, With<Sprite>>,
	mut point_query: Query<&mut Point, With<Sprite>>,
	food_query: Query<(Entity, &mut Food)>,
	mut app_state: ResMut<State<AppState>>,
	mut game_state: ResMut<State<GameState>>,
) {
for (parent, children, mut snake) in &mut parents_query {
	if snake.move_timer.tick(time.delta()).just_finished() {
		let mut is_eat_food = false;
		let mut is_game_over = false;
		if let Ok(mut head) = point_query.get_mut(children[0]) {
			// 蛇头的下一个位置
			let new_point = head.from_direction(&snake.move_direction);
			// 检查是否游戏失败 --- 撞墙或者撞到自己
			if new_point.x < -X || new_point.x > X || new_point.y < -Y || new_point.y > Y {
				is_game_over = true;
			}
			if is_game_over && app_state.current().clone() != AppState::GameOver {
				app_state.set(AppState::GameOver).unwrap();
				game_state.set(GameState::Quitted).unwrap();
				return;
			}

			for (food_entity, food) in &food_query {
				if food.0.x == new_point.x && food.0.y == new_point.y {
					is_eat_food = true;
					commands.entity(food_entity).despawn_recursive();
				}
			}

			if is_eat_food {
				let new_child = commands.spawn((
					SpriteBundle {
						transform: Transform {
							translation: new_point.translation(),
							..default()
						},
						sprite: Sprite {
							color: Color::rgb(0.4, 0.4, 0.8),
							custom_size: Some(Vec2::new(SQUARE_SIZE, SQUARE_SIZE)),
							..default()
						},
						..default()
					},
					new_point
				)).id();
				commands.entity(parent).insert_children(0, &vec![new_child]);

				commands.spawn(FoodTimer(Timer::from_seconds(1.0, TimerMode::Once)));
			} else {
				let tail_entity = children[children.len() - 1];
				if let Ok(mut point) = point_query.get_mut(tail_entity) {
					point.x = new_point.x;
					point.y = new_point.y;
					let translation = point.translation();

					if let Ok(mut transform) = transform_query.get_mut(tail_entity) {
						transform.translation.x = translation.x;
						transform.translation.y = translation.y;
					}
					commands.entity(parent).remove_children(&vec![tail_entity]);

					commands.entity(parent).insert_children(0, &vec![tail_entity]);
				}
			}

		}
	}
}
}

pub fn contral_snake(
	keyboard_input: ResMut<Input<KeyCode>>,
	mut snake_query: Query<&mut Snake, With<Sprite>>,
) {
	for mut snake in &mut snake_query {
		if keyboard_input.pressed(KeyCode::Up) && snake.move_direction != Direction::Down {
			snake.move_direction = Direction::Up;
		} else if keyboard_input.pressed(KeyCode::Down) && snake.move_direction != Direction::Up {
			snake.move_direction = Direction::Down;
		} else if keyboard_input.pressed(KeyCode::Left) && snake.move_direction != Direction::Right {
			snake.move_direction = Direction::Left;
		} else if keyboard_input.pressed(KeyCode::Right) && snake.move_direction != Direction::Left {
			snake.move_direction = Direction::Right;
		}
	}
}

pub fn pause_game(
	mut game_state: ResMut<State<GameState>>,
	mut keyboard_input: ResMut<Input<KeyCode>>,
	mut snake_query: Query<&mut Snake, With<Sprite>>,
) {
	for mut snake in &mut snake_query {
		if keyboard_input.pressed(KeyCode::Space) {
			if game_state.current().clone() == GameState::Pause {
				snake.move_timer.unpause();
				game_state.set(GameState::Playing).unwrap();
			} else {
				snake.move_timer.pause();
				game_state.set(GameState::Pause).unwrap();
			}
			keyboard_input.reset(KeyCode::Space);
		}
	}
}

pub fn clear_snake(mut commands: Commands, query: Query<Entity, With<Point>>) {
	for entity in &query {
		commands.entity(entity).despawn();
	}
}

pub fn clear_food(mut commands: Commands, query: Query<Entity, With<Food>>) {
	for entity in &query {
		commands.entity(entity).despawn();
	}
}
use bevy::math::Vec3;
use bevy::prelude::{Camera2dBundle, Commands, Query, Transform};
use bevy::sprite::{Sprite, SpriteBundle};
use bevy::utils::default;
use bevy::window::Window;
use crate::paddle::constant::{GAP_FROM_WALL, PADDLE_COLOR, PADDLE_SIZE};
use crate::paddle::entity::{AIControlled, Paddle, PaddleLocation, UserControlled};

pub fn paddle_init_systems(
    mut commands: Commands,
    window_query: Query<&Window>
) {
    let window = window_query.single();
    let right_bound = window.width() / 2.0;
    let left_bound = -1.0 * right_bound;

    let player_paddle_loc = Vec3::new(left_bound + GAP_FROM_WALL, 0.0, 0.0);
    let enemy_paddle_loc = Vec3::new(right_bound - GAP_FROM_WALL, 0.0, 0.0);

    commands.spawn(Camera2dBundle::default());

    commands.spawn((
        Paddle,
        UserControlled,
        PaddleLocation(player_paddle_loc),
        new_paddle_sprite(player_paddle_loc)
    ));
    commands.spawn((
        Paddle,
        AIControlled,
        PaddleLocation(enemy_paddle_loc),
        new_paddle_sprite(enemy_paddle_loc)
    ));
}

fn new_paddle_sprite(position: Vec3) -> SpriteBundle {
    SpriteBundle {
        transform: Transform {
            translation: position,
            scale: PADDLE_SIZE,
            ..default()
        },
        sprite: Sprite {
            color: PADDLE_COLOR,
            ..default()
        },
        ..default()
    }
}

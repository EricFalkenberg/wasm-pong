use bevy::input::Input;
use bevy::prelude::{KeyCode, Query, Res, Transform, With, Without};

use crate::paddle::entity::{AIControlled, PaddleLocation, UserControlled};

const PADDLE_SPEED: f32 = 10.0;

pub fn handle_keypress_system(
    keys: Res<Input<KeyCode>>,
    mut player_paddle_query: Query<(&mut PaddleLocation, &mut Transform), (With<UserControlled>, Without<AIControlled>)>
) {
    let (mut player_paddle_loc, mut transform) = player_paddle_query.single_mut();
    if keys.pressed(KeyCode::W) {
        transform.translation.y += PADDLE_SPEED;
        player_paddle_loc.0.y += PADDLE_SPEED;
    } else if keys.pressed(KeyCode::S) {
        transform.translation.y -= PADDLE_SPEED;
        player_paddle_loc.0.y -= PADDLE_SPEED;
    }
}
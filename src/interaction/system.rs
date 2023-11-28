use bevy::input::Input;
use bevy::prelude::{KeyCode, Query, Res, Transform, With, Without};
use bevy::window::Window;

use crate::paddle::entity::{AIControlled, PaddleLocation, UserControlled};

const PADDLE_SPEED: f32 = 10.0;

pub fn handle_keypress_system(
    keys: Res<Input<KeyCode>>,
    window_query: Query<&Window>,
    mut player_paddle_query: Query<(&mut PaddleLocation, &mut Transform), (With<UserControlled>, Without<AIControlled>)>
) {
    let window = window_query.single();
    let top_bound = window.height() / 2.0 - 60.0;
    let bottom_bound = -1.0 * top_bound;
    let (mut player_paddle_loc, mut transform) = player_paddle_query.single_mut();
    if keys.pressed(KeyCode::W) && player_paddle_loc.0.y < top_bound  {
        transform.translation.y += PADDLE_SPEED;
        player_paddle_loc.0.y += PADDLE_SPEED;
    } else if keys.pressed(KeyCode::S) && player_paddle_loc.0.y > bottom_bound {
        transform.translation.y -= PADDLE_SPEED;
        player_paddle_loc.0.y -= PADDLE_SPEED;
    }
}
use bevy::math::Vec3;
use bevy::prelude::Color;

pub const PADDLE_SIZE: Vec3 = Vec3::new(20.0, 120.0, 0.0);
pub const PADDLE_COLOR: Color = Color::rgb(0.3, 0.3, 0.7);
pub const GAP_FROM_WALL: f32 = 60.0;

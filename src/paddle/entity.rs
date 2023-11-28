use bevy::math::Vec3;
use bevy::prelude::Component;

#[derive(Component)]
pub struct Paddle;
#[derive(Component)]
pub struct UserControlled;
#[derive(Component)]
pub struct AIControlled;
#[derive(Component)]
pub struct PaddleLocation(pub Vec3);


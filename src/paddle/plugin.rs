use bevy::app::{App, Plugin, Startup};
use crate::paddle::system::paddle_init_systems;

pub struct PaddlePlugin;
impl Plugin for PaddlePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, paddle_init_systems);
    }
}


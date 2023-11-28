use bevy::app::{App, Plugin, Update};
use bevy::input::keyboard::keyboard_input_system;
use crate::interaction::system::handle_keypress_system;
use crate::log;

pub struct InteractionsPlugin;
impl Plugin for InteractionsPlugin {
    fn build(&self, app: &mut App) {
        log("Loading Interactions Plugin");

        app
            .add_systems(
                Update,
                (
                    keyboard_input_system,
                    handle_keypress_system
                )
            );
    }

}
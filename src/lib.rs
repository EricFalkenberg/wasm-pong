mod utils;
mod paddle;
mod interaction;

use bevy::app::{App, Plugin};
use bevy::DefaultPlugins;
use bevy::input::InputPlugin;
use bevy::prelude::{Bundle, Camera2dBundle, Commands, Component, Event, EventReader, EventWriter, Query, Res, ResMut, Resource, TextBundle, With, Style, Local, Transform, Color};
use bevy::time::{Timer};
use wasm_bindgen::prelude::*;
use crate::interaction::plugin::InteractionsPlugin;
use crate::paddle::plugin::PaddlePlugin;
use crate::utils::set_panic_hook;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log(s: &str);
}

#[derive(Component)]
struct Person;
#[derive(Bundle)]
struct PersonBundle {
    name: Name,
    xp: XP
}
#[derive(Component, Clone)]
struct Name(String);
#[derive(Component)]
struct XP(u32);
#[derive(Resource)]
struct GreetTimer(Timer);

fn add_people_system(mut commands: Commands) {
    for name in vec![
        "Eric Falkenberg",
        "Angela Freeman",
        "Sam Donneley",
        "Francisco Lopez",
        "Steve Silkey",
        "Eli Walsh",
        "Austin O'Seep",
        "John Sellers"
    ] {
        commands.spawn(
            (
                Person,
                PersonBundle {
                    name: Name(name.to_string()),
                    xp: XP(1000)
                }
            )
        );
    }
}

#[derive(Component)]
struct ColorText;
// #[derive(Component)]
// struct Paddle;

fn setup_camera_system(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    // const BOTTOM_WALL: f32 = -300.0;
    // const GAP_BETWEEN_PADDLE_AND_FLOOR: f32 = 60.0;
    // const PADDLE_SIZE: Vec3 = Vec3::new(120.0, 20.0, 0.0);
    // const PADDLE_COLOR: Color = Color::rgb(0.3, 0.3, 0.7);
    //
    // let paddle_y = BOTTOM_WALL + GAP_BETWEEN_PADDLE_AND_FLOOR;
    //
    // commands.spawn((
    //     SpriteBundle {
    //         transform: Transform {
    //             translation: Vec3::new(0.0, paddle_y, 0.0),
    //             scale: PADDLE_SIZE,
    //             ..default()
    //         },
    //         sprite: Sprite {
    //             color: PADDLE_COLOR,
    //             ..default()
    //         },
    //         ..default()
    //     },
    //     Paddle,
    // ));
}

// #[derive(Event)]
// struct GreetEvent(Name);
//
// fn greet_people_system(
//     mut ev_greet: EventWriter<GreetEvent>,
//     time: Res<Time>,
//     mut timer: ResMut<GreetTimer>,
//     query: Query<&Name, With<Person>>
// ) {
//     timer.0.tick(time.delta());
//     if timer.0.just_finished() {
//         for name in &query {
//             ev_greet.send(GreetEvent(name.clone()));
//         }
//     }
// }
//
// #[derive(Default)]
// struct LinesWritten(u32);
//
// fn write_greeting(
//     mut ev_greet: EventReader<GreetEvent>,
//     mut commands: Commands,
//     mut lines_written: Local<LinesWritten>
// ) {
//     for event in ev_greet.read() {
//         commands.spawn(
//             (
//                 TextBundle::from_section(
//                     format!("Hello {0}!", event.0.0),
//                     TextStyle {
//                         font_size: 20.0,
//                         ..default()
//                     },
//                 )
//                     .with_text_alignment(TextAlignment::Center)
//                     .with_style(Style {
//                         position_type: PositionType::Absolute,
//                         left: Val::Px(25.0),
//                         top: Val::Px(((lines_written.0) as f32)*25.0),
//                         ..default()
//                     }),
//                 ColorText
//             )
//         );
//         lines_written.0 += 1;
//     }
// }
//
// fn debug_greetings(
//     mut ev_greet: EventReader<GreetEvent>,
// ) {
//     for event in ev_greet.read() {
//         log(&format!("Greeting for {0} written to screen", event.0.0));
//     }
// }
//
// pub struct PeoplePlugin;
// impl Plugin for PeoplePlugin {
//     fn build(&self, app: &mut App) {
//         log("Loading People Plugin");
//         app
//             .insert_resource(GreetTimer(Timer::from_seconds(5.0, TimerMode::Repeating)))
//             .add_systems(
//                 Startup,
//                 (
//                     setup_camera_system,
//                     add_people_system
//                 )
//             )
//             .add_systems(
//                 Update,
//                 (
//                     greet_people_system,
//                     write_greeting,
//                     debug_greetings
//                 )
//             )
//             .add_event::<GreetEvent>();
//     }
// }

#[wasm_bindgen]
pub fn run() {
    set_panic_hook();
    App::new()
        .add_plugins(
            (
                DefaultPlugins,
                PaddlePlugin,
                InteractionsPlugin
            )
        )
        .run()
}

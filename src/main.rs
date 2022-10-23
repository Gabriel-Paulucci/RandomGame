mod components;
mod systems;

use bevy::{
    log::{Level, LogSettings},
    prelude::App,
    window::{MonitorSelection, PresentMode, WindowDescriptor, WindowPosition},
    DefaultPlugins,
};
use components::asteroide::spawn_asteroide;
use systems::{
    camera::spawn_camera,
    moviment::{move_asteroid_system, move_system},
    position::{update_mouse_position_system, MouseWorldPosition},
};

fn main() {
    let window = WindowDescriptor {
        title: String::from("Random Game"),
        present_mode: PresentMode::AutoVsync,
        position: WindowPosition::Centered(MonitorSelection::Primary),
        resizable: false,
        height: 720.,
        width: 1280.,
        ..Default::default()
    };

    App::new()
        .insert_resource(window)
        .init_resource::<MouseWorldPosition>()
        .add_plugins(DefaultPlugins)
        .insert_resource(LogSettings {
            filter: "debug".into(),
            level: Level::DEBUG,
        })
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_asteroide)
        .add_system(update_mouse_position_system)
        .add_system(move_system)
        .add_system(move_asteroid_system)
        .run()
}

use bevy::prelude::{Camera2d, Camera2dBundle, Color, Commands};

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle {
        camera_2d: Camera2d {
            clear_color: bevy::core_pipeline::clear_color::ClearColorConfig::Custom(
                Color::hex("0e022b").unwrap(),
            ),
        },
        ..Default::default()
    });
}

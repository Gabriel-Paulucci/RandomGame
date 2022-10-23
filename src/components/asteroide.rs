use bevy::{
    prelude::{debug, AssetServer, Commands, Component, Res, Transform, Vec3},
    sprite::SpriteBundle,
};

#[derive(Component, Debug)]
pub struct Asteroide;

pub fn spawn_asteroide(mut commands: Commands, asset_server: Res<AssetServer>) {
    debug!("Create asteroide");
    commands
        .spawn()
        .insert(Asteroide)
        .insert_bundle(SpriteBundle {
            texture: asset_server.load("asteroide.png"),
            transform: Transform {
                scale: Vec3::splat(1.),
                ..Default::default()
            },
            ..Default::default()
        });
}

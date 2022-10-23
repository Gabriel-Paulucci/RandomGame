use bevy::{
    prelude::{Commands, Component, Entity, Input, MouseButton, Query, Res, Transform, Vec3, With},
    time::Time,
};

use crate::components::asteroide::Asteroide;

use super::position::MouseWorldPosition;

#[derive(Component)]
pub struct GoTo {
    pub go: Vec3,
    pub to: Vec3,
}

pub fn move_system(
    mut commands: Commands,
    mouse: Res<MouseWorldPosition>,
    button: Res<Input<MouseButton>>,
    mut asteroide: Query<(Entity, &Transform), With<Asteroide>>,
) {
    if button.just_pressed(MouseButton::Left) {
        for (entity, trasform) in asteroide.iter_mut() {
            commands.entity(entity).insert(GoTo {
                to: Vec3 {
                    x: mouse.0.x,
                    y: mouse.0.y,
                    z: 0.,
                },
                go: trasform.translation,
            });
        }
    }
}

pub fn move_asteroid_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Transform, &GoTo), With<Asteroide>>,
) {
    for (entity, mut transform, goto) in query.iter_mut() {
        let scale = goto.to - goto.go;
        let distance = goto.go.distance(goto.to);

        transform.translation += scale / distance * time.delta_seconds() * 500.;

        if transform.translation == goto.go {
            commands.entity(entity).remove::<GoTo>();
        }
    }
}

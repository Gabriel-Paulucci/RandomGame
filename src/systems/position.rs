use bevy::{
    prelude::{Camera, Query, Res, ResMut, Transform, Vec2},
    window::Windows,
};

#[derive(Default)]
pub struct MouseWorldPosition(pub Vec2);

pub fn update_mouse_position_system(
    mut position: ResMut<MouseWorldPosition>,
    windows: Res<Windows>,
    query: Query<(&Camera, &Transform)>,
) {
    let window = windows.get_primary().unwrap();

    let (_, trasform) = query.single();

    let center = trasform.translation.truncate();
    let half_width = (window.width() / 2.) * trasform.scale.x;
    let half_height = (window.height() / 2.) * trasform.scale.y;
    let left = center.x - half_width;
    let bottom = center.y - half_height;

    if let Some(cursor) = window.cursor_position() {
        position.0 = Vec2::new(
            left + cursor.x * trasform.scale.x,
            bottom + cursor.y * trasform.scale.y,
        )
    }
}

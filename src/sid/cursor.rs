use crate::sid::*;

#[derive(Default, Resource)]
pub struct Cursor(pub Vec3);

pub fn update_cursor(
    camera: Query<(&Camera, &GlobalTransform)>,
    ground: Query<&GlobalTransform, With<Ground>>,
    window: Query<&Window>,
    mut cursor: ResMut<Cursor>,
) {
    let ground = ground.single();
    let (camera, transform) = camera.single();
    let Some(position) = window.single().cursor_position() else {
        return;
    };
    let Some(ray) = camera.viewport_to_world(transform, position) else {
        return;
    };
    if let Some(distance) =
        ray.intersect_plane(ground.translation(), InfinitePlane3d::new(ground.up()))
    {
        cursor.0 = ray.get_point(distance);
    };
}

pub fn draw_cursor(
    mut gizmos: Gizmos,
    cursor: Res<Cursor>,
    ground: Query<&Transform, With<Ground>>,
) {
    let cursor = cursor.0;
    let ground = ground.single();
    gizmos.circle(
        cursor + ground.up() * 0.01,
        ground.up(),
        0.3,
        Color::srgb(0.5, 0.0, 0.0),
    );
}

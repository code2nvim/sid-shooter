use crate::sid::*;

pub struct CursorPlugin;

impl Plugin for CursorPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Cursor((0.0, 0.0, 0.0).into()))
            .add_systems(Update, (update_cursor, draw_cursor).chain());
    }
}

#[derive(Resource)]
pub struct Cursor(pub Vec3);

fn update_cursor(
    camera: Query<(&Camera, &GlobalTransform)>,
    ground: Query<&GlobalTransform, With<Ground>>,
    windows: Query<&Window>,
    mut cursor: ResMut<Cursor>,
) {
    let ground = ground.single();
    let (camera, transform) = camera.single();
    let Some(position) = windows.single().cursor_position() else {
        return;
    };
    let Some(ray) = camera.viewport_to_world(transform, position) else {
        return;
    };
    let Some(distance) =
        ray.intersect_plane(ground.translation(), InfinitePlane3d::new(ground.up()))
    else {
        return;
    };
    cursor.0 = ray.get_point(distance);
}

fn draw_cursor(cursor: Res<Cursor>, ground: Query<&Transform, With<Ground>>, mut gizmos: Gizmos) {
    let cursor = cursor.0;
    let ground = ground.single();
    gizmos.circle(
        cursor + ground.up() * 0.01,
        ground.up(),
        0.3,
        bevy::color::palettes::css::DARK_RED
    );
}

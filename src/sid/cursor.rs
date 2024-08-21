use crate::sid::*;

pub struct CursorPlugin;

impl Plugin for CursorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_cursor)
            .add_systems(Update, (update_cursor, draw_cursor).chain());
    }
}

#[derive(Component)]
pub struct Cursor(pub Vec3);

fn spawn_cursor(mut commands: Commands) {
    commands.spawn(Cursor((0.0, 0.0, 0.0).into()));
}

fn update_cursor(
    camera: Query<(&Camera, &GlobalTransform)>,
    ground: Query<&GlobalTransform, With<Ground>>,
    windows: Query<&Window>,
    mut cursor: Query<&mut Cursor>,
) {
    let (camera, transform) = camera.single();
    let ground = ground.single();
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
    let point = ray.get_point(distance);
    let mut cursor = cursor.get_single_mut().unwrap();
    cursor.0 = point;
}

fn draw_cursor(ground: Query<&Transform, With<Ground>>, cursor: Query<&Cursor>, mut gizmos: Gizmos) {
    let ground = ground.single();
    let point = cursor.get_single().unwrap();
    gizmos.circle(point.0 + ground.up() * 0.01, ground.up(), 0.2, Color::WHITE);
}

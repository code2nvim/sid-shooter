use bevy::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera);
    }
}

#[derive(Component)]
struct Camera;

pub fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(-10.0, 30.0, 50.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        Camera,
    ));
}

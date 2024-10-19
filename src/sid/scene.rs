use crate::sid::*;

#[derive(Component)]
pub struct GameScene;

#[derive(Component)]
pub struct Ground;

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 35.0, -25.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        GameScene,
    ));
}

pub fn spawn_ground(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Plane3d::default().mesh().size(GROUND_WIDTH, GROUND_HEIGHT)),
            material: materials.add(Color::srgb(0.0, 1.0, 0.0)),
            ..default()
        },
        GameScene,
        Ground,
    ));
}

pub fn spawn_light(mut commands: Commands) {
    commands.spawn((
        PointLightBundle {
            transform: Transform::from_xyz(0.0, 40.0, -50.0),
            point_light: PointLight {
                intensity: 10_000_000.0,
                range: 1500.0,
                shadows_enabled: true,
                ..default()
            },
            ..default()
        },
        GameScene,
    ));
}

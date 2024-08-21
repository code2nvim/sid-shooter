use crate::sid::*;

pub struct ScenePlugin;

impl Plugin for ScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_ground, spawn_light));
    }
}

#[derive(Component)]
pub struct Ground;

fn spawn_ground(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Plane3d::default().mesh().size(30.0, 30.0)),
            material: materials.add(Color::srgb(0.0, 1.0, 0.0)),
            ..default()
        },
        Ground,
    ));
}

#[allow(unused)]
fn spawn_light(mut commands: Commands) {
    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(0.0, 20.0, 0.0),
        point_light: PointLight {
            range: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        ..default()
    });
}

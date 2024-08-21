use crate::sid::*;

pub struct ScenePlugin;

impl Plugin for ScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_scene);
    }
}

#[derive(Component)]
struct Ground;

fn spawn_scene(
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
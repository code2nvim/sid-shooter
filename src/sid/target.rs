use crate::sid::*;

#[derive(Component)]
pub struct Target;

#[derive(Resource)]
pub struct SpawnTimer(pub Timer);

pub fn spawn_target(
    cursor: Res<Cursor>,
    time: Res<Time>,
    mut timer: ResMut<SpawnTimer>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        commands.spawn((
            PbrBundle {
                mesh: meshes.add(Sphere::new(0.5)),
                material: materials.add(Color::srgb(0.0, 0.0, 0.0)),
                transform: Transform::from_translation(cursor.0),
                ..default()
            },
            Target,
        ));
    }
}

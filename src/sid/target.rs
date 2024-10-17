use crate::sid::*;

#[derive(Component)]
pub struct Target;

pub fn spawn_target(
    time: Res<Time>,
    mut timer: ResMut<TargetTimer>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        commands.spawn((
            PbrBundle {
                mesh: meshes.add(Sphere::new(TARGET_SIZE)),
                material: materials.add(Color::srgb(0.0, 0.0, 0.0)),
                transform: Transform::from_translation((0.0, 0.0, 30.0).into()),
                ..default()
            },
            Target,
            Speed(TARGET_SPEED),
        ));
    }
}

pub fn move_target(time: Res<Time>, mut targets: Query<(&mut Transform, &Speed), With<Target>>) {
    for (mut transform, speed) in targets.iter_mut() {
        let movement =
            Vec3::new(0.0, 0.0, -1.0).normalize_or_zero() * time.delta_seconds() * speed.0;
        transform.translation += movement;
    }
}

use crate::sid::*;

#[derive(Component)]
pub struct Wall;

#[derive(Resource)]
pub struct WallTimer(pub Timer);

pub fn spawn_wall(
    time: Res<Time>,
    mut timer: ResMut<WallTimer>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        commands
            .spawn((
                PbrBundle {
                    mesh: meshes.add(Cuboid::new(WALL_SIZE, WALL_SIZE, 0.001)),
                    material: materials.add(Color::srgba(0.0, 0.0, 0.0, 0.8)),
                    transform: Transform::from_translation(
                        (0.0, WALL_SIZE / 2.0, SPAWN_LINE).into(),
                    ),
                    ..default()
                },
                Wall,
                Speed(WALL_SPEED),
            ))
            .insert(RigidBody::Dynamic)
            .insert(Collider::cuboid(WALL_SIZE, WALL_SIZE, 0.001));
    }
}

pub fn move_wall(
    time: Res<Time>,
    mut targets: Query<(&mut Transform, &Speed), With<Wall>>,
) {
    for (mut transform, speed) in targets.iter_mut() {
        let movement =
            Vec3::new(0.0, 0.0, -1.0).normalize_or_zero() * time.delta_seconds() * speed.0;
        transform.translation += movement;
    }
}

use crate::sid::*;

#[derive(Component)]
pub struct Player;

pub fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Sphere::new(1.0)),
            material: materials.add(Color::srgb(0.5, 0.5, 0.5)),
            transform: Transform::from_xyz(0.0, 1.0, 0.0),
            ..default()
        },
        Player,
        Movement((0.0, 0.0, 0.0).into()),
    ));
}

pub fn update_direction(
    mut direction: Query<&mut Movement, With<Player>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    let Ok(mut direction) = direction.get_single_mut() else {
        return;
    };
    direction.0 = {
        let mut direction = Vec3::ZERO;
        if keys.pressed(KeyCode::KeyW) {
            direction += Vec3::new(0.0, 0.0, 1.0);
        }
        if keys.pressed(KeyCode::KeyA) {
            direction += Vec3::new(1.0, 0.0, 0.0);
        }
        if keys.pressed(KeyCode::KeyS) {
            direction += Vec3::new(0.0, 0.0, -1.0);
        }
        if keys.pressed(KeyCode::KeyD) {
            direction += Vec3::new(-1.0, 0.0, 0.0);
        }
        direction
    };
}

pub fn move_player(mut player: Query<(&mut Transform, &Movement), With<Player>>, time: Res<Time>) {
    let Ok((mut transform, direction)) = player.get_single_mut() else {
        return;
    };
    let movement = direction.0.normalize_or_zero() * time.delta_seconds() * PLAYER_SPEED;
    transform.translation += movement;
}

pub fn rotate_player(cursor: Res<Cursor>, mut player: Query<&mut Transform, With<Player>>) {
    let Ok(mut transform) = player.get_single_mut() else {
        return;
    };
    let cursor = cursor.0;
    let player = transform.translation;
    let direction = (cursor - player).normalize();
    transform.rotation = Quat::from_rotation_y(direction.x.atan2(direction.z));
}

use crate::sid::*;

#[derive(Component)]
pub struct Player;

pub fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        SceneBundle {
            scene: asset_server.load(GltfAssetLabel::Scene(0).from_asset("player.glb")),
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
    let mut direction = direction.get_single_mut().unwrap();
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

pub fn move_player(
    mut player: Query<(&mut Transform, &Movement), With<Player>>,
    time: Res<Time>,
) {
    for (mut transform, direction) in player.iter_mut() {
        let movement = direction.0.normalize_or_zero() * time.delta_seconds() * PLAYER_SPEED;
        transform.translation += movement;
    }
}

pub fn rotate_player(mut player: Query<&mut Transform, With<Player>>, cursor: Res<Cursor>) {
    let mut transform = player.get_single_mut().unwrap();
    let cursor = cursor.0;
    let player = transform.translation;
    let direction = (cursor - player).normalize();
    transform.rotation = Quat::from_rotation_y(direction.x.atan2(direction.z));
}

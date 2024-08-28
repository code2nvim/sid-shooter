use crate::sid::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, (update_direction, move_player, rotate_player));
    }
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct KeyDirection(Vec3);

#[derive(Component)]
struct Speed(f32);

fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        SceneBundle {
            scene: asset_server.load(GltfAssetLabel::Scene(0).from_asset("player.glb")),
            ..default()
        },
        Player,
        Speed(30.0),
        KeyDirection((0.0, 0.0, 0.0).into()),
    ));
}

fn update_direction(
    mut direction: Query<&mut KeyDirection, With<Player>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    let mut direction = direction.get_single_mut().unwrap();
    direction.0 = {
        let mut direction = Vec3::ZERO;
        if keys.pressed(KeyCode::KeyW) {
            direction += Vec3::new(15.0, 0.0, 0.0);
        }
        if keys.pressed(KeyCode::KeyA) {
            direction += Vec3::new(0.0, 0.0, -15.0);
        }
        if keys.pressed(KeyCode::KeyS) {
            direction += Vec3::new(-15.0, 0.0, 0.0);
        }
        if keys.pressed(KeyCode::KeyD) {
            direction += Vec3::new(0.0, 0.0, 15.0);
        }
        direction
    };
}

fn move_player(
    mut query: Query<(&mut Transform, &KeyDirection, &Speed), With<Player>>,
    time: Res<Time>,
) {
    for (mut transform, direction, speed) in query.iter_mut() {
        let movement = direction.0.normalize_or_zero() * speed.0 * time.delta_seconds();
        transform.translation += movement;
    }
}

fn rotate_player(mut query: Query<&mut Transform, With<Player>>, cursor: Res<Cursor>) {
    let mut transform = query.get_single_mut().unwrap();
    let cursor = cursor.0;
    let player = transform.translation;
    let direction = (cursor - player).normalize();
    transform.rotation = Quat::from_rotation_y(direction.x.atan2(direction.z));
}

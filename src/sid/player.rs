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
struct Direction(Vec3);

#[derive(Component)]
struct Speed(f32);

fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        SceneBundle {
            scene: asset_server.load(GltfAssetLabel::Scene(0).from_asset("player.glb")),
            ..default()
        },
        Player,
        Speed(15.0),
        Direction((0.0, 0.0, 9.0).into()),
    ));
}

fn update_direction(
    keys: Res<ButtonInput<KeyCode>>,
    camera: Query<&Transform, With<Camera3d>>,
    mut direction: Query<&mut Direction, With<Player>>,
) {
    let camera = camera.get_single().unwrap();
    let mut direction = direction.get_single_mut().unwrap();
    direction.0 = {
        let mut direction = Vec3::ZERO;
        if keys.pressed(KeyCode::KeyW) {
            direction += *camera.forward();
        }
        if keys.pressed(KeyCode::KeyS) {
            direction += *camera.back();
        }
        if keys.pressed(KeyCode::KeyA) {
            direction += *camera.left();
        }
        if keys.pressed(KeyCode::KeyD) {
            direction += *camera.right();
        }
        direction.y = 0.0;
        direction
    };
}

fn move_player(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &Direction, &Speed), With<Player>>,
) {
    for (mut transform, direction, speed) in query.iter_mut() {
        let movement = direction.0.normalize_or_zero() * speed.0 * time.delta_seconds();
        transform.translation += movement;
    }
}

fn rotate_player(cursor: Query<&Cursor>, mut query: Query<&mut Transform, With<Player>>) {
    let mut transform = query.get_single_mut().unwrap();
    let cursor = cursor.get_single().unwrap();
    let player = transform.translation;
    let direction = (cursor.0 - player).normalize();
    transform.rotation = Quat::from_rotation_y(direction.x.atan2(direction.z));
}

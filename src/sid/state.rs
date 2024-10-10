use crate::sid::*;

pub struct GamePlugin;
pub struct PlayingPlugin;
pub struct MenuPlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .init_resource::<Cursor>()
            .add_systems(Startup, (spawn_camera, spawn_ground, spawn_light))
            .add_systems(Update, switch_state);
    }
}

impl Plugin for PlayingPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SpawnTimer(Timer::from_seconds(0.5, TimerMode::Repeating)))
            .add_systems(OnEnter(GameState::Playing), spawn_player)
            .add_systems(
                Update,
                (
                    (update_cursor, draw_cursor),
                    (update_direction, move_player, rotate_player),
                    (spawn_target, move_target),
                    (shoot),
                )
                    .run_if(in_state(GameState::Playing)),
            );
    }
}

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnExit(GameState::Playing), teardown);
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
pub enum GameState {
    #[default]
    Menu,
    Playing,
}

pub fn switch_state(key: Res<ButtonInput<KeyCode>>, mut next: ResMut<NextState<GameState>>) {
    if key.pressed(KeyCode::Enter) {
        next.set(GameState::Playing);
    }
    if key.pressed(KeyCode::Space) {
        next.set(GameState::Menu);
    }
}

fn teardown(
    mut commands: Commands,
    entities: Query<Entity, (Without<GameScene>, Without<Window>)>,
) {
    for entity in &entities {
        commands.entity(entity).despawn();
    }
}

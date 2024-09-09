use crate::sid::*;

pub struct PlayingPlugin;

impl Plugin for PlayingPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .init_resource::<Cursor>()
            .insert_resource(SpawnTimer(Timer::from_seconds(0.5, TimerMode::Repeating)))
            .add_systems(Startup, (spawn_camera, spawn_ground, spawn_light))
            .add_systems(Update, press_to_start)
            .add_systems(OnEnter(GameState::Playing), spawn_player)
            .add_systems(
                Update,
                (
                    (update_cursor, draw_cursor).chain(),
                    (update_direction, move_player, rotate_player),
                    (spawn_target, move_target),
                )
                    .run_if(in_state(GameState::Playing)),
            );
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
pub enum GameState {
    #[default]
    Menu,
    Playing,
}

pub fn press_to_start(
    key: Res<ButtonInput<KeyCode>>,
    current: Res<State<GameState>>,
    mut next: ResMut<NextState<GameState>>,
) {
    if key.pressed(KeyCode::Enter) {
        next.set(match current.get() {
            GameState::Menu => GameState::Playing,
            GameState::Playing => GameState::Menu,
        });
    }
}

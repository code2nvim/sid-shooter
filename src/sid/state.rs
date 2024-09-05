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
                    (spawn_target),
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
    mut game_state: ResMut<NextState<GameState>>,
    kerpress: Res<ButtonInput<KeyCode>>,
) {
    if kerpress.pressed(KeyCode::Space) {
        game_state.set(GameState::Playing);
    }
}

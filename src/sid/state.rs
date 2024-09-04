use crate::sid::*;

pub struct PlayingPlugin;

impl Plugin for PlayingPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Cursor>()
            .init_state::<GameState>()
            .add_systems(Startup, (spawn_camera, spawn_ground, spawn_light))
            .add_systems(OnEnter(GameState::Playing), spawn_player)
            .add_systems(
                Update,
                (
                    (update_cursor, draw_cursor).chain(),
                    (update_direction, move_player, rotate_player),
                )
                    .run_if(in_state(GameState::Playing)),
            );
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
pub enum GameState {
    #[default]
    Playing,
}

mod sid;

use sid::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb_u8(23, 147, 209)))
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "sid-shooter".into(),
                    ..default()
                }),
                ..default()
            }),
            CameraPlugin,
            PlayerPlugin,
            ScenePlugin,
        ))
        .run();
}

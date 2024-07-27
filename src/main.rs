use bevy::prelude::*;
use sid::prelude::*;

mod sid;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.5, 0.5, 0.5)))
        .add_plugins((DefaultPlugins, CameraPlugin, PlayerPlugin, ScenePlugin))
        .run();
}

mod camera;
mod player;
mod scene;

pub mod prelude {
    pub use super::camera::CameraPlugin;
    pub use super::player::PlayerPlugin;
    pub use super::scene::ScenePlugin;
}

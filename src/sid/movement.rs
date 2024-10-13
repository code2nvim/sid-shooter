use crate::sid::*;

#[derive(Component)]
pub struct Movement(pub Vec3);

#[derive(Component)]
pub struct Speed(pub f32);

#[derive(Resource)]
pub struct SpawnTimer(pub Timer);

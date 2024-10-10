use crate::sid::*;

pub fn shoot(
    cursor: Res<Cursor>,
    target: Query<(Entity, &Transform), With<Target>>,
    mut commands: Commands,
) {
    for (entity, transform) in target.iter() {
        let distance = transform.translation.distance(cursor.0);
        if distance < TARGET_SIZE {
            commands.entity(entity).despawn();
        }
    }
}

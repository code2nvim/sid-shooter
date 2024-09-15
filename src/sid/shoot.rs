use crate::sid::*;

pub fn shoot(cursor: Res<Cursor>, target: Query<&Transform>) {
    for target in target.iter() {
        let distance = target.translation.distance(cursor.0);
        if distance < 1.0 {
            print!("shoot...");
        }
    }
}

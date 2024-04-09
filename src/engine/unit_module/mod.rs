use comfy::*;

use super::UNIT_Z_INDEX;

pub struct Unit;

pub fn initialize_units() {
    commands().spawn((Transform::position(vec2(10.0, 0.0)), Unit));
    commands().spawn((Transform::position(vec2(20.0, 0.0)), Unit));
    commands().spawn((Transform::position(vec2(30.0, 0.0)), Unit));
}

pub fn draw_units() {
    for (_, (transform, _)) in world().query::<(&Transform, &Unit)>().iter() {
        draw_circle(transform.position, 0.5, RED, UNIT_Z_INDEX);
    }
}

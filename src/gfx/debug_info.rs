use crate::phx::{BodySet, ColliderSet, ColliderTag};
use glam::Vec2;
use legion::systems::Resources;
use macroquad::{draw_rectangle, GREEN, YELLOW};
use resphys::{Collider, ColliderState};

pub fn visualize_hitboxes(resources: &Resources) {
    let bodies = resources.get::<BodySet>().unwrap();
    let colliders = resources.get::<ColliderSet>().unwrap();

    for (_, collider) in colliders.iter() {
        let body = &bodies[collider.owner];
        draw_collider(&collider, body.position);
    }
}

fn draw_collider(collider: &Collider<ColliderTag>, position: Vec2) {
    let mut color = match collider.state {
        ColliderState::Solid => GREEN,
        ColliderState::Sensor => YELLOW,
    };

    color.0[3] = (0.6 * 255.) as u8;

    let wh = collider.shape.half_exts;
    let x_pos = position.x() - wh.x() + collider.offset.x();
    let y_pos = position.y() - wh.y() + collider.offset.y();
    draw_rectangle(x_pos, y_pos, wh.x() * 2., wh.y() * 2., color);
}

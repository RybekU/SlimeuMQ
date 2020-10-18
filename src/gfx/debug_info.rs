use crate::game::combat::HurtQueue;
use crate::phx::{BodySet, ColliderSet, ColliderTag};
use glam::Vec2;
use legion::systems::Resources;
use macroquad::{draw_rectangle, GREEN, RED, YELLOW};
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

pub fn visualize_hurtboxes(resources: &Resources) {
    let hurt_queue = resources.get::<HurtQueue>().unwrap();

    let mut color = RED;
    color.0[3] = (0.6 * 255.) as u8;

    for hurt_info in hurt_queue.copy_msgs.iter() {
        let actual_pos = hurt_info.position - hurt_info.half_exts;
        draw_rectangle(
            actual_pos.x(),
            actual_pos.y(),
            hurt_info.half_exts.x() * 2.,
            hurt_info.half_exts.y() * 2.,
            color,
        );
    }
    //
}

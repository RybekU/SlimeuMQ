use crate::phx::{BodySet, ColliderSet, ColliderTag, PhysicsWorld, Velocity};
use crate::FRAMETIME;
use glam::Vec2;
use legion::{system, Resources};
use resphys::ColliderHandle;
#[derive(Debug)]
pub struct Gravity {
    enabled: bool,
    strength: Vec2,
}

impl Gravity {
    pub fn new(strength: Vec2) -> Self {
        Self {
            strength,
            enabled: true,
        }
    }
}

// FRAME_DEPENDANT
// https://answers.unity.com/questions/1528714/jump-not-framerate-independent.html
#[system(for_each)]
pub fn gravity(vel: &mut Velocity, gravity: &Gravity) {
    if gravity.enabled {
        vel.src += gravity.strength * FRAMETIME * 60.;
    }
}

#[derive(Debug)]
pub struct OnGround {
    pub sensor_handle: ColliderHandle,
    pub on_ground: bool,
}

impl OnGround {
    /// Places the sensor 1 pixel below the collider that it is supposed to check for.
    /// The sensor is 1 pixel tall and collider-1 pixels wide.
    /// It's mask should be set in a way it only collides with ground.
    pub fn new(resources: &Resources, checked_chandle: resphys::ColliderHandle) -> Self {
        let mut physics = resources.get_mut::<PhysicsWorld>().unwrap();
        let mut bodies = resources.get_mut::<BodySet>().unwrap();
        let mut colliders = resources.get_mut::<ColliderSet>().unwrap();

        let (sensor, owner) = {
            let checked_collider = &colliders[checked_chandle];
            let offset =
                checked_collider.offset + Vec2::new(0., checked_collider.shape.half_exts.y() + 0.5);

            // the sensor is under the collider, and tiny bit thinner to not trigger on walls
            let sensor = resphys::builder::ColliderDesc::new(
                resphys::AABB {
                    half_exts: Vec2::new(checked_collider.shape.half_exts.x() - 0.5, 0.5),
                },
                ColliderTag::Player,
            )
            .sensor()
            .with_offset(offset)
            .with_mask(super::Category::GROUND.bits());
            (sensor, checked_collider.owner)
        };

        let sensor_handle = colliders
            .insert(sensor.build(owner), &mut bodies, &mut physics)
            .unwrap();
        Self {
            sensor_handle,
            on_ground: true,
        }
    }
}

// could this be part of gravity system if all components will use both?
#[system(for_each)]
pub fn ground_check(#[resource] phys_world: &PhysicsWorld, ground_data: &mut OnGround) {
    ground_data.on_ground = phys_world
        .interactions_of(ground_data.sensor_handle)
        .next()
        .is_some();
}

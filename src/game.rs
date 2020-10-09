mod player;

use legion::{
    systems::{Resources, Schedule},
    world::World,
};
use macroquad::Texture2D;

use crate::util::{ButtonsState, SimpleCam2D};
type ImageStorage = fxhash::FxHashMap<String, Texture2D>;

pub struct Game {
    pub world: World,
    pub resources: Resources,
    pub schedule: Schedule,
    pub textures: ImageStorage,

    pub camera: SimpleCam2D,
}

impl Game {
    pub fn new() -> Self {
        let world = World::default();
        let resources = init_resources();
        let schedule = init_schedule();

        let images = ImageStorage::default();
        let camera = SimpleCam2D::with_zoom(crate::GAME_SCALE as f32);
        Self {
            world,
            resources,
            schedule,
            textures: images,
            camera,
        }
    }
    pub async fn init(&mut self) {
        use self::player::PlayerControlled;
        use crate::gfx::Sprite;
        use crate::phx::{Gravity, Hitbox, Position, Velocity};
        use glam::Vec2;

        let texture: Texture2D = macroquad::load_texture("media/slimeu_base-b.png").await;
        macroquad::set_texture_filter(texture, macroquad::FilterMode::Nearest);

        self.textures.insert("slimeu_base".into(), texture);

        self.world
            .push((Hitbox::new(makeshift_static_platform(&self.resources)),));
        self.world.push((
            Position {
                src: Vec2::new(10.0, 10.0),
            },
            Sprite::new(
                "slimeu_base".to_owned(),
                0.,
                0.,
                texture.width(),
                texture.height(),
            ),
        ));
        self.world.push((
            Position {
                src: Vec2::new(30.0, 10.0),
            },
            Sprite::new(
                "slimeu_base".to_owned(),
                0.,
                0.,
                texture.width(),
                texture.height(),
            ),
            Velocity {
                src: Vec2::new(0., 0.),
            },
            Gravity::new(Vec2::new(0.0, 8.0)),
            Hitbox::new(makeshift_dynamic_collider(&self.resources)),
            PlayerControlled::new(),
        ));
        self.world.push((
            Position {
                src: Vec2::new(50.0, 10.0),
            },
            Sprite::new(
                "slimeu_base".to_owned(),
                0.,
                0.,
                texture.width(),
                texture.height(),
            ),
        ));
    }
    pub fn update(&mut self) {
        // input should be updated on the main thread
        self.resources.get_mut::<ButtonsState>().unwrap().update();
        self.schedule.execute(&mut self.world, &mut self.resources);
    }
}

fn init_resources() -> Resources {
    let mut resources = Resources::default();
    resources.insert(crate::phx::PhysicsWorld::new());
    resources.insert(crate::phx::BodySet::new());
    resources.insert(crate::phx::ColliderSet::new());
    resources.insert(crate::util::ButtonsState::new());
    resources
}

fn init_schedule() -> Schedule {
    Schedule::builder()
        .add_system(crate::phx::gravity_system())
        .add_system(self::player::update_fsm_system())
        .add_system(crate::phx::resphys_presync_system())
        .add_system(crate::phx::resphys_sync_system())
        .add_system(crate::phx::temp::reset_velocity_system())
        .build()
}

fn makeshift_static_platform(resources: &Resources) -> resphys::ColliderHandle {
    use crate::phx::{BodySet, ColliderSet, ColliderTag, PhysicsWorld};
    use glam::Vec2;

    let mut physics = resources.get_mut::<PhysicsWorld>().unwrap();
    let mut bodies = resources.get_mut::<BodySet>().unwrap();
    let mut colliders = resources.get_mut::<ColliderSet>().unwrap();

    let body = resphys::builder::BodyDesc::new()
        .with_position(Vec2::new(70., 80.))
        .make_static()
        .build();
    let collider = resphys::builder::ColliderDesc::new(
        resphys::AABB {
            half_exts: Vec2::new(60., 8.),
        },
        ColliderTag::Tile,
    );

    let bhandle = bodies.insert(body);
    colliders
        .insert(collider.build(bhandle), &mut bodies, &mut physics)
        .unwrap()
}

fn makeshift_dynamic_collider(resources: &Resources) -> resphys::ColliderHandle {
    use crate::phx::{BodySet, ColliderSet, ColliderTag, PhysicsWorld};
    use glam::Vec2;

    let mut physics = resources.get_mut::<PhysicsWorld>().unwrap();
    let mut bodies = resources.get_mut::<BodySet>().unwrap();
    let mut colliders = resources.get_mut::<ColliderSet>().unwrap();

    let body = resphys::builder::BodyDesc::new()
        .with_position(Vec2::new(30., 10.))
        .build();
    let collider = resphys::builder::ColliderDesc::new(
        resphys::AABB {
            half_exts: Vec2::new(5., 4.),
        },
        ColliderTag::Player,
    )
    .with_offset(Vec2::new(0., 4.));

    let bhandle = bodies.insert(body);
    colliders
        .insert(collider.build(bhandle), &mut bodies, &mut physics)
        .unwrap()
}

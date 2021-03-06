mod ai;
pub mod combat;
mod player;

use legion::{
    systems::{Resources, Schedule},
    world::World,
};
use macroquad::texture::{load_texture, set_texture_filter, FilterMode, Texture2D};

use crate::gfx::{AnimationStorage, TextureStorage};
use crate::util::ButtonsState;

use macroquad::camera::Camera2D;

pub struct Game {
    pub world: World,
    pub resources: Resources,
    pub schedule: Schedule,
    pub textures: TextureStorage,

    pub camera: Camera2D,
}

impl Game {
    pub fn new() -> Self {
        let world = World::default();
        let resources = init_resources();
        let schedule = init_schedule();

        let textures = TextureStorage::default();
        let camera =
            Camera2D::from_display_rect(macroquad::math::Rect::new(0.0, 0.0, 320.0, 180.0));
        Self { world, resources, schedule, textures, camera }
    }
    pub async fn init(&mut self) {
        use self::ai::{AiControlled, HitMemory};
        use self::combat::CombatStats;
        use self::player::PlayerControlled;
        use crate::gfx::Sprite;
        use crate::phx::{Gravity, Hitbox, OnGround, Position, Velocity};
        use glam::Vec2;

        let slimeu_texture: Texture2D = load_texture("media/slimeu.png").await;
        set_texture_filter(slimeu_texture, FilterMode::Nearest);

        let goblin_texture: Texture2D = load_texture("media/goblin_base-b.png").await;
        set_texture_filter(goblin_texture, FilterMode::Nearest);

        self.textures.insert("slimeu".into(), slimeu_texture);
        self.textures.insert("goblin_base".into(), goblin_texture);

        {
            use crate::gfx::{AnimationTemplate, Frame};
            use macroquad::math::Rect;
            let mut animation_storage = self.resources.get_mut::<AnimationStorage>().unwrap();
            // TODO: Safer API that doesnt allow empty animation
            let slimeu_static = AnimationTemplate {
                rect: Rect::new(1., 0., 18., 18.),
                move_by: 17.,
                repeat: false,
                texture_name: "slimeu".to_owned(),
                frames: vec![Frame { duration: 0.1 }; 1],
            };
            animation_storage.insert("slimeu_static".into(), slimeu_static);
            let slimeu_idle = AnimationTemplate {
                rect: Rect::new(17., 0., 18., 18.),
                move_by: 17.,
                repeat: true,
                texture_name: "slimeu".to_owned(),
                frames: vec![Frame { duration: 0.1 }; 5],
            };
            animation_storage.insert("slimeu_idle".into(), slimeu_idle);
            let slimeu_run = AnimationTemplate {
                rect: Rect::new(1., 17., 18., 18.),
                move_by: 17.,
                repeat: true,
                texture_name: "slimeu".to_owned(),
                frames: vec![Frame { duration: 0.08 }; 8],
            };
            animation_storage.insert("slimeu_run".into(), slimeu_run);
        }

        let animation_storage = self.resources.get::<AnimationStorage>().unwrap();

        self.world.push((Hitbox::new(makeshift_static_platform(&self.resources)),));
        self.world.push((
            Position { src: Vec2::new(10.0, 10.0) },
            Sprite::new("slimeu".to_owned(), 17., 0., 18., 18.),
            crate::gfx::Animation::new(&animation_storage, "slimeu_run"),
        ));

        let (player_bhandle, player_chandle) = makeshift_player_dynamic_collider(&self.resources);
        let (player_sprite, player_animation) =
            crate::gfx::Animation::new_with_sprite(&animation_storage, "slimeu_idle");
        let player_entity = self.world.push((
            Position { src: Vec2::new(30.0, 10.0) },
            Velocity { src: Vec2::new(0., 0.) },
            Gravity::new(Vec2::new(0.0, 8.0)),
            OnGround::new(&self.resources, player_chandle),
            Hitbox::new(player_chandle),
            CombatStats::new(),
            PlayerControlled::new(),
            player_sprite,
            player_animation,
        ));

        let (enemy_bhandle, enemy_chandle) = makeshift_enemy_dynamic_collider(&self.resources);
        let enemy_entity = self.world.push((
            Position { src: Vec2::new(50.0, 10.0) },
            Sprite::new(
                "goblin_base".to_owned(),
                0.,
                0.,
                goblin_texture.width(),
                goblin_texture.height(),
            ),
            Velocity { src: Vec2::new(0., 0.) },
            Gravity::new(Vec2::new(0.0, 8.0)),
            Hitbox::new(enemy_chandle),
            CombatStats::new(),
            AiControlled::new(),
            HitMemory::new(),
        ));
        let mut body_entity_map = self.resources.get_mut::<crate::phx::BodyEntityMap>().unwrap();
        body_entity_map.insert(player_bhandle, player_entity);
        body_entity_map.insert(enemy_bhandle, enemy_entity);
    }
    pub fn update(&mut self) {
        // input should be updated on the main thread
        self.resources.get_mut::<ButtonsState>().unwrap().update();
        self.schedule.execute(&mut self.world, &mut self.resources);
    }
}

fn init_resources() -> Resources {
    let mut resources = Resources::default();
    resources.insert(AnimationStorage::default());
    resources.insert(crate::phx::PhysicsWorld::new());
    resources.insert(crate::phx::BodySet::new());
    resources.insert(crate::phx::ColliderSet::new());
    resources.insert(crate::util::ButtonsState::new());
    // TODO: Remove HurtQueue after replacing it with sensor hitbox
    resources.insert(crate::game::combat::HurtQueue::new());
    resources.insert(crate::game::combat::DamageQueue::new());
    resources.insert(crate::phx::BodyEntityMap::default());
    resources
}

fn init_schedule() -> Schedule {
    add_effect_systems(&mut Schedule::builder())
        .add_system(crate::gfx::animation::animate_system())
        .add_system(crate::phx::gravity_system())
        .add_system(crate::phx::ground_check_system())
        .add_system(self::player::update_fsm_system())
        .add_system(self::ai::update_fsm_system())
        .add_system(crate::phx::resphys_presync_system())
        .add_system(crate::phx::resphys_sync_system())
        .add_system(crate::game::combat::spread_pain_system())
        .add_system(crate::phx::temp::reset_velocity_system())
        .add_system(crate::game::combat::apply_damage_system())
        .build()
}

fn add_effect_systems(builder: &mut legion::systems::Builder) -> &mut legion::systems::Builder {
    builder
        .add_system(crate::effect::effect_update_system())
        .add_system(crate::effect::tint::tint_system())
}

fn makeshift_static_platform(resources: &Resources) -> resphys::ColliderHandle {
    use crate::phx::{BodySet, Category, ColliderSet, ColliderTag, PhysicsWorld};
    use glam::Vec2;

    let mut physics = resources.get_mut::<PhysicsWorld>().unwrap();
    let mut bodies = resources.get_mut::<BodySet>().unwrap();
    let mut colliders = resources.get_mut::<ColliderSet>().unwrap();

    let body =
        resphys::builder::BodyDesc::new().with_position(Vec2::new(70., 80.)).make_static().build();
    let collider = resphys::builder::ColliderDesc::new(
        resphys::AABB { half_exts: Vec2::new(60., 8.) },
        ColliderTag::Tile,
    )
    .with_category(Category::GROUND.bits());

    let bhandle = bodies.insert(body);
    colliders.insert(collider.build(bhandle), &mut bodies, &mut physics).unwrap()
}

fn makeshift_player_dynamic_collider(
    resources: &Resources,
) -> (resphys::BodyHandle, resphys::ColliderHandle) {
    use crate::phx::{BodySet, Category, ColliderSet, ColliderTag, PhysicsWorld};
    use glam::Vec2;

    let mut physics = resources.get_mut::<PhysicsWorld>().unwrap();
    let mut bodies = resources.get_mut::<BodySet>().unwrap();
    let mut colliders = resources.get_mut::<ColliderSet>().unwrap();

    let body = resphys::builder::BodyDesc::new()
        .with_position(Vec2::new(30., 10.))
        // .self_collision(false)
        .build();
    let collider = resphys::builder::ColliderDesc::new(
        resphys::AABB { half_exts: Vec2::new(5., 4.) },
        ColliderTag::Player,
    )
    .with_category(Category::PLAYER.bits())
    .with_mask(Category::GROUND.bits())
    .with_offset(Vec2::new(0., 4.));

    let bhandle = bodies.insert(body);
    (bhandle, colliders.insert(collider.build(bhandle), &mut bodies, &mut physics).unwrap())
}

fn makeshift_enemy_dynamic_collider(
    resources: &Resources,
) -> (resphys::BodyHandle, resphys::ColliderHandle) {
    use crate::phx::{BodySet, Category, ColliderSet, ColliderTag, PhysicsWorld};
    use glam::Vec2;

    let mut physics = resources.get_mut::<PhysicsWorld>().unwrap();
    let mut bodies = resources.get_mut::<BodySet>().unwrap();
    let mut colliders = resources.get_mut::<ColliderSet>().unwrap();

    let body = resphys::builder::BodyDesc::new()
        .with_position(Vec2::new(30., 10.))
        // .self_collision(false)
        .build();
    let collider = resphys::builder::ColliderDesc::new(
        resphys::AABB { half_exts: Vec2::new(5., 4.) },
        ColliderTag::Player,
    )
    .with_category(Category::ENEMY.bits())
    .with_mask(Category::GROUND.bits())
    .with_offset(Vec2::new(0., 4.));

    let bhandle = bodies.insert(body);
    (bhandle, colliders.insert(collider.build(bhandle), &mut bodies, &mut physics).unwrap())
}

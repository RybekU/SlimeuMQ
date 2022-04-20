pub mod agent;
mod ai;
pub mod combat;
pub mod resources;
pub mod stage;

use hecs::{CommandBuffer, World};

use macroquad::texture::{load_texture, FilterMode, Texture2D};

use crate::gfx::TextureStorage;

use self::agent::controller::update_fsm_system;
use self::resources::Resources;

pub struct Game {
    pub world: World,
    pub resources: Resources,
    pub textures: TextureStorage,
}

impl Game {
    pub fn new() -> Self {
        let world = World::new();
        let resources = Resources::new();

        let textures = TextureStorage::default();

        Self { world, resources, textures }
    }
    pub async fn init(&mut self) {
        use self::agent::controller::PlayerControlledV2;
        use self::ai::{AiControlled, HitMemory};
        use self::combat::CombatStats;
        use crate::gfx::Sprite;
        use crate::phx::{Gravity, Hitbox, OnGround, Position, Velocity};
        use glam::Vec2;

        let slimeu_texture: Texture2D = load_texture("media/slimeu.png").await.unwrap();
        slimeu_texture.set_filter(FilterMode::Nearest);

        let goblin_texture: Texture2D = load_texture("media/goblin_base.png").await.unwrap();
        goblin_texture.set_filter(FilterMode::Nearest);

        self.textures.insert("slimeu".into(), slimeu_texture);
        self.textures.insert("goblin_base".into(), goblin_texture);

        {
            use crate::gfx::{AnimationTemplate, Frame};
            use macroquad::math::Rect;
            let animation_storage = &mut self.resources.animations;
            // TODO: Safer API that doesnt allow empty animation
            let slimeu_static = AnimationTemplate {
                rect: Rect::new(0., 0., 16., 16.),
                move_by: 16.,
                repeat: false,
                texture_name: "slimeu".to_owned(),
                frames: vec![Frame { duration: 0.1 }; 1],
            };
            animation_storage.insert("slimeu_static".into(), slimeu_static);
            let slimeu_idle = AnimationTemplate {
                rect: Rect::new(16., 0., 16., 16.),
                move_by: 16.,
                repeat: true,
                texture_name: "slimeu".to_owned(),
                frames: vec![Frame { duration: 0.1 }; 5],
            };
            animation_storage.insert("slimeu_idle".into(), slimeu_idle);
            let slimeu_run = AnimationTemplate {
                rect: Rect::new(0., 16., 16., 16.),
                move_by: 16.,
                repeat: true,
                texture_name: "slimeu".to_owned(),
                frames: vec![Frame { duration: 0.08 }; 8],
            };
            animation_storage.insert("slimeu_run".into(), slimeu_run);
        }

        let (player_bhandle, player_chandle) =
            makeshift_player_dynamic_collider(&mut self.resources);

        let animation_storage = &self.resources.animations;

        self.world.spawn((
            Position { src: Vec2::new(10.0, 10.0) },
            Sprite::new("slimeu".to_owned(), 16., 0., 16., 16.),
            crate::gfx::Animation::new(animation_storage, "slimeu_run"),
        ));

        let (player_sprite, player_animation) =
            crate::gfx::Animation::new_with_sprite(animation_storage, "slimeu_idle");
        let player_entity = self.world.spawn((
            Position { src: Vec2::new(100.0, 60.0) },
            Velocity { src: Vec2::new(0., 0.) },
            Gravity::new(Vec2::new(0.0, 448. / 60.)),
            OnGround::new(&mut self.resources, player_chandle),
            Hitbox::new(player_chandle),
            CombatStats::new(),
            PlayerControlledV2::new(),
            player_sprite,
            player_animation,
        ));

        let (enemy_bhandle, enemy_chandle) = makeshift_enemy_dynamic_collider(&mut self.resources);
        let enemy_entity = self.world.spawn((
            Position { src: Vec2::new(80.0, 40.0) },
            Sprite::new(
                "goblin_base".to_owned(),
                0.,
                0.,
                goblin_texture.width(),
                goblin_texture.height(),
            ),
            Velocity { src: Vec2::new(0., 0.) },
            Gravity::new(Vec2::new(0.0, 448. / 60.)),
            Hitbox::new(enemy_chandle),
            CombatStats::new(),
            AiControlled::new(),
            HitMemory::new(),
        ));

        // setup camera to just follow player immediately, for now
        self.resources.camera.target = Some(player_entity);

        let body_entity_map = &mut self.resources.body_entity_map;
        body_entity_map.insert(player_bhandle, player_entity);
        body_entity_map.insert(enemy_bhandle, enemy_entity);
        {
            let tilemap = crate::map::tilemap::Tilemap::load().expect("Tilemap should exist here");

            let grid = &tilemap.grid[..];
            grid.chunks_exact(tilemap.width as usize).enumerate().for_each(|(column_id, row)| {
                row.iter().enumerate().for_each(|(row_id, &value)| {
                    if value > 0 {
                        makeshift_static_platform(
                            &mut self.resources,
                            (row_id as f32 * 16. + 8., column_id as f32 * 16. + 8.),
                            (8., 8.),
                        );
                    }
                });
            });
        }
    }
    pub fn update(&mut self) {
        // input should be updated on the main thread
        self.resources.input_buttons.update();
        schedule_execute(&mut self.world, &mut self.resources);
    }
}

fn schedule_execute(world: &mut World, resources: &mut Resources) {
    let mut cmd = CommandBuffer::new();

    // // effect entities
    crate::effect::effect_update_system(world, &mut cmd);
    crate::effect::tint::tint_system(world);

    crate::gfx::animation::animate_system(world, &resources.animations);
    crate::phx::gravity_system(world);
    crate::phx::ground_check_system(world, &resources.phys);
    update_fsm_system(world, resources);
    self::ai::update_fsm_system(world, resources);
    crate::phx::resphys_sync_system(
        world,
        &mut resources.phys,
        &mut resources.phys_bodies,
        &mut resources.phys_colliders,
    );
    crate::game::combat::spread_pain_system(
        &mut resources.hurt_queue,
        &mut resources.damage_queue,
        &resources.phys,
        &resources.phys_bodies,
        &resources.phys_colliders,
        &resources.body_entity_map,
    );
    crate::phx::temp::reset_velocity_system(world, &resources.phys);
    crate::game::combat::apply_damage_system(world, &mut resources.damage_queue, &mut cmd);

    resources.camera.update(world, resources.stage.current_room());

    // all new entities are created at frame end
    cmd.run_on(world);
}

fn makeshift_static_platform(
    resources: &mut Resources,
    position: (f32, f32),
    shape: (f32, f32),
) -> resphys::ColliderHandle {
    use crate::phx::{Category, ColliderTag};
    use glam::Vec2;

    let physics = &mut resources.phys;
    let bodies = &mut resources.phys_bodies;
    let colliders = &mut resources.phys_colliders;

    let body = resphys::builder::BodyDesc::new()
        .with_position(Vec2::new(position.0, position.1))
        .make_static()
        .build();
    let collider = resphys::builder::ColliderDesc::new(
        resphys::AABB { half_exts: Vec2::new(shape.0, shape.1) },
        ColliderTag::Tile,
    )
    .with_category(Category::GROUND.bits());

    let bhandle = bodies.insert(body);
    colliders.insert(collider.build(bhandle), bodies, physics).unwrap()
}

fn makeshift_player_dynamic_collider(
    resources: &mut Resources,
) -> (resphys::BodyHandle, resphys::ColliderHandle) {
    use crate::phx::{Category, ColliderTag};
    use glam::Vec2;

    let physics = &mut resources.phys;
    let bodies = &mut resources.phys_bodies;
    let colliders = &mut resources.phys_colliders;

    let body = resphys::builder::BodyDesc::new()
        .with_position(Vec2::new(100., 60.))
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
    (bhandle, colliders.insert(collider.build(bhandle), bodies, physics).unwrap())
}

fn makeshift_enemy_dynamic_collider(
    resources: &mut Resources,
) -> (resphys::BodyHandle, resphys::ColliderHandle) {
    use crate::phx::{Category, ColliderTag};
    use glam::Vec2;

    let physics = &mut resources.phys;
    let bodies = &mut resources.phys_bodies;
    let colliders = &mut resources.phys_colliders;

    let body = resphys::builder::BodyDesc::new()
        .with_position(Vec2::new(100., 40.))
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
    (bhandle, colliders.insert(collider.build(bhandle), bodies, physics).unwrap())
}

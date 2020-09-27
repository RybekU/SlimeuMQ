use legion::{
    systems::{Resources, Schedule},
    world::World,
};
use macroquad::Texture2D;

use crate::util::SimpleCam2D;
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
    pub fn update(&mut self) {
        self.schedule.execute(&mut self.world, &mut self.resources);
    }
}

fn init_resources() -> Resources {
    Resources::default()
}

fn init_schedule() -> Schedule {
    Schedule::builder().build()
}

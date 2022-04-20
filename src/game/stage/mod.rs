pub use self::room::Room;

pub mod room;

// TODO: Extend to support multiple rooms and traversal between them
// TODO Hint: Traversal could be implemented by generating 4 collision shapes and monitoring collision with them.
/// `Stage` is a collection of `Room`s defining how granular gameplay environments are linked together.
pub struct Stage {
    room: Room,
}

impl Stage {
    pub fn from_ldtk() -> Self {
        use ldtk_rust::Project;

        const BASE_DIR: &str = "media/tilemap/";
        const TEST_MAP: &str = "test2.ldtk";
        const MAGIC_ID: i64 = 25;

        let project = Project::new(BASE_DIR.to_owned() + TEST_MAP);

        let mut current_level_index: Option<usize> = None;

        for (idx, level) in project.levels.iter().enumerate() {
            if level.uid == MAGIC_ID {
                current_level_index = Some(idx);
                break;
            }
        }

        let only_level = &project.levels
            [current_level_index.unwrap_or_else(|| panic!("Level {} doesn't exist", MAGIC_ID))];

        let room = Room::from_ldtk(only_level);

        Self { room }
    }
    pub fn current_room(&self) -> &Room {
        &self.room
    }
}

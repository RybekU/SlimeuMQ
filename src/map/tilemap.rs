use ldtk_rust::Project;

const BASE_DIR: &str = "media/tilemap/";
const TEST_MAP: &str = "test2.ldtk";
const MAGIC_ID: i64 = 25;
const MAGIC_LAYER: &str = "Front";

// TODO: Better representation, define entry points in the LDtk file, replace module with `Stage` definition
//  and extract useful informations from the ldtk files.

// Tilesize is 16x16
pub struct Tilemap {
    pub width: u32,
    pub height: u32,
    pub grid: Vec<u32>,
}

impl Tilemap {
    // Loads one layer of one specified above tilemap only
    pub fn load() -> Result<Self, &'static str> {
        let project = Project::new(BASE_DIR.to_owned() + TEST_MAP);

        let mut current_level_index: Option<usize> = None;

        for (idx, level) in project.levels.iter().enumerate() {
            if level.uid == MAGIC_ID {
                current_level_index = Some(idx);
                break;
            }
        }

        let layer_instance = project.levels
            [current_level_index.unwrap_or_else(|| panic!("Level {} doesn't exist", MAGIC_ID))]
        .layer_instances
        .as_ref()
        .expect("No layer instances found")
        .iter()
        .find(|li| li.identifier == MAGIC_LAYER)
        .unwrap_or_else(|| panic!("No {} layer instance found.", MAGIC_LAYER));

        Ok(Tilemap {
            width: layer_instance.c_wid as u32,
            height: layer_instance.c_hei as u32,
            grid: layer_instance.int_grid_csv.iter().map(|&pos| pos as u32).collect(),
        })
    }
}

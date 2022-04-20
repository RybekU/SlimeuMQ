use ldtk_rust::Level;

/// `Room` is the smallest unit of representation for gameplay environment.
pub struct Room {
    // TODO: Define proper spawn/enter points and locations.
    pub top: f32,
    pub left: f32,
    pub right: f32,
    pub bottom: f32,
}

impl Room {
    pub fn from_ldtk(ldtk_level: &Level) -> Self {
        Room {
            top: 0.,
            left: 0.,
            right: ldtk_level.px_wid as f32,
            bottom: ldtk_level.px_hei as f32,
        }
    }
}

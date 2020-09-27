use std::time::Duration;
use std::time::Instant;

pub struct Timer {
    frametime: Duration,
    last_update: Instant,
}

impl Timer {
    pub fn with_fps(fps: f32) -> Timer {
        Self {
            frametime: Duration::from_secs_f32(1.0 / fps),
            last_update: Instant::now(),
        }
    }
    pub fn tick(&mut self) -> bool {
        if self.last_update.elapsed() > self.frametime {
            self.last_update += self.frametime;
            true
        } else {
            false
        }
    }
}

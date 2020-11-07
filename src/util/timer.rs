use std::time::Duration;
use std::time::Instant;

pub struct Timer {
    frametime: Duration,
    last_update: Instant,
}

impl Timer {
    pub fn with_fps(fps: f32) -> Timer {
        Self { frametime: Duration::from_secs_f32(1.0 / fps), last_update: Instant::now() }
    }
    // http://lspiroengine.com/?p=378
    pub fn tick(&mut self, current_time: &Instant) -> bool {
        if current_time.duration_since(self.last_update) > self.frametime {
            self.last_update += self.frametime;
            true
        } else {
            false
        }
    }
}

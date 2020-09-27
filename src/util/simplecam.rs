use macroquad::*;

#[derive(Clone, Copy)]
pub struct SimpleCam2D {
    pub zoom: f32,
    pub target: Vec2,
    pub rotation: f32,
}

impl Default for SimpleCam2D {
    fn default() -> SimpleCam2D {
        SimpleCam2D {
            zoom: 1.0,
            target: Vec2::zero(),
            rotation: 0.,
        }
    }
}

impl SimpleCam2D {
    pub fn with_zoom(zoom: f32) -> Self {
        SimpleCam2D {
            zoom,
            ..Default::default()
        }
    }
}

impl Camera for SimpleCam2D {
    fn matrix(&self) -> glam::Mat4 {
        glam::Mat4::orthographic_rh_gl(
            self.target.x(),
            screen_width() / self.zoom,
            screen_height() / self.zoom,
            self.target.y(),
            -1.,
            1.,
        )
    }

    fn depth_enabled(&self) -> bool {
        false
    }

    fn render_pass(&self) -> Option<miniquad::RenderPass> {
        None
    }
}

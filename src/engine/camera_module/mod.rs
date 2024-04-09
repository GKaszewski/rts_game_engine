use comfy::*;

pub struct RtsCamera {
    pub pos: Vec2,
    pub zoom: f32,
    pub speed: f32,
    pub auto_pan: bool,
    pub can_zoom: bool,
    pub can_move: bool,
    pub zoom_speed: f32,
    pub zoom_range: Vec2,
    pub width_range: Vec2,
    pub height_range: Vec2,
}

impl RtsCamera {
    pub fn new() -> Self {
        Self {
            pos: vec2(10.0, 10.0),
            zoom: 10.0,
            speed: 2.5,
            auto_pan: true,
            can_zoom: true,
            can_move: true,
            zoom_speed: 0.5,
            zoom_range: vec2(10.0, 50.0),
            width_range: vec2(-1000.0, 1000.0),
            height_range: vec2(-10000.0, 1000.0),
        }
    }

    pub fn update(&mut self) {
        let vel = (self.speed / (1.0 / self.zoom)) * delta();
        if is_key_down(KeyCode::W) {
            self.pos.y += vel;
        }
        if is_key_down(KeyCode::S) {
            self.pos.y -= vel;
        }
        if is_key_down(KeyCode::A) {
            self.pos.x -= vel;
        }
        if is_key_down(KeyCode::D) {
            self.pos.x += vel;
        }

        if self.can_zoom && mouse_wheel().1 != 0.0 {
            self.zoom -= mouse_wheel().1 * self.zoom_speed;
            self.zoom = self.zoom.clamp(self.zoom_range.x, self.zoom_range.y);
        }

        if self.auto_pan {
            let vel = (self.speed / self.zoom) * delta();
            if mouse_screen().x < 10.0 {
                self.pos.x -= vel;
            }
            if mouse_screen().x > screen_width() - 10.0 {
                self.pos.x += vel;
            }
            if mouse_screen().y < 10.0 {
                self.pos.y += vel;
            }
            if mouse_screen().y > screen_height() - 10.0 {
                self.pos.y -= vel;
            }
        }

        self.pos.x = self.pos.x.clamp(self.width_range.x, self.width_range.y);
        self.pos.y = self.pos.y.clamp(self.height_range.x, self.height_range.y);

        main_camera_mut().zoom = self.zoom;
        main_camera_mut().target = Some(self.pos);
    }
}

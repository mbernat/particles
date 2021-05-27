use macroquad::prelude::*;

struct Particle {
    pos: Vec2,
    vel: Vec2,
    acc: Vec2,

    color: Color
}

fn random_vec2(max: Vec2) -> Vec2 {
    let x = rand::gen_range(0.0, max.x);
    let y = rand::gen_range(0.0, max.y);
    Vec2::new(x, y)
}

fn random_in_annulus(min: f32, max: f32) -> Vec2 {
    loop {
        let x = rand::gen_range(-max, max);
        let y = rand::gen_range(-max, max);
        let p = Vec2::new(x, y);
        let r = p.length();
        if r >= min && r <= max {
            return p
        }
    }
}

fn random_color() -> Color {
    let r = rand::gen_range(0.0, 1.0);
    let g = rand::gen_range(0.0, 1.0);
    let b = rand::gen_range(0.0, 1.0);
    Color::new(r, g, b, 1.0)
}

impl Particle {
    fn new(pos: Vec2, vel: Vec2) -> Particle {
        let color = random_color();
        Particle{pos, vel, acc: Vec2::ZERO, color}
    }

    fn generate(max_x: f32, max_y: f32) -> Particle {
        let max_pos = Vec2::new(max_x, max_y);
        let pos = random_vec2(max_pos);
        Particle::new(pos, Vec2::ZERO)
    }

    fn step(&mut self, dt: f32) {
        self.vel += self.acc * dt;
        self.pos += self.vel * dt;
    }

    fn target(&mut self, place: Vec2, strength: f32, color: Color) {
        let cvec = color.to_vec();
        let svec = self.color.to_vec();
        let color_match = 1.0 - (cvec - svec).length_squared() / 3.0;
        let force = 1000.0 * strength * color_match.powi(8);
        let diff = place - self.pos;
        let mut dist = diff.length();
        if dist < 100.0 {
            dist = 100.0
        }
        self.acc += (force / dist) * diff.normalize();
    }

    fn wiggle(&mut self) {
        let force = 500.0;
        self.acc += random_in_annulus(0.0, force);
    }
    
    fn drag(&mut self) {
        self.acc += -2.0 * self.vel;
    }

    fn draw(&self) {
        draw_circle(self.pos.x, self.pos.y, 3.0, self.color);
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Particles".to_owned(),
        fullscreen: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let width = screen_width();
    let height = screen_height();
    let count: usize = 10000;
    let mut color = RED;

    let mut ps: Vec<Particle> = (1..=count).map(|_| Particle::generate(width, height)).collect();
    loop {
        clear_background(BLACK);

        let dt = get_frame_time();
        let (mx, my) = mouse_position();

        if is_mouse_button_pressed(MouseButton::Right) {
            color = random_color();
        }

        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        for p in &mut ps {
            p.acc = Vec2::ZERO;
            let strength = if is_mouse_button_down(MouseButton::Left) {
                1000.0
            } else {
                100.0
            };
            p.target(Vec2::new(mx, my), strength, color);
            p.drag();
            p.wiggle();
            p.step(dt);
        }

        draw_circle(mx, my, 10.0, color);

        for p in &ps {
            p.draw();
        }

        next_frame().await;
    }
}

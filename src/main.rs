use macroquad::prelude::*;

struct Particle {
    pos: Vec2,
    vel: Vec2,
    acc: Vec2
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

impl Particle {
    fn new(pos: Vec2, vel: Vec2) -> Particle {
        Particle{pos, vel, acc: Vec2::ZERO}
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

    fn target(&mut self, place: Vec2, strength: f32) {
        let force = 1000.0 * strength;
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
        draw_circle(self.pos.x, self.pos.y, 3.0, WHITE);
    }
}

#[macroquad::main("Particles")]
async fn main() {
    let width = screen_width();
    let height = screen_height();
    let count: usize = 10000;

    let mut ps: Vec<Particle> = (1..=count).map(|_| Particle::generate(width, height)).collect();
    loop {
        clear_background(BLACK);

        let dt = get_frame_time();

        for p in &mut ps {
            p.acc = Vec2::ZERO;
            let (mx, my) = mouse_position();
            let strength = if is_mouse_button_down(MouseButton::Left) {
                1000.0
            } else {
                100.0
            };
            p.target(Vec2::new(mx, my), strength);
            p.drag();
            p.wiggle();
            p.step(dt);
        }

        for p in &ps {
            p.draw();
        }

        next_frame().await;
    }
}

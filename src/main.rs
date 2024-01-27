use std::fs::File;
use std::io::Write;
use ray_tracer_challenge::{Point, point, normalize, Vector, vector, Canvas, color, canvas_to_ppm};

struct Environment {
    gravity: Vector,
    wind: Vector,
}

#[derive(Debug)]
struct Projectile {
    position: Point,
    velocity: Vector,
}

fn tick(env: &Environment, proj: Projectile) -> Projectile {
    let position = proj.position + proj.velocity;
    let velocity = proj.velocity + env.gravity + env.wind;
    Projectile { position, velocity }
}

fn main() {
    let start = point(0.0, 1.0, 0.0);
    let velocity = normalize(vector(1.0, 1.8, 0.0)) * 11.25;
    let mut p = Projectile { position: start, velocity };

    let gravity = vector(0.0, -0.1, 0.0);
    let wind = vector(-0.01, 0.0, 0.0);
    let e = Environment { gravity, wind };
    let mut c = Canvas::new(900, 550);
    while p.position.y > 0f32 {
        c.write_pixel(p.position.x.round() as i32, c.height - p.position.y.round() as i32, color(0.2, 0.4, 0.4));
        p = tick(&e, p);
    }
    let mut f = File::create("output.ppm").unwrap();
    f.write_all(canvas_to_ppm(c).as_bytes()).unwrap();
    f.sync_all().unwrap();
}
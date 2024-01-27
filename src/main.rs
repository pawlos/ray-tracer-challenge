use ray_tracer_challenge::{Point, point, normalize, Vector, vector};

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
    let mut p = Projectile { position: point(0f32, 1f32, 0f32), velocity: normalize(vector(1f32,1f32,0f32)) };
    let e = Environment { gravity: vector(0f32,-0.1f32,0f32), wind: vector(-0.01f32, 0f32,0f32)};
    while p.position.y > 0f32 {
        p = tick(&e, p);
        println!("{:?}", p);
    }
}
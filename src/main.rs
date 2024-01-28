use ray_tracer_challenge::{Point, Vector, Matrix, inverse, transpose};

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
    let a = Matrix::new4x4(
        [-5.0, 2.0, 6.0, -8.0],
        [1.0, -5.0, 1.0, 8.0],
        [7.0, 7.0, -6.0, -7.0],
        [1.0, -3.0, 7.0, 4.0]);
    println!("{:?}", inverse(transpose(a.clone())));
    println!("{:?}", transpose(inverse(a.clone())));
}
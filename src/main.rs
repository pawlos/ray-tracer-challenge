use std::f32::consts::PI;
use std::fs::File;
use std::io::Write;
use ray_tracer_challenge::*;


fn chapter5() {
    let ray_origin = point(0.0, 0.0, -5.0);
    let wall_z = 10.0;
    let wall_size = 7f32;
    let canvas_pixels = 300;

    let pixel_size = wall_size / canvas_pixels as f32;
    let half = wall_size / 2.0;

    let mut canvas = Canvas::new(canvas_pixels, canvas_pixels);
    let color = color(1.0, 0.0, 0.0);
    let shape = sphere();

    for y in 0..canvas.height {
        let world_y = half - pixel_size * y as f32;
        for x in 0..canvas.width {
            let world_x = half - pixel_size * x as f32;
            let position = point(world_x, world_y, wall_z);

            let r = ray(ray_origin, normalize(position - ray_origin));

            let mut xs = intersect(&shape, r);

            match hit(&mut xs) {
                | Some(_) => canvas.write_pixel(x ,y, color),
                _ => {}
            }
        }
    }
    let sphere_data = canvas_to_ppm(canvas);
    let mut f = File::create("sphere.ppm").unwrap();
    f.write_all(sphere_data.as_bytes()).unwrap();
    f.sync_all().unwrap();
}

#[allow(dead_code)]
fn chapter4() {
    let mut canvas = Canvas::new(400, 400);
    for i in 0..12 {
        let mut point = point(0.0, 1.0, 0.0);
        let rotation = rotation_z((i as f32 * PI)/6.0);
        point = rotation * point;
        point = scaling(75.0, 75.0, 1.0) * point;
        let move_translation = translation(canvas.width as f32 / 2.0 , canvas.height as f32 / 2.0, 0.0);
        point = move_translation * point;
        canvas.write_pixel(point.x.round() as i32, point.y.round() as i32, color(1.0, 1.0, 1.0));
    }
    let clock_data = canvas_to_ppm(canvas);
    let mut f = File::create("clock.ppm").unwrap();
    f.write_all(clock_data.as_bytes()).unwrap();
    f.sync_all().unwrap();
}

fn main() {
    chapter5();
    println!("Done");
}
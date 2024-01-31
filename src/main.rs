use std::f32::consts::PI;
use std::fs::File;
use std::io::Write;
use ray_tracer_challenge::{Canvas, canvas_to_ppm, color, point, rotation_z, scaling, translation};

fn main() {

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
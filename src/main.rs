use std::f32::consts::PI;
use std::fs::File;
use std::io::Write;
use ray_tracer_challenge::*;

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

#[allow(dead_code)]
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
fn chapter6() {
    let ray_origin = point(0.0, 0.0, -5.0);
    let wall_z = 10.0;
    let wall_size = 7f32;
    let canvas_pixels = 300;

    let pixel_size = wall_size / canvas_pixels as f32;
    let half = wall_size / 2.0;

    let mut canvas = Canvas::new(canvas_pixels, canvas_pixels);
    let mut shape = sphere();
    shape.material = material();
    shape.material.color = color(1.0, 0.2, 1.0);

    let light_position = point(-10.0, 10.0, -10.0);
    let light_color = color(1.0, 1.0, 1.0);

    let light = point_light(light_position, light_color);

    for y in 0..canvas.height {
        let world_y = half - pixel_size * y as f32;
        for x in 0..canvas.width {
            let world_x = half - pixel_size * x as f32;
            let p = point(world_x, world_y, wall_z);

            let r = ray(ray_origin, normalize(p - ray_origin));

            let mut xs = intersect(&shape, r);

            match hit(&mut xs) {
                | Some(hit) => {
                    let point = position(r, hit.t);
                    let normal = normal_at(hit.object, point);
                    let eye = r.direction;
                    let color = lightning(&hit.object.material, &light, point, eye, normal);
                    canvas.write_pixel(x ,y, color)
                },
                _ => {}
            }
        }
    }
    let sphere_data = canvas_to_ppm(canvas);
    let mut f = File::create("sphere_lightning.ppm").unwrap();
    f.write_all(sphere_data.as_bytes()).unwrap();
    f.sync_all().unwrap();
}

fn chapter7() {
    let mut floor = sphere();
    floor.transform = scaling(10.0, 0.01, 10.0);
    floor.material = material();
    floor.material.color = color(1.0, 0.0, 0.9);
    floor.material.specular = 0.0;

    let mut left_wall = sphere();
    left_wall.transform = translation(0.0, 0.0, 5.0) * rotation_y(-PI/4.0) * rotation_x(PI/2.0) *
                          scaling(10.0, 0.01, 10.0);
    left_wall.material = floor.material.clone();

    let mut right_wall = sphere();
    right_wall.transform = translation(0.0, 0.0, 5.0) * rotation_y(PI/4.0) * rotation_x(PI/2.0) *
                           scaling(10.0, 0.01, 10.0);

    let mut middle = sphere();
    middle.transform = translation(-0.5, 1.0, 0.5);
    middle.material = material();
    middle.material.color = color(0.1, 1.0, 0.5);
    middle.material.diffuse = 0.7;
    middle.material.specular = 0.3;

    let mut right = sphere();
    right.transform = translation(1.5, 0.5, -0.5) * scaling(0.5, 0.5, 0.5);
    right.material = material();
    right.material.color = color(0.5, 1.0, 0.1);
    right.material.diffuse = 0.7;
    right.material.specular = 0.3;

    let mut left = sphere();
    left.transform = translation(-1.5, 0.33, -0.75) * scaling(0.33, 0.33, 0.33);
    left.material.color = color(1.0, 0.8, 0.1);
    left.material.diffuse = 0.7;
    left.material.specular = 0.3;

    let mut world = world();
    world.lights.push(point_light(point(-10.0, 10.0, -10.0), color(1.0, 1.0, 1.0)));

    world.objects.push(floor);
    world.objects.push(left_wall);
    world.objects.push(right_wall);
    world.objects.push(middle);
    world.objects.push(right);
    world.objects.push(left);

    let mut camara = camera(800, 600, PI/3.0);
    camara.transform = view_transformation(
        point(0.0, 1.5, -5.0),
        point(0.0, 1.0, 0.0),
        vector(0.0, 1.0, 0.0));

    let canvas = render(&camara, &world);
    let sphere_data = canvas_to_ppm(canvas);
    let mut f = File::create("world.ppm").unwrap();
    f.write_all(sphere_data.as_bytes()).unwrap();
    f.sync_all().unwrap();

}
fn main() {
    chapter7();
    println!("Done");
}
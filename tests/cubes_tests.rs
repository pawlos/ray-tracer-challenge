use ray_tracer_challenge::*;

#[cfg(test)]
mod cubes {
    use super::*;

    macro_rules! cube_tests {
    ($($name:ident: $value:expr,)*) => {
    $(
        #[test]
        fn $name() {
            let (origin, direction, t1, t2) = $value;

            let c = cube();

            let r = ray(origin, direction);
            let xs = c.local_intersect(r);

            assert_eq!(2, xs.len());
            assert_eq!(t1, xs[0].t);
            assert_eq!(t2, xs[1].t);
        }
    )*}
    }

    macro_rules! cube_tests_miss {
    ($($name:ident: $value:expr,)*) => {
    $(
        #[test]
        fn $name() {
            let (origin, direction) = $value;

            let c = cube();

            let r = ray(origin, direction);
            let xs = c.local_intersect(r);

            assert_eq!(0, xs.len());
        }
    )*}
    }

    macro_rules! cube_tests_normal {
    ($($name:ident: $value:expr,)*) => {
    $(
        #[test]
        fn $name() {
            let (point, normal) = $value;

            let c = cube();

            let computed_normal = c.local_normal_at(point);

            assert_eq!(normal, computed_normal);
        }
    )*}
    }

    // A ray intersects a cube p. 168
    cube_tests! {
        a_ray_intersects_a_cube_1: (point(5.0, 0.5, 0.0), vector(-1.0, 0.0, 0.0), 4.0, 6.0),
        a_ray_intersects_a_cube_2: (point(-5.0, 0.5, 0.0), vector(1.0, 0.0, 0.0), 4.0, 6.0),
        a_ray_intersects_a_cube_3: (point(0.5, 5.0, 0.0), vector(0.0, -1.0, 0.0), 4.0, 6.0),
        a_ray_intersects_a_cube_4: (point(0.5, -5.0, 0.0), vector(0.0, 1.0, 0.0), 4.0, 6.0),
        a_ray_intersects_a_cube_5: (point(0.5, 0.0, 5.0), vector(0.0, 0.0, -1.0), 4.0, 6.0),
        a_ray_intersects_a_cube_6: (point(0.5, 0.0, -5.0), vector(0.0, 0.0, 1.0), 4.0, 6.0),
        a_ray_intersects_a_cube_7: (point(0.0, 0.5, 0.0), vector(0.0, 0.0, 1.0), -1.0, 1.0),
    }

    cube_tests_miss! {
        a_ray_misses_a_cube_1: (point(-2.0, 0.0, 0.0), vector(0.2673, 0.5345, 0.8018)),
        a_ray_misses_a_cube_2: (point(0.0, -2.0, 0.0), vector(0.8018, 0.2673, 0.5345)),
        a_ray_misses_a_cube_3: (point(0.0, 0.0, -2.0), vector(0.5345, 0.8018, 0.2673)),
        a_ray_misses_a_cube_4: (point(2.0, 0.0, 2.0), vector(0.0, 0.0, -1.0)),
        a_ray_misses_a_cube_5: (point(0.0, 2.0, 2.0), vector(0.0, -1.0, 0.0)),
        a_ray_misses_a_cube_6: (point(2.0, 2.0, 0.0), vector(-1.0, 0.0, 0.0)),
    }

    cube_tests_normal! {
        cube_normal_1: (point(1.0, 0.5, -0.8), vector(1.0, 0.0, 0.0)),
        cube_normal_2: (point(-1.0, -0.2, 0.9), vector(-1.0, 0.0, 0.0)),
        cube_normal_3: (point(-0.4, 1.0, -0.1), vector(0.0, 1.0, 0.0)),
        cube_normal_4: (point(0.3, -1.0, -0.7), vector(0.0, -1.0, 0.0)),
        cube_normal_5: (point(-0.6, 0.3, 1.0), vector(0.0, 0.0, 1.0)),
        cube_normal_6: (point(0.4, 0.4, -1.0), vector(0.0, 0.0, -1.0)),
        cube_normal_7: (point(1.0, 1.0, 1.0), vector(1.0, 0.0, 0.0)),
        cube_normal_8: (point(-1.0, -1.0, -1.0), vector(-1.0, 0.0, 0.0)),
    }
}
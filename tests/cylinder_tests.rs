use ray_tracer_challenge::*;

#[cfg(test)]
mod cylinders {
    use super::*;

    macro_rules! cylinder_tests_miss {
    ($($name:ident: $value:expr,)*) => {
    $(
        #[test]
        fn $name() {
            let (origin, direction) = $value;

            let c = cylinder();

            let direction = normalize(direction);

            let r = ray(origin, direction);
            let xs = c.local_intersect(r);

            assert_eq!(0, xs.len());
        }
    )*}
    }

    cylinder_tests_miss! {
        a_ray_misses_a_cylinder_1: (point(1.0, 0.0, 0.0),  vector(0.0, 1.0, 0.0)),
        a_ray_misses_a_cylinder_2: (point(0.0, 0.0, 0.0),  vector(0.0, 1.0, 0.0)),
        a_ray_misses_a_cylinder_3: (point(0.0, 0.0, -5.0), vector(1.0, 1.0, 1.0)),
    }

    macro_rules! cylinder_tests_hit {
    ($($name:ident: $value:expr,)*) => {
    $(
        #[test]
        fn $name() {
            let (origin, direction, t1, t2) = $value;

            let c = cylinder();

            let direction = normalize(direction);

            let r = ray(origin, direction);
            let xs = c.local_intersect(r);

            assert_eq!(2, xs.len());
            assert_eq!(t1, xs[0].t);
            assert_eq!(t2, xs[1].t);
        }
    )*}
    }

    cylinder_tests_hit! {
        a_ray_intersects_a_cylinder_1: (point(1.0, 0.0, -5.0), vector(0.0, 0.0, 1.0), 5.0, 5.0),
        a_ray_intersects_a_cylinder_2: (point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0), 4.0, 6.0),
        a_ray_intersects_a_cylinder_3: (point(0.5, 0.0, -5.0), vector(0.1, 1.0, 1.0), 6.80798, 7.08872),
    }
}
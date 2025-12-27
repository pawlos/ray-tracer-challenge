use ray_tracer_challenge::*;

#[cfg(test)]
mod cones {
    use super::*;

    macro_rules! cone_tests_hit {
    ($($name:ident: $value:expr,)*) => {
    $(
        #[test]
        fn $name() {
            let (origin, direction, t0, t1) = $value;

            let c = cone(None, None, None);

            let direction = normalize(direction);

            let r = ray(origin, direction);
            let xs = c.local_intersect(r);

            assert_eq!(2, xs.len());
            assert!((t0 - xs[0].t).abs() < EPS);
            assert!((t1 - xs[1].t).abs() < EPS);
        }
    )*}
    }

    cone_tests_hit! {
        a_ray_hit_a_cone_1: (point(0.0, 0.0, -5.0),  vector(0.0, 0.0, 1.0), 5.0, 5.0),
        a_ray_hit_a_cone_2: (point(0.0, 0.0, -5.0),  vector(1.0, 1.0, 1.0), 8.66025, 8.66025),
        a_ray_hit_a_cone_3: (point(1.0, 1.0, -5.0), vector(-0.5, -1.0, 1.0), 4.55006, 49.44994),
    }

    #[test]
    /// Intersecting a cone with a ray parallel to one of its halves p. 190
    fn intersecting_a_cone_with_ray_parallel_to_one_of_its_halves() {
        let shape = cone(None, None, None);
        let direction = normalize(vector(0.0, 1.0, 1.0));
        let ray = ray(point(0.0, 0.0, -1.0), direction);
        let xs = shape.local_intersect(ray);

        assert_eq!(xs.len(), 1);
        assert!((xs[0].t - 0.35355).abs() < EPS);
    }
}
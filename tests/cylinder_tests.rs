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
        a_ray_intersects_a_cylinder_3: (point(0.5, 0.0, -5.0), vector(0.1, 1.0, 1.0), 6.808006/*6.80798*/, 7.0886984/*7.08872*/),
    }

    macro_rules! cylinder_normal {
    ($($name:ident: $value:expr,)*) => {
    $(
        #[test]
        fn $name() {
            let (point, normal) = $value;

            let c = cylinder();

            let n = c.local_normal_at(point);

            assert_eq!(n, normal);
        }
    )*}
    }

    cylinder_normal! {
        normal_vector_on_cylinder_1: (point(1.0, 0.0, 0.0), vector(1.0, 0.0, 0.0)),
        normal_vector_on_cylinder_2: (point(0.0, 5.0, -1.0), vector(0.0, 0.0, -1.0)),
        normal_vector_on_cylinder_3: (point(0.0, -2.0, 1.0), vector(0.0, 0.0, 1.0)),
        normal_vector_on_cylinder_4: (point(-1.0, 1.0, 0.0), vector(-1.0, 0.0, 0.0)),
    }

    #[test]
    /// The default minimum and maximum for a cylinder
    fn default_minimum_and_maximum_for_cylinder() {
        let cyl = cylinder();
        let deref_cyl = cyl.as_any().downcast_ref::<Cylinder>().unwrap();
        assert_eq!(f32::NEG_INFINITY, deref_cyl.minimum);
        assert_eq!(f32::INFINITY, deref_cyl.maximum);
    }

    macro_rules! constraint_cylinder_intersecting {
    ($($name:ident: $value:expr,)*) => {
    $(
        #[test]
        fn $name() {
            let (point, direction, count) = $value;

            let mut c = cylinder();
            let deref_cyl = c.as_mut_any().downcast_mut::<Cylinder>().unwrap();
            deref_cyl.set_minimum(1.0);
            deref_cyl.set_maximum(2.0);

            let direction = normalize(direction);

            let r = ray(point, direction);
            let xs = c.local_intersect(r);

            assert_eq!(count, xs.len());
        }
    )*}
    }

    constraint_cylinder_intersecting! {
        constraint_cylinder_1: (point(0.0, 1.5, 0.0),  vector(0.1, 1.0, 0.0), 0),
        constraint_cylinder_2: (point(0.0, 3.0, -5.0), vector(0.0, 0.0, 1.0), 0),
        constraint_cylinder_3: (point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0), 0),
        constraint_cylinder_4: (point(0.0, 2.0, -5.0), vector(0.0, 0.0, 1.0), 0),
        constraint_cylinder_5: (point(0.0, 1.0, -5.0), vector(0.0, 0.0, 1.0), 0),
        constraint_cylinder_6: (point(0.0, 1.5, -2.0), vector(0.0, 0.0, 1.0), 2),
    }

    #[test]
    /// The default closed value for a cylinder p. 185
    fn default_closed_value_for_cylinder() {
        let cyl = cylinder();
        let deref_cyl = cyl.as_any().downcast_ref::<Cylinder>().unwrap();
        assert!(!deref_cyl.closed);
    }

    macro_rules! capped_cylinder_intersecting {
    ($($name:ident: $value:expr,)*) => {
    $(
        #[test]
        fn $name() {
            let (point, direction, count) = $value;

            let mut c = cylinder();
            let deref_cyl = c.as_mut_any().downcast_mut::<Cylinder>().unwrap();
            deref_cyl.set_minimum(1.0);
            deref_cyl.set_maximum(2.0);
            deref_cyl.set_closed(true);

            let direction = normalize(direction);

            let r = ray(point, direction);
            let xs = c.local_intersect(r);

            assert_eq!(count, xs.len());
        }
    )*}
    }

    capped_cylinder_intersecting! {
        capped_cylinder_1: (point(0.0, 3.0, 0.0),  vector(0.0, -1.0, 0.0), 2),
        capped_cylinder_2: (point(0.0, 3.0, -2.0), vector(0.0, -1.0, 2.0),  2),
        capped_cylinder_3: (point(0.0, 4.0, -2.0), vector(0.0, -1.0, 1.0),  2),
        capped_cylinder_4: (point(0.0, 0.0, -2.0), vector(0.0, 1.0, 2.0),  2),
        capped_cylinder_5: (point(0.0, -1.0, -2.0), vector(0.0, 1.0, 1.0),  2),
    }
}
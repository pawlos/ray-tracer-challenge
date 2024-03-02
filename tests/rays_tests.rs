use ray_tracer_challenge::*;

#[cfg(test)]
mod rays {
    use super::*;

    #[test]
    /// Creating and querying a ray
    fn creating_and_querying_a_ray() {
        let origin = point(1.0, 2.0, 3.0);
        let direction = vector(4.0, 5.0, 6.0);

        let r = ray(origin, direction);

        assert_eq!(r.origin, origin);
        assert_eq!(r.direction, direction);
    }

    #[test]
    /// Computing a point from a distance
    fn computing_a_point_from_a_distance() {
        let r = ray(point(2.0, 3.0, 4.0), vector(1.0, 0.0, 0.0));

        assert_eq!(position(r, 0.0), point(2.0, 3.0, 4.0));
        assert_eq!(position(r, 1.0), point(3.0, 3.0, 4.0));
        assert_eq!(position(r, -1.0), point(1.0, 3.0, 4.0));
        assert_eq!(position(r, 2.5), point(4.5, 3.0, 4.0));
    }

    #[test]
    /// Transforming a ray
    fn transforming_a_ray() {
        let r = ray(point(1.0, 2.0, 3.0), vector(0.0, 1.0, 0.0));
        let m = translation(3.0, 4.0, 5.0);

        let r2 = transform(r, m);

        assert_eq!(r2.origin, point(4.0, 6.0, 8.0));
        assert_eq!(r2.direction, vector(0.0, 1.0, 0.0));
    }

    #[test]
    /// Scaling a ray
    fn scaling_a_ray() {
        let r = ray(point(1.0, 2.0, 3.0), vector(0.0, 1.0, 0.0));
        let m = scaling(2.0, 3.0, 4.0);

        let r2 = transform(r, m);

        assert_eq!(r2.origin, point(2.0, 6.0, 12.0));
        assert_eq!(r2.direction, vector(0.0, 3.0, 0.0));
    }
}
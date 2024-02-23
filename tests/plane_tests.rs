#[cfg(test)]
mod plane {
    use ray_tracer_challenge::{plane, point, ray, vector};

    #[test]
    /// The normal of a plane is constant everywhere
    fn normal_of_a_plane_is_constant_everywhere() {
        let p = plane();

        let n1 = p.local_normal_at(point(0.0, 0.0, 0.0));
        let n2 = p.local_normal_at(point(10.0, 0.0, -10.0));
        let n3 = p.local_normal_at(point(-5.0, 0.0, 150.0));

        assert_eq!(n1, vector(0.0, 1.0, 0.0));
        assert_eq!(n2, vector(0.0, 1.0, 0.0));
        assert_eq!(n3, vector(0.0, 1.0, 0.0));
    }

    #[test]
    /// Intersect with a ray parallel to the plane
    fn intersect_with_a_ray_parallel_to_the_plane() {
        let p = plane();

        let r = ray(point(0.0, 10.0, 0.0), vector(0.0, 0.0, 1.0));

        let xs = p.local_intersect(r);

        assert!(xs.is_empty());
    }

    #[test]
    /// Intersect with a coplanar ray
    fn intersect_with_a_coplanar_ray() {
        let p = plane();
        let r = ray(point(0.0, 0.0, 0.0), vector(0.0, 0.0, 1.0));

        let xs = p.local_intersect(r);

        assert!(xs.is_empty());
    }

    #[test]
    /// A ray intersecting a plane from above
    fn ray_intersecting_a_plane_from_above() {
        let p = plane();
        let ray = ray(point(0.0, 1.0, 0.0), vector(0.0, -1.0, 0.0));

        let xs = p.local_intersect(ray);

        assert_eq!(xs.len(), 1);
        assert_eq!(xs[0].t, 1.0);
        assert_eq!(xs[0].object.id(), p.id());
    }

    #[test]
    /// A ray intersecting a plane from below
    fn ray_intersecting_a_plane_from_below() {
        let p = plane();
        let ray = ray(point(0.0, -1.0, 0.0), vector(0.0, 1.0, 0.0));

        let xs = p.local_intersect(ray);

        assert_eq!(xs.len(), 1);
        assert_eq!(xs[0].t, 1.0);
        assert_eq!(xs[0].object.id(), p.id());
    }
}
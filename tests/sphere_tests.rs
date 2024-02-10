use ray_tracer_challenge::*;

#[cfg(test)]
mod spheres {
    use super::*;

    #[test]
    /// A ray intersects a sphere at two points
    fn a_ray_intersects_a_sphere_at_two_points() {
        let r = ray(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let s = sphere();

        let xs = intersect(&s, r);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 4.0);
        assert_eq!(xs[1].t, 6.0);
    }

    #[test]
    /// A ray intersects a sphere at a tangent
    fn a_ray_intersects_a_sphere_at_a_tangent() {
        let r = ray(point(0.0, 1.0, -5.0), vector(0.0, 0.0, 1.0));
        let s = sphere();

        let xs = intersect(&s, r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 5.0);
        assert_eq!(xs[1].t, 5.0);
    }

    #[test]
    /// A ray misses a sphere
    fn a_ray_misses_a_sphere() {
        let r = ray(point(0.0, 2.0, -5.0), vector(0.0, 0.0, 1.0));
        let s = sphere();

        let xs = intersect(&s, r);
        assert_eq!(xs.len(), 0);
    }

    #[test]
    /// A ray originates inside a sphere
    fn a_ray_originates_inside_a_sphere() {
        let r = ray(point(0.0, 0.0, 0.0), vector(0.0, 0.0, 1.0));
        let s = sphere();

        let xs = intersect(&s, r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, -1.0);
        assert_eq!(xs[1].t, 1.0);
    }

    #[test]
    /// A sphere is behind a ray
    fn a_sphere_is_behind_a_ray() {
        let r = ray(point(0.0, 0.0, 5.0), vector(0.0, 0.0, 1.0));
        let s = sphere();

        let xs = intersect(&s, r);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, -6.0);
        assert_eq!(xs[1].t, -4.0);
    }

    #[test]
    /// A sphere's default transformation
    fn sphere_default_transformation() {
        let s = sphere();

        assert_eq!(s.transform, Matrix::identity4x4());
    }

    #[test]
    /// Changing a sphere's transformation
    fn changing_a_sphere_transformation() {
        let mut s = sphere();

        let t = translation(2.0, 3.0, 4.0);

        set_transform(&mut s, t.clone());

        assert_eq!(s.transform, t);
    }

    #[test]
    /// Intersecting a scaled sphere with a ray
    fn intersecting_a_scaled_sphere_with_a_ray() {
        let r = ray(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));

        let mut s = sphere();

        set_transform(&mut s, scaling(2.0, 2.0, 2.0));
        let xs = intersect(&s, r);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 3.0);
        assert_eq!(xs[1].t, 7.0);
    }

    #[test]
    /// Intersecting a translated sphere with a ray
    fn intersecting_a_translated_sphere_with_a_ray() {
        let r = ray(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));

        let mut s = sphere();

        set_transform(&mut s, translation(5.0, 0.0, 0.0));

        let xs = intersect(&s, r);

        assert_eq!(xs.len(), 0);
    }
}

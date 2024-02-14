use ray_tracer_challenge::*;

#[cfg(test)]
mod spheres {
    use std::f32::consts::PI;
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

    #[test]
    /// The normal on a sphere at a point on the x-axis
    fn normal_on_a_sphere_at_a_point_on_the_x_axis() {
        let s = sphere();
        let n = normal_at(&s, point(1.0, 0.0, 0.0));

        assert_eq!(n, vector(1.0, 0.0, 0.0));
    }

    #[test]
    /// The normal on a sphere at a point on the y-axis
    fn normal_on_a_sphere_at_a_point_on_the_y_axis() {
        let s = sphere();
        let n = normal_at(&s, point(0.0, 1.0, 0.0));

        assert_eq!(n, vector(0.0, 1.0, 0.0));
    }

    #[test]
    /// The normal on a sphere at a point on the z-axis
    fn normal_on_a_sphere_at_a_point_on_the_z_axis() {
        let s = sphere();
        let n = normal_at(&s, point(0.0, 0.0, 1.0));

        assert_eq!(n, vector(0.0, 0.0, 1.0));
    }

    #[test]
    /// The normal on a sphere at a point on a non-axial point
    fn normal_on_a_sphere_at_a_point_on_a_non_axial_axis() {
        let s = sphere();
        let n = normal_at(&s, point(3f32.sqrt()/3.0, 3f32.sqrt()/3.0, 3f32.sqrt()/3.0));

        assert_eq!(n, vector(3f32.sqrt()/3.0, 3f32.sqrt()/3.0, 3f32.sqrt()/3.0));
    }

    #[test]
    /// The normal is a normalized vector
    fn normal_is_a_normalized_vector() {
        let s = sphere();
        let n = normal_at(&s, point(3f32.sqrt()/3.0, 3f32.sqrt()/3.0, 3f32.sqrt()/3.0));

        assert_eq!(n, normalize(n));
    }

    #[test]
    /// Computing the normal on a translated sphere
    fn computing_the_normal_on_a_translated_sphere() {
        let mut s = sphere();
        set_transform(&mut s, translation(0.0, 1.0, 0.0));

        // std::f32::consts::FRAC_1_SQRT_2 = 0.70711
        let n = normal_at(&s, point(0.0, 1.70711, -std::f32::consts::FRAC_1_SQRT_2));

        assert_eq!(n, vector(0.0, std::f32::consts::FRAC_1_SQRT_2, -std::f32::consts::FRAC_1_SQRT_2))
    }

    #[test]
    /// Computing the normal on a transformed sphere
    fn computing_the_normal_on_a_transformed_sphere() {
        let mut s = sphere();
        let m = scaling(1.0, 0.5, 1.0) * rotation_z(PI / 5.0);
        set_transform(&mut s, m);

        let n = normal_at(&s, point(0.0, 2.0_f32.sqrt()/2.0f32, -(2.0_f32.sqrt()/2.0f32)));

        assert_eq!(n, vector(0.0, 0.97014, -0.24254))
    }

    #[test]
    /// A sphere has a default material
    fn sphere_has_a_default_material() {
        let s = sphere();
        assert_eq!(s.material, material());
    }

    #[test]
    /// A sphere may be assigned a material
    fn sphere_may_be_assigned_a_material() {
        let mut s = sphere();
        let mut m = material();
        m.ambient = 1.0;

        s.material = m.clone();
        assert_eq!(s.material, m);
    }
}

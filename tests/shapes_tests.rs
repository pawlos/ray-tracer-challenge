use ray_tracer_challenge::*;

#[cfg(test)]
mod spheres {
    use super::*;

    #[test]
    /// The default transformation
    fn default_transformation() {
        let s = test_shape();

        assert_eq!(s.transform(), Matrix::identity4x4())
    }

    #[test]
    /// Assigning a transformation
    fn assigning_a_transformation() {
        let mut s = test_shape();
        s.set_transform(translation(2.0, 3.0, 4.0));

        assert_eq!(s.transform(), translation(2.0, 3.0, 4.0))
    }

    #[test]
    /// The default material
    fn default_material() {
        let s = test_shape();

        assert_eq!(*s.material(), material())
    }

    #[test]
    /// Assigning a material
    fn assigning_a_material() {
        let mut s = test_shape();
        let mut m = material();
        m.ambient = 1.0;
        s.set_material(m);

        assert_eq!(s.material().ambient, 1.0)
    }

    #[test]
    /// Intersecting a scaled shape with a ray
    fn intersecting_a_scaled_shape_with_ray() {
        let r = ray(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));

        let mut s = test_shape();
        s.set_transform(scaling(2.0, 2.0, 2.0));

        s.intersect(r);

        //assert_eq!(s.saved_ray().origin, point(0.0, 0.0, -2.5));
        //assert_eq!(s.saved_ray().direction, vector(0.0, 0.0, 0.5));
    }

    #[test]
    /// Intersecting a translated shape with a ray
    fn intersecting_a_translated_shape_with_a_ray() {
        let r = ray(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));

        let mut s = test_shape();
        s.set_transform(translation(5.0, 0.0, 0.0));

        s.intersect(r);

        //assert_eq!(s.saved_ray().origin, point(-5.0, 0.0, -5.0));
        //assert_eq!(s.saved_ray().direction, vector(0.0, 0.0, 1.0));
    }

    #[test]
    /// Computing the normal on a translated shape
    fn computing_the_normal_on_a_translated_shape() {
        let mut s = test_shape();
        s.set_transform(translation(0.0, 1.0, 0.0));

        let n = s.normal_at(point(0.0, 1.70711, -std::f32::consts::FRAC_1_SQRT_2));

        assert_eq!(n, vector(0.0, std::f32::consts::FRAC_1_SQRT_2, -std::f32::consts::FRAC_1_SQRT_2))
    }

    #[test]
    /// A helper for producing a sphere with a glassy material
    fn helper_for_producing_a_sphere_with_a_glassy_material() {
        let s = glass_sphere();
        assert_eq!(s.transform(), Matrix::identity4x4());
        assert_eq!(s.material().transparency, 1.0);
        assert_eq!(s.material().refractive_index, 1.5);
    }
}
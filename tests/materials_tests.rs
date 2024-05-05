#[cfg(test)]
mod materials {
    use std::ops::Deref;
    use ray_tracer_challenge::*;

    pub fn setup() -> (Material, Point) {
        (material(), point(0.0, 0.0, 0.0))
    }
    #[test]
    /// The default material
    fn the_default_material() {
        let m = material();

        assert_eq!(m.color, color(1.0, 1.0, 1.0));
        assert_eq!(m.ambient, 0.1);
        assert_eq!(m.diffuse, 0.9);
        assert_eq!(m.specular, 0.9);
        assert_eq!(m.shininess, 200.0);
    }

    #[test]
    /// Lighting with the eye between the light and the surface
    fn lighting_with_the_eye_between_the_light_and_the_surface() {
        let (m, position) = setup();
        let sphere = sphere();
        let eye_v = vector(0.0, 0.0, -1.0);
        let normal_v = vector(0.0, 0.0, -1.0);
        let light = point_light(point(0.0, 0.0, -18.0), color(1.0, 1.0, 1.0));

        let result = lightning(&m, sphere.deref(), &light, position, eye_v, normal_v, false);

        assert_eq!(result, color(1.9, 1.9, 1.9))
    }

    #[test]
    /// Lightning with the eye between the light and the surface, eye offset 45°
    fn lightning_with_the_eye_between_light_and_surface_eye_offset_45deg() {
        let (m, position) = setup();
        let sphere = sphere();
        let eye_v = vector(0.0, 2.0_f32.sqrt() / 2.0, 2.0_f32.sqrt() / 2.0);
        let normal_v = vector(0.0, 0.0, -1.0);

        let light = point_light(point(0.0, 0.0, -10.0), color(1.0, 1.0, 1.0));

        let result = lightning(&m, sphere.deref(), &light, position, eye_v, normal_v, false);

        assert_eq!(result, color(1.0, 1.0, 1.0));
    }

    #[test]
    /// Lightning with eye opposite surface, light offset 45°
    fn lightning_with_eye_opposite_surface_light_offset_45deg() {
        let (m, position) = setup();
        let sphere = sphere();
        let eye_v = vector(0.0, 0.0, -1.0);
        let normal_v = vector(0.0, 0.0, -1.0);

        let light = point_light(point(0.0, 10.0, -10.0), color(1.0, 1.0, 1.0));

        let result = lightning(&m, sphere.deref(), &light, position, eye_v, normal_v, false);

        assert_eq!(result, color(0.7364, 0.7364, 0.7364));
    }

    #[test]
    /// Lightning with eye in the path of the reflection vector
    fn lightning_with_eye_in_the_path_of_the_reflection_vector() {
        let (m, position) = setup();
        let sphere = sphere();
        let eye_v = vector(0.0, -(2.0_f32.sqrt() / 2.0), -(2.0_f32.sqrt() / 2.0));
        let normal_v = vector(0.0, 0.0, -1.0);

        let light = point_light(point(0.0, 10.0, -10.0), color(1.0, 1.0, 1.0));

        let result = lightning(&m, sphere.deref(), &light, position, eye_v, normal_v, false);

        assert_eq!(result, color(1.6364, 1.6364, 1.6364));
    }

    #[test]
    /// Lightning with the light behind the surface
    fn lightning_with_the_light_behind_the_surface() {
        let (m, position) = setup();
        let sphere = sphere();
        let eye_v = vector(0.0, 0.0, -1.0);
        let normal_v = vector(0.0, 0.0, -1.0);

        let light = point_light(point(0.0, 0.0, 10.0), color(1.0, 1.0, 1.0));

        let result = lightning(&m, sphere.deref(), &light, position, eye_v, normal_v, false);

        assert_eq!(result, color(0.1, 0.1, 0.1));
    }

    #[test]
    /// Lightning with the surface in shadow
    fn lightning_with_the_surface_in_shadow() {
        let (m, position) = setup();
        let sphere = sphere();
        let eye_v = vector(0.0, 0.0, -1.0);
        let normal_v = vector(0.0, 0.0, -1.0);
        let light = point_light(point(0.0, 0.0, -10.0), color(1.0, 1.0, 1.0));
        let in_shadow = true;

        let result = lightning(&m, sphere.deref(), &light, position, eye_v, normal_v, in_shadow);

        assert_eq!(result, color(0.1, 0.1, 0.1));
    }

    #[test]
    /// Lightning with a pattern applied
    fn lightning_with_a_pattern_applied() {
        let (mut m, _) = setup();
        let sphere = sphere();
        m.pattern = Some(Box::new(stripe_pattern(color(1.0, 1.0, 1.0), color(0.0, 0.0, 0.0))));
        m.ambient = 1.0;
        m.diffuse = 0.0;
        m.specular = 0.0;

        let eyev = vector(0.0, 0.0, -1.0);
        let normalv = vector(0.0, 0.0, -1.0);
        let light = point_light(point(0.0, 0.0, -10.0), color(1.0, 1.0, 1.0));

        let c1 = lightning(&m, sphere.deref(), &light, point(0.9, 0.0, 0.0), eyev, normalv, false);
        let c2 = lightning(&m, sphere.deref(), &light, point(1.1, 0.0, 0.0), eyev, normalv, false);

        assert_eq!(c1, color(1.0, 1.0, 1.0));
        assert_eq!(c2, color(0.0, 0.0, 0.0));
    }

    #[test]
    /// Reflectivity for the default material
    fn reflectivity_for_the_default_material() {
        let m = material();
        assert_eq!(m.reflective, 0.0);
    }

    #[test]
    /// Precomputing the reflection vector
    fn precomputing_the_reflection_vector() {
        let shape = plane();
        let r = ray(point(0.0, 1.0, -1.0), vector(0.0, -2.0f32.sqrt()/2.0, 2.0f32.sqrt()/2.0));
        let i = intersection(2.0f32.sqrt(), shape.deref());
        let intersections = [].to_vec();
        let comps = prepare_computations(i, r, &intersections);
        assert_eq!(comps.reflect_v, vector(0.0, 2.0f32.sqrt()/2.0, 2.0f32.sqrt()/2.0));
    }

    #[test]
    /// Transparency and Refractive Index for the default material
    fn transparency_and_refractive_index_for_the_default_material() {
        let m = material();

        assert_eq!(m.transparency, 0.0);
        assert_eq!(m.refractive_index, 1.0);
    }
}
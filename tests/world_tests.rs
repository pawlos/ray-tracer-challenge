#[cfg(test)]
mod world {
    use ray_tracer_challenge::*;

    #[test]
    /// Creating a world
    fn creating_a_world() {
        let w = world();

        assert_eq!(w.objects.len(), 0);
        assert_eq!(w.lights.len(), 0);
    }

    #[test]
    /// The default world
    fn the_default_world() {
        let light = point_light(point(-10.0, -10.0, -10.0), color(1.0, 1.0, 1.0));

        let mut s1 = sphere();
        s1.material = material();
        s1.material.color = color(0.8, 1.0, 0.6);
        s1.material.diffuse = 0.7;
        s1.material.specular = 0.2;

        let mut s2 = sphere();
        s2.transform = scaling(0.5, 0.5, 0.5);

        let w = default_world();

        assert!(w.lights.contains(&light));
        assert!(w.objects.contains(&s1));
        assert!(w.objects.contains(&s2));
    }
}
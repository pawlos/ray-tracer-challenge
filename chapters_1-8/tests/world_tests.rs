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
        let light = point_light(point(-10.0, 10.0, -10.0), color(1.0, 1.0, 1.0));

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

    #[test]
    /// Intersects a world with a ray
    fn intersects_a_world_with_a_ray() {
        let w = default_world();
        let r = ray(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));

        let xs = intersect_world(&w, r);

        assert_eq!(xs.len(), 4);
        assert_eq!(xs[0].t, 4.0);
        assert_eq!(xs[1].t, 4.5);
        assert_eq!(xs[2].t, 5.5);
        assert_eq!(xs[3].t, 6.0);
    }

    #[test]
    /// There is no shadow when nothing is collinear with the point and light
    fn there_is_no_shadow_when_nothing_is_collinear_with_point_and_light() {
        let w = default_world();
        let p = point(0.0, 10.0, 0.0);

        assert!(!is_shadowed(&w, p));
    }

    #[test]
    /// The shadow when an object is between the point and the light
    fn shadow_when_an_object_is_between_the_point_and_the_light() {
        let w = default_world();
        let p = point(10.0, -10.0, 10.0);

        assert!(is_shadowed(&w, p));
    }

    #[test]
    /// There is no shadow when an object is behind the light
    fn there_is_no_shadow_when_an_object_is_behind_the_light() {
        let w = default_world();
        let p = point(-20.0, 20.0, -20.0);

        assert!(!is_shadowed(&w, p));
    }

    #[test]
    /// There is no shadow when an object is behind the point
    fn there_is_no_shadow_when_an_object_is_behind_the_point() {
        let w = default_world();
        let p = point(-2.0, 2.0, -2.0);

        assert!(!is_shadowed(&w, p));
    }

    #[test]
    /// shade_hit() is given_an_intersection_in_shadow
    fn shade_hit_is_given_an_intersection_in_shadow() {
        let mut w = world();
        w.lights.push(point_light(point(0.0, 0.0, -10.0), color(1.0, 1.0, 1.0)));
        let s1 = sphere();
        w.objects.push(s1);

        let mut s2 = sphere();
        s2.transform = translation(0.0, 0.0, 10.0);
        w.objects.push(s2);

        let r = ray(point(0.0, 0.0, 5.0), vector(0.0, 0.0, 1.0));

        let i = intersection(4.0, &w.objects[1]);

        let comps = prepare_computations(i, r);
        let c = shade_hit(&w, &comps);

        assert_eq!(c, color(0.1, 0.1, 0.1));
    }
}
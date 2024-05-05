#[cfg(test)]
mod world {
    use std::ops::{Deref, DerefMut};
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
        let mut m = material();
        m.color = color(0.8, 1.0, 0.6);
        m.diffuse = 0.7;
        m.specular = 0.2;
        s1.set_material(m);

        let mut s2 = sphere();
        s2.set_transform(scaling(0.5, 0.5, 0.5));

        let w = default_world();

        assert!(w.lights.contains(&light));
        //TODO: fix
        //assert!(w.objects.contains(&s1));
        //assert!(w.objects.contains(&s2));
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
        s2.set_transform(translation(0.0, 0.0, 10.0));
        w.objects.push(s2);

        let r = ray(point(0.0, 0.0, 5.0), vector(0.0, 0.0, 1.0));

        let i = intersection(4.0, w.objects[1].deref());

        let intersections = [].to_vec();
        let comps = prepare_computations(i, r, &intersections);
        let c = shade_hit(&w, &comps, DEFAULT_REFLECTION_NUMBER);

        assert_eq!(c, color(0.1, 0.1, 0.1));
    }

    #[test]
    /// The reflected color for a non-reflective material
    fn reflected_color_for_a_non_reflective_material() {
        let mut w = default_world();
        let r = ray(point(0.0, 0.0, 0.0), vector(0.0, 0.0, 1.0));

        let shape = &mut w.objects[1];
        let mut material = material();
        material.ambient = 1.0;
        shape.set_material(material);

        let intersections = [].to_vec();
        let i = intersection(1.0, w.objects[1].deref());
        let comps = prepare_computations(i, r, &intersections);

        let reflected_color = reflected_color(&w, &comps, DEFAULT_REFLECTION_NUMBER);

        assert_eq!(reflected_color, color(0.0, 0.0, 0.0));
    }

    #[test]
    /// The reflected color for a reflective material
    fn reflected_color_for_a_reflective_material() {
        let mut w = default_world();

        let mut shape = plane();
        let shape_ref = shape.deref_mut();
        shape_ref.mut_material().reflective = 0.5;
        shape_ref.set_transform(translation(0.0, -1.0, 0.0));
        w.objects.push(shape);

        let r = ray(point(0.0, 0.0, -3.0), vector(0.0, -(2.0f32.sqrt()/2.0), 2.0f32.sqrt()/2.0));
        let i = intersection(2.0f32.sqrt(), w.objects[2].deref());

        let intersections = [].to_vec();
        let comps = prepare_computations(i, r, &intersections);
        let reflected_color = reflected_color(&w, &comps, DEFAULT_REFLECTION_NUMBER);

        assert_eq!(reflected_color, color(0.19032, 0.2379, 0.14274))
    }

    #[test]
    /// shade_hit() with a reflective material
    fn shade_hit_with_a_reflective_material() {
        let mut w = default_world();
        let mut shape = plane();
        let shape_ref = shape.deref_mut();
        shape_ref.mut_material().reflective = 0.5;
        shape_ref.set_transform(translation(0.0, -1.0, 0.0));
        w.objects.push(shape);

        let r = ray(point(0.0, 0.0, -3.0), vector(0.0, -(2.0f32.sqrt()/2.0), 2.0f32.sqrt()/2.0));
        let i = intersection(2.0f32.sqrt(), w.objects[2].deref());

        let intersections = [].to_vec();
        let comps = prepare_computations(i, r, &intersections);

        let c = shade_hit(&w, &comps, DEFAULT_REFLECTION_NUMBER);

        assert_eq!(c, color(0.87677, 0.92436, 0.82918));
    }

    #[test]
    /// color_at() with mutually reflective surfaces
    fn color_at_with_mutually_reflective_surfaces() {
        let mut w = world();
        w.lights.push(point_light(point(0.0, 0.0, 0.0), color(1.0, 1.0, 1.0)));
        let mut lower = plane();
        lower.mut_material().reflective  = 1.0;
        lower.set_transform(translation(0.0, -1.0, 0.0));
        w.objects.push(lower);

        let mut upper = plane();
        upper.mut_material().reflective  = 1.0;
        upper.set_transform(translation(0.0, 1.0, 0.0));
        w.objects.push(upper);

        let r = ray(point(0.0, 0.0, 0.0), vector(0.0, 1.0, 0.0));

        color_at(&w, r, DEFAULT_REFLECTION_NUMBER);
    }

    #[test]
    /// The refracted color with an opaque surface p. 155
    fn refracted_color_with_an_opaque_surface() {
        let w = default_world();
        let shape = w.objects.first().unwrap();
        let r = ray(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let xs = [intersection(4.0, shape.deref()), intersection(6.0, shape.deref())].to_vec();

        let comps = prepare_computations(xs[0], r, &xs);
        let c = refracted_color(&w, &comps, 5);

        assert_eq!(c, color(0.0,0.0,0.0))
    }

    #[test]
    /// The refracted color at the maximum recursive depth p. 156
    fn refracted_color_at_maximum_recursive_depth() {
        let mut w = default_world();
        let r = ray(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));

        let shape = &mut w.objects[0];
        shape.mut_material().transparency = 1.0;
        shape.mut_material().refractive_index = 1.5;

        let xs = [intersection(4.0, w.objects[0].deref()), intersection(6.0, w.objects[0].deref())].to_vec();

        let comps = prepare_computations(xs[0], r, &xs);
        let c = refracted_color(&w, &comps, 0);

        assert_eq!(c, color(0.0,0.0,0.0))
    }

    #[test]
    /// The refracted color under total internal reflection
    fn refracted_color_under_total_internal_reflection() {
        let mut w = default_world();
        let r = ray(point(0.0, 0.0, 2.0f32.sqrt()/2.0), vector(0.0, 1.0, 0.0));

        let shape = &mut w.objects[0];
        shape.mut_material().transparency = 1.0;
        shape.mut_material().refractive_index = 1.5;

        let xs = [intersection(-(2.0f32.sqrt()/2.0), w.objects[0].deref()),
                                 intersection(2.0f32.sqrt()/2.0, w.objects[0].deref())]
                                .to_vec();

        let comps = prepare_computations(xs[1], r, &xs);
        let c = refracted_color(&w, &comps, 5);

        assert_eq!(c, color(0.0,0.0,0.0))
    }

    #[test]
    /// The refracted color with a refracted ray
    fn refracted_color_with_a_refracted_ray() {
        let mut w = default_world();
        let a = &mut w.objects[0];
        a.mut_material().ambient = 1.0;
        a.mut_material().pattern = Some(Box::new(test_pattern()));

        let b = &mut w.objects[1];
        b.mut_material().transparency = 1.0;
        b.mut_material().refractive_index = 1.5;

        let r = ray(point(0.0, 0.0, 0.1), vector(0.0, 1.0, 0.0));
        let xs = [intersection(-0.9899, w.objects[0].deref()), intersection(-0.4899, w.objects[1].deref()),
                               intersection(0.4899, w.objects[1].deref()), intersection(0.9899, w.objects[0].deref())].to_vec();

        let comps = prepare_computations(xs[2], r, &xs);

        let c = refracted_color(&w, &comps, 5);

        assert_eq!(c, color(0.0, 0.99888, 0.04725));
    }

    #[test]
    /// shade_hit() with a transparent material p. 159
    fn shade_hit_with_a_transparent_material() {
        let mut w = default_world();
        let mut floor = plane();
        floor.set_transform(translation(0.0, -1.0, 0.0));
        floor.mut_material().transparency = 0.5;
        floor.mut_material().refractive_index = 1.5;

        w.objects.push(floor);

        let mut ball = sphere();
        ball.mut_material().color = color(1.0, 0.0, 0.0);
        ball.mut_material().ambient = 0.5;
        ball.set_transform(translation(0.0, -3.5, -0.5));
        w.objects.push(ball);

        let r = ray(point(0.0, 0.0, -3.0), vector(0.0, -(2.0f32.sqrt()/2.0), 2.0f32.sqrt()/2.0));

        let xs = [intersection(2.0f32.sqrt(), w.objects[2].deref())].to_vec();

        let comps = prepare_computations(xs[0], r, &xs);

        let c = shade_hit(&w, &comps, 5);
        assert_eq!(c, color(0.93642, 0.68642, 0.68642))
    }
}
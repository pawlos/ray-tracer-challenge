use ray_tracer_challenge::*;

#[cfg(test)]
mod intersection {
    use std::ops::{Deref, DerefMut};
    use super::*;

    #[test]
    /// An intersection encapsulates t and object
    fn an_intersection_encapsulates_t_and_object() {
        let s = sphere();
        let i = intersection(3.5, s.deref());

        assert_eq!(i.t, 3.5);
        assert_eq!(i.object.id(), s.id());
    }

    #[test]
    /// Aggregating intersections
    fn aggregating_intersections() {
        let s = sphere();
        let i1 = intersection(1.0, s.deref());
        let i2 = intersection(2.0, s.deref());

        let xs = [i1, i2];

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 1.0);
        assert_eq!(xs[1].t, 2.0);
    }

    #[test]
    /// Intersect sets the object on the intersection
    fn intersect_sets_the_object_on_the_intersection() {
        let r = ray(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let s = sphere();

        let xs = s.intersect(r);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].object.id(), s.id());
        assert_eq!(xs[1].object.id(), s.id());
    }

    #[test]
    /// The hit, when all intersections have positive t
    fn hit_when_all_intersections_have_positive_t() {
        let s = sphere();
        let i1 = intersection(1.0, s.deref());
        let i2 = intersection(2.0, s.deref());

        let mut xs = [i2, i1].to_vec();

        let i = hit(&mut xs);

        assert_eq!(i.unwrap(), i1);
    }

    #[test]
    /// The hit, when some intersections have negative t
    fn hit_when_some_intersections_have_negative_t() {
        let s = sphere();
        let i1 = intersection(-1.0, s.deref());
        let i2 = intersection(1.0, s.deref());

        let mut xs = [i2, i1].to_vec();

        let i = hit(&mut xs);

        assert_eq!(i.unwrap(), i2);
    }

    #[test]
    /// The hit, when all intersections have negative t
    fn hit_when_all_intersections_have_negative_t() {
        let s = sphere();
        let i1 = intersection(-2.0, s.deref());
        let i2 = intersection(-1.0, s.deref());

        let mut xs = [i2, i1].to_vec();

        let i = hit(&mut xs);

        assert_eq!(i, None);
    }

    #[test]
    /// The hit is always the lowest non-negative intersection
    fn hit_is_always_the_lowest_non_negative_intersection() {
        let s = sphere();
        let i1 = intersection(5.0, s.deref());
        let i2 = intersection(7.0, s.deref());
        let i3 = intersection(-3.0, s.deref());
        let i4 =intersection(2.0, s.deref());

        let mut xs  = [i1, i2, i3, i4].to_vec();

        let i = hit(&mut xs);

        assert_eq!(i.unwrap(), i4)
    }

    #[test]
    /// Precomputing the state of an intersection
    fn precomputing_the_state_of_an_intersection() {
        let r = ray(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let shape = sphere();
        let i = intersection(4.0, shape.deref());

        let intersections = [].to_vec();
        let comps = prepare_computations(i, r, &intersections);

        assert_eq!(comps.t, i.t);
        assert_eq!(comps.object.id(), i.object.id());
        assert_eq!(comps.point, point(0.0, 0.0, -1.0));
        assert_eq!(comps.eye_v, vector(0.0, 0.0, -1.0));
        assert_eq!(comps.normal_v, vector(0.0, 0.0, -1.0));
    }

    #[test]
    /// The hit, when an intersection occurs on the outside
    fn hit_when_an_intersection_occurs_on_the_outside() {
        let r = ray(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let shape = sphere();
        let i = intersection(4.0, shape.deref());

        let intersections = [].to_vec();
        let comps = prepare_computations(i, r, &intersections);
        assert!(!comps.inside)
    }

    #[test]
    /// The hit, when an intersection occurs on the inside
    fn hit_when_an_intersection_occurs_on_the_inside() {
        let r = ray(point(0.0, 0.0, 0.0), vector(0.0, 0.0, 1.0));
        let shape = sphere();
        let i = intersection(1.0, shape.deref());

        let intersections = [].to_vec();
        let comps = prepare_computations(i, r, &intersections);
        assert_eq!(comps.point, point(0.0, 0.0, 1.0));
        assert_eq!(comps.eye_v, vector(0.0, 0.0, -1.0));
        assert!(comps.inside);
        assert_eq!(comps.normal_v, vector(0.0, 0.0, -1.0));
    }

    #[test]
    /// Shading an intersection
    fn shading_an_intersection() {
        let w = default_world();
        let r = ray(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let shape = w.objects.first().unwrap();

        let i = intersection(4.0, shape.deref());

        let intersections = [].to_vec();
        let comps = prepare_computations(i, r, &intersections);
        let c = shade_hit(&w, &comps, DEFAULT_REFLECTION_NUMBER);

        assert_eq!(c, color(0.38066, 0.47583, 0.2855));
    }

    #[test]
    /// Shading an intersection from the inside
    fn shading_an_intersection_from_the_inside() {
        let mut w = default_world();
        w.lights[0] = point_light(point(0.0, 0.25, 0.0), color(1.0, 1.0, 1.0));
        let r = ray(point(0.0, 0.0, 0.0), vector(0.0, 0.0, 1.0));
        let shape = &w.objects[1];

        let i = intersection(0.5, shape.deref());

        let intersections = [].to_vec();
        let comps = prepare_computations(i, r, &intersections);
        let c = shade_hit(&w, &comps, DEFAULT_REFLECTION_NUMBER);

        assert_eq!(c, color(0.90498, 0.90498, 0.90498));
    }

    #[test]
    /// The color when a ray misses
    fn color_when_ray_misses() {
        let w = default_world();
        let r = ray(point(0.0, 0.0, -5.0), vector(0.0, 1.0, 0.0));
        let c = color_at(&w, r, DEFAULT_REFLECTION_NUMBER);

        assert_eq!(c, color(0.0, 0.0, 0.0));
    }

    #[test]
    /// The color when a ray hits
    fn color_when_a_ray_hits() {
        let w = default_world();
        let r = ray(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let c = color_at(&w, r, DEFAULT_REFLECTION_NUMBER);

        assert_eq!(c, color(0.38066, 0.47583, 0.2855));
    }

    #[test]
    /// The color with an intersection behind the ray
    fn color_with_an_intersection_behind_the_ray() {
        let mut w = default_world();
        let mut m1 = material();
        m1.ambient = 1.0;
        w.objects[0].set_material(m1);
        let mut m2 = material();
        m2.ambient = 1.0;
        w.objects[1].set_material(m2);

        let r = ray(point(0.0, 0.0, 0.75), vector(0.0, 0.0, -1.0));
        let c = color_at(&w, r, DEFAULT_REFLECTION_NUMBER);

        assert_eq!(c, w.objects[1].material().color);
    }

    #[test]
    /// The hit should offset the point
    fn hit_should_offset_the_point() {
        let r = ray(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let mut shape = sphere();
        shape.set_transform(translation(0.0, 0.0, 1.0));

        let i = intersection(5.0, shape.deref());

        let intersections = [].to_vec();
        let comps = prepare_computations(i, r, &intersections);

        assert!(comps.over_point.z < -EPS/2.0);
        assert!(comps.point.z > comps.over_point.z);
    }

    #[test]
    /// Finding n1 and n2 at various intersections p.152
    fn finding_n1_and_n2_at_various_intersections() {
        let mut a = glass_sphere();
        let a = a.deref_mut();
        a.set_transform(scaling(2.0, 2.0, 2.0));
        a.mut_material().refractive_index = 1.5;

        let mut b = glass_sphere();
        let b = b.deref_mut();
        b.set_transform(translation(0.0, 0.0, -0.25));
        b.mut_material().refractive_index = 2.0;

        let mut c = glass_sphere();
        let c = c.deref_mut();
        c.set_transform(translation(0.0, 0.0, 0.25));
        c.mut_material().refractive_index = 2.5;

        let r = ray(point(0.0, 0.0, -4.0), vector(0.0, 0.0, 1.0));

        let xs1 = intersection(2.0, a);
        let xs2 = intersection(2.75, b);
        let xs3 = intersection(3.25, c);
        let xs4 = intersection(4.75, b);
        let xs5 = intersection(5.25, c);
        let xs6 = intersection(6.0, a);

        let xs = [xs1, xs2, xs3, xs4, xs5, xs6].to_vec();

        let comps1 = prepare_computations(xs1, r, &xs);
        let comps2 = prepare_computations(xs2, r, &xs);
        let comps3 = prepare_computations(xs3, r, &xs);
        let comps4 = prepare_computations(xs4, r, &xs);
        let comps5 = prepare_computations(xs5, r, &xs);
        let comps6 = prepare_computations(xs6, r, &xs);

        assert_eq!(comps1.n1, 1.0);
        assert_eq!(comps1.n2, 1.5);

        assert_eq!(comps2.n1, 1.5);
        assert_eq!(comps2.n2, 2.0);

        assert_eq!(comps3.n1, 2.0);
        assert_eq!(comps3.n2, 2.5);

        assert_eq!(comps4.n1, 2.5);
        assert_eq!(comps4.n2, 2.5);

        assert_eq!(comps5.n1, 2.5);
        assert_eq!(comps5.n2, 1.5);

        assert_eq!(comps6.n1, 1.5);
        assert_eq!(comps6.n2, 1.0);
    }

    #[test]
    /// The under point is offset below the surface p. 154
    fn under_point_is_offset_below_the_surface() {
        let r = ray(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));

        let mut shape = glass_sphere();
        shape.set_transform(translation(0.0, 0.0, 1.0));

        let i = intersection(5.0, shape.deref());
        let xs = [i].to_vec();

        let comps = prepare_computations(i, r, &xs);

        assert!(comps.under_point.z > EPS/2.0);
        assert!(comps.point.z < comps.under_point.z);
    }
}
use ray_tracer_challenge::*;

#[cfg(test)]
mod intersection {
    use super::*;

    #[test]
    /// An intersection encapsulates t and object
    fn an_intersection_encapsulates_t_and_object() {
        let s = sphere();
        let i = intersection(3.5, &s);

        assert_eq!(i.t, 3.5);
        assert_eq!(*i.object, s);
    }

    #[test]
    /// Aggregating intersections
    fn aggregating_intersections() {
        let s = sphere();
        let i1 = intersection(1.0, &s);
        let i2 = intersection(2.0, &s);

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

        let xs = intersect(&s, r);

        assert_eq!(xs.len(), 2);
        assert_eq!(*xs[0].object, s);
        assert_eq!(*xs[1].object, s);
    }

    #[test]
    /// The hit, when all intersections have positive t
    fn hit_when_all_intersections_have_positive_t() {
        let s = sphere();
        let i1 = intersection(1.0, &s);
        let i2 = intersection(2.0, &s);

        let mut xs = [i2, i1];

        let i = hit(&mut xs[..]);

        assert_eq!(i.unwrap(), i1);
    }

    #[test]
    /// The hit, when some intersections have negative t
    fn hit_when_some_intersections_have_negative_t() {
        let s = sphere();
        let i1 = intersection(-1.0, &s);
        let i2 = intersection(1.0, &s);

        let mut xs = [i2, i1];

        let i = hit(&mut xs[..]);

        assert_eq!(i.unwrap(), i2);
    }

    #[test]
    /// The hit, when all intersections have negative t
    fn hit_when_all_intersections_have_negative_t() {
        let s = sphere();
        let i1 = intersection(-2.0, &s);
        let i2 = intersection(-1.0, &s);

        let mut xs = [i2, i1];

        let i = hit(&mut xs[..]);

        assert_eq!(i, None);
    }

    #[test]
    /// The hit is always the lowest non-negative intersection
    fn hit_is_always_the_lowest_non_negative_intersection() {
        let s = sphere();
        let i1 = intersection(5.0, &s);
        let i2 = intersection(7.0, &s);
        let i3 = intersection(-3.0, &s);
        let i4 =intersection(2.0, &s);

        let mut xs  = [i1, i2, i3, i4];

        let i = hit(&mut xs[..]);

        assert_eq!(i.unwrap(), i4)
    }

    #[test]
    /// Precomputing the state of an intersection
    fn precomputing_the_state_of_an_intersection() {
        let r = ray(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let shape = sphere();
        let i = intersection(4.0, &shape);

        let comps = prepare_computations(i, r);

        assert_eq!(comps.t, i.t);
        assert_eq!(comps.object, i.object);
        assert_eq!(comps.point, point(0.0, 0.0, -1.0));
        assert_eq!(comps.eye_v, vector(0.0, 0.0, -1.0));
        assert_eq!(comps.normal_v, vector(0.0, 0.0, -1.0));
    }

    #[test]
    /// The hit, when an intersection occurs on the outside
    fn hit_when_an_intersection_occurs_on_the_outside() {
        let r = ray(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let shape = sphere();
        let i = intersection(4.0, &shape);

        let comps = prepare_computations(i, r);
        assert!(!comps.inside)
    }

    #[test]
    /// The hit, when an intersection occurs on the inside
    fn hit_when_an_intersection_occurs_on_the_inside() {
        let r = ray(point(0.0, 0.0, 0.0), vector(0.0, 0.0, 1.0));
        let shape = sphere();
        let i = intersection(1.0, &shape);

        let comps = prepare_computations(i, r);
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

        let i = intersection(4.0, shape);

        let comps = prepare_computations(i, r);
        let c = shade_hit(&w, &comps);

        assert_eq!(c, color(0.38066, 0.47583, 0.2855));
    }

    #[test]
    /// Shading an intersection from the inside
    fn shading_an_intersection_from_the_inside() {
        let mut w = default_world();
        w.lights[0] = point_light(point(0.0, 0.25, 0.0), color(1.0, 1.0, 1.0));
        let r = ray(point(0.0, 0.0, 0.0), vector(0.0, 0.0, 1.0));
        let shape = &w.objects[1];

        let i = intersection(0.5, shape);

        let comps = prepare_computations(i, r);
        let c = shade_hit(&w, &comps);

        assert_eq!(c, color(0.90498, 0.90498, 0.90498));
    }

    #[test]
    /// The color when a ray misses
    fn color_when_ray_misses() {
        let w = default_world();
        let r = ray(point(0.0, 0.0, -5.0), vector(0.0, 1.0, 0.0));
        let c = color_at(&w, r);

        assert_eq!(c, color(0.0, 0.0, 0.0));
    }

    #[test]
    /// The color when a ray hits
    fn color_when_a_ray_hits() {
        let w = default_world();
        let r = ray(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let c = color_at(&w, r);

        assert_eq!(c, color(0.38066, 0.47583, 0.2855));
    }

    #[test]
    /// The color with an intersection behind the ray
    fn color_with_an_intersection_behind_the_ray() {
        let mut w = default_world();
        w.objects[0].material.ambient = 1.0;
        w.objects[1].material.ambient = 1.0;

        let r = ray(point(0.0, 0.0, 0.75), vector(0.0, 0.0, -1.0));
        let c = color_at(&w, r);

        assert_eq!(c, w.objects[1].material.color);
    }

    #[test]
    /// The hit should offset the point
    fn hit_should_offset_the_point() {
        let r = ray(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let mut shape = sphere();
        shape.transform = translation(0.0, 0.0, 1.0);

        let i = intersection(5.0, &shape);

        let comps = prepare_computations(i, r);

        assert!(comps.over_point.z < -EPS/2.0);
        assert!(comps.point.z > comps.over_point.z);
    }
}
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
    // The hit, when some intersections have negative t
    fn hit_when_some_intersections_have_negative_t() {
        let s = sphere();
        let i1 = intersection(-1.0, &s);
        let i2 = intersection(1.0, &s);

        let mut xs = [i2, i1];

        let i = hit(&mut xs[..]);

        assert_eq!(i.unwrap(), i2);
    }

    #[test]
    // The hit, when all intersections have negative t
    fn hit_when_all_intersections_have_negative_t() {
        let s = sphere();
        let i1 = intersection(-2.0, &s);
        let i2 = intersection(-1.0, &s);

        let mut xs = [i2, i1];

        let i = hit(&mut xs[..]);

        assert_eq!(i, None);
    }

    #[test]
    // The hit is always the lowest non-negative intersection
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
}
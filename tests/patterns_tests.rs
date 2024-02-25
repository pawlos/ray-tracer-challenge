use ray_tracer_challenge::*;

#[cfg(test)]
mod patterns_tests {
    use std::ops::Deref;
    use super::*;

    fn setup() -> (Color, Color) {
        (color(1.0, 1.0, 1.0), color(0.0, 0.0, 0.0))
    }

    #[test]
    /// Creating a stripe pattern
    fn creating_a_stripe_pattern() {
        let (white, black) = setup();
        let pattern = stripe_pattern(white, black);

        assert_eq!(pattern.a, white);
        assert_eq!(pattern.b, black);
    }

    #[test]
    /// A stripe pattern is constant in y
    fn stripe_pattern_is_constant_in_y() {
        let (white, black) = setup();

        let pattern = stripe_pattern(white, black);

        assert_eq!(pattern.pattern_at(point(0.0, 0.0, 0.0)), white);
        assert_eq!(pattern.pattern_at(point(0.0, 1.0, 0.0)), white);
        assert_eq!(pattern.pattern_at(point(0.0, 2.0, 0.0)), white);
    }

    #[test]
    /// A stripe pattern is constant in z
    fn stripe_pattern_is_constant_in_z() {
        let (white, black) = setup();

        let pattern = stripe_pattern(white, black);

        assert_eq!(pattern.pattern_at(point(0.0, 0.0, 0.0)), white);
        assert_eq!(pattern.pattern_at(point(0.0, 0.0, 1.0)), white);
        assert_eq!(pattern.pattern_at(point(0.0, 0.0, 2.0)), white);
    }

    #[test]
    /// A stripe pattern is constant in z
    fn stripe_pattern_alternates_in_x() {
        let (white, black) = setup();

        let pattern = stripe_pattern(white, black);

        assert_eq!(pattern.pattern_at(point(1.0, 0.0, 0.0)), black);
        assert_eq!(pattern.pattern_at(point(0.0, 0.0, 0.0)), white);
        assert_eq!(pattern.pattern_at(point(-0.1, 0.0, 0.0)), black);
        assert_eq!(pattern.pattern_at(point(0.9, 0.0, 0.0)), white);
        assert_eq!(pattern.pattern_at(point(-1.0, 0.0, 0.0)), black);
        assert_eq!(pattern.pattern_at(point(-1.1, 0.0, 0.0)), white);
    }

    #[test]
    /// Stripes with an object transformation
    fn stripes_with_an_object_transformation() {
        let (white, black) = setup();
        let mut object = sphere();
        object.set_transform(scaling(2.0, 2.0, 2.0));
        let pattern = stripe_pattern(white, black);

        let c = pattern.pattern_at_shape(object.deref(), point(1.5, 0.0, 0.0));

        assert_eq!(c, white)
    }

    #[test]
    /// Stripes with a pattern transformation
    fn stripes_with_a_pattern_transformation() {
        let (white, black) = setup();
        let object = sphere();
        let mut pattern = stripe_pattern(white, black);
        set_pattern_transformation(&mut pattern, scaling(2.0, 2.0, 2.0));

        let c = pattern.pattern_at_shape(object.deref(), point(1.5, 0.0, 0.0));

        assert_eq!(c, white)
    }

    #[test]
    /// Stripes with both an object and a pattern transformation
    fn stripes_with_both_an_object_and_pattern_transformation() {
        let (white, black) = setup();
        let mut object = sphere();
        object.set_transform(scaling(2.0, 2.0, 2.0));
        let mut pattern = stripe_pattern(white, black);
        set_pattern_transformation(&mut pattern, translation(0.5, 0.0, 2.0));

        let c = pattern.pattern_at_shape(object.deref(), point(2.5, 0.0, 0.0));

        assert_eq!(c, white)
    }

    #[test]
    /// The default pattern transformation
    fn default_pattern_transformation() {
        let pattern = test_pattern();

        assert_eq!(pattern.transform(), Matrix::identity4x4());
    }

    #[test]
    /// Assigning a transformation
    fn assigning_a_transformation() {
        let mut pattern = test_pattern();
        pattern.set_transform(translation(1.0, 2.0, 3.0));

        assert_eq!(pattern.transform(), translation(1.0, 2.0, 3.0));
    }
}
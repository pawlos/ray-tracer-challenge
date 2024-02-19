use ray_tracer_challenge::*;

#[cfg(test)]
mod transformations {
    use std::f32::consts::PI;

    use super::*;

    #[test]
    /// Multiplying by translation matrix
    fn multiplying_by_translation_matrix() {
        let transform = translation(5.0, -3.0, 2.0);
        let p = point(-3.0, 4.0, 5.0);

        assert_eq!(transform * p, point(2.0, 1.0, 7.0))
    }

    #[test]
    /// Multiplying by the inverse of a translation matrix
    fn multiplying_by_the_inverse_of_a_translation_matrix() {
        let transform = translation(5.0, -3.0, 2.0);
        let inv = inverse(&transform);
        let p = point(-3.0, 4.0, 5.0);

        assert_eq!(inv * p, point(-8.0, 7.0, 3.0))
    }

    #[test]
    /// Translation does not affect vectors
    fn translation_does_not_affect_vectors() {
        let transform = translation(5.0, -3.0, 2.0);
        let v = vector(-3.0, 4.0, 5.0);
        assert_eq!(transform * v, v);
    }

    #[test]
    /// A scaling matrix applied to a point
    fn scaling_matrix_applied_to_a_point() {
        let transform = scaling(2.0, 3.0, 4.0);
        let p = point(-4.0, 6.0, 8.0);
        assert_eq!(transform * p, point(-8.0, 18.0, 32.0));
    }

    #[test]
    /// A scaling matrix applied to a vector
    fn scaling_matrix_applied_to_a_vector() {
        let transform = scaling(2.0, 3.0, 4.0);
        let v = vector(-4.0, 6.0, 8.0);
        assert_eq!(transform * v, vector(-8.0, 18.0, 32.0));
    }

    #[test]
    /// Multiplying by the inverse of a scaling matrix
    fn multiplying_by_the_inverse_of_a_scaling_matrix() {
        let transform = scaling(2.0, 3.0, 4.0);
        let inv = inverse(&transform);
        let v = vector(-4.0, 6.0, 8.0);

        assert_eq!(inv * v, vector(-2.0, 2.0, 2.0))
    }

    #[test]
    /// Reflection is scaling by a negative value
    fn reflection_is_scaling_by_a_negative_value() {
        let transform = scaling(-1.0, 1.0, 1.0);
        let p = point(2.0, 3.0, 4.0);

        assert_eq!(transform * p, point(-2.0, 3.0, 4.0))
    }

    #[test]
    /// Rotating a point around the x-axis
    fn rotating_a_point_around_the_x_axis() {
        let p = point(0.0, 1.0, 0.0);
        let half_quarter = rotation_x(PI / 4.0);
        let full_quarter = rotation_x(PI / 2.0);

        assert_eq!(half_quarter * p, point( 0.0, 2.0f32.sqrt() / 2.0, 2.0f32.sqrt() / 2.0));
        assert_eq!(full_quarter * p, point( 0.0, 0.0, 1.0));
    }

    #[test]
    /// Rotating a point around the y-axis
    fn rotating_a_point_around_y_axis() {
        let p = point(0.0, 0.0, 1.0);
        let half_quarter = rotation_y(PI / 4.0);
        let full_quarter = rotation_y(PI / 2.0);

        assert_eq!(half_quarter * p, point(2.0f32.sqrt() / 2.0, 0.0, 2.0f32.sqrt() / 2.0));
        assert_eq!(full_quarter * p, point(1.0, 0.0, 0.0));
    }

    #[test]
    /// Rotating a point around the z aix
    fn rotation_a_point_around_z_axis() {
        let p = point(0.0, 1.0, 0.0);
        let half_quarter = rotation_z(PI / 4.0);
        let full_quarter = rotation_z(PI / 2.0);

        assert_eq!(half_quarter * p, point(-(2.0f32.sqrt() / 2.0), 2.0f32.sqrt() / 2.0, 0.0));
        assert_eq!(full_quarter * p, point(-1.0, 0.0, 0.0));
    }

    #[test]
    ///A shearing transformation moves x in proportion to y
    fn shearing_transformation_moves_x_in_proportion_to_y() {
        let transform = shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        let p = point(2.0, 3.0, 4.0);

        assert_eq!(transform * p, point(5.0, 3.0, 4.0));
    }

    #[test]
    /// A shearing transformation moves x in proportion to z
    fn shearing_transformation_moves_x_in_proportion_to_z()
    {
        let transform = shearing(0.0, 1.0, 0.0, 0.0, 0.0, 0.0);
        let p = point(2.0, 3.0, 4.0);

        assert_eq!(transform * p, point(6.0, 3.0, 4.0));
    }

    #[test]
    ///A shearing transformation moves y in proportion to x
    fn shearing_transformation_moves_y_in_proportion_to_x() {
        let transform = shearing(0.0, 0.0, 1.0, 0.0, 0.0, 0.0);
        let p = point(2.0, 3.0, 4.0);

        assert_eq!(transform * p, point(2.0, 5.0, 4.0));
    }

    #[test]
    /// A shearing transformation moves y in proportion to z
    fn shearing_transformation_moves_y_in_proportion_to_z()
    {
        let transform = shearing(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
        let p = point(2.0, 3.0, 4.0);

        assert_eq!(transform * p, point(2.0, 7.0, 4.0));
    }

    #[test]
    ///A shearing transformation moves z in proportion to x
    fn shearing_transformation_moves_z_in_proportion_to_x() {
        let transform = shearing(0.0, 0.0, 0.0, 0.0, 1.0, 0.0);
        let p = point(2.0, 3.0, 4.0);

        assert_eq!(transform * p, point(2.0, 3.0, 6.0));
    }

    #[test]
    /// A shearing transformation moves z in proportion to y
    fn shearing_transformation_moves_z_in_proportion_to_y()
    {
        let transform = shearing(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
        let p = point(2.0, 3.0, 4.0);

        assert_eq!(transform * p, point(2.0, 3.0, 7.0));
    }

    #[test]
    /// Individual transformations are applied in sequence
    fn individual_transformations_are_applied_in_sequence() {
        let p = point(1.0, 0.0, 1.0);
        let a = rotation_x(PI /2.0);
        let b = scaling(5.0, 5.0, 5.0);
        let c = translation(10.0, 5.0, 7.0);

        let p2 = a * p;
        assert_eq!(p2, point(1.0, -1.0, 0.0));

        let p3 = b * p2;
        assert_eq!(p3, point(5.0, -5.0, 0.0));

        let p4 = c * p3;
        assert_eq!(p4, point(15.0, 0.0, 7.0));
    }

    #[test]
    /// Chained transformations must be applied in reverse order
    fn chained_transformations_must_be_applied_in_reverse_order() {
        let p = point(1.0, 0.0, 1.0);
        let a = rotation_x(PI /2.0);
        let b = scaling(5.0, 5.0, 5.0);
        let c = translation(10.0, 5.0, 7.0);

        let t = c * b * a;

        assert_eq!(t * p, point(15.0, 0.0, 7.0));
    }

    #[test]
    /// The transformation matrix for the default orientation
    fn transformation_matrix_for_the_default_orientation() {
        let from = point(0.0, 0.0, 0.0);
        let to = point(0.0, 0.0, -1.0);
        let up = vector(0.0, 1.0, 0.0);

        let t = view_transformation(from, to, up);
        assert_eq!(t, Matrix::identity4x4());
    }

    #[test]
    /// A view transformation matrix looking in positive z direction
    fn view_transformation_matrix_looking_in_positive_z_direction() {
        let from = point(0.0, 0.0, 0.0);
        let to = point(0.0, 0.0, 1.0);
        let up = vector(0.0, 1.0, 0.0);

        let t = view_transformation(from, to, up);

        assert_eq!(t, scaling(-1.0, 1.0, -1.0));
    }

    #[test]
    /// The view transformation moves the world
    fn view_transformation_moves_the_world() {
        let from = point(0.0, 0.0, 8.0);
        let to = point(0.0, 0.0, 0.0);
        let up = vector(0.0, 1.0, 0.0);

        let t = view_transformation(from, to, up);

        assert_eq!(t, translation(0.0, 0.0, -8.0));
    }

    #[test]
    /// An arbitrary view transformation
    fn arbitrary_view_transformation() {
        let from = point(1.0, 3.0, 2.0);
        let to = point(4.0, -2.0, 8.0);
        let up = vector(1.0, 1.0, 0.0);

        let t = view_transformation(from, to, up);

        assert_eq!(t, Matrix::new4x4(
            [-0.50709, 0.50709, 0.67612, -2.36643],
            [0.76772, 0.60609, 0.12122, -2.82843],
            [-0.35857, 0.59761, -0.71714, 0.00000],
            [0.00000, 0.00000, 0.00000, 1.00000]))
    }
}
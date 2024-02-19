use ray_tracer_challenge::*;

#[cfg(test)]
mod operations {
    use super::*;

    #[test]
    /// Adding two tuples
    fn adding_two_tuples() {
        let a1 = Tuple { x: 3f32, y: -2f32, z: 5f32, w: 1f32 };
        let a2 = Tuple { x: -2f32, y: 3f32, z: 1f32, w: 0f32 };
        assert_eq!(a1 + a2, Tuple { x: 1f32, y: 1f32, z: 6f32, w: 1f32 });
    }

    #[test]
    /// Subtracting two points
    fn subtracting_two_points() {
        let p1 = point(3f32, 2f32, 1f32);
        let p2 = point(5f32, 6f32, 7f32);
        assert_eq!(p1 - p2, vector(-2f32, -4f32, -6f32));
    }

    #[test]
    /// Subtracting a vector from a point
    fn subtracting_a_vector_from_a_point() {
        let p = point(3f32, 2f32, 1f32);
        let v = vector(5f32, 6f32, 7f32);
        assert_eq!(p - v, point(-2f32, -4f32, -6f32));
    }

    #[test]
    /// Subtracting two vectors
    fn subtracting_a_vector_from_a_vector() {
        let v1 = vector(3f32, 2f32, 1f32);
        let v2 = vector(5f32, 6f32, 7f32);
        assert_eq!(v1 - v2, vector(-2f32, -4f32, -6f32));
    }

    #[test]
    /// Subtracting a vector from a zero vector
    fn subtracting_a_vector_from_a_zero_vector() {
        let zero = vector(0f32, 0f32, 0f32);
        let v = vector(1f32, -2f32, 3f32);
        assert_eq!(zero - v, vector(-1f32, 2f32, -3f32));
    }

    #[test]
    /// Negating a tuple
    fn negating_a_tuple() {
        let a = Tuple { x: 1f32, y: -2f32, z: 3f32, w: -4f32 };
        assert_eq!(-a, Tuple { x: -1f32, y: 2f32, z: -3f32, w: 4f32 });
    }

    #[test]
    /// Multiplying a tuple by a scalar
    fn multiply_tuple_by_a_scalar() {
        let a = Tuple { x: 1f32, y: -2f32, z: 3f32, w: -4f32 };
        assert_eq!(a * 3.5f32, Tuple { x: 3.5f32, y: -7f32, z: 10.5f32, w: -14f32 });
    }

    #[test]
    /// Multiply a tuple by a fraction
    fn multiply_tuple_by_a_fraction() {
        let a = Tuple { x: 1f32, y: -2f32, z: 3f32, w: -4f32 };
        assert_eq!(a * 0.5f32, Tuple { x: 0.5f32, y: -1f32, z: 1.5f32, w: -2f32 })
    }

    #[test]
    /// Divide a tuple by a scalar
    fn divide_tuple_by_a_scalar() {
        let a = Tuple { x: 1f32, y: -2f32, z: 3f32, w: -4f32 };
        assert_eq!(a / 2f32, Tuple { x: 0.5f32, y: -1f32, z: 1.5f32, w: -2f32 })
    }
}
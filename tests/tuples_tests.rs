use ray_tracer_challenge::*;

#[cfg(test)]
mod tuples {
    use super::*;

    #[test]
    /// A tuple with w=1.0 is a point
    fn a_tuple_with_w_1_is_a_point() {
        let a = Tuple { x: 4.3, y: -4.2, z: 3.1, w: 1.0 };
        assert_eq!(a.x, 4.3);
        assert_eq!(a.y, -4.2);
        assert_eq!(a.z, 3.1);
        assert_eq!(a.w, 1.0);
        // 'a' is a point?
        // 'a' is not a vector?
    }

    #[test]
    /// A tuple with w=0.0 is a vector
    fn a_tuple_with_w_0_is_a_vector() {
        let a = Tuple { x: 4.3, y: -4.2, z: 3.1, w: 0.0 };
        assert_eq!(a.x, 4.3);
        assert_eq!(a.y, -4.2);
        assert_eq!(a.z, 3.1);
        assert_eq!(a.w, 0.0);
        // 'a' is not a point?
        // 'a' is a vector?
    }

    #[test]
    /// point() creates a tuple with w=1.0
    fn point_creates_a_tuple_with_w_1() {
        let a = point(4f32, -4f32, 3f32);
        assert_eq!(a, Point { x: 4f32, y: -4f32, z: 3f32, w: 1f32 });
    }

    #[test]
    /// vector() creates a tuple with w=0.0
    fn vector_creates_a_tuple_with_w_0() {
        let a = vector(4f32, -4f32, 3f32);
        assert_eq!(a, Vector { x: 4f32, y: -4f32, z: 3f32, w: 0f32 });
    }
}
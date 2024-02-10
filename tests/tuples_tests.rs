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

    #[test]
    /// Reflecting a vector approaching at 45°
    fn reflecting_a_vector_approaching_at_45deg() {
        let v = vector(1.0, -1.0, 0.0);
        let n = vector(0.0, 1.0, 0.0);

        let r = reflect(v, n);

        assert_eq!(r, vector(1.0, 1.0, 0.0));
    }

    #[test]
    /// Reflecting a vector off a slanted surface
    fn reflecting_a_vector_off_a_slanted_surface() {
        let v = vector(0.0, -1.0, 0.0);

        let n = vector(2.0_f32.sqrt() /2.0, 2.0_f32.sqrt() / 2.0, 0.0);

        let r = reflect(v, n);

        assert_eq!(r, vector(1.0, 0.0, 0.0));
    }
}
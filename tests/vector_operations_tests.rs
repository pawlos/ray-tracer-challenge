use std::ops::Sub;
use ray_tracer_challenge::*;

#[cfg(test)]
mod vector_operations {
    use super::*;

    #[test]
    /// Computing the magnitude of a vector(1,0,0)
    fn magnitude_of_vector_1_0_0() {
        let v = vector(1f32, 0f32, 0f32);
        assert_eq!(magnitude(v), 1f32);
    }

    #[test]
    /// Computing the magnitude of vector(0, 1, 0)
    fn magnitude_of_vector_0_1_0() {
        let v = vector(0f32, 1f32, 0f32);
        assert_eq!(magnitude(v), 1f32);
    }

    #[test]
    /// Computing the magnitude of vector(0, 0, 1)
    fn magnitude_of_vector_0_0_1() {
        let v = vector(0f32, 0f32, 1f32);
        assert_eq!(magnitude(v), 1f32);
    }

    #[test]
    /// Computing the magnitude of vector(1, 2, 3)
    fn magnitude_of_vector_1_2_3() {
        let v = vector(1f32, 2f32, 3f32);
        assert_eq!(magnitude(v), 14f32.sqrt());
    }

    #[test]
    /// Computing the magnitude of vector(-1, -2, -3)
    fn magnitude_of_vector_minus1_minus2_minus3() {
        let v = vector(-1f32, -2f32, -3f32);
        assert_eq!(magnitude(v), 14f32.sqrt());
    }

    #[test]
    /// Normalizing vector(4, 0, 0) gives (1, 0, 0)
    fn normalizing_vector_4_0_0_gives_1_0_0() {
        let v = vector(4f32, 0f32, 0f32);
        assert_eq!(normalize(v), vector(1f32, 0f32, 0f32));
    }

    #[test]
    /// Normalizing vector(1, 2, 3)
    fn normalizing_vector_1_2_3() {
        let v = vector(1f32, 2f32, 3f32);
        assert_eq!(normalize(v), vector(0.26726f32, 0.53452f32, 0.80178f32));
    }

    #[test]
    /// Magnitude of normalized vector is 1
    fn magnitude_of_normalized_vector() {
        let v = vector(1f32, 2f32, 3f32);
        assert!(magnitude(normalize(v)).sub(1f32).abs() < EPS);
    }

    #[test]
    /// the dot product of two tuples
    fn dot_product_of_two_tuples() {
        let v1 = vector(1f32, 2f32, 3f32);
        let v2 = vector(2f32, 3f32, 4f32);
        assert_eq!(dot(v1, v2), 20f32)
    }

    #[test]
    /// The cross product of two vectors
    fn cross_product_of_two_vectors() {
        let a = vector(1f32,2f32,3f32);
        let b = vector(2f32,3f32,4f32);
        assert_eq!(cross(a,b), vector(-1f32,2f32,-1f32));
        assert_eq!(cross(b,a), vector(1f32,-2f32,1f32))
    }
}
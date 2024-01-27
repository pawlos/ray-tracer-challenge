use std::ops::{Add, Sub, Neg, Mul, Div};

const EPS: f32 = 1e-5;

#[derive(Debug, Copy, Clone)]
struct Tuple {
    x: f32,
    y: f32,
    z: f32,
    w: f32,
}

impl Add for Tuple {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Tuple { x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z, w: self.w + rhs.w }
    }
}

impl Sub for Tuple {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Tuple { x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z, w: self.w - rhs.w }
    }
}

impl Neg for Tuple {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Tuple { x: -self.x, y: -self.y, z: -self.z, w: -self.w }
    }
}

impl Mul<f32> for Tuple {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Tuple { x: self.x * rhs, y: self.y * rhs, z: self.z * rhs, w: self.w * rhs }
    }
}

impl Div<f32> for Tuple {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Tuple { x: self.x / rhs, y: self.y / rhs, z: self.z / rhs, w: self.w / rhs }
    }
}

impl PartialEq for Tuple {
    fn eq(&self, rhs: &Self) -> bool {
        self.x.sub(rhs.x).abs() < EPS &&
            self.y.sub(rhs.y).abs() < EPS &&
            self.z.sub(rhs.z).abs() < EPS
    }
}

fn point(x: f32, y: f32, z: f32) -> Tuple {
    Tuple { x, y, z, w: 1.0 }
}

fn vector(x: f32, y: f32, z: f32) -> Tuple {
    Tuple { x, y, z, w: 0.0 }
}

fn magnitude(v: Tuple) -> f32 {
    assert_eq!(v.w, 0.0f32);

    (v.x * v.x + v.y * v.y + v.z * v.z).sqrt()
}

fn normalize(v: Tuple) -> Tuple {
    assert_eq!(v.w, 0.0f32);
    let magnitude = magnitude(v);
    Tuple {
        x: v.x / magnitude,
        y: v.y / magnitude,
        z: v.z / magnitude,
        w: v.w / magnitude,
    }
}

fn dot(v1: Tuple, v2: Tuple) -> f32 {
    assert_eq!(v1.w, 0.0f32);
    assert_eq!(v2.w, 0.0f32);

    v1.x * v2.x + v1.y * v2.y + v1.z * v2.z + v1.w * v2.w
}

fn cross(a: Tuple, b: Tuple) -> Tuple {
    assert_eq!(a.w, 0.0f32);
    assert_eq!(b.w, 0.0f32);

    vector(a.y * b.z - a.z * b.y,
           a.z * b.x - a.x * b.z,
           a.x * b.y - a.y * b.x)
}

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
        assert_eq!(a, Tuple { x: 4f32, y: -4f32, z: 3f32, w: 1f32 });
    }

    #[test]
    /// vector() creates a tuple with w=0.0
    fn vector_creates_a_tuple_with_w_0() {
        let a = vector(4f32, -4f32, 3f32);
        assert_eq!(a, Tuple { x: 4f32, y: -4f32, z: 3f32, w: 0f32 });
    }
}

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


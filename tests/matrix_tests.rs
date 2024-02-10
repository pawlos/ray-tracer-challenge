use ray_tracer_challenge::*;

#[cfg(test)]
mod matrix {
    use super::*;

    #[test]
    /// Constructing and inspecting a 4x4 matrix
    fn constructing_and_inspecting_4x4_matrix() {
        let m = Matrix::new4x4(
            [1.0, 2.0, 3.0, 4.0],
            [5.5, 6.5, 7.5, 8.5],
            [9.0, 10.0, 11.0, 12.0],
            [13.5, 14.5, 15.5, 16.5]);

        assert_eq!(m.at(0, 0), 1.0);
        assert_eq!(m.at(0, 3), 4.0);
        assert_eq!(m.at(1, 0), 5.5);
        assert_eq!(m.at(1, 2), 7.5);
        assert_eq!(m.at(2, 2), 11.0);
        assert_eq!(m.at(3, 0), 13.5);
        assert_eq!(m.at(3, 2), 15.5);
    }

    #[test]
    /// A 2x2 matrix ought to be representable
    fn matrix_2x2_ought_to_be_representable() {
        let m = Matrix::new2x2([-3.0, 5.0],[1.0, -2.0]);

        assert_eq!(m.at(0, 0), -3.0);
        assert_eq!(m.at(0, 1), 5.0);
        assert_eq!(m.at(1, 0), 1.0);
        assert_eq!(m.at(1, 1), -2.0);
    }

    #[test]
    /// A 3x3 matrix ought to be representable
    fn matrix_3x3_ought_to_be_representable() {
        let m = Matrix::new3x3(
            [-3.0, 5.0, 0.0],
            [1.0, -2.0, -7.0],
            [0.0, 1.0, 1.0]);

        assert_eq!(m.at(0, 0), -3.0);
        assert_eq!(m.at(1, 1), -2.0);
        assert_eq!(m.at(2, 2), 1.0);
    }

    #[test]
    /// Matrix equality with identical matrices
    fn matrix_equality_with_identical_matrices() {
        let a = Matrix::new4x4(
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        );
        let b = Matrix::new4x4(
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        );
        assert_eq!(a, b);
    }

    #[test]
    /// Matrix equality with different matrices
    fn matrix_equality_with_different_matrices() {
        let a = Matrix::new4x4(
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0]
        );
        let b = Matrix::new4x4(
            [2.0, 3.0, 4.0, 5.0],
            [6.0, 7.0, 8.0, 9.0],
            [8.0, 7.0, 6.0, 5.0],
            [4.0, 3.0, 2.0, 1.0]
        );
        assert_ne!(a,b);
    }

    #[test]
    /// Multiplying two matrices
    fn multiplying_two_matrices() {
        let a = Matrix::new4x4(
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0]);
        let b = Matrix::new4x4(
            [-2.0, 1.0, 2.0, 3.0],
            [3.0, 2.0, 1.0, -1.0],
            [4.0, 3.0, 6.0, 5.0],
            [1.0, 2.0, 7.0, 8.0]
        );

        assert_eq!(a * b, Matrix::new4x4(
            [20.0, 22.0, 50.0, 48.0],
            [44.0, 54.0, 114.0, 108.0],
            [40.0, 58.0, 110.0, 102.0],
            [16.0, 26.0, 46.0, 42.0]
        ))
    }

    #[test]
    /// A matrix multiplied by a tuple
    fn matrix_multiplied_by_a_tuple() {
        let a = Matrix::new4x4(
            [1.0, 2.0, 3.0, 4.0],
            [2.0, 4.0, 4.0, 2.0],
            [8.0, 6.0, 4.0, 1.0],
            [0.0, 0.0, 0.0, 1.0]
        );
        let b = Tuple { x: 1.0, y: 2.0, z: 3.0, w: 1.0 };
        assert_eq!(a * b, Tuple { x: 18.0, y: 24.0, z: 33.0, w: 1.0});
    }

    #[test]
    /// Multiplying a matrix by the identity matrix
    fn matrix_multiplied_by_identity_matrix() {
        let a = Matrix::new4x4(
            [0.0, 1.0, 2.0, 4.0],
            [1.0, 2.0, 4.0, 8.0],
            [2.0, 4.0, 8.0, 16.0],
            [4.0, 8.0, 16.0, 32.0]
        );
        assert_eq!(a.clone() * Matrix::identity4x4(), a);
    }

    #[test]
    /// Multiplying the identity matrix by a tuple
    fn multiplying_identity_matrix_by_a_tuple() {
        let a = Tuple {x: 1.0, y: 2.0, z: 3.0, w: 4.0};
        assert_eq!(Matrix::identity4x4() * a, a);
    }

    #[test]
    /// Transposing a matrix
    fn transposing_a_matrix() {
        let a = Matrix::new4x4(
            [0.0, 9.0, 3.0, 0.0],
            [9.0, 8.0, 0.0, 8.0],
            [1.0, 8.0, 5.0, 3.0],
            [0.0, 0.0, 5.0, 8.0]
        );

        assert_eq!(transpose(a), Matrix::new4x4(
            [0.0, 9.0, 1.0, 0.0],
            [9.0, 8.0, 8.0, 0.0],
            [3.0, 0.0, 5.0, 5.0],
            [0.0, 8.0, 3.0, 8.0]
        ))
    }

    #[test]
    /// Transposing the identity matrix
    fn transposing_the_identity_matrix() {
        let identity = Matrix::identity4x4();
        assert_eq!(transpose(identity.clone()), identity);
    }

    #[test]
    /// Calculating the determinant of a 2x2 matrix
    fn calculating_the_determinant_of_2x2_matrix() {
        let a = Matrix::new2x2(
            [1.0, 5.0],
            [-3.0, 2.0]);

        assert_eq!(determinant(a), 17.0);
    }

    #[test]
    /// A submatrix of a 3x3 matrix is a 2x2 matrix
    fn submatrix_of_3x3_matrix_is_2x2_matrix() {
        let a = Matrix::new3x3(
            [1.0, 5.0, 0.0],
            [-3.0, 2.0, 7.0],
            [0.0, 6.0, -3.0]);

        assert_eq!(submatrix(a, 0, 2), Matrix::new2x2([-3.0, 2.0],[0.0, 6.0]))
    }

    #[test]
    /// A submatrix of a 4x4 matrix is a 3x3 matrix
    fn submatrix_of_4x4_matrix_is_3x3_matrix() {
        let a = Matrix::new4x4(
            [-6.0, 1.0, 1.0, 6.0],
            [-8.0, 5.0, 8.0, 6.0],
            [-1.0, 0.0, 8.0, 2.0],
            [-7.0, 1.0, -1.0, 1.0]);

        assert_eq!(submatrix(a, 2, 1), Matrix::new3x3(
            [-6.0, 1.0, 6.0],
            [-8.0, 8.0, 6.0],
            [-7.0, -1.0, 1.0]
        ));
    }

    #[test]
    /// Calculating a minor of a 3x3 matrix
    fn calculating_a_minor_of_3x3_matrix() {
        let a = Matrix::new3x3(
            [3.0, 5.0, 0.0],
            [2.0, -1.0, -7.0],
            [6.0, -1.0, 5.0]);
        let b = submatrix(a.clone(), 1, 0);
        assert_eq!(determinant(b), 25.0);
        assert_eq!(minor(a.clone(), 1, 0), 25.0);
    }

    #[test]
    /// Calculating cofactor of a 3x3 matrix
    fn calculating_cofactor_of_a_3x3_matrix() {
        let a = Matrix::new3x3(
            [3.0, 5.0, 0.0],
            [2.0, -1.0, -7.0],
            [6.0, -1.0, 5.0]);

        assert_eq!(minor(a.clone(), 0, 0), -12.0);
        assert_eq!(cofactor(a.clone(), 0, 0), -12.0);
        assert_eq!(minor(a.clone(), 1, 0), 25.0);
        assert_eq!(cofactor(a.clone(), 1, 0), -25.0)
    }

    #[test]
    /// Calculating the determinant of 3x3 matrix
    fn calculating_the_determinant_of_3x3_matrix() {
        let a = Matrix::new3x3(
            [1.0, 2.0, 6.0],
            [-5.0, 8.0, -4.0],
            [2.0, 6.0, 4.0]);

        assert_eq!(cofactor(a.clone(), 0,0), 56.0);
        assert_eq!(cofactor(a.clone(), 0,1), 12.0);
        assert_eq!(cofactor(a.clone(),0,2), -46.0);
        assert_eq!(determinant(a.clone()), -196.0);
    }

    #[test]
    /// Calculating the determinant of 4x4 matrix
    fn calculating_the_determinant_of_4x4_matrix() {
        let a = Matrix::new4x4(
            [-2.0, -8.0, 3.0, 5.0],
            [-3.0, 1.0, 7.0, 3.0],
            [1.0, 2.0, -9.0, 6.0],
            [-6.0, 7.0, 7.0, -9.0]);

        assert_eq!(cofactor(a.clone(), 0,0), 690.0);
        assert_eq!(cofactor(a.clone(), 0,1), 447.0);
        assert_eq!(cofactor(a.clone(), 0,2), 210.0);
        assert_eq!(cofactor(a.clone(), 0,3), 51.0);
        assert_eq!(determinant(a.clone()), -4071.0);
    }

    #[test]
    /// Testing an invertible matrix for invertibility
    fn testing_an_invertible_matrix_for_invertibility() {
        let a = Matrix::new4x4(
            [6.0, 4.0, 4.0, 4.0],
            [5.0, 5.0, 7.0, 6.0],
            [4.0, -9.0, 3.0, -7.0],
            [9.0, 1.0, 7.0, -6.0]);

        assert_eq!(determinant(a.clone()), -2120.0);
        assert!(a.is_invertible());
    }

    #[test]
    /// Testing a noninvertible matrix for invertibility
    fn testing_a_noninvertible_matrix_for_invertibility() {
        let a = Matrix::new4x4(
            [-4.0, 2.0, -2.0, -3.0],
            [0.0, 6.0, 2.0, 6.0],
            [0.0, -5.0, 1.0, -5.0],
            [0.0, 0.0, 0.0, 0.0]);

        assert_eq!(determinant(a.clone()), 0.0);
        assert!(!a.is_invertible());
    }

    #[test]
    /// Calculating rhe inverse of a matrix
    fn calculating_a_inverse_of_a_matrix() {
        let a = Matrix::new4x4(
            [-5.0, 2.0, 6.0, -8.0],
            [1.0, -5.0, 1.0, 8.0],
            [7.0, 7.0, -6.0, -7.0],
            [1.0, -3.0, 7.0, 4.0]);
        let b = inverse(a.clone());
        assert_eq!(determinant(a.clone()), 532.0);
        assert_eq!(cofactor(a.clone(), 2, 3), -160.0);
        assert_eq!(b.at(3,2), -160.0/532.0);
        assert_eq!(cofactor(a.clone(), 3, 2), 105.0);
        assert_eq!(b.at(2,3), 105.0/532.0);
        assert_eq!(b, Matrix::new4x4(
            [0.21805, 0.45113, 0.24060, -0.04511],
            [-0.80827, -1.45677, -0.44361, 0.52068],
            [-0.07895, -0.22368, -0.05263, 0.19737],
            [-0.52256, -0.81391, -0.30075, 0.30639]
        ));
    }

    #[test]
    /// Calculating the inverse of another matrix
    fn calculating_the_inverse_of_another_matrix() {
        let a = Matrix::new4x4(
            [8.0, -5.0, 9.0, 2.0],
            [7.0, 5.0, 6.0, 1.0],
            [-6.0, 0.0, 9.0, 6.0],
            [-3.0, 0.0, -9.0, -4.0]);

        assert_eq!(inverse(a), Matrix::new4x4(
            [-0.15385, -0.15385, -0.28205, -0.53846],
            [-0.07692, 0.12308, 0.02564, 0.03077],
            [0.35897, 0.35897, 0.43590, 0.92308],
            [-0.69231, -0.69231, -0.76923, -1.92308]));
    }

    #[test]
    /// Calculating the inverse of a third matrix
    fn calculating_the_inverse_of_a_third_matrix() {
        let a = Matrix::new4x4(
            [9.0, 3.0, 0.0, 9.0],
            [-5.0, -2.0, -6.0, -3.0],
            [-4.0, 9.0, 6.0, 4.0],
            [-7.0, 6.0, 6.0, 2.0]);

        assert_eq!(inverse(a), Matrix::new4x4(
            [-0.04074, -0.07778, 0.14444, -0.22222],
            [-0.07778, 0.03333, 0.36667, -0.33333],
            [-0.02901, -0.14630, -0.10926, 0.12963],
            [0.17778, 0.06667, -0.26667, 0.33333]))
    }

    #[test]
    /// Multiplying a product by its inverse
    fn multiplying_a_product_by_irs_inverse() {
        let a = Matrix::new4x4(
            [3.0, -9.0, 7.0, 3.0],
            [3.0, -8.0, 2.0, -9.0],
            [-4.0, 4.0, 4.0, 1.0],
            [-6.0, 5.0, -1.0, 1.0]);

        let b = Matrix::new4x4(
            [8.0, 2.0, 2.0, 2.0],
            [3.0, -1.0, 7.0, 0.0],
            [7.0, 0.0, 5.0, 4.0],
            [6.0, -2.0, 0.0, 5.0]);

        let c = a.clone() * b.clone();

        assert_eq!(c * inverse(b), a);
    }
}
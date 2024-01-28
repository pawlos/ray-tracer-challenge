use std::ops::{Add, Sub, Neg, Mul, Div};
use std::vec;

const EPS: f32 = 1e-5;

#[derive(Debug, Clone, Copy)]
pub struct Tuple {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    w: f32,
}

#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub red: f32,
    pub green: f32,
    pub blue: f32
}

pub struct Canvas {
    pub width: i32,
    pub height: i32,
    pixels: Vec<Color>,
}

#[derive(Debug, Clone)]
pub struct Matrix {
    size: usize,
    elems: Vec<f32>,
}


impl Canvas {
    pub fn new(width: i32, height: i32) -> Self {
        Canvas {width , height, pixels: vec![color(0.0,0.0,0.0); (width * height) as usize]}
    }

    pub fn write_pixel(&mut self, x: i32, y: i32, c: Color) {
        let idx = (y*self.width + x) as usize;
        self.pixels[idx] = c;
    }

    pub fn pixel_at(&self, x: i32, y: i32) -> Color {
        let idx = (y*self.width + x) as usize;
        self.pixels[idx]
    }
}

impl Matrix {
    pub fn new2x2(row1: [f32; 2], row2: [f32; 2]) -> Self {

        let mut elems = vec![0.0; row1.len() * row2.len()];

        for (i,e) in row1.iter().enumerate() {
            elems[i] = *e;
        }

        for (i,e) in row2.iter().enumerate() {
            elems[2 + i] = *e;
        }

        Matrix { elems, size: 2 }
    }

    pub fn new3x3(
        row1: [f32; 3],
        row2: [f32; 3],
        row3: [f32; 3]) -> Self {

        let mut elems = vec![0.0; row1.len() * row2.len()];

        for (i,e) in row1.iter().enumerate() {
            elems[i] = *e;
        }

        for (i,e) in row2.iter().enumerate() {
            elems[3 + i] = *e;
        }

        for (i,e) in row3.iter().enumerate() {
            elems[6 + i] = *e;
        }

        Matrix { elems, size: 3 }
    }

    pub fn new4x4(
        row1: [f32; 4],
        row2: [f32; 4],
        row3: [f32; 4],
        row4: [f32; 4]) -> Self {

        let mut elems = vec![0.0; row1.len() * row2.len()];

        for (i,e) in row1.iter().enumerate() {
            elems[i] = *e;
        }

        for (i,e) in row2.iter().enumerate() {
            elems[4 + i] = *e;
        }

        for (i,e) in row3.iter().enumerate() {
            elems[8 + i] = *e;
        }

        for (i,e) in row4.iter().enumerate() {
            elems[12 + i] = *e;
        }

        Matrix { elems, size: 4 }
    }

    fn is_invertible(&self) -> bool {
        determinant(self.clone()) != 0.0
    }

    pub fn at(&self, row: usize, col: usize) -> f32 {
        self.elems[row * self.size + col]
    }

    pub fn identity4x4() -> Self {
        Matrix::new4x4([1.0, 0.0, 0.0, 0.0], [0.0, 1.0, 0.0, 0.0], [0.0, 0.0, 1.0, 0.0], [0.0, 0.0, 0.0, 1.0])
    }
}
pub type Point = Tuple;
pub type Vector = Tuple;

impl Add for Tuple {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Tuple { x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z, w: self.w + rhs.w }
    }
}

impl Add for Color {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Color { red: self.red + rhs.red, green: self.green + rhs.green, blue: self.blue + rhs.blue }
    }
}

impl Sub for Tuple {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Tuple { x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z, w: self.w - rhs.w }
    }
}

impl Sub for Color {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Color { red: self.red - rhs.red, green: self.green - rhs.green, blue: self.blue - rhs.blue}
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

impl Mul<f32> for Color {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Color {red: self.red * rhs, green: self.green * rhs, blue: self.blue * rhs}
    }
}

impl Mul<Color> for Color {
    type Output = Color;

    fn mul(self, rhs: Color) -> Self::Output {
        Color { red: self.red * rhs.red, green: self.green * rhs.green, blue: self.blue * rhs.blue}
    }
}

impl Mul<Matrix> for Matrix {
    type Output = Matrix;

    fn mul(self, rhs: Matrix) -> Self::Output {
        assert_eq!(self.size, rhs.size);
        assert_eq!(self.size, 4);

        let mut values = [[0.0; 4]; 4];
        for (i, row) in values.iter_mut().enumerate() {
            for (j, val) in row.iter_mut().enumerate() {
                *val = self.at(i,0) * rhs.at(0, j) +
                    self.at(i,1) * rhs.at(1, j) +
                    self.at(i,2) * rhs.at(2, j) +
                    self.at(i,3) * rhs.at(3, j);
            }
        }

        Matrix::new4x4(values[0],values[1], values[2], values[3])
    }
}

impl Mul<Tuple> for Matrix {
    type Output = Tuple;
    fn mul(self, rhs: Tuple) -> Self::Output {
        assert_eq!(self.size, 4);
        let mut values = [0.0; 4];
        for (i,v) in values.iter_mut().enumerate() {
            *v = self.at(i, 0) * rhs.x +
                 self.at(i, 1) * rhs.y +
                 self.at(i, 2) * rhs.z +
                 self.at(i, 3) * rhs.w;
        }

        Tuple { x: values[0], y: values[1], z: values[2], w: values[3] }
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

impl PartialEq for Color {
    fn eq(&self, rhs: &Self) -> bool {
        self.red.sub(rhs.red).abs() < EPS &&
            self.green.sub(rhs.green).abs() < EPS &&
            self.blue.sub(rhs.blue).abs() < EPS
    }
}

impl PartialEq for Matrix {
    fn eq(&self, other: &Self) -> bool {
        if self.size != other.size {
            return false;
        }
        else {
            for i in 0..self.size {
                for j in 0 .. self.size  {
                    if self.at(i,j) != other.at(i,j) {
                        return false;
                    }
                }
            }
        }
        true
    }
}

pub fn point(x: f32, y: f32, z: f32) -> Point {
    Point { x, y, z, w: 1.0 }
}

pub fn vector(x: f32, y: f32, z: f32) -> Vector {
    Vector { x, y, z, w: 0.0 }
}

pub fn color(red: f32, green: f32, blue: f32) -> Color {
    Color { red, green, blue }
}

pub fn magnitude(v: Vector) -> f32 {
    assert_eq!(v.w, 0.0f32);

    (v.x * v.x + v.y * v.y + v.z * v.z).sqrt()
}

pub fn normalize(v: Vector) -> Tuple {
    assert_eq!(v.w, 0.0f32);
    let magnitude = magnitude(v);
    Vector {
        x: v.x / magnitude,
        y: v.y / magnitude,
        z: v.z / magnitude,
        w: v.w / magnitude,
    }
}

fn dot(v1: Vector, v2: Vector) -> f32 {
    assert_eq!(v1.w, 0.0f32);
    assert_eq!(v2.w, 0.0f32);

    v1.x * v2.x + v1.y * v2.y + v1.z * v2.z + v1.w * v2.w
}

fn cross(a: Vector, b: Vector) -> Vector {
    assert_eq!(a.w, 0.0f32);
    assert_eq!(b.w, 0.0f32);

    vector(a.y * b.z - a.z * b.y,
           a.z * b.x - a.x * b.z,
           a.x * b.y - a.y * b.x)
}

fn transpose(a: Matrix) -> Matrix {
    let mut values = [[0.0;4];4];
    for i in 0..a.size {
        for (j, row) in values.iter_mut().enumerate() {
            (*row)[i] = a.at(i, j);
        }
    }

    Matrix::new4x4(
        values[0],
        values[1],
        values[2],
        values[3]
    )
}

fn determinant2x2(a: Matrix) -> f32 {
    assert_eq!(a.size, 2);

    a.at(0,0)*a.at(1,1) - a.at(0, 1) * a.at(1, 0)
}

fn determinant(a: Matrix) -> f32 {
    if a.size == 2 {
        return determinant2x2(a);
    }
    //assert_eq!(a.size, 3);
    let mut det = 0.0;
    for i in 0..a.size {
        det += a.at(0,i) * cofactor(a.clone(), 0, i);
    }
    det
}

fn submatrix2x2(a: Matrix, skip_row: usize, skip_col: usize) -> Matrix {
    assert_eq!(a.size, 3);
    let mut values = [[0.0; 2]; 2];

    let mut i_idx = 0;
    let mut j_idx;
    for i in 0 .. 3 {
        j_idx = 0;
        if i == skip_row {
            continue;
        }
        for j in 0 .. a.size {
            if j == skip_col
            {
                continue;
            }
            values[i_idx][j_idx] = a.at(i, j);
            j_idx += 1;
        }
        i_idx += 1;
    }

    Matrix::new2x2(values[0], values[1])
}

fn submatrix3x3(a: Matrix, skip_row: usize, skip_col: usize) -> Matrix {
    assert_eq!(a.size, 4);
    let mut values = [[0.0; 3]; 3];

    let mut i_idx = 0;
    let mut j_idx;
    for i in 0 .. 4 {
        j_idx = 0;
        if i == skip_row {
            continue;
        }
        for j in 0 .. a.size {
            if j == skip_col
            {
                continue;
            }
            values[i_idx][j_idx] = a.at(i, j);
            j_idx += 1;
        }
        i_idx += 1;
    }

    Matrix::new3x3(values[0], values[1], values[2])
}

fn submatrix(a: Matrix, start_row: usize, start_col: usize) -> Matrix {
    match a.size - 1 {
        3 => submatrix3x3(a, start_row, start_col),
        2 => submatrix2x2(a, start_row, start_col),
        _ => panic!("Unsupported size")
    }
}

fn minor(a: Matrix, row: usize, col: usize) -> f32 {
    determinant(submatrix(a, row, col))
}

fn cofactor3x3(a: Matrix, row: usize, col: usize) -> f32 {
    assert_eq!(a.size, 3);
    let cofactors = Matrix::new3x3([1.0, -1.0, 1.0],
    [-1.0, 1.0, -1.0],
    [1.0, -1.0, 1.0]);

    minor(a, row, col) * cofactors.at(row, col)
}

fn cofactor(a: Matrix, row: usize, col: usize) -> f32 {
    if a.size == 3 {
        return cofactor3x3(a, row, col);
    }

    assert_eq!(a.size, 4);
    let cofactors = Matrix::new4x4([1.0, -1.0, 1.0, -1.0],
                                   [-1.0, 1.0, -1.0, 1.0],
                                   [1.0, -1.0, 1.0, -1.0],
                                   [-1.0, 1.0, -1.0, 1.0]);

    minor(a, row, col) * cofactors.at(row, col)
}

fn append_string_or_new_line(c: f32, line_len: usize) -> (String, usize, bool) {
    let c = c.mul(255.0).clamp(0.0, 255.0);
    let c_str = format!("{} ", c.round());
    let mut content = String::new();
    let mut new_line_len = line_len + c_str.len();
    let mut new_line = false;

    if new_line_len > 70 {
        content.push('\n');
        content.push_str(c_str.as_str());
        new_line_len = c_str.len();
        new_line = true
    } else {
        content.push_str(c_str.as_str());
        new_line_len = c_str.len();
    }

    (content, new_line_len, new_line)
}

pub fn canvas_to_ppm(c: Canvas) -> String {
    let mut content = String::from("P3\n");
    content.push_str(format!("{} {}\n", c.width, c.height).as_str());
    content.push_str("255\n");

    let mut line_len = 0;
    for y in 0..c.height {
        for x in 0..c.width {
            let color = c.pixel_at(x, y);
            let (content_red, line_len_red, new_line) = append_string_or_new_line(color.red, line_len);
            if new_line {
                content = content.trim().to_string();
                line_len = 0;
            }
            content.push_str(content_red.as_str());
            line_len += line_len_red;

            let (content_green, line_len_green, new_line) = append_string_or_new_line(color.green, line_len);
            if new_line {
                content = content.trim().to_string();
                line_len = 0;
            }
            content.push_str(content_green.as_str());
            line_len += line_len_green;

            let (content_blue, line_len_blue, new_line) = append_string_or_new_line(color.blue, line_len);
            if new_line {
                content = content.trim().to_string();
                line_len = 0;
            }
            content.push_str(content_blue.as_str());
            line_len += line_len_blue;

            if x == c.width - 1 && y != c.height - 1 {
                content = content.trim_end().to_string();
                content.push('\n');
                line_len = 0;
            }
        }
        if y != c.height - 1  {
            content = content.trim_end().to_string();
            content.push('\n');
            line_len = 0;
        }
    }
    content = content.trim_end().to_string();
    content.push('\n');
    content
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
        assert_eq!(a, Point { x: 4f32, y: -4f32, z: 3f32, w: 1f32 });
    }

    #[test]
    /// vector() creates a tuple with w=0.0
    fn vector_creates_a_tuple_with_w_0() {
        let a = vector(4f32, -4f32, 3f32);
        assert_eq!(a, Vector { x: 4f32, y: -4f32, z: 3f32, w: 0f32 });
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

#[cfg(test)]
mod colors {
    use super::*;

    #[test]
    /// Colors are (red, green, blue) tuples
    fn colors_are_tuples() {
        let c = color(-0.5, 0.4, 1.7);
        assert_eq!(c.red, -0.5);
        assert_eq!(c.green, 0.4);
        assert_eq!(c.blue, 1.7);
    }

    #[test]
    /// Adding colors
    fn adding_colors() {
        let c1 = color(0.9, 0.6, 0.75);
        let c2 = color(0.7, 0.1, 0.25);
        assert_eq!(c1 + c2, color(1.6, 0.7, 1.0));
    }

    #[test]
    /// Subtracting colors
    fn subtracting_colors() {
        let c1 = color(0.9, 0.6, 0.75);
        let c2 = color(0.7, 0.1, 0.25);
        assert_eq!(c1 - c2, color( 0.2, 0.5, 0.5));
    }

    #[test]
    /// Multiplying a color by a scalar
    fn multiply_color_by_scalar() {
        let c = color(0.2, 0.3, 0.4);
        assert_eq!(c * 2.0, color(0.4, 0.6, 0.8));
    }

    #[test]
    /// Multiplying a color by a color
    fn multiply_color_by_color() {
        let c1 = color(1.0, 0.2, 0.4);
        let c2 = color(0.9, 1.0, 0.1);
        assert_eq!(c1 * c2, color(0.9, 0.2, 0.04));
    }
}

#[cfg(test)]
mod canvas {
    use super::*;

    #[test]
    /// Creating a canvas
    fn creating_a_canvas() {
        let c = Canvas::new(10, 20);
        assert_eq!(c.width, 10);
        assert_eq!(c.height, 20);
        assert!(c.pixels.iter().all(|c| *c == color(0.0,0.0,0.0)));
    }

    #[test]
    /// Writing pixels to a canvas
    fn writing_pixels_to_a_canvas() {
        let mut c = Canvas::new(10,20);
        let red = color(1.0,0.0,0.0);
        c.write_pixel(2,3, red);
        assert_eq!(c.pixel_at(2,3), red);
    }

    #[test]
    /// Constructing the PPM header
    fn constructing_ppm_header() {
        let c = Canvas::new(5, 3);
        let ppm = canvas_to_ppm(c);
        assert_eq!(ppm, r#"P3
5 3
255
0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
"#)
    }

    #[test]
    /// Constructing the PPM data
    fn constructing_ppm_data() {
        let mut c = Canvas::new(5,3);
        let c1 = color (1.5, 0.0, 0.0);
        let c2 = color (0.0, 0.5, 0.0);
        let c3 = color (-0.5, 0.0, 1.0);
        c.write_pixel(0, 0, c1);
        c.write_pixel(2, 1, c2);
        c.write_pixel(4, 2, c3);

        let ppm = canvas_to_ppm(c);
        assert_eq!(ppm, r#"P3
5 3
255
255 0 0 0 0 0 0 0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 128 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0 0 0 0 0 0 0 255
"#)
    }

    #[test]
    /// Splitting long lines in PPM files
    fn splitting_long_lines_in_ppm_files() {
        let mut c = Canvas::new(10, 2);

        for y in 0..2 {
            for x in 0..10 {
                c.write_pixel(x, y, color(1.0, 0.8, 0.6))
            }
        }

        let ppm = canvas_to_ppm(c);
        assert_eq!(ppm, r#"P3
10 2
255
255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204
153 255 204 153 255 204 153 255 204 153 255 204 153
255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204
153 255 204 153 255 204 153 255 204 153 255 204 153
"#)
    }

    #[test]
    /// PPM files are terminated by a newline character
    fn ppm_files_are_terminated_by_newline_character() {
        let c = Canvas::new(5,3);
        let ppm = canvas_to_ppm(c);
        assert_eq!(ppm, r#"P3
5 3
255
0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
"#)
    }
}

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
}
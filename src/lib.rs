use std::cmp::Ordering;
use std::fmt::{Debug, Formatter};
use std::ops::{Add, Div, Mul, Neg, Sub};
use std::vec;
use uuid::Uuid;

pub const EPS: f32 = 0.0001;

pub const DEFAULT_REFLECTION_NUMBER :u8 = 4;

pub trait Shape {
    fn id(&self) -> Uuid;
    fn transform(&self) -> Matrix;
    fn material(&self) -> &Material;
    fn mut_material(&mut self) -> &mut Material;
    fn set_transform(&mut self, transform: Matrix);
    fn set_material(&mut self, material: Material);

    fn local_intersect(&self, ray: Ray) -> Vec<Intersection>;

    fn local_normal_at(&self, point: Point) -> Vector;

    fn intersect(&self, ray: Ray) -> Vec<Intersection> {
        let local_ray = transform(ray, inverse(&self.transform()));
        self.local_intersect(local_ray)
    }

    fn normal_at(&self, point: Point) -> Vector {
        let local_point = inverse(&self.transform()) * point;
        let local_normal = self.local_normal_at(local_point);
        let mut world_normal = transpose(inverse(&self.transform())) * local_normal;
        world_normal.w = 0.0;

        normalize(world_normal)
    }
}

pub trait Pattern {
    fn set_transform(&mut self, transform: Matrix);

    fn transform(&self) -> Matrix;

    fn pattern_at(&self, point: Point) -> Color;

    fn pattern_at_shape(&self, object: &dyn Shape, point: Point) -> Color
    {
        let object_point = inverse(&object.transform()) * point;
        let pattern_point = inverse(&self.transform()) * object_point;
        self.pattern_at(pattern_point)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Tuple {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
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
    pub pixels: Vec<Color>,
}

#[derive(Debug, Clone)]
pub struct Matrix {
    size: usize,
    elems: Vec<f32>,
}

#[derive(Debug, Copy, Clone)]
pub struct Ray {
    pub origin: Point,
    pub direction: Vector
}

#[derive(Debug)]
pub struct Sphere {
    id: Uuid,
    pub transform: Matrix,
    pub material: Material,
}

#[derive(Debug)]
pub struct Plane {
    id: Uuid,
    pub transform: Matrix,
    pub material: Material,
}

#[derive(Debug)]
pub struct TestShape {
    id: Uuid,
    pub transform: Matrix,
    pub saved_ray: Ray,
    pub material: Material,
}

#[derive(Debug)]
pub struct Cube {
    id: Uuid,
    pub transform: Matrix,
    pub material: Material,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StripePattern {
    pub a: Color,
    pub b: Color,
    pub transform: Matrix,
}

pub struct TestPattern {
    transform: Matrix,
}

pub struct GradientPattern {
    a: Color,
    b: Color,
    transform: Matrix,
}


pub struct CheckersPattern {
    a: Color,
    b: Color,
    transform: Matrix,
}

pub struct RingPattern {
    pub a: Color,
    pub b: Color,
    transform: Matrix,
}

pub struct RadialGradient {
    a: Color,
    b: Color,
    transform: Matrix,
}

impl Pattern for StripePattern {
    fn set_transform(&mut self, transform: Matrix) {
        self.transform = transform;
    }

    fn transform(&self) -> Matrix {
        self.transform.clone()
    }

    fn pattern_at(&self, point: Point) -> Color {
        if point.x.rem_euclid(2.0).floor() == 0.0 {
            self.a
        } else {
            self.b
        }
    }
}

impl Pattern for TestPattern {
    fn set_transform(&mut self, transform: Matrix) {
        self.transform = transform;
    }

    fn transform(&self) -> Matrix {
        self.transform.clone()
    }

    fn pattern_at(&self, point: Point) -> Color {
        color(point.x, point.y, point.z)
    }
}

impl Pattern for GradientPattern {
    fn set_transform(&mut self, transform: Matrix) {
        self.transform = transform;
    }

    fn transform(&self) -> Matrix {
        self.transform.clone()
    }

    fn pattern_at(&self, point: Point) -> Color {
        let v_r = self.a.red + (self.b.red - self.a.red)*(point.x - point.x.floor());
        let v_g = self.a.green + (self.b.green - self.a.green)*(point.x - point.x.floor());
        let v_b = self.a.blue + (self.b.blue - self.a.blue)*(point.x - point.x.floor());
        
        color(v_r, v_g, v_b)
    }
}

impl Pattern for CheckersPattern {
    fn set_transform(&mut self, transform: Matrix) {
        self.transform = transform;
    }

    fn transform(&self) -> Matrix {
        self.transform.clone()
    }

    fn pattern_at(&self, point: Point) -> Color {
        let v = point.x.abs() + point.y.abs() + point.z.abs();
        if v.rem_euclid(2.0).floor() == 0.0 {
            self.a
        } else {
            self.b
        }
    }
}

impl Pattern for RingPattern {
    fn set_transform(&mut self, transform: Matrix) {
        self.transform = transform;
    }

    fn transform(&self) -> Matrix {
        self.transform.clone()
    }

    fn pattern_at(&self, point: Point) -> Color {
        let v = (point.x.powi(2) + point.z.powi(2)).sqrt().rem_euclid(2.0).floor();
        if v == 0.0 {
            self.a
        } else {
            self.b
        }
    }
}

impl Pattern for RadialGradient {
    fn set_transform(&mut self, transform: Matrix) {
        self.transform = transform;
    }

    fn transform(&self) -> Matrix {
        self.transform.clone()
    }

    fn pattern_at(&self, point: Point) -> Color {
        let v = (point.x.powi(2) + point.z.powi(2)).sqrt().rem_euclid(2.0);

        let v_r = self.a.red + (self.b.red - self.a.red)*(v - v.floor());
        let v_g = self.a.green + (self.b.green - self.a.green)*(v - v.floor());
        let v_b = self.a.blue + (self.b.blue - self.a.blue)*(v - v.floor());


        color(v_r, v_g, v_b)
    }
}

impl Shape for Sphere {
    fn id(&self) -> Uuid {
        self.id
    }

    fn transform(&self) -> Matrix {
        self.transform.clone()
    }

    fn material(&self) -> &Material {
        &self.material
    }

    fn mut_material(&mut self) -> &mut Material {
        &mut self.material
    }

    fn set_transform(&mut self, transform: Matrix) {
        self.transform = transform;
    }

    fn set_material(&mut self, material: Material) {
        self.material = material;
    }

    fn local_intersect(&self, ray: Ray) -> Vec<Intersection> {
        let sphere_to_ray = ray.origin - point(0.0, 0.0, 0.0);

        let a = dot(ray.direction, ray.direction);
        let b = 2.0 * dot(ray.direction, sphere_to_ray);
        let c = dot(sphere_to_ray, sphere_to_ray) - 1.0;

        let discriminant = b*b - 4.0*a*c;

        if discriminant < 0.0 {
            return vec![];
        }

        let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
        let t2 = (-b + discriminant.sqrt()) / (2.0 * a);

        [intersection(t1, self), intersection(t2, self)].to_vec()
    }

    fn local_normal_at(&self, point: Point) -> Vector {
        vector(point.x, point.y, point.z)
    }
}

impl Shape for Plane {
    fn id(&self) -> Uuid {
        self.id
    }

    fn transform(&self) -> Matrix {
        self.transform.clone()
    }

    fn material(&self) -> &Material {
        &self.material
    }

    fn mut_material(&mut self) -> &mut Material {
        &mut self.material
    }

    fn set_transform(&mut self, transform: Matrix) {
        self.transform = transform;
    }

    fn set_material(&mut self, material: Material) {
        self.material = material;
    }

    fn local_intersect(&self, ray: Ray) -> Vec<Intersection> {
        match ray.direction.y.abs() < EPS {
            | true => [].to_vec(),
            | _ => [intersection(-ray.origin.y / ray.direction.y, self)].to_vec()
        }
    }

    fn local_normal_at(&self, _point: Point) -> Vector {
        vector(0.0, 1.0, 0.0)
    }
}

impl Shape for TestShape {
    fn id(&self) -> Uuid {
        self.id
    }

    fn transform(&self) -> Matrix {
        self.transform.clone()
    }

    fn material(&self) -> &Material {
        &self.material
    }

    fn mut_material(&mut self) -> &mut Material {
        &mut self.material
    }

    fn set_transform(&mut self, transform: Matrix) {
        self.transform = transform;
    }

    fn set_material(&mut self, material: Material) {
        self.material = material;
    }

    fn local_intersect(&self, _ray: Ray) -> Vec<Intersection> {
        [].to_vec()
    }

    fn local_normal_at(&self, point: Point) -> Vector {
        vector(point.x, point.y, point.z)
    }
}

impl Shape for Cube {
    fn id(&self) -> Uuid { self.id }

    fn transform(&self) -> Matrix { self.transform.clone() }

    fn material(&self) -> &Material { &self.material }

    fn mut_material(&mut self) -> &mut Material { &mut self.material }

    fn set_transform(&mut self, transform: Matrix) { self.transform = transform; }

    fn set_material(&mut self, material: Material) { self.material = material; }

    fn local_intersect(&self, ray: Ray) -> Vec<Intersection> {
        let (xt_min, xt_max) = check_axis(ray.origin.x, ray.direction.x);
        let (yt_min, yt_max) = check_axis(ray.origin.y, ray.direction.y);
        let (zt_min, zt_max) = check_axis(ray.origin.z, ray.direction.z);

        let t_min = xt_min.max(yt_min.max(zt_min));
        let t_max = xt_max.min(yt_max.min(zt_max));

        if t_min > t_max {
            return [].to_vec();
        }

        [intersection(t_min, self), intersection(t_max, self)].to_vec()
    }

    fn local_normal_at(&self, point: Point) -> Vector {
        let max_c = point.x.abs().max(point.y.abs().max(point.z.abs()));

        if max_c == point.x.abs() {
            return vector(point.x, 0.0, 0.0);
        } else if max_c == point.y.abs() {
            return vector(0.0, point.y, 0.0);
        }
        vector(0.0, 0.0, point.z)
    }
}

#[derive(Clone, Copy)]
pub struct Intersection<'a> {
    pub t: f32,
    pub object: &'a dyn Shape,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PointLight {
    pub position: Point,
    pub intensity: Color,
}

pub struct Material {
    pub color: Color,
    pub ambient: f32,
    pub diffuse: f32,
    pub specular: f32,
    pub shininess: f32,
    pub reflective: f32,
    pub transparency: f32,
    pub refractive_index: f32,
    pub pattern: Option<Box<dyn Pattern>>,
}

pub struct World {
    pub objects: Vec<Box<dyn Shape>>,
    pub lights: Vec<PointLight>,
}

pub struct Computation<'a> {
    pub t: f32,
    pub object: &'a dyn Shape,
    pub point: Point,
    pub eye_v: Vector,
    pub normal_v: Vector,
    pub inside: bool,
    pub over_point: Point,
    pub under_point: Point,
    pub reflect_v: Vector,
    pub n1: f32,
    pub n2: f32,
}

pub struct Camera {
    pub hsize: i32,
    pub vsize: i32,
    pub half_width: f32,
    pub half_height: f32,
    pub field_of_view: f32,
    pub transform: Matrix,
    pub pixel_size: f32,
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

    pub fn is_invertible(&self) -> bool {
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
                    if (self.at(i,j) - other.at(i,j)).abs() > EPS {
                        return false;
                    }
                }
            }
        }
        true
    }
}

impl<'a> PartialEq for Intersection<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.t == other.t && self.object.id() == other.object.id()
    }
}

impl PartialEq for dyn Pattern {
    fn eq(&self, other: &Self) -> bool {
        self.transform() == other.transform()
    }
}

impl PartialEq for Material {
    fn eq(&self, other: &Self) -> bool {
        self.color == other.color &&
            self.specular == other.specular &&
            self.diffuse == other.diffuse &&
            self.ambient == other.ambient &&
            self.pattern == other.pattern
    }
}

impl<'a> Debug for Intersection<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("Intersection at {:?} for {:?}", self.t, self.object.id()).as_str())
    }
}

impl Debug for Material {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("Material - color: {:?}, specular: {:?}", self.color, self.specular).as_str())
    }
}

impl PartialEq for Sphere {
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id()
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

pub fn normalize(v: Vector) -> Vector {
    assert_eq!(v.w, 0.0f32);
    let magnitude = magnitude(v);
    Vector {
        x: v.x / magnitude,
        y: v.y / magnitude,
        z: v.z / magnitude,
        w: v.w / magnitude,
    }
}

pub fn dot(v1: Vector, v2: Vector) -> f32 {
    assert_eq!(v1.w, 0.0f32);
    assert_eq!(v2.w, 0.0f32);

    v1.x * v2.x + v1.y * v2.y + v1.z * v2.z + v1.w * v2.w
}

pub fn cross(a: Vector, b: Vector) -> Vector {
    assert_eq!(a.w, 0.0f32);
    assert_eq!(b.w, 0.0f32);

    vector(a.y * b.z - a.z * b.y,
           a.z * b.x - a.x * b.z,
           a.x * b.y - a.y * b.x)
}

pub fn transpose(a: Matrix) -> Matrix {
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

pub fn determinant(a: Matrix) -> f32 {
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

pub fn submatrix(a: Matrix, start_row: usize, start_col: usize) -> Matrix {
    match a.size - 1 {
        3 => submatrix3x3(a, start_row, start_col),
        2 => submatrix2x2(a, start_row, start_col),
        _ => panic!("Unsupported size")
    }
}

pub fn minor(a: Matrix, row: usize, col: usize) -> f32 {
    determinant(submatrix(a, row, col))
}

fn cofactor3x3(a: Matrix, row: usize, col: usize) -> f32 {
    assert_eq!(a.size, 3);
    let cofactors = Matrix::new3x3([1.0, -1.0, 1.0],
    [-1.0, 1.0, -1.0],
    [1.0, -1.0, 1.0]);

    minor(a, row, col) * cofactors.at(row, col)
}

pub fn cofactor(a: Matrix, row: usize, col: usize) -> f32 {
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

pub fn inverse(a: &Matrix) -> Matrix {
    assert!(a.clone().is_invertible());

    let mut values = [[0.0; 4]; 4];
    let size = a.clone().size;
    let determinant = determinant(a.clone());
    for row in 0..size {
        for col in 0..size {
            let c = cofactor(a.clone(), row, col);

            values[col][row] = c / determinant;
        }
    }

    Matrix::new4x4(values[0], values[1], values[2], values[3])
}

pub fn translation(tx: f32, ty: f32, tz: f32) -> Matrix {
    Matrix::new4x4([1.0, 0.0, 0.0, tx],
    [0.0, 1.0, 0.0, ty],
    [0.0, 0.0, 1.0, tz],
    [0.0, 0.0, 0.0, 1.0])
}

pub fn scaling(sx: f32, sy: f32, sz: f32) -> Matrix {
    Matrix::new4x4([sx, 0.0, 0.0, 0.0],
    [0.0, sy, 0.0, 0.0],
    [0.0, 0.0, sz, 0.0],
    [0.0, 0.0, 0.0, 1.0])
}

pub fn rotation_x(r: f32) -> Matrix {
    Matrix::new4x4(
        [1.0, 0.0, 0.0, 0.0],
    [0.0, r.cos(), -r.sin(), 0.0],
    [0.0, r.sin(), r.cos(), 0.0],
    [0.0, 0.0, 0.0, 1.0])
}

pub fn rotation_y(r: f32) -> Matrix {
    Matrix::new4x4(
        [r.cos(), 0.0, r.sin(), 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [-r.sin(), 0.0, r.cos(), 0.0],
        [0.0, 0.0, 0.0, 1.0])
}

pub fn rotation_z(r: f32) -> Matrix {
    Matrix::new4x4(
        [r.cos(), -r.sin(), 0.0, 0.0],
        [r.sin(), r.cos(), 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0])
}

pub fn shearing(sxy: f32, sxz: f32, syx: f32, syz: f32, szx: f32, szy: f32) -> Matrix {
    Matrix::new4x4(
        [1.0, sxy, sxz, 0.0],
        [syx, 1.0, syz, 0.0],
        [szx, szy, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0])
}

pub fn ray(origin: Point, direction: Vector) -> Ray {
    Ray { origin, direction }
}

pub fn position(ray: Ray, t: f32) -> Point {
    ray.origin + ray.direction * t
}

pub fn sphere() -> Box<dyn Shape> {
    Box::new(Sphere {
        id: Uuid::new_v4(),
        transform: Matrix::identity4x4(),
        material: material()})
}

pub fn glass_sphere() -> Box<dyn Shape> {
    let mut material = material();
    material.transparency = 1.0;
    material.refractive_index = 1.5;
    Box::new(Sphere {
        id: Uuid::new_v4(),
        transform: Matrix::identity4x4(),
        material,
    })
}

pub fn plane() -> Box<dyn Shape> {
    Box::new(Plane {
        id: Uuid::new_v4(),
        transform: Matrix::identity4x4(),
        material: material()})
}

pub fn test_shape() -> Box<dyn Shape> {
    Box::new(TestShape {
        id: Uuid::new_v4(),
        transform: Matrix::identity4x4(),
        material: material(),
        saved_ray: ray(point(0.0, 0.0, 0.0), vector(0.0,0.0,0.0)) })
}

pub fn cube() -> Box<dyn Shape> {
    Box::new( Cube {
        id: Uuid::new_v4(),
        transform: Matrix::identity4x4(),
        material: material()})
}

pub fn intersection(t:f32, object: &dyn Shape) -> Intersection {
    Intersection { t, object }
}

pub fn intersect_world(w: &World, r: Ray) -> Vec<Intersection> {
    let mut intersections = w.objects.iter().flat_map(|o| o.intersect(r))
        .collect::<Vec<_>>();
    intersections.sort_by(|i, j|
        if i.t <= j.t {
            Ordering::Less
        } else {
            Ordering::Greater
        });

    intersections
}

fn calculate_point(hit: Intersection, i: &Intersection, containers: &[Uuid], uuids : &[(Uuid, f32)]) -> Option<f32>
{
    if *i == hit {
        return if containers.is_empty() {
            Some(1.0f32)
        } else {
            let last_uuid = containers.last().unwrap();
            let elem = uuids.iter().find(|e| e.0 == *last_uuid).unwrap();
            Some(elem.1)
        }
    }

    None
}

pub fn prepare_computations<'a>(hit: Intersection<'a>, r: Ray, xs: &'a Vec<Intersection<'a>>) -> Computation<'a> {
    let point = position(r, hit.t);
    let mut normal_v = hit.object.normal_at(point);
    let inside = dot(normal_v, -r.direction) < 0.0;
    if inside {
        normal_v = -normal_v;
    }
    let reflect_v = reflect(r.direction, normal_v);
    let mut containers : Vec<Uuid> = [].to_vec();
    let uuids_with_refractive_index : Vec<_> = xs.iter().map(|el| (
        el.object.id(), el.object.material().refractive_index)).collect();
    let mut n1 = None;
    let mut n2 = None;

    for j in xs {
        n1 = calculate_point(hit, j, &containers, &uuids_with_refractive_index);

        if containers.iter().any(|el| *el == j.object.id()) {
            containers.remove(containers.iter().position(|el| *el == j.object.id()).unwrap());
        } else {
            containers.push(j.object.id());
        }

        n2 = calculate_point(hit, j, &containers, &uuids_with_refractive_index);

        if n1.is_some() && n2.is_some() {
            break;
        }
    }
    Computation {
        t: hit.t,
        object: hit.object,
        point,
        eye_v: -r.direction,
        inside,
        normal_v,
        over_point: point + normal_v * EPS,
        under_point: point - normal_v * EPS,
        reflect_v,
        n1: n1.unwrap_or(0.0),
        n2: n2.unwrap_or(0.0),
    }
}

pub fn hit<'a>(xs: &[Intersection<'a>]) -> Option<Intersection<'a>> {
    let mut new_vec = xs.to_vec();
    new_vec.sort_by(|i, j| i.t.total_cmp(&j.t));

    let filtered = new_vec.iter().filter(|i| i.t >= 0.0).take(1).collect::<Vec<_>>();
    match filtered.len() {
        | 0 => None,
        | _ => Some(*filtered[0])
    }
}

pub fn transform(r: Ray, m: Matrix) -> Ray {
    Ray {origin: m.clone()*r.origin, direction: m*r.direction }
}

pub fn reflect(i: Vector, normal: Vector) -> Vector {
    i - normal * 2.0 * dot(i, normal)
}

pub fn point_light(position: Point, intensity: Color) -> PointLight {
    PointLight { position, intensity }
}

pub fn material() -> Material {
    Material {
        color: color(1.0, 1.0, 1.0),
        ambient: 0.1,
        diffuse: 0.9,
        specular: 0.9,
        shininess: 200.0,
        pattern: None,
        reflective: 0.0,
        transparency: 0.0,
        refractive_index: 1.0,
    }
}

pub fn lightning(m: &Material, object: &dyn Shape, l: &PointLight, point: Point, eye_v: Vector, normal_v: Vector, in_shadow: bool) -> Color {

    let color_for_lightning = match m.pattern.as_deref() {
        | None => m.color,
        | Some(p) => p.pattern_at_shape(object, point)
    };
    let effective_color = color_for_lightning * l.intensity;

    let light_v = normalize(l.position - point);

    let ambient = effective_color * m.ambient;

    let light_dot_normal = dot(light_v, normal_v);

    let (diffuse, specular) = if light_dot_normal < 0.0 {
        (color(0.0, 0.0, 0.0), color(0.0, 0.0, 0.0))
    } else {
        let diffuse = effective_color * m.diffuse * light_dot_normal;
        let reflect_v = reflect(-light_v, normal_v);
        let reflect_dot_eye = dot(reflect_v, eye_v);

        let specular = if reflect_dot_eye <= 0.0 {
            color(0.0, 0.0, 0.0)
        } else {
            let factor = reflect_dot_eye.powf(m.shininess);
            l.intensity*m.specular*factor
        };
        (diffuse, specular)
    };

    if in_shadow {
        ambient
    } else {
        ambient + diffuse + specular
    }
}

pub fn world() -> World {
    World { objects: vec![], lights: vec![] }
}

pub fn default_world() -> World {
    let light = point_light(point(-10.0, 10.0, -10.0), color(1.0, 1.0, 1.0));

    let mut s1 = sphere();
    let mut m = material();
    m.color = color(0.8, 1.0, 0.6);
    m.diffuse = 0.7;
    m.specular = 0.2;
    s1.set_material(m);

    let mut s2 = sphere();
    s2.set_transform(scaling(0.5, 0.5, 0.5));
    World { objects: vec![s1, s2], lights: vec![light] }
}

pub fn stripe_pattern(a: Color, b: Color) -> StripePattern {
   StripePattern { a, b, transform: Matrix::identity4x4() }
}

pub fn test_pattern() -> TestPattern {
    TestPattern { transform: Matrix::identity4x4() }
}

pub fn gradient_pattern(a: Color, b: Color) -> GradientPattern {
    GradientPattern {
        a,
        b,
        transform: Matrix::identity4x4() 
    }
}

pub fn checkers_pattern(a: Color, b: Color) -> CheckersPattern {
    CheckersPattern { a, b, transform: Matrix::identity4x4() }
}

pub fn ring_pattern(a: Color, b: Color) -> RingPattern {
    RingPattern {
        a,
        b,
        transform: Matrix::identity4x4()
    }
}

pub fn ring_gradient(a: Color, b: Color) -> RadialGradient {
    RadialGradient {
        a,
        b,
        transform: Matrix::identity4x4()
    }
}

pub fn set_pattern_transformation(pattern: &mut StripePattern, transform: Matrix) {
    pattern.transform = transform;
}

pub fn shade_hit(w: &World, c: &Computation, remaining: u8) -> Color {
    let is_shadowed = is_shadowed(w, c.over_point);
    let surface = lightning(c.object.material(), c.object, &w.lights[0], c.over_point, c.eye_v, c.normal_v, is_shadowed);

    let reflected = reflected_color(w, c, remaining);
    let refracted = refracted_color(w, c, remaining);

    let material =c.object.material();
    if material.reflective > 0.0 && material.transparency > 0.0 {
        let reflectance = schlick(c);
        surface + reflected * reflectance + refracted * (1.0f32 - reflectance)
    }
    else {
        surface + reflected + refracted
    }
}

pub fn reflected_color(w: &World, c: &Computation, remaining: u8) -> Color {
    if remaining == 0 {
        return color(0.0, 0.0, 0.0);
    }
    if c.object.material().reflective == 0.0 {
        return color(0.0, 0.0, 0.0);
    }
    let reflect_ray = ray(c.over_point, c.reflect_v);
    let color = color_at(w, reflect_ray, remaining - 1);

    color * c.object.material().reflective
}

pub fn refracted_color(w: &World, c: &Computation, remaining: u8) -> Color {
    if remaining == 0 {
        return color(0.0, 0.0, 0.0);
    }
    if c.object.material().transparency == 0.0 {
        return color(0.0, 0.0, 0.0);
    }
    let n_ratio = c.n1 / c.n2;

    let cos_i = dot(c.eye_v, c.normal_v);

    let sin2_t = n_ratio.powf(2.0) * (1.0 - cos_i.powf(2.0));

    if sin2_t > 1.0 {
        return color(0.0, 0.0, 0.0);
    }

    let cos_t = (1.0 - sin2_t).sqrt();

    let direction = c.normal_v * (n_ratio * cos_i - cos_t) - c.eye_v * n_ratio;

    let refracted_ray = ray(c.under_point, direction);

    color_at(w, refracted_ray, remaining - 1) * c.object.material().transparency
}

pub fn color_at<'a>(w: &'a World, r: Ray, remaining: u8) -> Color {
    let intersections: Vec<Intersection<'a>> = intersect_world(w, r);

    let hit= hit(&intersections);
    match hit
    {
        | None => color(0.0, 0.0, 0.0),
        | Some(i) => {
            let comp = prepare_computations(i, r, &intersections);
            shade_hit(w, &comp, remaining)
        }
    }
}

pub fn schlick(c: &Computation) -> f32 {

    let mut cos = dot(c.eye_v, c.normal_v);

    if c.n1 > c.n2 {
        let n = c.n1 / c.n2;
        let sin2_t = n.powf(2.0) * (1.0 - cos.powf(2.0));
        if sin2_t > 1.0 {
            return 1.0;
        }

        let cos_t = (1.0 - sin2_t.powf(2.0)).sqrt();

        cos = cos_t;
    }
    let r0 = ((c.n1 - c.n2) / (c.n1 + c.n2)).powf(2.0);
    r0 + (1.0f32 - r0) * (1.0 - cos).powf(5.0)
}

fn check_axis(origin: f32, direction: f32) -> (f32, f32) {
    let tmin_numerator = -1.0f32 - origin;
    let tmax_numerator = 1.0f32 - origin;

    let (mut tmin, mut tmax) =
        if direction.abs() >= EPS {
            (tmin_numerator / direction, tmax_numerator / direction)
        } else {
            (tmin_numerator * f32::INFINITY, tmax_numerator * f32::INFINITY)
        };

    if tmin > tmax {
        std::mem::swap(&mut tmin, &mut tmax);
    }
    (tmin, tmax)
}

pub fn view_transformation(from: Point, to: Point, up: Vector) -> Matrix {
    let forward = normalize(to - from);
    let left = cross(forward, normalize(up));
    let true_up = cross(left, forward);
    Matrix::new4x4(
        [left.x, left.y, left.z, 0.0],
        [true_up.x, true_up.y, true_up.z, 0.0],
        [-forward.x, -forward.y, -forward.z, 0.0],
        [0.0, 0.0, 0.0, 1.0]) * translation(-from.x, -from.y, -from.z)
}

pub fn camera(hsize: i32, vsize: i32, field_of_view: f32) -> Camera {
    let half_view = (field_of_view / 2.0).tan();
    let aspect = (hsize as f32) / (vsize as f32);
    let (half_width, half_height) = if aspect >= 1.0 {
        (half_view, half_view / aspect)
    } else {
        (half_view * aspect, half_view)
    };
    Camera {
        hsize,
        vsize,
        field_of_view,
        half_width,
        half_height,
        transform: Matrix::identity4x4(),
        pixel_size: half_width * 2.0 / (hsize as f32) }
}

pub fn ray_for_pixel(c: &Camera, px: i32, py: i32) -> Ray {
    let x_offset = (px as f32 + 0.5) * c.pixel_size;
    let y_offset = (py as f32 + 0.5) * c.pixel_size;

    let world_x = c.half_width - x_offset;
    let world_y = c.half_height - y_offset;

    let pixel = inverse(&c.transform) * point(world_x, world_y, -1.0);
    let origin = inverse(&c.transform) * point(0.0, 0.0, 0.0);
    let direction = normalize(pixel - origin);
    Ray { origin, direction }
}

pub fn render(camera: &Camera, world: &World) -> Canvas {
    let mut c = Canvas::new(camera.hsize, camera.vsize);
    for y in 0..camera.vsize {
        for x in 0..camera.hsize {
            let ray = ray_for_pixel(camera, x, y);
            let color = color_at(world, ray, DEFAULT_REFLECTION_NUMBER);
            c.write_pixel(x, y, color)
        }
    }
    c
}

pub fn is_shadowed(w: &World, p: Point) -> bool {
    let v = w.lights[0].position - p;
    let distance = magnitude(v);
    let direction = normalize(v);
    let r = ray(p, direction);

    let intersections = intersect_world(w, r);
    let h = hit(&intersections);

    match h {
        | Some(intersection) => intersection.t < distance,
        | None => false,
    }
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
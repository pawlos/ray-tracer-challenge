use std::cmp::Ordering;
use std::ops::{Add, Div, Mul, Neg, Sub};
use std::vec;

pub const EPS: f32 = 1e-4;

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

#[derive(Copy, Clone)]
pub struct Ray {
    pub origin: Point,
    pub direction: Vector
}

#[derive(Debug)]
pub struct Sphere {
    id: uuid::Uuid,
    pub transform: Matrix,
    pub material: Material,
}

#[derive(Debug, Clone, Copy)]
pub struct Intersection<'a> {
    pub t: f32,
    pub object: &'a Sphere, //for now only Sphere
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PointLight {
    pub position: Point,
    pub intensity: Color,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Material {
    pub color: Color,
    pub ambient: f32,
    pub diffuse: f32,
    pub specular: f32,
    pub shininess: f32
}

pub struct World {
    pub objects: Vec<Sphere>,
    pub lights: Vec<PointLight>,
}

pub struct Computation<'a> {
    pub t: f32,
    pub object: &'a Sphere,
    pub point: Point,
    pub eye_v: Vector,
    pub normal_v: Vector,
    pub inside: bool,
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
                    if self.at(i,j) - other.at(i,j) > EPS {
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
        self.t == other.t && self.object.id == other.object.id
    }
}

impl PartialEq for Sphere {
    fn eq(&self, other: &Self) -> bool {
        self.material == other.material &&
        self.transform == other.transform
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

pub fn sphere() -> Sphere {
    Sphere { id: uuid::Uuid::new_v4(), transform: Matrix::identity4x4(), material: material() }
}

pub fn intersection(t:f32, object: &Sphere) -> Intersection {
    Intersection { t, object }
}

pub fn intersect(s: &Sphere, r: Ray) -> Vec<Intersection> {
    let r2 = transform(r, inverse(&s.transform));
    let sphere_to_ray = r2.origin - point(0.0, 0.0, 0.0);

    let a = dot(r2.direction, r2.direction);
    let b = 2.0 * dot(r2.direction, sphere_to_ray);
    let c = dot(sphere_to_ray, sphere_to_ray) - 1.0;

    let discriminant = b*b - 4.0*a*c;

    if discriminant < 0.0 {
        return [].to_vec();
    }

    let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
    let t2 = (-b + discriminant.sqrt()) / (2.0 * a);

    [intersection(t1, s), intersection(t2, s)].to_vec()
}

pub fn intersect_world(w: &World, r: Ray) -> Vec<Intersection> {
    let mut intersections = w.objects.iter().flat_map(|o| intersect(o, r)).collect::<Vec<_>>();
    intersections.sort_by(|i, j|
        if i.t < j.t {
            Ordering::Less
        } else {
            Ordering::Greater
        });

    intersections
}

pub fn prepare_computations(i: Intersection, r: Ray) -> Computation {
    let point = position(r, i.t);
    let mut normal_v = normal_at(i.object, point);
    let inside = dot(normal_v, -r.direction) < 0.0;
    if inside {
        normal_v = -normal_v;
    }
    Computation {
        t: i.t,
        object: i.object,
        point,
        eye_v: -r.direction,
        inside,
        normal_v,
    }
}

pub fn hit<'a>(xs: &'a mut [Intersection]) -> Option<Intersection<'a>> {
    xs.sort_by(|i, j| i.t.total_cmp(&j.t));

    let filtered = xs.iter().filter(|i| i.t >= 0.0).take(1).collect::<Vec<_>>();
    match filtered.len() {
        | 0 => None,
        | _ => Some(*filtered[0])
    }
}

pub fn transform(r: Ray, m: Matrix) -> Ray {
    Ray {origin: m.clone()*r.origin, direction: m*r.direction }
}

pub fn set_transform(s: &mut Sphere, t: Matrix) {
    s.transform = t;
}

pub fn normal_at(s: &Sphere, p: Point) -> Vector {
    let object_point = inverse(&s.transform) * p;
    let object_normal = object_point - point(0.0, 0.0, 0.0);
    let mut  world_normal = transpose(inverse(&s.transform)) * object_normal;
    world_normal.w = 0.0;
    normalize(world_normal)
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
    }
}

pub fn lightning(m: &Material, l: &PointLight, point: Point, eye_v: Vector, normal_v: Vector) -> Color {
    let effective_color = m.color * l.intensity;

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
    ambient + diffuse + specular
}

pub fn world() -> World {
    World { objects: vec![], lights: vec![] }
}

pub fn default_world() -> World {
    let light = point_light(point(-10.0, -10.0, -10.0), color(1.0, 1.0, 1.0));

    let mut s1 = sphere();
    s1.material = material();
    s1.material.color = color(0.8, 1.0, 0.6);
    s1.material.diffuse = 0.7;
    s1.material.specular = 0.2;

    let mut s2 = sphere();
    s2.transform = scaling(0.5, 0.5, 0.5);
    World { objects: vec![s1, s2], lights: vec![light] }
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
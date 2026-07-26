//#![allow(unused)]
#![allow(unused_imports, dead_code, unused_mut, unused_parens)]
#![feature(generic_const_exprs)]
#![allow(incomplete_features)]
#![feature(unboxed_closures)]

use std::{
    any::TypeId,
    array,
    convert::identity,
    fmt::{Debug, Display},
    mem::MaybeUninit,
    ops::{Add, AddAssign, Deref, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
    ptr::NonNull,
};

// might be useful? idk lol
//pub trait MatrixTrait {}
//impl<const HEIGHT: usize, const WIDTH: usize, T> MatrixTrait for ConstShapeMat<HEIGHT, WIDTH, T> {}
//#[derive(Debug)]
//pub struct TypestateMatrix<S: MatrixTrait> {
//    state: S
//}
//impl<const HEIGHT: usize, const WIDTH: usize, T> TypestateMatrix<ConstShapeMat<HEIGHT, WIDTH, T>> {
//    pub fn new_const_mn(x: [[T; WIDTH]; HEIGHT]) -> Self {
//        Self { state: ConstShapeMat { data: x } }
//    }
//}

use ordered_float::NotNan;
use rand_distr::Distribution;

// rows is height
// cols is width
//Mat<N, M> = n * m matrix
//#[derive(Clone, Debug)]
//pub struct ConstShapeMat<const HEIGHT: usize, const WIDTH: usize, T> {
//    data: [[T; WIDTH]; HEIGHT],
//}
//
//impl<const ROWS: usize, const COLS: usize, T> ConstShapeMat<ROWS, COLS, T> {
//    pub fn new_uninit() -> ConstShapeMat<ROWS, COLS, MaybeUninit<T>> {
//        ConstShapeMat {
//            data: core::array::from_fn(|_| core::array::from_fn(|_| MaybeUninit::uninit())),
//        }
//    }
//}
//
//impl<const ROWS: usize, const COLS: usize, T> From<[[T; COLS]; ROWS]>
//    for ConstShapeMat<ROWS, COLS, T>
//{
//    fn from(value: [[T; COLS]; ROWS]) -> Self {
//        ConstShapeMat { data: value }
//    }
//}

//impl<const ROWS: usize, const COLS: usize, T> Display for ConstShapeMat<ROWS, COLS, T>
//where
//    T: Display,
//{
//    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//        write!(f, "[")?;
//        for (i, x) in self.data.iter().enumerate() {
//            if i != 0 {
//                write!(f, " ")?;
//            }
//            for (j, y) in x.iter().enumerate() {
//                if i == ROWS - 1 && j == COLS - 1 {
//                    write!(f, "{}", y)?;
//                } else {
//                    write!(f, "{}, ", y)?;
//                }
//            }
//            if i != ROWS - 1 {
//                write!(f, "\n")?;
//            }
//        }
//        write!(f, "]")?;
//        Ok(())
//    }
//}

// Y is ROWS (height) of mat A
// X is cols of mat A
// TODO make mul work on slices
type MulOut<T, U> = <T as Mul<U>>::Output;
//impl<const X: usize, const Y: usize, const Z: usize, T, U> Mul<ConstShapeMat<X, Z, U>>
//    for ConstShapeMat<Y, X, T>
///* mat 1 */
//where
//    T: Mul<U>,
//    MulOut<T, U>: Add<Output = MulOut<T, U>>,
//    T: Clone,
//    U: Clone,
//{
//    type Output = ConstShapeMat<Y, Z, <<T as Mul<U>>::Output as Add>::Output>;
//    fn mul(self, rhs: ConstShapeMat<X, Z, U>) -> Self::Output {
//        core::array::from_fn(|i| {
//            /* i being the row index of c */
//            core::array::from_fn(|j| {
//                /* j being the col index of c */
//                // c[i][j] = a[i][0] b[0][j] + a[i][1] b[1][j] + ... + a[i][n] b[n][j]
//                let mut sum = self.data[i][0].clone() * rhs.data[0][j].clone();
//                for n in 1..X
//                /* width of Mat A */
//                {
//                    sum = sum + self.data[i][n].clone() * rhs.data[n][j].clone();
//                }
//                sum // c[i][j] = sum
//            })
//        })
//        .into()
//    }
//}

//impl<const WIDTH: usize, const HEIGHT: usize, T, U> Add<ConstShapeMat<HEIGHT, WIDTH, U>>
//    for ConstShapeMat<HEIGHT, WIDTH, T>
//where
//    T: Add<U>,
//{
//    type Output = ConstShapeMat<HEIGHT, WIDTH, <T as Add<U>>::Output>;
//    fn add(mut self, rhs: ConstShapeMat<HEIGHT, WIDTH, U>) -> Self::Output {
//        self.data
//            .zip(rhs.data)
//            .map(|x| x.0.zip(x.1).map(|x| x.0 + x.1))
//            .into()
//    }
//}

trait Zip<const N: usize, T, U> {
    fn zip(self, b: [U; N]) -> [(T, U); N];
}
impl<const N: usize, T, U> Zip<N, T, U> for [T; N] {
    fn zip(self, b: [U; N]) -> [(T, U); N] {
        let mut b = b.into_iter();
        self.map(|v| (v, b.next().unwrap()))
    }
}

//impl<const N: usize, T, U> Add<VecN<N, U>> for VecN<N, T>
//where
//    T: Add<U>,
//{
//    type Output = VecN<N, <T as Add<U>>::Output>;
//    fn add(self, rhs: VecN<N, U>) -> Self::Output {
//        VecN {
//            data: self.data.zip(rhs.data).map(|(x, y)| x + y),
//        }
//    }
//}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let distribution = rand_distr::Normal::new(0.0, 0.5)?;
    let rng = &mut rand::rng();
    let mut network = Network::new([2, 3, 3, 1], || {
        ordered_float::NotNan::new(distribution.sample(rng)).unwrap()
    });

    #[rustfmt::skip]
    let input_data = [
        150.0, 70.0, 
        254.0, 73.0, 
        312.0, 68.0, 
        120.0, 60.0, 
        154.0, 61.0, 
        212.0, 65.0, 
        216.0, 67.0,
        145.0, 67.0, 
        184.0, 64.0, 
        130.0, 69.0,
    ];
    // 2 * 10 matrix
    let input_matrix = Matrix {
        data: Box::new(input_data),
        height: 10,
    }
    .map(|x| NotNan::new(x).unwrap())
    .transpose();
    let y = [0.0, 1.0, 1.0, 0.0, 0.0, 1.0, 1.0, 0.0, 1.0, 0.0];
    let y = Matrix {
        data: Box::new(y),
        height: 1,
    }
    .map(|x| NotNan::new(x).unwrap());
    let one = NotNan::new(1.0).unwrap();
    let sigmoid = |x: NotNan<f64>| NotNan::new(1.0 / (1.0 + f64::exp(-x.into_inner()))).unwrap();
    let d_sigmoid = |x| sigmoid(x) * (one - sigmoid(x));
    for i in 0..1000000 {
        let y = y.clone();
        let (a, z) = network.feed_forward(&input_matrix, sigmoid);
        if i & 100 == 0 {
            let y_hat = a.last().unwrap();
            let mut sum: NotNan<f64> = NotNan::new(f64::powi(
                y.get_owned(0, 0).into_inner() - y_hat.get_owned(0, 0).into_inner(),
                2,
            ))
            .unwrap();
            for i in 0..y.height() {
                sum = sum
                    + NotNan::new(f64::powi(
                        y.get_owned(i, 0).into_inner() - y_hat.get_owned(i, 0).into_inner(),
                        2,
                    ))
                    .unwrap();
            }
            sum = sum / NotNan::new(y.height() as f64).unwrap();
            println!("{:?}", sum);
        }
        let delta = network.back_prop(a.clone(), z, y, d_sigmoid);
        network.gradient_descent(NotNan::new(0.000003).unwrap(), delta, a);
    }
    let x = network.feed_forward(&input_matrix, sigmoid);
    println!("{:?}", x.0.last().unwrap());

    Ok(())
}

#[derive(Debug, Clone)]
struct Matrix<T> {
    /* layout is
    [1, 2, 3
     4, 5, 6] */
    data: Box<[T]>,
    height: usize,
}

impl Scalar for NotNan<f64> {
    const ONE: Self = const { unsafe { NotNan::new_unchecked(1.0) } };
    const ZERO: Self = const { unsafe { NotNan::new_unchecked(0.0) } };
}

impl<T> Matrix<T> {
    pub fn from_fn<F>(height: usize, width: usize, mut f: F) -> Self
    where
        F: FnMut(usize, usize) -> T,
    {
        let mut out = Vec::new();
        for i in 0..height {
            for j in 0..width {
                out.push(f(i, j));
            }
        }
        Matrix {
            data: out.into(),
            height,
        }
    }
}

// i can change the layout of this to all be one contiguous block of memory,
// the size is known when creating the model, and does not change
// size of biases = sum(heights)
// size of weights = sum i = 0 to layer_count (heights[i] * heights[i + 1])
#[derive(Debug)]
struct Network<const LAYER_COUNT: usize, T>
where
    [(); LAYER_COUNT - 1]:,
{
    layout_heights: [usize; LAYER_COUNT],
    biases: [Box<[T]>; LAYER_COUNT - 1],
    weights: [Matrix<T>; LAYER_COUNT - 1],
}

// layer count includes input layer? idk lol
impl<const LAYER_COUNT: usize, T> Network<LAYER_COUNT, T>
where
    [(); LAYER_COUNT - 1]:,
{
    pub fn new<F>(heights: [usize; LAYER_COUNT], mut randfn: F) -> Self
    where
        F: FnMut() -> T,
    {
        let biases = array::from_fn(|i| {
            let height = heights[i + 1];
            let mut out = Vec::new();
            for _ in 0..height {
                out.push(randfn());
            }
            out.into()
        });
        let weights = array::from_fn(|i| {
            let height = heights[i + 1];
            let width = heights[i];
            Matrix::from_fn(height, width, |_, _| randfn())
        });
        Network {
            layout_heights: heights,
            biases,
            weights,
        }
    }
    pub fn feed_forward<'a, F>(
        &self,
        input: impl Into<&'a Matrix<T>>,
        activation_fn: F,
    ) -> ([Matrix<T>; LAYER_COUNT], [Matrix<T>; LAYER_COUNT - 1])
    where
        F: Fn(T) -> T,
        T: Mul<Output = T> + Add<Output = T> + Clone + Debug,
        T: 'a,
    {
        let input = input.into();
        // formula
        // W[i] * a[i - 1] + b[i] = z[i]
        // a[i] = g(z[i])
        let mut layers_a = Vec::new();
        layers_a.push(input.clone());
        let mut layers_z = Vec::new();
        for i in 0..LAYER_COUNT - 1 {
            let biases_layer = self.biases[i].as_ref();
            let layer = &layers_a[i];
            let z = (&self.weights[i] * layer).add_extended_slice(biases_layer);
            layers_z.push(z.clone());
            let a = z.map(&activation_fn);
            layers_a.push(a);
        }
        (layers_a.try_into().unwrap(), layers_z.try_into().unwrap())
    }
    #[allow(non_snake_case)]
    pub fn back_prop<F>(
        &self,
        a: [Matrix<T>; LAYER_COUNT], /* can be changed to y_hat */
        z: [Matrix<T>; LAYER_COUNT - 1],
        y: Matrix<T>,
        dActivation_fn: F,
    ) -> Matrix<Matrix<T>>
    /* more like a matrix of vectors */
    where
        T: Sub<Output = T>,
        T: Add<Output = T>,
        T: Scalar + Debug,
        T: Mul<T, Output = T>,
        T: From<u32>,
        F: Fn(T) -> T,
    {
        let y_hat = a.last().unwrap();
        let mut delta_mat = Vec::new();
        // implemented like this because i dont know how to do it vectorised
        for sample in 0..y.width() {
            let sample_y_hat = y_hat.get_column(sample);
            let sample_y = y.get_column(sample);
            let sample_z_L = z.last().unwrap().get_column(sample);
            // using quadratic cost
            // nabla  a C = (a[L] - y)
            let nabla_a_C = sample_y_hat - sample_y;

            let layer_delta = nabla_a_C.hadamard(sample_z_L.map(&dActivation_fn));
            let mut range = (0..LAYER_COUNT - 1).rev();
            let successors = core::iter::successors(range.next().map(|i| (i, layer_delta)), |x| {
                let i = range.next()?;
                let z = z[i].clone();
                let w = &self.weights[i + 1].transpose_clone();
                let layer_delta = (w * &x.1).hadamard(z.map(&dActivation_fn));
                Some((i, layer_delta))
            });
            // reverse the data because its BACK propagation
            // the data is given from front to back
            let mut intermediate_vec = Vec::new();
            for (_i, delta) in successors {
                intermediate_vec.push(delta);
            }
            for x in intermediate_vec.into_iter().rev() {
                delta_mat.push(x);
            }
        }
        Matrix {
            data: delta_mat.into(),
            height: y.width(),
        }
    }
    fn gradient_descent(
        &mut self,
        learning_rate: T,
        delta: Matrix<Matrix<T>>,
        a: [Matrix<T>; LAYER_COUNT],
    ) where
        T: Sub<Output = T>,
        T: Mul<Output = T>,
        //T: Div<Output = T>,
        T: From<u32>,
        T: Scalar + Clone + Debug,
    {
        for layer in 0..LAYER_COUNT - 1 {
            let mut biases_sum = delta.get_owned(0, layer);
            let mut weights_sum = delta.get_owned(0, layer) * (&a[layer]).get_column(0).transpose();
            for x in 1..delta.height() {
                // this one is a vector -> 1 * p
                let a = (&a[layer]).get_column(x).transpose();
                // this one is a vector -> m * 1
                // need to be multiplied by a 1 * p matrix, another vector
                let delta_l = delta.get_owned(x, layer); // infinite?
                //println!("{:?}", a);
                weights_sum = weights_sum + (delta_l * a);
                biases_sum = biases_sum + delta.get_owned(x, layer);
            }
            biases_sum = biases_sum * (learning_rate / (delta.height() as u32).into());
            weights_sum = weights_sum * (learning_rate / (delta.height() as u32).into());
            // because i stupidly made self.biases Box<[T]> instead of my own type
            for (i, bias) in self.biases[layer].iter_mut().enumerate() {
                *bias = *bias - biases_sum.get_owned(i, 0);
            }
            if let Some(weight) = self.weights.get_mut(layer) {
                *weight = weight.clone() - weights_sum;
            }
        }
    }
}

trait TestInvalid {
    fn is_inf(&self) -> bool;
    fn is_nan(&self) -> bool;
}

impl TestInvalid for f64 {
    fn is_inf(&self) -> bool {
        !self.is_finite()
    }
    fn is_nan(&self) -> bool {
        f64::is_nan(*self)
    }
}

impl<T, U> Mul<Matrix<U>> for Matrix<T>
where
    T: Mul<U>,
    T: Clone,
    U: Clone,
    <T as Mul<U>>::Output: Add<Output = <T as Mul<U>>::Output>,
{
    type Output = Matrix<MulOut<T, U>>;
    fn mul(self, rhs: Matrix<U>) -> Self::Output {
        let m = self.height();
        //println!("m = {}", m);
        let n = rhs.height();
        //println!("n = {}", n);
        let p = rhs.width();
        //println!("p = {}", p);
        let x = matrix_mul(&self.data, rhs.data, m, n, p);
        Matrix { data: x, height: m }
    }
}
impl<T, U> Add<Matrix<U>> for Matrix<T>
where
    T: Add<U>,
{
    type Output = Matrix<<T as Add<U>>::Output>;
    fn add(self, rhs: Matrix<U>) -> Self::Output {
        let height = self.height();
        let x = matrix_map_a_b(self.data, rhs.data, |x, y| x + y);
        Matrix { data: x, height }
    }
}
impl<'a, T> Neg for &'a mut Matrix<T>
where
    T: Neg<Output = T> + Clone + 'a,
{
    type Output = &'a mut Matrix<T>;
    fn neg(self) -> Self::Output {
        self.map_ref(|x| -x)
    }
}
impl<T> Neg for Matrix<T>
where
    T: Neg<Output = T> + Clone,
{
    type Output = Matrix<T>;
    fn neg(self) -> Self::Output {
        self.map(|x| -x)
    }
}
impl<T, U> Mul<U> for Matrix<T>
where
    T: Mul<U>,
    U: Clone + Scalar,
{
    type Output = Matrix<<T as Mul<U>>::Output>;
    fn mul(self, rhs: U) -> Self::Output {
        self.map(|x| x * rhs.clone())
    }
}
impl<T, U> Add<U> for Matrix<T>
where
    T: Add<U>,
    U: Clone + Scalar,
{
    type Output = Matrix<<T as Add<U>>::Output>;
    fn add(self, rhs: U) -> Self::Output {
        self.map(|x| x + rhs.clone())
    }
}

impl Scalar for f64 {
    const ONE: Self = 1.0;
    const ZERO: Self = 0.0;
}
impl Scalar for i32 {
    const ONE: Self = 1;
    const ZERO: Self = 0;
}
pub trait Scalar:
    Copy
    + Sized
    + Send
    + Sync
    + PartialEq
    + PartialOrd
    + Mul<Output = Self>
    + MulAssign
    + Add<Output = Self>
    + AddAssign
    + Sub<Output = Self>
    + SubAssign
    + Neg<Output = Self>
    + Div<Output = Self>
    + DivAssign
    + 'static
{
    const ONE: Self;
    const ZERO: Self;
}

impl<T, U> Mul<&Matrix<U>> for &Matrix<T>
where
    T: Mul<U>,
    T: Clone,
    U: Clone,
    <T as Mul<U>>::Output: Add<Output = <T as Mul<U>>::Output>,
{
    type Output = Matrix<MulOut<T, U>>;
    fn mul(self, rhs: &Matrix<U>) -> Self::Output {
        let m = self.height();
        let n = rhs.height();
        let p = rhs.width();
        let x = matrix_mul(&self.data, &rhs.data, m, n, p);
        Matrix { data: x, height: m }
    }
}

impl<T> Matrix<T> {
    pub fn get_column(&self, j: usize) -> Matrix<T>
    where
        T: Clone,
    {
        let mut out = Vec::new();
        for i in 0..self.height() {
            out.push(self.get_owned(i, j));
        }
        Matrix {
            data: out.into(),
            height: self.height(),
        }
    }
    pub fn get_row(&self, i: usize) -> Matrix<T>
    where
        T: Clone,
    {
        let width = self.width();
        let mut out = Vec::new();
        for j in 0..width {
            out.push(self.get_owned(i, j));
        }
        Matrix {
            data: out.into(),
            height: 1,
        }
    }
    pub fn get_owned(&self, i: usize, j: usize) -> T
    where
        T: Clone,
    {
        assert!(i < self.height());
        assert!(j < self.width());
        self.data[i * self.width() + j].clone()
    }
    pub fn height(&self) -> usize {
        self.height
    }
    pub fn width(&self) -> usize {
        self.data.len() / self.height
    }
    pub fn shape(&self) -> (usize, usize) {
        (self.height(), self.width())
    }
    pub fn add_extended_slice<'a, U>(&self, rhs: impl Into<&'a [U]>) -> Matrix<AddOut<T, U>>
    where
        T: Add<U>,
        T: Clone,
        U: Clone + 'a,
    {
        let rhs = rhs.into();
        assert_eq!(
            self.height(),
            rhs.len(),
            "height of both matricies must match"
        );
        let m = self.height();
        let n = self.width();
        let x = add_extended(&self.data, rhs, m, n);
        Matrix { data: x, height: m }
    }
    pub fn add_extended<'a, U>(&self, rhs: impl Into<&'a Matrix<U>>) -> Matrix<AddOut<T, U>>
    where
        T: Add<U>,
        T: Clone,
        U: Clone + 'a,
    {
        let rhs = rhs.into();
        let m = self.height();
        let n = self.width();
        let x = add_extended(&self.data, &rhs.data, m, n);
        Matrix { data: x, height: m }
    }
    pub fn map_ref<F>(&mut self, f: F) -> &mut Self
    where
        F: Fn(T) -> T,
        T: Clone,
    {
        for i in 0..self.data.len() {
            self.data[i] = f(self.data[i].clone());
        }
        self
    }
    //pub fn map<F>(mut self, f: F) -> Self
    //where
    //    F: Fn(T) -> T,
    //    T: Clone,
    //{
    //    for i in 0..self.data.len() {
    //        self.data[i] = f(self.data[i].clone());
    //    }
    //    self
    //}
    pub fn map<F, U>(mut self, f: F) -> Matrix<U>
    where
        F: Fn(T) -> U,
    {
        let mut out = Vec::new();
        let height = self.height();
        for x in self.data.into_iter() {
            out.push(f(x));
        }
        Matrix {
            data: out.into(),
            height,
        }
    }
    pub fn transpose_clone(&self) -> Self
    where
        T: Clone,
    {
        let height = self.height();
        let width = self.width();
        let out_height = width;
        let out_width = height;
        Matrix::from_fn(out_height, out_width, |i, j| self.get_owned(j, i))
    }
    // i can instead do this in place to avoid allocation of two vecs
    pub fn transpose(self) -> Self {
        let out_width = self.height();
        let out_height = self.width();
        let mut x: Vec<Option<T>> = self.data.into_iter().map(|x| Some(x)).collect();
        let mut output = Vec::new();
        for i in 0..out_height {
            for j in 0..out_width {
                // TODO FIX
                output.push(x[j * out_height + i].take().unwrap());
            }
        }
        Matrix {
            data: output.into(),
            height: out_height,
        }
    }
    pub fn hadamard<U>(self, rhs: Matrix<U>) -> Matrix<<T as Mul<U>>::Output>
    where
        T: Mul<U>,
    {
        assert_eq!(
            self.height(),
            rhs.height(),
            "both vectors must have the same size for hadamard"
        );
        let height = self.height();
        Matrix {
            data: matrix_map_a_b(self.data, rhs.data, |x, y| x * y),
            height,
        }
    }
}

fn matrix_mul<T, U>(
    lhs: impl AsRef<[T]>,
    rhs: impl AsRef<[U]>,
    m: usize,
    n: usize,
    p: usize,
) -> Box<[MulOut<T, U>]>
where
    T: Mul<U>,
    T: Clone,
    U: Clone,
    <T as Mul<U>>::Output: Add<Output = <T as Mul<U>>::Output>,
{
    // c[i][j] = a[i][0] b[0][j] + a[i][1] b[1][j] + ... + a[i][n] b[n][j]
    // n is the width of A
    // (m * n) mul (n * p) = (m * p) matrix
    let lhs = lhs.as_ref();
    let rhs = rhs.as_ref();
    assert_eq!(
        m * n,
        lhs.len(),
        "lhs matrix must have a size (total length) of m * n"
    );
    assert_eq!(
        n * p,
        rhs.len(),
        "rhs matrix must have a size (total length) of n * p"
    );
    let mut out = Vec::new();
    for i in 0..m {
        for j in 0..p {
            let mut sum = lhs[i * n + 0].clone() * rhs[0 * n + j].clone();
            for k in 1..n {
                /* multiply by the width */
                sum = sum + lhs[i * n + k].clone() * rhs[k * p + j].clone();
            }
            out.push(sum);
        }
    }
    out.into()
}
fn matrix_map_a_b<T, U, V, F>(lhs: Box<[T]>, rhs: Box<[U]>, f: F) -> Box<[V]>
where
    F: Fn(T, U) -> V,
{
    let lhs = lhs.into_iter();
    let rhs = rhs.into_iter();
    let mut out = Vec::new();
    for (x, y) in lhs.zip(rhs) {
        out.push(f(x, y));
    }
    out.into()
}
fn matrix_sub<T, U>(lhs: Box<[T]>, rhs: Box<[U]>) -> Box<[<T as Sub<U>>::Output]>
where
    T: Sub<U>,
{
    matrix_map_a_b(lhs, rhs, |a, b| a - b)
}
impl<T, U> Sub<Matrix<U>> for Matrix<T>
where
    T: Sub<U>,
{
    type Output = Matrix<<T as Sub<U>>::Output>;
    fn sub(self, rhs: Matrix<U>) -> Self::Output {
        let m = self.height();
        //let n = self.width();
        Matrix {
            data: matrix_sub(self.data, rhs.data),
            height: m,
        }
    }
}

type AddOut<T, U> = <T as Add<U>>::Output;

// lets you add an m * n matrix to a m * 1 matrix by extending it to width n
// lhs is the m * n matrix, rhs is the m * 1 matrix
fn add_extended<T, U>(
    lhs: impl AsRef<[T]>,
    rhs: impl AsRef<[U]>,
    m: usize,
    n: usize,
) -> Box<[AddOut<T, U>]>
where
    T: Add<U>,
    T: Clone,
    U: Clone,
{
    let lhs = lhs.as_ref();
    let rhs = rhs.as_ref();
    let mut out = Vec::with_capacity(m * n); /* the output matrix */
    for i in 0..m {
        for j in 0..n {
            let number = lhs[i * m + j].clone() + rhs[i].clone();
            out.push(number);
        }
    }
    out.into()
}

//impl<const INPUT_WIDTH: usize, const LAYER_COUNT: usize, T> Network<INPUT_WIDTH, LAYER_COUNT, T>
//where
//    [(); LAYER_COUNT - 1]:,
//{
//    pub fn new<F>(heights: [usize; LAYER_COUNT], mut randfn: F) -> Self
//        where F: FnMut() -> T
//    {
//        let layers: [Box<[[Option<T>; INPUT_WIDTH]]>; LAYER_COUNT] = array::from_fn(|i| {
//            let height = heights[i];
//            let mut x = Vec::new();
//            for j in 0..height {
//                x.push(array::from_fn(|x| None));
//            }
//            x.into()
//        });
//        let biases: [Box<[T]>; LAYER_COUNT] = array::from_fn(|i| {
//            let height = heights[i];
//            let mut x = Vec::new();
//            for j in 0..height {
//                x.push(randfn());
//            }
//            x.into()
//        });
//        let weights: [Box<[Box<[T]>]>; LAYER_COUNT - 1] = array::from_fn(|i| {
//            let height = heights[i + 1];
//            let width = heights[i];
//            // size of W[n] = height[n] * height[n - 1]
//            // for programming (zero indexed)
//            // size of W[n] = height[n + 1] * height[n]
//            // size of W[1] = height[1] * height[0]
//            let mut x = Vec::new();
//            for j in 0..height {
//                let mut y = Vec::new();
//                for k in 0..width {
//                    y.push(randfn());
//                }
//                x.push(y.into());
//            }
//            x.into()
//        });
//        let out = Self {
//            layout_heights: heights,
//            layers,
//            biases,
//            weights
//        };
//        out
//    }
//    pub fn feed_forward(input: &[[T; INPUT_WIDTH]]) -> Box<[[T; INPUT_WIDTH]]>{
//        todo!()
//    }
//}

// backprop code old, doesnt really work at all

//let y_hat = model_out.last().unwrap().clone();
//assert_eq!(y_hat.shape(), y.shape());
//let m = y_hat.width();
//let delC_delZ_L = (y_hat - y) * (<T as Scalar>::ONE / (m as u32).into());
//let mut layer_delC_delZ = Box::new(delC_delZ_L);
//let mut delC_delW_list = Vec::new();
//let mut delC_delB_list = Vec::new();
//for i in (0..LAYER_COUNT - 1).rev() {
//    let layer_weights = &self.weights[i];
//    //let layer_biases = &self.biases[i]; // i guess this isnt needed?
//    let delC_delZ = layer_delC_delZ;

//    /* calculate delC / delW for this layer */
//    let delZ_delW = &model_out[i - 1].transpose_clone();
//    let delC_delW = delC_delZ.as_ref() * delZ_delW;
//    delC_delW_list.push(delC_delW);

//    /* calculate delC / delB for this layer */
//    // TODO turn this into a function
//    // TODO i dont know if the maths here is correct
//    let mut delC_delB = Vec::new();
//    for i in 0..delC_delZ.height() {
//        let mut sum = delC_delZ.get_owned(i, 0);
//        for j in 1..delC_delZ.width() {
//            sum = sum + delC_delZ.get_owned(i, j);
//        }
//        delC_delB.push(sum);
//    }
//    let delC_delB: Box<[T]> = delC_delB.into();
//    let delC_delB = Matrix {
//        data: delC_delB,
//        height: delC_delZ.height(),
//    };
//    assert_eq!(delC_delB.width(), 1, "delC / delB must have a width of 1");
//    assert_eq!(delC_delB.height(), self.biases[i].len(), "del C / del B must have the same height as Biases");
//    delC_delB_list.push(delC_delB);

//    /* calculate delC / delZ for the next layer */
//    let delZ_delAsub1 = layer_weights.transpose_clone();
//    let delC_delAsub1 = &delZ_delAsub1 * delC_delZ.as_ref();
//    // this is the issue line lol
//    //println!("{:?}", model_out[i - 1].clone().shape());
//    //println!("{:?}", model_out[i - 1].clone().data.len());
//    //let x = (-model_out[i - 1].clone() * <T as Scalar>::ONE);
//    //println!("{:?}", x.shape());
//    // has a shape of n[i - 1] * m
//    let delAsub1_delZsub1 =
//        model_out[i - 1].clone() * (-model_out[i - 1].clone() + <T as Scalar>::ONE).transpose();
//    println!("gets here");
//    let x = delC_delAsub1 * delAsub1_delZsub1;
//    layer_delC_delZ = Box::new(x);

//    //let next_delC_delZ =
//    //let delC_delW = layer_delC_delZ *
//}
//(
//    delC_delW_list.try_into().unwrap(),
//    delC_delB_list.try_into().unwrap(),
//)

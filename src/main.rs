//#![allow(unused)]
#![allow(unused_imports, dead_code, unused_mut, unused_parens)]
#![feature(generic_const_exprs)]
#![feature(read_le)]
#![allow(incomplete_features)]
#![feature(unboxed_closures)]

use std::{
    any::TypeId, array::{self, TryFromSliceError}, borrow::Borrow, convert::identity, error::Error, fmt::{Debug, Display}, fs::File, io::{Cursor, Read}, mem::{self, ManuallyDrop, MaybeUninit, swap}, ops::{Add, AddAssign, Deref, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign}, ptr::NonNull, slice, time,
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

use ordered_float::{FloatCore, NotNan};
use rand_distr::{Distribution, Normal, num_traits};

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

struct MnistData {
    sizes: Vec<i32>,
    data: Vec<u8>,
}
impl MnistData {
    pub fn new<T: Read>(r: &mut T) -> Result<Self, Box<dyn Error>> {
        let magic_number: i32 = r.read_be()?;

        let mut sizes: Vec<i32> = Vec::new();
        let mut data: Vec<u8> = Vec::new();
        match magic_number {
            2049 => {
                sizes.push(r.read_be()?);
            }
            2051 => {
                for _ in 0..3 {
                    sizes.push(r.read_be()?);
                }
            }
            _ => {
                eprintln!("{}", magic_number);
                Err("magic number should be 2049 or 2051")?
            }
        }
        r.read_to_end(&mut data)?;
        Ok(MnistData { sizes, data })
    }
}

//fn meow<T, F>(images_data: MnistData, f: F) -> Box<[Box<[T]>]>
//where
//    F: Fn(u8) -> T,
//{
//    let image_shape = (images_data.sizes[1] * images_data.sizes[2]) as usize;
//    let mut images: Vec<Box<[T]>> = Vec::new();
//    for i in 0..images_data.sizes[0] as usize {
//        let start = i * image_shape;
//        let image_data = images_data.data[start..start + image_shape].to_vec();
//        let image_data: Vec<T> = image_data.into_iter().map(|x| f(x)).collect();
//        images.push(image_data.into());
//    }
//    let images: Box<[Box<[T]>]> = images.into();
//    images
//}
fn meow<T, F>(images_data: MnistData, f: F) -> Box<[T]>
where
    F: Fn(u8) -> T,
{
    let image_shape = (images_data.sizes[1] * images_data.sizes[2]) as usize;
    let mut images: Vec<T> = Vec::new(); // with capacity idk
    for i in 0..images_data.sizes[0] as usize {
        let start = i * image_shape;
        //let image_data = images_data.data[start..start + image_shape].to_vec();
        //let image_data: Vec<T> = image_data.into_iter().map(|x| f(x)).collect();
        //images.push(image_data.into());
        for x in images_data.data[start..start + image_shape].into_iter() {
            images.push(f(*x));
        }
    }
    let images: Box<[T]> = images.into();
    images
}

//struct CachedExp {
//}

impl<T: FloatCore> Deref for Nan<T> {
    type Target = NotNan<T>;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
fn not_nan<T: FloatCore>(x: T) -> Nan<T> {
    Nan {
        inner: NotNan::new(x).unwrap(),
    }
}
type ScalarType = f32;
type NonNanType = Nan<ScalarType>;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start = time::Instant::now();
    let mut file = File::open("train-labels.idx1-ubyte")?;
    let label_data = MnistData::new(&mut file)?;
    let images_data = MnistData::new(&mut File::open("train-images.idx3-ubyte")?)?;
    let image_shape = (images_data.sizes[1] * images_data.sizes[2]) as usize;

    let images = meow(images_data, |x| {
        not_nan(Into::<ScalarType>::into(x) / 255.0)
    });
    let training_set_size = images.len();
    let classification = label_data.data;
    let classification: Box<[NonNanType]> = {
        let mut out = Vec::new();
        for x in classification {
            for i in 0..10 {
                if i == x {
                    out.push(not_nan(1.0));
                } else {
                    out.push(not_nan(0.0));
                }
            }
        }
        out.into()
    };

    let one = not_nan(1.0);
    //let zero = not_nan(0.0);
    let distribution = rand_distr::Normal::new(0.0, 1.0)?;
    //let weights_dist = rand_distr::Normal::new(0.0, 1.0 / (image_shape as ScalarType).sqrt())?;
    let rng = &mut rand::rng();
    let rng2 = &mut rand::rng();
    let mut network = Network::new(
        [image_shape, 100, 10],
        || not_nan(distribution.sample(rng)),
        |connections| {
            let distribution = Normal::new(0.0, 1.0 / (connections as ScalarType).sqrt()).unwrap();
            not_nan(distribution.sample(rng2))
        },
    );
    let minibatch_size = 50;
    // prepare minibatch
    let activation = |x: NonNanType| one / (one + (-x).exp());
    let del_activation = |x| activation(x) * (one - activation(x));
    let del_cost = |_, a: Matrix<_>, y| a.sub_self(y);
    for epoch in 0..1 {
        println!("starting epoch {epoch}");
        network.run_epoch(
            &images,
            &classification,
            minibatch_size,
            image_shape,
            10,
            activation,
            del_cost,
            del_activation,
            not_nan(0.4),
            not_nan(5.0),
            training_set_size.into(),
        );
    }

    // forward prop on test sample,
    // check for many were correct,
    // report that as a percentage
    //let
    let test_images_data = MnistData::new(&mut File::open("t10k-images.idx3-ubyte")?)?;
    let test_labels = MnistData::new(&mut File::open("t10k-labels.idx1-ubyte")?)?;
    let x = meow(test_images_data, |x| {
        not_nan(Into::<ScalarType>::into(x) / 255.0)
    });
    let height = x.len();
    //let x: Vec<_> = x.into_iter().flatten().collect();
    let test_images = Matrix {
        data: x.into(),
        height,
    }
    .transpose();
    network.feed_forward(&test_images, activation);
    let (a, _) = network.feed_forward_results();
    let y_hat = a.last().unwrap();
    let mut correct_count = 0;
    for j in 0..height {
        let mut max = y_hat.get_owned(0, j);
        let mut max_index = 0;
        for i in 0..10 {
            let x = y_hat.get_owned(i, j);
            if x > max {
                max = x;
                max_index = i;
            }
        }
        if max_index == test_labels.data[j] as usize {
            correct_count += 1;
        }
    }
    let correct_ratio = correct_count as f64 / height as f64;
    println!("correct percentage: {}%", correct_ratio * 100.0);
    println!("correct count: {}", correct_count);

    let end = time::Instant::now();
    let diff = end - start;
    println!("time taken: {}s", diff.as_secs_f32());
    Ok(())
}

impl Scalar for NotNan<f64> {
    const ONE: Self = const { unsafe { NotNan::new_unchecked(1.0) } };
    const ZERO: Self = const { unsafe { NotNan::new_unchecked(0.0) } };
}
impl Scalar for NotNan<f32> {
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
    a_buffer: Vec<Matrix<T>>,
    z_buffer: Vec<Matrix<T>>,
}

// layer count includes input layer? idk lol
impl<const LAYER_COUNT: usize, T> Network<LAYER_COUNT, T>
where
    [(); LAYER_COUNT - 1]:,
{
    pub fn new<BFn, WFn>(
        heights: [usize; LAYER_COUNT],
        mut biases_fn: BFn,
        mut weights_fn: WFn,
    ) -> Self
    where
        BFn: FnMut() -> T,
        WFn: FnMut(usize) -> T,
    {
        let biases = array::from_fn(|i| {
            let height = heights[i + 1];
            let mut out = Vec::new();
            for _ in 0..height {
                out.push(biases_fn());
            }
            out.into()
        });
        let weights = array::from_fn(|i| {
            let height = heights[i + 1];
            let width = heights[i];
            // i think using width here is correct lol, idk
            Matrix::from_fn(height, width, |_, _| weights_fn(width))
        });
        let a_buffer = Vec::new();
        let z_buffer = Vec::new();
        Network {
            layout_heights: heights,
            biases,
            weights,
            a_buffer,
            z_buffer,
        }
    }
    //pub fn feed_forward<'a, F>(&mut self, input: impl Into<&'a Matrix<T>>, activation_fn: F)
    pub fn feed_forward<'a, F>(&mut self, input: impl AsMat<T>, activation_fn: F)
    /*-> (&[Matrix<T>; LAYER_COUNT], &[Matrix<T>; LAYER_COUNT - 1])*/
    /* a, z */
    where
        F: Fn(T) -> T,
        T: Mul<Output = T> + Add<Output = T> + Clone,
        T: 'a,
    {
        //let input = input.into();
        // TODO REMOVE AFTER TESTING
        self.a_buffer.clear();
        self.z_buffer.clear();
        let layers_a = &mut self.a_buffer;
        let layers_z = &mut self.z_buffer;
        // formula
        // W[i] * a[i - 1] + b[i] = z[i]
        // a[i] = g(z[i])
        layers_a.push(input.clone_to_matrix());
        for i in 0..LAYER_COUNT - 1 {
            let biases_layer = self.biases[i].as_ref();
            let layer = &layers_a[i];
            // TODO add_extended_slice_self
            let z = (&self.weights[i] * layer).add_extended_slice(biases_layer);
            layers_z.push(z.clone());
            //let a = z.map(&activation_fn);
            let a = z.map_self(&activation_fn);
            layers_a.push(a);
        }
    }
    pub fn feed_forward_results(
        &self,
    ) -> (&[Matrix<T>; LAYER_COUNT], &[Matrix<T>; LAYER_COUNT - 1]) {
        let layers_a = &self.a_buffer;
        let layers_z = &self.z_buffer;
        (layers_a.as_array().unwrap(), layers_z.as_array().unwrap())
    }
    #[allow(non_snake_case)]
    pub fn back_prop<'a, F, DCf>(
        &self,
        y_hat: impl AsMat<T>,
        z: &[Matrix<T>; LAYER_COUNT - 1],
        y: impl AsMat<T>,
        del_activation_fn: F,
        del_cost: DCf,
    ) -> Matrix<Matrix<T>>
    /* more like a matrix of vectors */
    where
        T: Sub<Output = T>,
        T: Add<Output = T>,
        T: Scalar,
        T: Mul<T, Output = T>,
        F: Fn(T) -> T,
        DCf: Fn(Matrix<T>, Matrix<T>, Matrix<T>) -> Matrix<T>,
    {
        //let y_hat = a.last().unwrap();
        //let y_hat = y_hat.into();
        //let y = y.into();
        let mut delta_mat = Vec::with_capacity(LAYER_COUNT - 1);
        // implemented like this because i dont know how to do it vectorised
        for sample in 0..y.width() {
            let sample_y_hat = y_hat.get_column(sample);
            let sample_y = y.get_column(sample);
            let sample_z_L = z.last().unwrap().get_column(sample);

            let layer_delta = del_cost(sample_z_L, sample_y_hat, sample_y);
            let mut range = (0..LAYER_COUNT - 1).rev();
            let successors = core::iter::successors(range.next().map(|i| (i, layer_delta)), |x| {
                let i = range.next()?;
                let z = z[i].clone();
                let w = &self.weights[i + 1].transpose_clone();
                //let layer_delta = (w * &x.1).hadamard(z.map(&del_activation_fn));
                //let layer_delta = (w * &x.1).hadamard_self(z.map(&del_activation_fn));
                let layer_delta = (w * &x.1).hadamard_self(z.map_self(&del_activation_fn));
                Some((i, layer_delta))
            });
            // reverse the data because its BACK propagation
            // the data is given from front to back
            let mut intermediate_vec = Vec::with_capacity(LAYER_COUNT - 1);
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
        lambda: T,
        training_set_size: T,
    ) where
        T: Sub<Output = T>,
        T: Mul<Output = T>,
        T: From<u16>,
        T: Scalar + Clone,
    {
        let a: &[Matrix<T>; LAYER_COUNT] = self.a_buffer.as_array().unwrap();
        for layer in 0..LAYER_COUNT - 1 {
            let mut biases_sum = delta.get_owned(0, layer);
            let mut weights_sum = delta.get_owned(0, layer) * (&a[layer]).get_column(0).transpose();
            for x in 1..delta.height() {
                // this one is a vector -> 1 * p
                let a = (&a[layer]).get_column(x).transpose();
                // this one is a vector -> m * 1
                // need to be multiplied by a 1 * p matrix, another vector
                let delta_l = delta.get_owned(x, layer); // infinite?
                //weights_sum = weights_sum + (delta_l * a);
                weights_sum = weights_sum.add_self(delta_l * a);
                //biases_sum = biases_sum + delta.get_owned(x, layer);
                biases_sum = biases_sum.add_self(delta.get_owned(x, layer));
            }
            let factor = learning_rate / (delta.height() as u16).into();
            biases_sum = biases_sum * factor;
            weights_sum = weights_sum * factor;
            // because i stupidly made self.biases Box<[T]> instead of my own type
            for (i, bias) in self.biases[layer].iter_mut().enumerate() {
                *bias = *bias - biases_sum.get_owned(i, 0);
            }
            if let Some(weight) = self.weights.get_mut(layer) {
                let weight_decay =
                    <T as Scalar>::ONE - ((learning_rate * lambda) / training_set_size);
                *weight = weight.clone() * weight_decay - weights_sum;
            }
        }
    }
    fn run_epoch(
        &mut self,
        train_data: &[T],
        train_y: &[T],
        minibatch_size: usize,
        input_height: usize,
        y_height: usize,
        activation_fn: impl Fn(T) -> T,
        del_cost: impl Fn(Matrix<T>, Matrix<T>, Matrix<T>) -> Matrix<T>,
        del_activation_fn: impl Fn(T) -> T,
        learning_rate: T,
        lambda: T,
        training_set_size: T,
    ) where
        T: Clone + Mul<Output = T> + Add<Output = T> + Scalar,
        T: From<u16>,
    {
        for (train_data, train_y) in train_data
            .chunks(minibatch_size * input_height)
            .zip(train_y.chunks(minibatch_size * y_height))
        {
            let train_mat = BorrowedMat {
                data: train_data,
                height: minibatch_size,
                transposed: true,
            };
            let train_y_mat = BorrowedMat {
                data: train_y,
                height: minibatch_size,
                transposed: true,
            };
            self.feed_forward(train_mat, &activation_fn);
            let (a, z) = self.feed_forward_results();
            let delta = self.back_prop(
                a.last().unwrap(),
                z,
                train_y_mat,
                &del_activation_fn,
                &del_cost,
            );
            self.gradient_descent(learning_rate, delta, lambda, training_set_size);
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
        let n = rhs.height();
        let p = rhs.width();
        let x = matrix_mul(&self.data[..], rhs.data, m, n, p);
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
        let width = self.width();
        let x = matrix_map_a_b(self.data, rhs.data, height, width, |x, y| x + y);
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
    //+ MulAssign
    + Add<Output = Self>
    //+ AddAssign
    + Sub<Output = Self>
    //+ SubAssign
    + Neg<Output = Self>
    + Div<Output = Self>
    //+ DivAssign
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
        let x = matrix_mul(&self.data[..], &rhs.data[..], m, n, p);
        Matrix { data: x, height: m }
    }
}

pub trait AsMat<T> {
    fn as_mat<'a>(&'a self) -> BorrowedMat<'a, T>;
    fn height(&self) -> usize {
        self.as_mat().height
    }
    fn width(&self) -> usize {
        self.as_mat().data.len() / self.height()
    }
    fn get_column(&self, j: usize) -> Matrix<T>
    where
        T: Clone,
    {
        let mut out = Vec::with_capacity(self.height());
        for i in 0..self.height() {
            out.push(self.get_owned(i, j));
        }
        Matrix {
            data: out.into(),
            height: self.height(),
        }
    }
    fn is_transposed(&self) -> bool {
        self.as_mat().transposed
    }
    fn get_owned(&self, i: usize, j: usize) -> T
    where
        T: Clone,
    {
        let mut new_i = i;
        let mut new_j = j;
        if self.is_transposed() {
            swap(&mut new_i, &mut new_j);
        }
        assert!(new_i < self.height());
        assert!(new_j < self.width());
        self.as_mat().data[new_i * self.width() + new_j].clone()
    }
    fn shape(&self) -> (usize, usize) {
        (self.height(), self.width())
    }
    fn data(&self) -> &[T] {
        self.as_mat().data
    }
    fn add_extended_slice<'a, U>(&self, rhs: impl Into<&'a [U]>) -> Matrix<AddOut<T, U>>
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
        let x = add_extended(self.data(), rhs, m, n);
        Matrix { data: x, height: m }
    }
    fn add_extended<'a, U>(&self, rhs: impl AsMat<U>) -> Matrix<AddOut<T, U>>
    where
        T: Add<U>,
        T: Clone,
        U: Clone + 'a,
    {
        let m = self.height();
        let n = self.width();
        let x = add_extended(self.data(), rhs.data(), m, n);
        Matrix { data: x, height: m }
    }
    fn transpose_clone(&self) -> Matrix<T>
    where
        T: Clone,
    {
        let height = self.height();
        let width = self.width();
        let out_height = width;
        let out_width = height;
        Matrix::from_fn(out_height, out_width, |i, j| self.get_owned(j, i))
    }
    fn clone_to_matrix(&self) -> Matrix<T>
    where
        T: Clone,
    {
        Matrix {
            data: self.data().into(),
            height: self.height(),
        }
    }
}
impl<T> Matrix<T> {
    pub fn add_self(self, rhs: Self) -> Self
    where
        T: Add<Output = T>,
    {
        assert_eq!(self.shape(), rhs.shape());
        let height = self.height();
        let width = self.width();
        Matrix {
            data: matrix_map_a_b_override_a(self.data, rhs.data, height, width, Add::add),
            height,
        }
    }
    pub fn sub_self(self, rhs: Self) -> Self
    where
        T: Sub<Output = T>,
    {
        assert_eq!(self.shape(), rhs.shape());
        let height = self.height();
        let width = self.width();
        Matrix {
            data: matrix_map_a_b_override_a(self.data, rhs.data, height, width, Sub::sub),
            height,
        }
    }
    //pub fn get_row(&self, i: usize) -> Matrix<T>
    //where
    //    T: Clone,
    //{
    //    let width = self.width();
    //    let mut out = Vec::new();
    //    for j in 0..width {
    //        out.push(self.get_owned(i, j));
    //    }
    //    Matrix {
    //        data: out.into(),
    //        height: 1,
    //    }
    //}
    // most of the time T and U will be the exact same,
    // but the added flexibility might be nice
    pub fn add_extended_slice_self<U>(self, rhs: Box<[U]>) -> Matrix<T>
    where
        T: Add<U, Output = T>,
        U: Clone,
    {
        let m = self.height();
        let n = self.width();
        Matrix {
            data: map_extended_self(self.data, rhs, m, n, |x, y| x + y),
            height: m,
        }
    }
    pub fn map_ref(&mut self, f: impl Fn(T) -> T) -> &mut Self
    where
        T: Clone,
    {
        for i in 0..self.data.len() {
            self.data[i] = f(self.data[i].clone());
        }
        self
    }
    pub fn map_self(mut self, f: impl Fn(T) -> T) -> Self
    {
        let self_ptr = self.data.as_ptr();
        for i in 0..self.data.len() {
            let self_val = unsafe { self_ptr.add(i).read() };
            self.data[i] = f(self_val);
        }
        self
    }
    pub fn map<U>(mut self, f: impl Fn(T) -> U) -> Matrix<U>
    {
        let mut out = Vec::with_capacity(self.data.len());
        let height = self.height();
        for x in self.data.into_iter() {
            out.push(f(x));
        }
        Matrix {
            data: out.into(),
            height,
        }
    }
    // i can instead do this in place to avoid allocation of two vecs
    pub fn transpose(self) -> Self {
        let self_len = self.data.len();
        let out_width = self.height();
        let out_height = self.width();
        let mut x: Vec<Option<T>> = self.data.into_iter().map(|x| Some(x)).collect();
        let mut output = Vec::with_capacity(self_len);
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
    pub fn hadamard_self(self, rhs: Self) -> Self
    where
        T: Mul<Output = T>,
    {
        assert_eq!(
            self.height(),
            rhs.height(),
            "both vectors must have the same size for hadamard"
        );
        let height = self.height();
        let width = self.width();
        Matrix {
            data: matrix_map_a_b_override_a(self.data, rhs.data, height, width, |x, y| x * y),
            height,
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
        let width = self.width();
        Matrix {
            data: matrix_map_a_b(self.data, rhs.data, height, width, |x, y| x * y),
            height,
        }
    }
}

//fn matrix_mul<T, U>(
//    lhs: impl AsRef<[T]>,
//    rhs: impl AsRef<[U]>,
//    m: usize,
//    n: usize,
//    p: usize,
//) -> Box<[MulOut<T, U>]>
//where
//    T: Mul<U>,
//    T: Clone,
//    U: Clone,
//    <T as Mul<U>>::Output: Add<Output = <T as Mul<U>>::Output>,
//{
//    // c[i][j] = a[i][0] b[0][j] + a[i][1] b[1][j] + ... + a[i][n] b[n][j]
//    // n is the width of A
//    // (m * n) mul (n * p) = (m * p) matrix
//    let lhs = lhs.as_ref();
//    let rhs = rhs.as_ref();
//    assert_eq!(
//        m * n,
//        lhs.len(),
//        "lhs matrix must have a size (total length) of m * n"
//    );
//    assert_eq!(
//        n * p,
//        rhs.len(),
//        "rhs matrix must have a size (total length) of n * p"
//    );
//    let mut out = Vec::with_capacity(m * p);
//    for i in 0..m {
//        for j in 0..p {
//            let mut sum = lhs[i * n + 0].clone() * rhs[0 * n + j].clone();
//            for k in 1..n {
//                /* multiply by the width */
//                sum = sum + lhs[i * n + k].clone() * rhs[k * p + j].clone();
//            }
//            out.push(sum);
//        }
//    }
//    out.into()
//}
fn matrix_mul<T, U>(
    lhs: impl WidthGetCloned<T>,
    rhs: impl WidthGetCloned<U>,
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
    let mut out = Vec::with_capacity(m * p);
    for i in 0..m {
        for j in 0..p {
            //let mut sum = lhs[i * n + 0].clone() * rhs[0 * n + j].clone();
            let mut sum = lhs.width_get_cloned(i, n, 0) * rhs.width_get_cloned(0, n, j);
            for k in 1..n {
                sum = sum + lhs.width_get_cloned(i, n, k) * rhs.width_get_cloned(k, p, j);
            }
            out.push(sum);
        }
    }
    out.into()
}
//fn matrix_map_a_b<T, U, V, F>(lhs: Box<[T]>, rhs: Box<[U]>, f: F) -> Box<[V]>
//where
//    F: Fn(T, U) -> V,
//{
//    let len = lhs.len();
//    let lhs = lhs.into_iter();
//    let rhs = rhs.into_iter();
//    let mut out = Vec::with_capacity(len);
//    for (x, y) in lhs.zip(rhs) {
//        out.push(f(x, y));
//    }
//    out.into()
//}
//fn matrix_map_a_b<T, U, V>(lhs: Box<[T]>, rhs: Box<[U]>, f: impl Fn(T, U) -> V) -> Box<[V]>
//{
//    let len = lhs.len();
//    let lhs = lhs.into_iter();
//    let rhs = rhs.into_iter();
//    let mut out = Vec::with_capacity(len);
//    for (x, y) in lhs.zip(rhs) {
//        out.push(f(x, y));
//    }
//    out.into()
//}
// Looks cool but wont work with the accessing strategy
fn matrix_map_a_b<T, U, V>(
    lhs: impl WidthGetUnsafe<T>,
    rhs: impl WidthGetUnsafe<U>,
    m: usize,
    n: usize,
    f: impl Fn(T, U) -> V,
) -> Box<[V]> {
    let mut out = Vec::with_capacity(n * m);
    for i in 0..m {
        for j in 0..n {
            let left = unsafe { lhs.width_get_unsafe(i, n, j) };
            let right = unsafe { rhs.width_get_unsafe(i, n, j) };
            out.push(f(left, right));
        }
    }
    let _ = ManuallyDrop::new(lhs);
    let _ = ManuallyDrop::new(rhs);
    out.into()
}
//fn matrix_map_a_b_override_a<T, U, F>(mut lhs: Box<[T]>, rhs: Box<[U]>, f: F) -> Box<[T]>
//where
//    F: Fn(T, U) -> T,
//{
//    let mut rhs = rhs.into_iter();
//    let lhs_ptr = lhs.as_ptr();
//    for i in 0..lhs.len() {
//        // SAFETY: intuitively it seems safe
//        let lhs_val = unsafe { lhs_ptr.add(i).read() };
//        lhs[i] = f(lhs_val, rhs.next().unwrap());
//    }
//    lhs
//}
fn matrix_map_a_b_override_a<T, U, Out>(mut lhs: Out, rhs: impl WidthGetUnsafe<U>, m: usize, n: usize, f: impl Fn(T, U) -> T) -> Out
where
    Out: WidthGetUnsafe<T> + WidthAssign<T>,
{
    for i in 0..m {
        for j in 0..n {
            let left = unsafe {
                lhs.width_get_unsafe(i, n, j)
            };
            // SAFETY: idk
            let right = unsafe { rhs.width_get_unsafe(i, n, j) };
            lhs.assign(f(left, right), i, n, j);
        }
    }
    let _ = ManuallyDrop::new(rhs);
    lhs
}
//fn matrix_sub<T, U>(lhs: Box<[T]>, rhs: Box<[U]>) -> Box<[<T as Sub<U>>::Output]>
//where
//    T: Sub<U>,
//{
//    matrix_map_a_b(lhs, rhs, |a, b| a - b)
//}
fn matrix_sub<T, U>(lhs: impl WidthGetUnsafe<T>, rhs: impl WidthGetUnsafe<U>, m: usize, n: usize) -> Box<[<T as Sub<U>>::Output]>
where
    T: Sub<U>,
{
    matrix_map_a_b(lhs, rhs, m, n, |a, b| a - b)
}
impl<T, U> Sub<Matrix<U>> for Matrix<T>
where
    T: Sub<U>,
{
    type Output = Matrix<<T as Sub<U>>::Output>;
    fn sub(self, rhs: Matrix<U>) -> Self::Output {
        let m = self.height();
        let n = self.width();
        Matrix {
            data: matrix_sub(self.data, rhs.data, m, n),
            height: m,
        }
    }
}

type AddOut<T, U> = <T as Add<U>>::Output;

// lets you add an m * n matrix to a m * 1 matrix by extending it to width n
// lhs is the m * n matrix, rhs is the m * 1 matrix
//fn add_extended<T, U>(
//    lhs: impl AsRef<[T]>,
//    rhs: impl AsRef<[U]>,
//    m: usize,
//    n: usize,
//) -> Box<[AddOut<T, U>]>
//where
//    T: Add<U>,
//    T: Clone,
//    U: Clone,
//{
//    let lhs = lhs.as_ref();
//    let rhs = rhs.as_ref();
//    assert!(rhs.len() == m);
//    assert_eq!(lhs.len(), m * n);
//    let mut out = Vec::with_capacity(m * n); /* the output matrix */
//    for i in 0..m {
//        for j in 0..n {
//            let left = get_cloned(lhs, n, i, j);
//            //let number = lhs[i * n + j].clone() + rhs[i].clone();
//            let number = left + rhs[i].clone();
//            out.push(number);
//        }
//    }
//    out.into()
//}
fn add_extended<T, U>(
    lhs: impl WidthGetCloned<T>,
    rhs: impl AsRef<[U]>,
    m: usize,
    n: usize,
) -> Box<[AddOut<T, U>]>
where
    T: Add<U>,
    T: Clone,
    U: Clone,
{
    //let lhs = lhs.as_ref();
    let rhs = rhs.as_ref();
    assert!(rhs.len() == m);
    //assert_eq!(lhs.len(), m * n);
    let mut out = Vec::with_capacity(m * n); /* the output matrix */
    for i in 0..m {
        for j in 0..n {
            let left = lhs.width_get_cloned(i, n, j);
            //let left = get_cloned(lhs, n, i, j);
            //let number = lhs[i * n + j].clone() + rhs[i].clone();
            let number = left + rhs[i].clone();
            out.push(number);
        }
    }
    out.into()
}

// lhs is a m * n matrix
// rhs is a m * 1 matrix
// rhs elements are cloned
//fn map_extended_self<T, U, F>(
//    mut lhs: Box<[T]>,
//    rhs: Box<[U]>,
//    m: usize,
//    n: usize,
//    f: F,
//) -> Box<[T]>
//where
//    F: Fn(T, U) -> T,
//    U: Clone,
//{
//    let lhs_ptr = lhs.as_ptr();
//    for i in 0..m {
//        for j in 0..n {
//            let lhs_val = unsafe { lhs_ptr.add(i * n + j).read() };
//            lhs[i * n + j] = f(lhs_val, rhs[i].clone());
//        }
//    }
//    lhs
//}
fn map_extended_self<T, U, F, Out>(
    mut lhs: Out,
    rhs: Box<[U]>,
    m: usize,
    n: usize,
    f: F,
) -> Out
where
    F: Fn(T, U) -> T,
    U: Clone,
    Out: WidthGetUnsafe<T> + WidthAssign<T>
{
    for i in 0..m {
        for j in 0..n {
            let lhs_val = unsafe { lhs.width_get_unsafe(i, n, j) };
            lhs.assign(f(lhs_val, rhs[i].clone()), i, n, j);
        }
    }
    lhs
}

//fn get_cloned<T>(x: impl AsRef<[T]>, n: usize, i: usize, j: usize) -> T
//where
//    T: Clone,
//{
//    let x = x.as_ref();
//    x.get(i * n + j).unwrap().clone()
//}

#[derive(PartialEq, PartialOrd, Clone, Copy, Debug)]
struct Nan<T: FloatCore> {
    inner: NotNan<T>,
}

#[derive(Debug, Clone)]
pub struct Matrix<T> {
    data: Box<[T]>,
    height: usize,
}

// borrowed form of Matrix
#[derive(Debug, Clone)]
pub struct BorrowedMat<'a, T> {
    data: &'a [T],
    height: usize,
    transposed: bool,
}

impl<'a, T> BorrowedMat<'a, T> {
    fn clone_to_matrix(&self) -> Matrix<T>
    where
        T: Clone,
    {
        Matrix {
            data: self.data.into(),
            height: self.height,
        }
    }
}

//impl<'a, T: Clone> From<BorrowedMat<'a, T>> for Matrix<T> {
//    fn from(value: BorrowedMat<'a, T>) -> Self {
//        value.clone_to_matrix()
//    }
//}

impl<T> AsMat<T> for &Matrix<T> {
    fn as_mat<'a>(&'a self) -> BorrowedMat<'a, T> {
        BorrowedMat {
            data: &self.data,
            height: self.height,
            transposed: false,
        }
    }
}
impl<T> AsMat<T> for Matrix<T> {
    fn as_mat<'a>(&'a self) -> BorrowedMat<'a, T> {
        BorrowedMat {
            data: &self.data,
            height: self.height,
            transposed: false,
        }
    }
}
impl<'b, T> AsMat<T> for BorrowedMat<'b, T> {
    fn as_mat<'a>(&'a self) -> BorrowedMat<'a, T> {
        BorrowedMat {
            data: self.data,
            height: self.height,
            transposed: false,
        }
    }
}

pub trait Get<T> {
    fn get(&self, i: usize, j: usize) -> &T;
}
pub trait GetCloned<T> {
    fn get_cloned(&self, i: usize, j: usize) -> T;
}
pub trait WidthGetCloned<T> {
    fn width_get_cloned(&self, i: usize, n: usize, j: usize) -> T;
}
pub trait GetUnsafe<T> {
    unsafe fn get_unsafe(&self, i: usize, j: usize) -> T;
}
pub trait WidthGetUnsafe<T> {
    unsafe fn width_get_unsafe(&self, i: usize, n: usize, j: usize) -> T;
}

pub trait WidthAssign<T> {
    fn assign(&mut self, x: T, i: usize, n: usize, j: usize);
}

impl<T> WidthGetCloned<T> for &[T]
where
    T: Clone,
{
    fn width_get_cloned(&self, i: usize, n: usize, j: usize) -> T {
        self[i * n + j].clone()
    }
}
impl<T> WidthAssign<T> for [T] {
    fn assign(&mut self, x: T, i: usize, n: usize, j: usize) {
        self[i * n + j] = x;
    }
}
impl<T> WidthAssign<T> for Box<[T]> {
    fn assign(&mut self, x: T, i: usize, n: usize, j: usize) {
        self[i * n + j] = x;
    }
    
}

impl<T, U: AsMat<T>> Get<T> for U {
    fn get(&self, mut i: usize, mut j: usize) -> &T {
        // TODO could probably add some asserts
        // axes swap
        if self.is_transposed() {
            swap(&mut i, &mut j);
        }
        self.data().get(i * self.width() + j).unwrap()
    }
}
impl<T, U: AsMat<T>> GetCloned<T> for U
where
    T: Clone,
{
    fn get_cloned(&self, i: usize, j: usize) -> T {
        self.get_owned(i, j)
    }
}
impl<T> WidthGetUnsafe<T> for Box<[T]> {
    unsafe fn width_get_unsafe(&self, i: usize, n: usize, j: usize) -> T {
        let ptr = self.as_ptr();
        unsafe { ptr.add(i * n + j).read() }
    }
}
impl<T> WidthGetCloned<T> for Box<[T]> 
where T: Clone
{
    fn width_get_cloned(&self, i: usize, n: usize, j: usize) -> T {
        self[i * n + j].clone()
    }
}
impl<T, U: AsMat<T>> GetUnsafe<T> for U {
    unsafe fn get_unsafe(&self, mut i: usize, mut j: usize) -> T {
        if self.is_transposed() {
            swap(&mut i, &mut j);
        }
        let ptr = self.data().as_ptr();
        unsafe { ptr.add(i * self.width() + j).read() }
    }
}

impl From<u16> for Nan<f32> {
    fn from(value: u16) -> Self {
        not_nan(Into::<f32>::into(value))
    }
}
impl From<usize> for Nan<f32> {
    fn from(value: usize) -> Self {
        not_nan(value as f32)
    }
}

impl Nan<f32> {
    // may panic lol
    fn exp(self) -> Self {
        not_nan(self.into_inner().exp())
    }
    fn sqrt(self) -> Self {
        not_nan(self.into_inner().sqrt())
    }
}

impl Scalar for Nan<f32> {
    const ONE: Self = Nan {
        inner: unsafe { NotNan::new_unchecked(1.0) },
    };
    const ZERO: Self = Nan {
        inner: unsafe { NotNan::new_unchecked(0.0) },
    };
}

impl<T: FloatCore> Mul for Nan<T> {
    type Output = Nan<<T as Mul>::Output>;
    fn mul(self, rhs: Self) -> Self::Output {
        Nan {
            inner: self.inner * rhs.inner,
        }
    }
}
impl<T: FloatCore> Add for Nan<T> {
    type Output = Nan<<T as Add>::Output>;
    fn add(self, rhs: Self) -> Self::Output {
        Nan {
            inner: self.inner + rhs.inner,
        }
    }
}
impl<T: FloatCore> Sub for Nan<T> {
    type Output = Nan<<T as Sub>::Output>;
    fn sub(self, rhs: Self) -> Self::Output {
        Nan {
            inner: self.inner - rhs.inner,
        }
    }
}
impl<T: FloatCore> Div for Nan<T> {
    type Output = Nan<<T as Div>::Output>;
    fn div(self, rhs: Self) -> Self::Output {
        Nan {
            inner: self.inner / rhs.inner,
        }
    }
}
impl<T: FloatCore> Neg for Nan<T> {
    type Output = Nan<<T as Neg>::Output>;
    fn neg(self) -> Self::Output {
        Nan { inner: -self.inner }
    }
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
//for epoch in 0..5 {
//    println!("starting epoch: {epoch}");
//    for (images_chunk, classifications) in images
//        .chunks(minibatch_size)
//        .zip(classification.chunks(minibatch_size))
//    {
//        assert_eq!(images_chunk.len(), classifications.len());
//        // make y_hat which is a matrix like this with height 10
//        // [0.0, 0.0, 1.0, ...], [1.0, 0.0, ...]
//        let mut y_vec = Vec::with_capacity(classifications.len() * 10);
//        for x in classifications {
//            let x = *x;
//            for i in 0..10 {
//                if i == x {
//                    y_vec.push(not_nan(1.0));
//                } else {
//                    y_vec.push(not_nan(0.0));
//                }
//            }
//        }
//        let len = y_vec.len();
//        let y = Matrix {
//            data: y_vec.into(),
//            height: len / 10,
//        }
//        .transpose();
//        let height = images_chunk.len();
//        let vec: Vec<_> = images_chunk.iter().flatten().map(|x| *x).collect();
//        let mat = Matrix {
//            data: vec.into(),
//            height,
//        }
//        .transpose();
//        network.feed_forward(&mat, activation);
//        let (a, z) = network.feed_forward_results();

//        let delta = network.back_prop(a.last().unwrap(), z, y, del_activation, del_cost);
//        network.gradient_descent(
//            not_nan(0.4),
//            delta,
//            //a,
//            not_nan(50.0),
//            training_set_size.into(),
//        );
//    }
//}

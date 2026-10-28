//#![allow(unused)]
//#![allow(unused_imports, dead_code, unused_mut, unused_parens)]
#![allow(dead_code)]
#![allow(unused_imports, unused_mut, unused_parens)]
// generic const
#![feature(min_generic_const_args)]
#![feature(generic_const_args)]
#![feature(macroless_generic_const_args)]
#![feature(generic_const_items)]
#![feature(read_le)]
//#![feature(vec_from_fn)]
#![allow(incomplete_features)]
#![feature(unboxed_closures)]

// TODO make threadpool

use std::{
    any::TypeId,
    array::{self, TryFromSliceError},
    borrow::Borrow,
    convert::identity,
    error::Error,
    fmt::{Debug, Display},
    fs::File,
    io::{Cursor, Read},
    mem::{self, MaybeUninit, swap},
    ops::{Add, AddAssign, Deref, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
    ptr::NonNull,
    slice,
    sync::{Mutex, atomic::Ordering},
    thread, time,
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
use threadpool::ThreadPool;

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

//impl<T: FloatCore> Deref for Nan<T> {
//    type Target = NotNan<T>;
//    fn deref(&self) -> &Self::Target {
//        &self.inner
//    }
//}
//fn not_nan<T: FloatCore>(x: T) -> Nan<T> {
//    Nan {
//        inner: NotNan::new(x).unwrap(),
//    }
//}
type Inner = f32;
type ScalarType = Alg<Inner>;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start = time::Instant::now();
    let mut file = File::open("/home/gfjnf/programming/machine_learning/train-labels.idx1-ubyte")?;
    let label_data = MnistData::new(&mut file)?;
    let images_data = MnistData::new(&mut File::open(
        "/home/gfjnf/programming/machine_learning/train-images.idx3-ubyte",
    )?)?;
    let image_shape = (images_data.sizes[1] * images_data.sizes[2]) as usize;

    let images = meow(images_data, |x| Alg(Into::<Inner>::into(x) / 255.0));
    let training_set_size = images.len();
    let classification = label_data.data;
    let classification: Box<[ScalarType]> = {
        let mut out = Vec::new();
        for x in classification {
            for i in 0..10 {
                if i == x {
                    out.push(Alg(1.0));
                } else {
                    out.push(Alg(0.0));
                }
            }
        }
        out.into()
    };

    let one = <ScalarType as Scalar>::ONE;
    let distribution = rand_distr::Normal::new(0.0, 1.0)?;
    let rng = &mut rand::rng();
    let rng2 = &mut rand::rng();
    let mut network = Network::new(
        [image_shape, 100, 10],
        || Alg(distribution.sample(rng)),
        |connections| {
            let distribution = Normal::new(0.0, 1.0 / (connections as Inner).sqrt()).unwrap();
            Alg(distribution.sample(rng2))
        },
    );
    let minibatch_size = 40;
    // prepare minibatch
    let activation = |x: ScalarType| one / (one + (-x).exp());
    let del_activation = |x| activation(x) * (one - activation(x));
    let del_cost = |_, a: Matrix<_>, y| a.sub_self(y);

    let test_images_data = MnistData::new(&mut File::open(
        "/home/gfjnf/programming/machine_learning/t10k-images.idx3-ubyte",
    )?)?;
    let test_labels = MnistData::new(&mut File::open(
        "/home/gfjnf/programming/machine_learning/t10k-labels.idx1-ubyte",
    )?)?;
    let x = meow(test_images_data, |x| Alg(Into::<Inner>::into(x) / 255.0));
    let height = x.len() / image_shape;
    let test_images = Matrix {
        data: x.into(),
        height,
    }
    .transpose();

    let mut previous_correct_count = 0;
    let mut no_improvement_in_counter = 0;
    let mut learning_rate = Alg(0.35);
    let initial_learning_rate = learning_rate;
    for epoch in 0..30 {
        println!("starting epoch {epoch}");
        println!("learning rate: {}", learning_rate);
        // classification and images seem to be okay
        network.run_epoch(
            &images,
            &classification,
            minibatch_size,
            activation,
            del_cost,
            del_activation,
            learning_rate,
            Alg(250.0),
            training_set_size.into(),
        );
        let test_matrix = BorrowedMat {
            data: &test_labels.data,
            height: test_labels.data.len(),
            transposed: false,
        };
        // this should probably be evaluated based on cost,
        // or instead tested on the training batch,
        // it feels cheatey using the training batch during testing
        let correct_count = network.test(&test_images, test_matrix, activation);
        let correct_ratio = correct_count as f64 / height as f64;
        println!(
            "correct count: {}, correct percentage: {}%",
            correct_count,
            correct_ratio * 100.0
        );
        println!();
        if previous_correct_count >= correct_count {
            no_improvement_in_counter += 1;
        } else {
            no_improvement_in_counter = 0;
        }
        if no_improvement_in_counter >= 1 {
            learning_rate = learning_rate / Alg(10.0);
        }
        if learning_rate < initial_learning_rate / Alg(200000.0) || no_improvement_in_counter == 8 {
            break;
        }
        previous_correct_count = correct_count;
    }

    // forward prop on test sample,
    // check for many were correct,
    // report that as a percentage
    //let
    //assert_eq!(test_images.width(), test_labels.data.len());
    if false {
        for _ in 0..100 {
            network.feed_forward(&test_images, activation);
            let (a, _) = network.feed_forward_results();
            let y_hat = a.last().unwrap();
            let mut correct_count = 0;
            for j in 0..y_hat.width() {
                let mut max = y_hat.get_owned(0, j);
                let mut max_index = 0;
                for i in 1..10 {
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
            println!("correct percent: {}", correct_ratio * 100.0)
        }
        //println!("correct percentage: {}%", correct_ratio * 100.0);
        //println!("correct count: {}", correct_count);
    }

    let end = time::Instant::now();
    let diff = end - start;
    println!("time taken: {}m, {}s", diff.as_secs() / 60, diff.as_secs());
    Ok(())
}

//impl Scalar for NotNan<f64> {
//    const ONE: Self = const { unsafe { NotNan::new_unchecked(1.0) } };
//    const ZERO: Self = const { unsafe { NotNan::new_unchecked(0.0) } };
//}
//impl Scalar for NotNan<f32> {
//    const ONE: Self = const { unsafe { NotNan::new_unchecked(1.0) } };
//    const ZERO: Self = const { unsafe { NotNan::new_unchecked(0.0) } };
//}

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
//type const Sub<const A: usize, const B: usize>: usize = const {A - 1};
const SUB<const T: usize, const U: usize>: usize = const { T - U };
#[derive(Debug)]
struct Network<const LAYER_COUNT: usize, T>
//where
//[(); LAYER_COUNT - 1]:,
{
    _layout_heights: [usize; LAYER_COUNT],
    biases: [Box<[T]>; SUB::<LAYER_COUNT, 1>],
    weights: [Matrix<T>; SUB::<LAYER_COUNT, 1>],
    a_buffer: Vec<Matrix<T>>,
    z_buffer: Vec<Matrix<T>>,
}

// layer count includes input layer? idk lol
impl<const LAYER_COUNT: usize, T> Network<LAYER_COUNT, T> {
    pub fn new(
        heights: [usize; LAYER_COUNT],
        mut biases_fn: impl FnMut() -> T,
        mut weights_fn: impl FnMut(usize) -> T,
    ) -> Self {
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
            _layout_heights: heights,
            biases,
            weights,
            a_buffer,
            z_buffer,
        }
    }
    // TODO verify that the logic here is correct
    pub fn feed_forward<'a>(&mut self, input: impl AsMat<T>, activation_fn: impl Fn(T) -> T)
    /*-> (&[Matrix<T>; LAYER_COUNT], &[Matrix<T>; LAYER_COUNT - 1])*/
    /* a, z */
    where
        T: Mul<Output = T> + Add<Output = T> + Clone,
        T: 'a,
        T: Send + Sync,
        T: Debug,
    {
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
            //let z = (&self.weights[i] * layer).add_extended_slice(biases_layer);
            let left = (&self.weights[i] * layer);
            // seems to work
            let z = left.add_extended_slice_self(biases_layer);

            //let z = (&self.weights[i] * layer).add_extended_slice_self(biases_layer);
            layers_z.push(z.clone());
            //let a = z.map(&activation_fn);
            let a = z.map_self(&activation_fn);
            layers_a.push(a);
        }
    }
    pub fn feed_forward_results(
        &self,
    ) -> (
        &[Matrix<T>; LAYER_COUNT],
        &[Matrix<T>; SUB::<LAYER_COUNT, 1>],
    ) {
        let layers_a = &self.a_buffer;
        let layers_z = &self.z_buffer;
        (layers_a.as_array().unwrap(), layers_z.as_array().unwrap())
    }
    //#[allow(non_snake_case)]
    pub fn back_prop_vectorised(
        &self,
        y_hat: impl AsMat<T>,
        z: &[Matrix<T>; SUB::<LAYER_COUNT, 1>],
        a: &[Matrix<T>; SUB::<LAYER_COUNT, 1>], // exptected results
        del_activation_fn: impl Fn(T) -> T,
        _del_cost: impl Fn(Matrix<T>, Matrix<T>, Matrix<T>) -> Matrix<T>,
        _m: T,
    ) -> Vec<Matrix<T>>
    where
        T: Sub<Output = T>,
        T: Add<Output = T>,
        T: Scalar,
        T: Mul<T, Output = T>,
    {
        let mut delta_mat = Vec::with_capacity(LAYER_COUNT - 1);
        let mut weights = Vec::with_capacity(LAYER_COUNT - 1);
        //let mut weight_mat = Vec::with_capacity(LAYER_COUNT - 1);
        //let last_z = z.last().unwrap().clone();
        let y = a.last().unwrap().clone_to_matrix();
        let y_hat = y_hat.clone_to_matrix();
        let y_height = y.height();
        let y_width = y.width();
        let cost_gradient =
            matrix_map_a_b_override_a(y.data, y_hat.data, y_height, y_width, |y, y_hat| -y_hat / y);
        let cost_gradient = Matrix {
            data: cost_gradient,
            height: y_height,
        };
        let mut range = (0..LAYER_COUNT - 1).rev();
        let successors =
            core::iter::successors(range.next().map(|x| (x, cost_gradient)), |del_c_del_a| {
                let i = range.next()?;
                let z = z[i + 1].clone();
                let del_c_del_z = del_c_del_a
                    .1
                    .clone()
                    .hadamard_self(z.map(&del_activation_fn));
                delta_mat.push(del_c_del_z.clone());
                let mut w = self.weights[i].as_mat();
                w.transpose();
                let next = w * del_c_del_z.as_mat();
                Some((i, next))
            });
        for (i, del_c_del_a) in successors {
            let a = a[i].clone();
            let dw = del_c_del_a * a.transpose();
            weights.push(dw);
        }
        delta_mat
    }

    pub fn back_prop(
        &self,
        y_hat: impl AsMat<T>,
        z: &[Matrix<T>; SUB::<LAYER_COUNT, 1>],
        y: impl AsMat<T>,
        del_activation_fn: impl Fn(T) -> T,
        del_cost: impl Fn(Matrix<T>, Matrix<T>, Matrix<T>) -> Matrix<T>,
    ) -> Matrix<Matrix<T>>
    /* more like a matrix of vectors */
    where
        T: Sub<Output = T>,
        T: Add<Output = T>,
        T: Scalar,
        T: Mul<T, Output = T>,
    {
        //assert_eq!(y.width(), y_hat.width());
        let mut delta_mat = Vec::with_capacity(LAYER_COUNT - 1);
        // implemented like this because i dont know how to do it vectorised
        for sample in 0..y.width() {
            let sample_y_hat = y_hat.get_column(sample);
            let sample_y = y.get_column(sample);
            #[allow(non_snake_case)]
            let sample_z_L = z.last().unwrap().get_column(sample);

            let layer_delta = del_cost(sample_z_L, sample_y_hat, sample_y);
            let mut range = (0..LAYER_COUNT - 1).rev();
            let successors = core::iter::successors(range.next().map(|i| (i, layer_delta)), |x| {
                let i = range.next()?;
                //let z = z[i].clone();
                let z = z[i].get_column(sample);
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
                //println!("{}", delta_l.height() * a.width());
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
    fn gradient_descent_vectorised(
        &mut self,
        _learning_rate: T,
        delta: Vec<Matrix<T>>,
        _lambda: T,
        _training_set_size: T,
    ) where
        T: Sub<Output = T>,
        T: Mul<Output = T>,
        T: From<u16>,
        T: Scalar + Clone,
    {
        let _a: &[Matrix<T>; LAYER_COUNT] = self.a_buffer.as_array().unwrap();
        for layer in 0..LAYER_COUNT - 1 {
            let _delta = delta.get(layer).unwrap();
            //let del_c_del_w =
        }
    }
    fn run_epoch(
        &mut self,
        train_data: &[T],
        train_y: &[T],
        // these two are implicitly part of layout_heights
        minibatch_size: usize,
        //input_height: usize,
        //y_height: usize,
        activation_fn: impl Fn(T) -> T,
        del_cost: impl Fn(Matrix<T>, Matrix<T>, Matrix<T>) -> Matrix<T>,
        del_activation_fn: impl Fn(T) -> T,
        learning_rate: T,
        lambda: T,
        training_set_size: T,
    ) where
        T: Clone + Mul<Output = T> + Add<Output = T> + Scalar,
        T: From<u16>,
        T: From<usize>,
        T: Debug,
    {
        let input_height = *self._layout_heights.first().unwrap();
        let y_height = *self._layout_heights.last().unwrap();
        for (train_data, train_y) in train_data
            .chunks(minibatch_size * input_height)
            .zip(train_y.chunks(minibatch_size * y_height))
        {
            // the dimensions of these two seem fine
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
            //let delta = self.back_prop_vectorised(a.last().unwrap(), z, train_y_mat, &del_activation_fn, &del_cost, minibatch_size.into());

            //let delta = self.back_prop(
            //    a.last().unwrap(),
            //    z,
            //    train_y_mat,
            //    &del_activation_fn,
            //    &del_cost,
            //);
            //self.gradient_descent_vectorised(learning_rate, delta, lambda, training_set_size);
            self.gradient_descent(learning_rate, delta, lambda, training_set_size);
        }
    }
    fn test<U>(
        &mut self,
        test_data: impl AsMat<T>,
        test_labels: impl AsMat<U>,
        activation_fn: impl Fn(T) -> T,
    ) -> u32
    where
        T: Clone + Add<Output = T> + Mul<Output = T> + PartialOrd,
        U: Into<usize> + Clone,
        T: Debug,
        T: Send + Sync,
    {
        self.feed_forward(test_data, activation_fn);
        let (a, _) = self.feed_forward_results();
        let y_hat = a.last().unwrap();
        let mut correct_count = 0;
        for j in 0..y_hat.width() {
            let mut max = y_hat.get_owned(0, j);
            let mut max_index = 0;
            for i in 1..10 {
                let x = y_hat.get_owned(i, j);
                if x > max {
                    max = x;
                    max_index = i;
                }
            }
            if max_index == test_labels.as_mat().data[j].clone().into() {
                correct_count += 1;
            }
        }
        correct_count
    }
}

impl<T, U> Mul<Matrix<U>> for Matrix<T>
where
    T: Mul<U> + Sync,
    T: Clone,
    U: Clone + Sync,
    <T as Mul<U>>::Output: Add<Output = <T as Mul<U>>::Output> + Send,
{
    type Output = Matrix<MulOut<T, U>>;
    fn mul(self, rhs: Matrix<U>) -> Self::Output {
        let m = self.height();
        let n = rhs.height();
        let p = rhs.width();
        //let x = matrix_mul(&self.data[..], &rhs.data[..], m, n, p);
        let x = matrix_mul(self.data, rhs.data, m, n, p);
        Matrix { data: x, height: m }
    }
}
impl<'a, 'b, T, U> Mul<BorrowedMat<'b, U>> for BorrowedMat<'a, T>
where
    T: Mul<U> + Sync,
    T: Clone,
    U: Clone + Sync,
    <T as Mul<U>>::Output: Add<Output = <T as Mul<U>>::Output> + Send,
{
    type Output = Matrix<MulOut<T, U>>;
    fn mul(self, rhs: BorrowedMat<'b, U>) -> Self::Output {
        let m = self.height();
        let n = rhs.height();
        let p = rhs.width();
        let x = matrix_mul(self, rhs, m, n, p);
        Matrix { data: x, height: m }
    }
}
impl<T, U> Add<Matrix<U>> for Matrix<T>
where
    T: Add<U> + Clone,
    U: Clone,
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
    T: Clone + Sync,
    U: Clone + Sync,
    <T as Mul<U>>::Output: Add<Output = <T as Mul<U>>::Output> + Send,
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
        if !self.is_transposed() {
            self.as_mat().height
        } else {
            self.as_mat().data.len() / self.as_mat().height
        }
    }
    fn width(&self) -> usize {
        if self.is_transposed() {
            self.as_mat().height
        } else {
            self.as_mat().data.len() / self.as_mat().height
        }
    }
    fn data_height(&self) -> usize {
        self.as_mat().height
    }
    fn data_width(&self) -> usize {
        self.as_mat().data.len() / self.as_mat().height
    }
    fn shape(&self) -> (usize, usize) {
        (self.height(), self.width())
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
        assert!(new_i < self.height());
        assert!(new_j < self.width());
        if self.is_transposed() {
            swap(&mut new_i, &mut new_j);
        }
        self.as_mat().data[new_i * self.data_width() + new_j].clone()
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
        let x = map_extended(self.as_mat(), rhs, m, n, Add::add);
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
        // clone to matrix so the transposition is kept
        let x = map_extended(self.as_mat(), rhs.clone_to_matrix().data, m, n, Add::add);
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
        // this must be done because it might be transposed
        let mut x = Vec::new();
        for i in 0..self.height() {
            for j in 0..self.width() {
                x.push(self.get_owned(i, j));
            }
        }
        Matrix {
            data: x.into(),
            height: self.height(),
        }
    }
}
impl<T> Matrix<T> {
    pub fn add_self(self, rhs: Self) -> Self
    where
        T: Add<Output = T> + Clone,
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
        T: Sub<Output = T> + Clone,
    {
        assert_eq!(self.shape(), rhs.shape());
        let height = self.height();
        let width = self.width();
        Matrix {
            data: matrix_map_a_b_override_a(self.data, rhs.data, height, width, Sub::sub),
            height,
        }
    }
    pub fn add_extended_slice_self<U>(self, rhs: &[U]) -> Matrix<T>
    where
        T: Add<U, Output = T>,
        U: Clone,
    {
        let m = self.height();
        let n = self.width();
        Matrix {
            data: map_extended_self(self.data, rhs, m, n, Add::add),
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
    pub fn map_self(mut self, f: impl Fn(T) -> T) -> Self {
        let self_ptr = self.data.as_ptr();
        for i in 0..self.data.len() {
            let self_val = unsafe { self_ptr.add(i).read() };
            self.data[i] = f(self_val);
        }
        self
    }
    pub fn map<U>(mut self, f: impl Fn(T) -> U) -> Matrix<U> {
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
                // TODO check if this works
                // seems to work
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
        T: Mul<Output = T> + Clone,
    {
        //assert_eq!(self.width(), 1, "hadamard only works on vectors");
        //assert_eq!(rhs.width(), 1, "hadamard only works on vectors");
        //assert_eq!(self.width(), rhs.width())
        //assert_eq!(
        //    self.height(),
        //    rhs.height(),
        //    "both vectors must have the same size for hadamard"
        //);
        assert_eq!(self.data.len(), rhs.data.len());
        let height = self.height();
        let width = self.width();
        Matrix {
            data: matrix_map_a_b_override_a(self.data, rhs.data, height, width, |x, y| x * y),
            height,
        }
    }
    // can move to the trait
    pub fn hadamard<U>(self, rhs: Matrix<U>) -> Matrix<<T as Mul<U>>::Output>
    where
        T: Mul<U> + Clone,
        U: Clone,
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

fn matrix_mul<T, U>(
    lhs: impl WidthGetCloned<T> + Sync,
    rhs: impl WidthGetCloned<U> + Sync,
    m: usize,
    n: usize,
    p: usize,
) -> Box<[MulOut<T, U>]>
where
    T: Mul<U>,
    T: Clone,
    U: Clone,
    MulOut<T, U>: Add<Output = <T as Mul<U>>::Output> + Send,
{
    // c[i][j] = a[i][0] b[0][j] + a[i][1] b[1][j] + ... + a[i][n] b[n][j]
    // n is the width of A
    // (m * n) mul (n * p) = (m * p) matrix
    //if m > 10 {
    //matrix_mul_parallel(lhs, rhs, m, n, p)
    //}
    //else {
    matrix_mul_singlethreaded(lhs, rhs, m, n, p)
    //}
}
fn matrix_mul_parallel<T, U>(
    lhs: impl WidthGetCloned<T> + Sync,
    rhs: impl WidthGetCloned<U> + Sync,
    m: usize,
    n: usize,
    p: usize,
) -> Box<[MulOut<T, U>]>
where
    T: Mul<U>,
    T: Clone,
    U: Clone,
    MulOut<T, U>: Add<Output = <T as Mul<U>>::Output> + Send,
{
    let mut out = Vec::from_fn(m * p, |_| MaybeUninit::uninit());
    let mut chunks = out.chunks_mut(p);
    //println!("before threadpool scope");
    thread::scope(|x| {
        let lhs = &lhs;
        let rhs = &rhs;
        for i in 0..m {
            let chunk = chunks.next().unwrap();
            x.spawn(move || {
                for j in 0..p {
                    let mut sum = lhs.width_get_cloned(i, n, 0) * rhs.width_get_cloned(0, n, j);
                    for k in 1..n {
                        sum = sum + lhs.width_get_cloned(i, n, k) * rhs.width_get_cloned(k, p, j);
                    }
                    chunk[j] = MaybeUninit::new(sum);
                }
            });
        }
    });
    let out: Box<[_]> = out.into();
    unsafe { out.assume_init() }
}
fn matrix_mul_singlethreaded<T, U>(
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
    MulOut<T, U>: Add<Output = <T as Mul<U>>::Output>,
{
    let mut out = Vec::with_capacity(m * p);
    for i in 0..m {
        for j in 0..p {
            let mut sum = lhs.width_get_cloned(i, n, 0) * rhs.width_get_cloned(0, n, j);
            for k in 1..n {
                sum = sum + lhs.width_get_cloned(i, n, k) * rhs.width_get_cloned(k, p, j);
            }
            out.push(sum);
        }
    }
    out.into()
}

fn matrix_map_a_b<T, U, V>(
    lhs: impl WidthGetCloned<T>,
    rhs: impl WidthGetCloned<U>,
    m: usize,
    n: usize,
    f: impl Fn(T, U) -> V,
) -> Box<[V]> {
    let mut out = Vec::with_capacity(n * m);
    for i in 0..m {
        for j in 0..n {
            let left = lhs.width_get_cloned(i, n, j);
            let right = rhs.width_get_cloned(i, n, j);
            out.push(f(left, right));
        }
    }
    out.into()
}
fn matrix_map_a_b_override_a<T, U, Out>(
    mut lhs: Out,
    rhs: impl WidthGetCloned<U>,
    m: usize,
    n: usize,
    f: impl Fn(T, U) -> T,
) -> Out
where
    Out: WidthGetUnsafe<T> + WidthAssign<T>,
{
    for i in 0..m {
        for j in 0..n {
            let left = unsafe { lhs.width_get_unsafe(i, n, j) };
            // SAFETY: idk
            let right = rhs.width_get_cloned(i, n, j);
            lhs.assign(f(left, right), i, n, j);
        }
    }
    lhs
}
fn matrix_sub<T, U>(
    lhs: impl WidthGetCloned<T>,
    rhs: impl WidthGetCloned<U>,
    m: usize,
    n: usize,
) -> Box<[<T as Sub<U>>::Output]>
where
    T: Sub<U>,
{
    matrix_map_a_b(lhs, rhs, m, n, |a, b| a - b)
}
impl<T, U> Sub<Matrix<U>> for Matrix<T>
where
    T: Sub<U> + Clone,
    U: Clone,
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

fn map_extended<T, U, V>(
    lhs: impl WidthGetCloned<T>,
    rhs: impl AsRef<[U]>,
    m: usize,
    n: usize,
    f: impl Fn(T, U) -> V,
) -> Box<[V]>
where
    U: Clone,
{
    let rhs = rhs.as_ref();
    assert!(rhs.len() == m);
    let mut out = Vec::with_capacity(m * n); /* the output matrix */
    for i in 0..m {
        for j in 0..n {
            let left = lhs.width_get_cloned(i, n, j);
            let number = f(left, rhs[i].clone());
            out.push(number);
        }
    }
    out.into()
}

fn map_extended_self<T, U, F, Out>(mut lhs: Out, rhs: &[U], m: usize, n: usize, f: F) -> Out
where
    F: Fn(T, U) -> T,
    U: Clone,
    Out: WidthGetUnsafe<T> + WidthAssign<T>,
{
    for i in 0..m {
        for j in 0..n {
            let lhs_val = unsafe { lhs.width_get_unsafe(i, n, j) };
            lhs.assign(f(lhs_val, rhs[i].clone()), i, n, j);
        }
    }
    lhs
}

//#[derive(PartialEq, PartialOrd, Clone, Copy)]
//#[repr(transparent)]
//struct Nan<T: FloatCore> {
//    inner: NotNan<T>,
//}

//impl<T> Debug for Nan<T>
//where
//    T: Debug + Clone + FloatCore,
//{
//    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//        write!(f, "{:?}", self.inner.clone().into_inner())
//    }
//}

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
    fn transpose(&mut self) {
        self.transposed = !self.transposed
    }
}

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
            transposed: self.transposed,
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
    #[inline(always)]
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
        self.data().get(i * self.data_width() + j).unwrap()
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
where
    T: Clone,
{
    #[inline(always)]
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
        unsafe { ptr.add(i * self.data_width() + j).read() }
    }
}

impl<'a, T> WidthGetCloned<T> for BorrowedMat<'a, T>
where
    T: Clone,
{
    fn width_get_cloned(&self, i: usize, _n: usize, j: usize) -> T {
        self.get_owned(i, j)
    }
}

//impl From<u16> for Nan<f32> {
//    fn from(value: u16) -> Self {
//        not_nan(Into::<f32>::into(value))
//    }
//}
//impl From<usize> for Nan<f32> {
//    fn from(value: usize) -> Self {
//        not_nan(value as f32)
//    }
//}
//impl From<Nan<f32>> for usize {
//    fn from(value: Nan<f32>) -> Self {
//        value.into_inner() as usize
//    }
//}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
struct Alg<T>(T);
impl Alg<f32> {
    fn exp(self) -> Alg<f32> {
        Alg(self.0.exp())
    }
}
impl<T> Display for Alg<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl From<usize> for Alg<f32> {
    fn from(value: usize) -> Self {
        Alg(value as f32)
    }
}
impl From<u16> for Alg<f32> {
    fn from(value: u16) -> Self {
        Alg(value as f32)
    }
}
impl Add for Alg<f32> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Alg(self.0.algebraic_add(rhs.0))
    }
}
impl Sub for Alg<f32> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Alg(self.0.algebraic_sub(rhs.0))
    }
}
impl Mul for Alg<f32> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Alg(self.0.algebraic_mul(rhs.0))
    }
}
impl Div for Alg<f32> {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        Alg(self.0.algebraic_div(rhs.0))
    }
}
impl Neg for Alg<f32> {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Alg(-self.0)
    }
}
impl Scalar for Alg<f32> {
    const ONE: Self = Alg(1.0);
    const ZERO: Self = Alg(0.0);
}

//impl Scalar for Nan<f32> {
//    const ONE: Self = Nan {
//        inner: unsafe { NotNan::new_unchecked(1.0) },
//    };
//    const ZERO: Self = Nan {
//        inner: unsafe { NotNan::new_unchecked(0.0) },
//    };
//}

//impl<T: FloatCore> Mul for Nan<T> {
//    type Output = Nan<<T as Mul>::Output>;
//    fn mul(self, rhs: Self) -> Self::Output {
//        Nan {
//            inner: self.inner * rhs.inner,
//        }
//    }
//}
//impl<T: FloatCore> Add for Nan<T> {
//    type Output = Nan<<T as Add>::Output>;
//    fn add(self, rhs: Self) -> Self::Output {
//        Nan {
//            inner: self.inner + rhs.inner,
//        }
//    }
//}
//impl<T: FloatCore> Sub for Nan<T> {
//    type Output = Nan<<T as Sub>::Output>;
//    fn sub(self, rhs: Self) -> Self::Output {
//        Nan {
//            inner: self.inner - rhs.inner,
//        }
//    }
//}
//impl<T: FloatCore> Div for Nan<T> {
//    type Output = Nan<<T as Div>::Output>;
//    fn div(self, rhs: Self) -> Self::Output {
//        Nan {
//            inner: self.inner / rhs.inner,
//        }
//    }
//}
//impl<T: FloatCore> Neg for Nan<T> {
//    type Output = Nan<<T as Neg>::Output>;
//    fn neg(self) -> Self::Output {
//        Nan { inner: -self.inner }
//    }
//}

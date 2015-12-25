use std::marker::PhantomData;

use num::traits::Num;
use num::traits::ToPrimitive;

use blocks::neural::neuron::*;
#[allow(unused_imports)]
use blocks::neural::testdata::*;


// basic non performant weights good for debugging and comparisons
#[derive(Copy, Clone)]
pub struct DefaultNeuron<T> where T:ActivationFunction<f32,f32> {    _m: PhantomData<T> }

// fixme DefaultNeuron should return usize for integer types.
//enum_primitive crate,
// we will need custom version NeuronNeuronNeuron toprimative which checks ranges and never fails nor construct a option
impl <W:Num + ToPrimitive , N: ActivationFunction<f32,f32>> Neuron<W , f32 > for DefaultNeuron<N>
{
    //type ActivationFunction = N;
    #[inline]
    fn eval(v: &[f32], weights: &[W]) -> f32
    {
        if  v.len() != weights.len()         {
            panic!("weight length not the same as input vector");
        }

        let mut sum = 0f32;
        for (vn , weight) in v.iter().zip(weights.iter()) {
                sum = sum + vn * weight.to_f32().unwrap();
        }
        N::activate(sum)
//unsafe {
//     let a = [0u8, 0u8, 0u8, 0u8];
//
//     let b = mem::transmute::<[u8; 4], u32>(a);
    }
}

impl <W:Num + ToPrimitive , N: ActivationFunction<f32,f32>> Neuron<W , f64 > for DefaultNeuron<N>
//impl <W:Num + ToPrimitive> Neuron<W , f64 > for DefaultNeuron
{
    #[inline]
    fn eval(v: &[f64], weights: &[W]) -> f64
    {
        if  v.len() != weights.len()         {
            panic!("weight length not the same as input vector");
        }

        let mut sum = 0f64;

        for (vn , weight) in v.iter().zip(weights.iter()) {
                sum = sum + vn * weight.to_f64().unwrap();
        }

        N::activate(sum as f32) as f64
    }
}

//floats with byte weights
impl <W:Num + ToPrimitive , N: ActivationFunction<f32,f32>> Neuron<W , u8 > for DefaultNeuron<N>
//impl <W:Num+ ToPrimitive> Neuron<W , u8 > for DefaultNeuron
{
    #[inline]
    fn eval(v: &[u8], weights: &[W]) -> u8
    {
        if  v.len() != weights.len()         {
            panic!("weight length not the same as input vector");
        }

        let mut sum = 0usize;
        for (vn , weight) in v.iter().zip(weights.iter()) {
            let add = weight.to_u8().unwrap() as usize;
            let mult = *vn as usize * add;
            sum = sum + (mult) as usize;
        }
        if sum > 255{ return 255u8; }

        N::activate(sum as f32) as u8
    }
}

impl <W:Num + ToPrimitive , N: ActivationFunction<f32,f32>> Neuron<W , i8 > for DefaultNeuron<N>
{
    #[inline]
    fn eval(v: &[i8], weights: &[W]) -> i8
    {
        if  v.len() != weights.len()         {
            panic!("weight length not the same as input vector");
        }

        let mut sum = 0isize;
        for (vn , weight) in v.iter().zip(weights.iter()) {

             let add = weight.to_i8().unwrap() as isize;
             let mult = *vn as isize * add;
             sum = sum + (mult) as isize;

        }
        if  sum > 128 {
                 println!("sum exceeded {}",  sum);
            return 127i8;
        }
        if  sum < -127 { return -128i8;}
        N::activate(sum as f32) as i8
    }
}



//uper::block::
// #[derive(Copy, Cltion = DefaultNeuron;
//   type BiasNeuron = PositiveOneBiasFunction;
// }
//one)] pub struct DefaultErrorGradient;
//
// impl ErrorGradient for DefaultErrorGradient {
//   #[inline(always)]
//   fn errhidden<A>(act: f64, sum: f64) -> f64 where A : ActivationFunction {
//     A::derivative(act) * sum
//   }
//   #[inline(always)]
//   fn erroutput<A>(exp: f64, act: f64) -> f64 where A : ActivationFunction {
//     A::derivative(act) * (exp - act)
//   }
// }
//
//
// /// Bias function that returns a random weight between -0.5 and 0.5.
// ///
// #[derive(Copy, Clone)] pub struct RandomBiasNeuron;
//,2f32,3f32{
//     let range = Range::new(-0.5f64, 0.5f64);
//     range.ind_sample(&mut thread_rng())
//   }
// }
//
//
// /// Returns -1 for each bias node.
// ///
// #[derive(Copy, Clone)] pub struct NegativeOneBiasFunction;
//
// impl BiasNeuron for NegativeOneBiasFunction {
//   #[inline] fn biasw() -> f64 { -1f64 }
// }
//
//
// /// Returns 1 for each bias node.
// ///
// #[derive(Copy, Clone)] pub struct PositiveOneBiasFunction;
//
// impl BiasNeuron for PositiveOneBiasFunction {
//   #[inline] fn biasw() -> f64 { 1f64 }
// }
//
//
// /// MSE Error function.
// ///
// #[derive(Copy, Clone)] pub struct MSEFunction;
//
// impl ErrorFunction for MSEFunction {
//   fn error<'a, I>(predictions: I, expected: I) -> f64
//     where I : Iterator<Item = &'a f64>
//   {
//     let mut n = 0f64;
//     let sum = predictions
//       .zip(expected)
//       .fold(0f64, |acc, (act, exp)| { n += 1f64; acc + num::pow((act - exp), 2) });
//     (1f64 / n) * sum
//   }
// }
//
//
// /// Cross Entropy error function.
// ///
// #[derive(Copy, Clone)] pub struct CEFunction;
//
// impl ErrorFunction for CEFunction {
//   fn error<'a, I>(predictions: I, expected: I) -> f64
//     where I : Iterator<Item = &'a f64>
//   {
//     let mut n = 0f64;
//     let sum = predictions
//       .zip(expected)
//       .fold(0f64, |acc, (act, exp)| { n += 1f64; acc + act.ln() * exp });
//     -(1f64 / n) * sum
//   }
// }uper::block::

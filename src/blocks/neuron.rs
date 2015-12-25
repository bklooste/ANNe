use num::traits::Num;
use num::{Float};



pub trait ActivationFunction<O : Num>
{
   fn activate(x: O) -> O;
}

pub trait Neuron<W : Num , O: Num >
{
   fn eval(ins: &[O], outs: &[W]) -> O;
}

pub trait NeuralNetParameters<W : Num , O: Num >
{
   type ActivationFunction : ActivationFunction<O>;
   type Neuron : Neuron<W, O>;
}

#[derive(Copy, Clone, RustcEncodable, RustcDecodable)]
pub struct LogisticNeuralNet;

impl ActivationFunction<u8> for LogisticNeuralNet {
  #[inline(always)] fn activate(x: u8) -> u8 { 1  }
}

impl ActivationFunction<f32> for LogisticNeuralNet {
  #[inline(always)] fn activate(x: f32) -> f32 { 1f32 / (1f32 + (-x).exp()) }
}

impl ActivationFunction<f64> for LogisticNeuralNet {
  #[inline(always)] fn activate(x: f64) -> f64 { 1f64 / (1f64 + (-x).exp()) }
}
//
// impl NeuralNetParameters for LogisticNeuralNet {
//   type ActivationFunction = LogisticNeuralNet;
//   type Neuron = DefaultNeuron;
//   //type BiasNeuron = NegativeOneBiasFunction;
// }
//
//
// /// Default Parameters for a Tanh Neural Net.
// ///
// #[derive(Copy, Clone, RustcEncodable, RustcDecodable)]
// pub struct TanhNeuralNet;
//
// impl ActivationFunction for TanhNeuralNet {
//   #[inline(always)] fn activate(x: f64) -> f64 { x.tanh() }
//   #[inline(always)] fn derivative(x: f64) -> f64 { 1f64 - x.tanh().powi(2) }
// }
//
// impl NeuralNetParameters for TanhNeuralNet {
//   type ActivationFunction = TanhNeuralNet;
//   type Neuron = DefaultNeuron;
//   type BiasNeuron = PositiveOneBiasFunction;
// }
//
//

#[derive(Copy, Clone)] pub struct DefaultNeuron;

impl<W : Num , O: Num + Default> Neuron<W , O > for DefaultNeuron
{
    #[inline]
    fn eval(ins: &[O], outs: &[W]) -> O
    {
        if ( ins.len() != outs.let())
            panic!("lenght of input and output slices do not match!");
        let sum = O::default();
        sum
    }
}
//
//
// /// Default Error Gradient functions.
// ///
// #[derive(Copy, Clone)] pub struct DefaultErrorGradient;
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
//
// impl BiasNeuron for RandomBiasNeuron {
//   #[inline]
//   fn biasw() -> f64 {
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
// }

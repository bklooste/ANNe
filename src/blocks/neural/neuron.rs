use num::traits::Num;
use num::{Float};


//sigmoid activation & SIMD weights

// fast logistic

/// Activation Function
///
pub trait ActivationFunction<O : Num>
{
   fn activation(x: O) -> O;
}

pub trait WeightFunction<W : Num , O: Num >
{
   fn calc_weight(ins: &[O], outs: &[W]) -> O;
}

pub trait NeuralNetParameters<W : Num , O: Num >
{
   type ActivationFunction : ActivationFunction<O>;
   type WeightFunction : WeightFunction<W, O>;
}


//common activations
#[derive(Copy, Clone, RustcEncodable, RustcDecodable)]
pub struct LogisticNeuralNet;

impl ActivationFunction<u8> for LogisticNeuralNet {
  #[inline(always)] fn activation(x: u8) -> u8 { 1  }
}

impl ActivationFunction<f32> for LogisticNeuralNet {
  #[inline(always)] fn activation(x: f32) -> f32 { 1f32 / (1f32 + (-x).exp()) }
}

impl ActivationFunction<f64> for LogisticNeuralNet {
  #[inline(always)] fn activation(x: f64) -> f64 { 1f64 / (1f64 + (-x).exp()) }
}

#[derive(Copy, Clone, RustcEncodable, RustcDecodable)]
pub struct TanhNeuralNet;

impl ActivationFunction<u8> for TanhNeuralNet {
  #[inline(always)] fn activation(x: u8) -> u8 { 1  }
}

impl ActivationFunction<f32> for TanhNeuralNet {
  #[inline(always)] fn activation(x: f32) -> f32 { x.tanh() }
}

impl ActivationFunction<f64> for TanhNeuralNet {
  #[inline(always)] fn activation(x: f64) -> f64 { x.tanh() }
  //derivative #[inline(always)] fn derivative(x: f64) -> f64 { 1f64 - x.tanh().powi(2) }
}

pub fn add_foura(a: i32) -> i32 {
    a + 4
}

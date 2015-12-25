use num::traits::Num;

//sigmoid activation & SIMD weights

// fast logistic

/// Activation Function
///
pub trait ActivationFunction<I : Num , O : Num>
{
   fn activation(x: I) -> O;
}

// pub trait WeightFunction<W : Num , O: Num >
// {
//    fn calc_weight(ins: &[O], outs: &[W]) -> O;
// }
//
// pub trait NeuralNetParameters<I : Num , W : Num , O: Num >
// {
//    type ActivationFunction : ActivationFunction<I , O>;
//    type WeightFunction : WeightFunction<W, O>;
// }

pub trait WeightFunction<W : Num , O: Num  >
{
//    type ActivationFunction : ActivationFunction<O>;
    fn calc_weight(data: &[O], weights: &[W]) -> O;
}
// //common activations
// #[derive(Copy, Clone, RustcEncodable, RustcDecodable)]
// pub struct LogisticNeuralNet;
//
// impl ActivationFunction<u8,isize> for LogisticNeuralNet {
//   #[inline(always)] fn activation(x: u8) -> u8 { 1  }
// }
//
// impl ActivationFunction<f32 , f32> for LogisticNeuralNet {
//   #[inline(always)] fn activation(x: f32) -> f32 { 1f32 / (1f32 + (-x).exp()) }
// }
//
// impl ActivationFunction<f64> for LogisticNeuralNet {
//   #[inline(always)] fn activation(x: f64) -> f64 { 1f64 / (1f64 + (-x).exp()) }
// }
//
// #[derive(Copy, Clone, RustcEncodable, RustcDecodable)]
// pub struct TanhNeuralNet;
//
// impl ActivationFunction<u8> for TanhNeuralNet {
//   #[inline(always)] fn activation(x: u8) -> u8 { 1  }
// }
//
// impl ActivationFunction<f32> for TanhNeuralNet {
//   #[inline(always)] fn activation(x: f32) -> f32 { x.tanh() }
// }
//
// impl ActivationFunction<f64> for TanhNeuralNet {
//   #[inline(always)] fn activation(x: f64) -> f64 { x.tanh() }
//   //derivative #[inline(always)] fn derivative(x: f64) -> f64 { 1f64 - x.tanh().powi(2) }
// }

pub fn add_foura(a: i32) -> i32 {
    a + 4
}

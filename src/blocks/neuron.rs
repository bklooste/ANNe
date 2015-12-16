use num::traits::Num;
use num::{Float};


//sigmoid activation & SIMD weights

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

//type f32 neuron
// impl<T: Num + Copy>   Neuron<T> for Sigmoid
// {
//     type Output = T;
//
//     fn calc (weights: &[T] ,  inputs: &[Self::Output] ) -> Self::Output
//     {
//         let r = Self::calculate_sum(weights , inputs);
//         Self::activate(r)
//     }
//
//     fn activate (output : Self::Output )  -> Self::Output { output }
//
//     fn calculate_sum  (weights: &[T] ,  inputs: &[Self::Output] ) -> Self::Output
//     {
//         weights[0] * inputs[0]
//     }
//
//
// }



// impl<'a, T:  Neuron<f32, Output=f32> , B: BlockBehaviour<Output=f32> > Block<B>
// for FullMeshBlock<'a, f32, f32 , T>
// {
//     //type Output = f32;
//
//     fn load_vector(&self , data: &[f32] , port: BlockPort )  { self.load_vector(data); }
//
//     fn process(&self) -> Vec<f32>
//     {
//         let mut vec  =  Vec::<f32>::with_capacity(self.neuron_count as usize);
//         for nc in 0..self.neuron_count as usize
//         {
//             let in_vec_for_neuron : &[f32] = self.get_weights_for_neuron(nc as u32);
//             vec[nc] =  self.behaviour.process(self.weights);
//
//         }
//         vec
//     }
//
// }

// can we use hardware port as offset = 16 , 32 etc
// we can impliment full mesh here


//move to activbation



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
//
// impl NeuralNetParameters for LogisticNeuralNet {
//   type ActivationFunction = LogisticNeuralNet;
//   type WeightFunction = DefaultWeightFunction;
//   //type BiasWeightFunction = NegativeOneBiasFunction;
// }
//
//
// /// Default Parameters for a Tanh Neural Net.
// ///
// #[derive(Copy, Clone, RustcEncodable, RustcDecodable)]
// pub struct TanhNeuralNet;
//
// impl ActivationFunction for TanhNeuralNet {
//   #[inline(always)] fn activation(x: f64) -> f64 { x.tanh() }
//   #[inline(always)] fn derivative(x: f64) -> f64 { 1f64 - x.tanh().powi(2) }
// }
//
// impl NeuralNetParameters for TanhNeuralNet {
//   type ActivationFunction = TanhNeuralNet;
//   type WeightFunction = DefaultWeightFunction;
//   type BiasWeightFunction = PositiveOneBiasFunction;
// }
//
//

#[derive(Copy, Clone)] pub struct DefaultWeightFunction;

impl<W : Num , O: Num + Default> WeightFunction<W , O > for DefaultWeightFunction
{
    #[inline]
    fn calc_weight(ins: &[O], outs: &[W]) -> O
    {
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
// #[derive(Copy, Clone)] pub struct RandomBiasWeightFunction;
//
// impl BiasWeightFunction for RandomBiasWeightFunction {
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
// impl BiasWeightFunction for NegativeOneBiasFunction {
//   #[inline] fn biasw() -> f64 { -1f64 }
// }
//
//
// /// Returns 1 for each bias node.
// ///
// #[derive(Copy, Clone)] pub struct PositiveOneBiasFunction;
//
// impl BiasWeightFunction for PositiveOneBiasFunction {
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

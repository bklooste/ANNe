

use num::traits::Num;
use num::traits::One;

// use num::Float;
// use prelude::*;
// use rand::thread_rng;
// use rand::distributions::IndependentSample;
// use rand::distributions::range::Range;

use super::neuron::*;




/// Activation Function
///
pub trait TrainingActivationFunction<O : Num , I : Num> : ActivationFunction<O, I>{
   //fn activate(x: O) -> O;
   fn derivative(x: O) -> O;
}

/// The weight function to generate the initial weights.
///
pub trait GenerateNeuron<W : Num > {
   fn initw(ins: usize, outs: usize) -> W;
}



/// The weight function to generate the bias nodes' weights.
///
pub trait GenerateBiasNeuron<W : Num> {
   fn biasw() -> W;
}

pub trait TrainNetParameters<W : Num , O: Num > {
   type TrainingActivationFunction : TrainingActivationFunction<O , O>;
   type GenerateNeuron : GenerateNeuron<W>;
   type GenerateBiasNeuron : GenerateBiasNeuron<W>;
}


#[derive(Copy, Clone)]
pub struct DefaultGenerateNeuron;

impl <O:Num + One> GenerateNeuron<O> for DefaultGenerateNeuron {

  #[inline]
  fn initw(_ins: usize, _: usize) -> O {
    // let lb =   - O::one()  / (ins ).sqrt();
    // let ub =  O::one()/ (ins ).sqrt();
    // let range = Range::new(lb, ub);
    //
    // range.ind_sample(&mut thread_rng())
    O::one()
  }
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

//
//
// /// Default Parameters for a Logistic Neural Net.
// ///
// #[derive(Copy, Clone, RustcEncodable, RustcDecodable)]
// pub struct LogisticNeuralNet;
//
// impl ActivationFunction for LogisticNeuralNet {
//   #[inline(always)] fn activate(x: f64) -> f64 { 1f64 / (1f64 + (-x).exp()) }
//   #[inline(always)] fn derivative(x: f64) -> f64 { x * (1f64 - x) }
// }
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
// /// Default weight function that is dependent on the input size.
// ///
// #[derive(Copy, Clone)] pub struct DefaultNeuron;
//
// impl Neuron for DefaultNeuron {
//   #[inline]
//   fn initw(ins: usize, _: usize) -> f64 {
//     let lb = -1f64 / (ins as f64).sqrt();
//     let ub =  1f64 / (ins as f64).sqrt();
//     let range = Range::new(lb, ub);
//
//     range.ind_sample(&mut thread_rng())
//   }
// }
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

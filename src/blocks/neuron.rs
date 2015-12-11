extern crate num;

use self::num::traits::Num;
use core::*;

//sigmoid activation & SIMD weights

pub struct Sigmoid;


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

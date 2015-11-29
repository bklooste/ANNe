extern crate num;
use self::num::traits::Num;

// check following ..
// https://github.com/ferristseng/rust-nnet/blob/master/rust-nnet/src/prelude.rs

pub type BlockId = u32;
pub type BlockPort = u32;
pub type NeuronNum = u32;

#[derive(Default , Clone)]
pub struct ConnectionDestination
{
        destination : BlockId,
        port : i32
}

#[derive( Clone)]
pub enum Connection
{
    Connector { destination: ConnectionDestination},
    Loom { destination: ConnectionDestination , size:u32 },
    Mesh { destination: ConnectionDestination , interval:u32 , size:u32 },
    RandomMesh { destination: ConnectionDestination , intervalrate:f32 , size:u32 },
    FullMesh { destination: ConnectionDestination , size:u32},
    Output { destination: ConnectionDestination}
}

// should make activate a seperate trait ?
pub trait Neuron<W: Num>
{
    type Output: Num;

    fn calc (weights: &[W] ,  inputs: &[Self::Output] ) -> Self::Output ;
    fn activate (output : Self::Output )  -> Self::Output ;
    fn calculate_sum  (weights: &[W] ,  inputs: &[Self::Output] ) -> Self::Output ;
}
    // pub trait ActivationFunction {
    //   #[allow(missing_docs)] fn activation(x: f64) -> f64;
    //   #[allow(missing_docs)] fn derivative(x: f64) -> f64;
    // }

// pub trait WeightFunction {
//   #[allow(missing_docs)] fn initw(ins: usize, outs: usize) -> f64;
// }

//manages input data

pub trait BlockBehaviour
{
    type Output: Num;

    fn save_input(&self , data: &[Self::Output] , port: BlockPort  );
    fn get_input_for_neuron (&self  , neuron_num : u32 ) -> &[Self::Output];
}

// so a block is defined over its generic neuronbehaviour
// how does it know how to handle full mesh or not ?
// in this case the base type has it ..
// which means the neuron is just the weights calculation and activation behaviour ..
// eg  sigmoid activation & SIMD weights over full mesh
pub trait Block<W: Num , T: Neuron< W>> : BlockBehaviour
{
    fn process(&self) -> Vec<Self::Output>;
    //fn save_vector(&self , data: &[Self::Output] , port: BlockPort );
}

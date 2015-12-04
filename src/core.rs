extern crate num;
use self::num::traits::Num;



// fixme remove


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
    //fn derivative
    fn calculate_sum  (weights: &[W] ,  inputs: &[Self::Output] ) -> Self::Output ;
}

pub trait BlockBehaviour
{
    type Output: Num;

    fn save_input(&self , data: &[Self::Output] , port: BlockPort  );
//    fn get_input_for_neuron (&self  , neuron_num : u32 ) -> &[Self::Output];
}

pub trait NeuronBlockBehaviour : BlockBehaviour
{
//    type Output: Num;
    fn get_input_for_neuron (&self  , neuron_num : u32 ) -> &[Self::Output];
}

// so a block is defined over its generic neuronbehaviour
// how does it know how to handle full mesh or not ?
// in this case the base type has it ..
// which means the neuron is just the weights calculation and activation behaviour ..
// eg  sigmoid activation & SIMD weights over full mesh
pub trait Block<W: Num , T: Neuron< W>> : BlockBehaviour
{
    // this is the key
    // ideas
           //allow the output ti be written in a swappable buffer eg toggle in and outputs
           // we can add a get byte[] but may not be needed in rustc
           // we save inputs and then process output a 2 phase step.. needed for multiple inputs.
           // I think this code should be change to work with references since the module
           // can own all data and know the maximum this is a key concept that must be retained.
                // however this can be hard with non linear mappings eg a random input to neuron map.
                // this must be solved and their are a number of ways including special blocks eg randomizer ,junction
           // note the design shows the type
           // having code work with multiple types of inputs is invaluable for code



    fn process(&self) -> Vec<Self::Output>;
    //fn save_vector(&self , data: &[Self::Output] , port: BlockPort );
}

// helpers for most common weights and outputs f32 floats very common
pub trait Blockf32<N:Neuron<f32 , Output = f32>>  : Block<f32 , N  , Output = f32> {}
pub trait Blockf64<N:Neuron<f64 , Output = f64>>  : Block<f64 , N  , Output = f64> {}
pub trait Blocki8<N:Neuron<i8 , Output = u8>>  : Block<i8 , N  , Output = u8> {}

//todo !
impl<N: Neuron<f32 , Output = f32>> Blockf32<N> for Block<f32,N , Output = f32> {}



//type SBlock<W, O, Neuron> = Block<W ,  Neuron< W , Output = O>  ,  Output = O >;
//type BlockF32<Neuron>  =  Block<f32  ,  Neuron< f32 , Output = f32> , Output = f32>;
//Block<W: Num , T: Neuron< W>> : BlockBehaviour

use num::traits::Num;

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
    IntervalMesh { destination: ConnectionDestination , interval:u32 , size:u32 },
    RandomMesh { destination: ConnectionDestination , intervalrate:f32 , size:u32 },
    FullMesh { destination: ConnectionDestination , size:u32},
    Output { destination: ConnectionDestination}
}

// should make activate a seperate trait ?
// pub trait Neuron<W: Num>
// {
//     type Output: Num;
//
//     fn calc (weights: &[W] ,  inputs: &[Self::Output] ) -> Self::Output ;
//     fn activate (output : Self::Output )  -> Self::Output ;
//     //fn derivative
//     fn calculate_sum  (weights: &[W] ,  inputs: &[Self::Output] ) -> Self::Output ;
// }

// try this if it doesnt work then we can use an enum for diffirent node types

pub trait BufferManager <O: Num>
{
    // get weights
    // gets inputs.
}


    //fn process(& mut self , weights: & 'a [W] , inputs: & 'a [& 'a [O]] , outputs: & 'a mut [O]);

pub trait NeuronBlock < O: Num  ,  W: Num> : Block
{
    fn process_input(& self , weights: & [W] , inputs: & [O] , outputs: & mut [O]);
}


pub trait BlockBehaviour <'a, O: Num + 'a ,  W: Num + 'a>
{

    fn set_buffers(& mut self , weights: & 'a [W] , inputs: & 'a [& 'a [O]] , outputs: & 'a mut [O]);

    // fn set_mod_buffers(& mut self , weights: & 'a [u8] , inputs: & 'a mut [& 'a [u8]] , outputs: & 'a mut [u8])
    // {
    //     // manifest a slice out of thin air!
    //     let ptr = 0x1234 as *const usize;
    //     let amt = 10;
    //     //set_buffers
    //     unsafe{
    //         use std::slice;
    //
    //
    //
    //             let slice = slice::from_raw_parts(ptr, amt);
    //
    //         let weight: & 'a [W] = slice::from_raw_parts( weights.as_ptr(), weights.len()/ mem::size_of::<W>());
    //     }
    // }
}

//     use std::slice;
//
// // manifest a slice out of thin air!
// let ptr = 0x1234 as *const usize;
// let amt = 10;
// unsafe {
//     let slice = slice::from_raw_parts(ptr, amt);
// }

    // fn get_weights_size()-> usize
    // {
    //     mem::size_of::<W>()
    //     //sizeof(W);
    // }
    //
    // fn get_input_size()-> usize
    // {
    //     mem::size_of::<O>()
    // }

    // this would seem a lot better with module managing it,
    //fn process(& mut self , weights: & 'a [W] , inputs: & 'a [& 'a [O]] , outputs: & 'a mut [O]);
//}

//fixme rename to block

pub trait IBlock
{
    // fn as_blocktype(&self) -> BlockType ;
    fn get_id(&self) -> BlockId;
}


// not sure if inputs should be mutable , the buffer may be mutable but not for this function
pub trait Block :IBlock
{
    fn process(&self , data: & [u8] , inputs: & [u8] , outputs: & mut [u8]) ; // or return slice
}




pub trait MutBlock :IBlock
{
    fn process(&mut self , mut_data: & mut  [u8] , inputs: & [u8] , outputs: & mut [u8]) ; // or return slice
}


pub trait FunctionBlock :IBlock
{
    fn process(&self , inputs: & [u8] , outputs: & mut [u8]) ; // or return slice
}




pub enum BlockType<'a>  {
    MutBlock(& 'a mut MutBlock),
    Block (& 'a Block),
    FunctionBlock (& 'a FunctionBlock )
}



// for external calls we will move the paramaters in
// pub trait PureBlock< O : Num>
// {
//     // pure block , only has local state ( though may use buffer manager to change that)
//     fn process(&[ &[O] ]) -> Vec<O>;
// }


// // so a block is defined over its generic neuronbehaviour
// // how does it know how to handle full mesh or not ?
// // in this case the base type has it ..
// // which means the neuron is just the weights calculation and activate behaviour ..
// // eg  sigmoid activate & SIMD weights over full mesh
// pub trait Block<W: Num , T: Neuron< W>> : NeuronBlockBehaviour<W>
// {
//     // this is the key
//     // ideas
//            //allow the output ti be written in a swappable buffer eg toggle in and outputs
//            // we can add a get byte[] but may not be needed in rustc
//            // we save inputs and then process output a 2 phase step.. needed for multiple inputs.
//            // I think this code should be change to work with references since the module
//            // can own all data and know the maximum this is a key concept that must be retained.
//                 // however this can be hard with non linear mappings eg a random input to neuron map.
//                 // this must be solved and their are a number of ways including special blocks eg randomizer ,junction
//            // note the design shows the type
//            // having code work with multiple types of inputs is invaluable for code
//
//     fn process_neuron(&self) -> Vec<Self::Output>;
//     //fn save_vector(&self , data: &[Self::Output] , port: BlockPort );
// }



// helpers for most common weights and outputs f32 floats very common
// pub trait Blockf32<N:Neuron<f32 , Output = f32>>  : Block<f32 , N  , Output = f32> {}
// pub trait Blockf64<N:Neuron<f64 , Output = f64>>  : Block<f64 , N  , Output = f64> {}
// pub trait Blocki8<N:Neuron<i8 , Output = u8>>  : Block<i8 , N  , Output = u8> {}
//
// //todo !
// impl<N: Neuron<f32 , Output = f32>> Blockf32<N> for Block<f32,N , Output = f32> {}



//type SBlock<W, O, Neuron> = Block<W ,  Neuron< W , Output = O>  ,  Output = O >;
//type BlockF32<Neuron>  =  Block<f32  ,  Neuron< f32 , Output = f32> , Output = f32>;
//Block<W: Num , T: Neuron< W>> : BlockBehaviour

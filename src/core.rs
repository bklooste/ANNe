use num::traits::Num;
//use core::marker::Sized;
//
// use std::mem;
// use std::slice;


pub type BlockIndex = u32;
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

// replaces base num
pub trait Numb {}







pub trait MutableBlock <O: Num  ,W: Num >
{

    // later we can pass in a buffer via pointer.
    fn add_data<'a>(& mut self , weights: & 'a [W] , inputs: & 'a [O]);
    fn set_buffers(& mut self , weights: Vec<W> , inputs: Vec<O> , outputs: Vec<O>);
    fn get_output(& self) -> Vec<O>;

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


//fixme rename to block
#[derive(Debug)]
pub enum BlockBehaviour
{
    Unknown,
    Immutable ,
    Mutable { copy_out: bool}
}

// this will be created with module passing in modulebuffer,
pub trait ErrorInfo
{
    fn support_back_prop(&self) ->  bool;

    fn get_buffers(&self);
//    fn get_derivative(&self) -> BlockId;

    //caller stores , gets rought  errors , than applies detivative iun bulk to set network errors.
    // can then set buffer.
    // pass in slice ?
    // should have access to weights via module buffer or internal
    // errors should be floats regardless
    fn get_error(&self , next_layer_errors: &[f32] ) -> Box<[f32]>;


    //how to set weights? or expose mutable weights
    fn add_weights_update(&self , update_weights: Vec<&[f32]>) ;

}

// we can probably design this blcok behaviour and process better
pub trait IBlock
{
//    fn as_blocktype(&self) -> Box<BlockType> ;
    fn behaviour(&self) -> BlockBehaviour;
    fn get_id(&self) -> BlockId;
    fn process(&self , data: & mut [u8] , inputs: &[ & [u8]] , outputs: & mut [u8]) ;  // or an array ????
    fn process_self_copy_output(& mut self) -> Vec<u8> ;
    fn get_prop_info(& self) -> Option<Box<ErrorInfo>>;
    // GetBackProOption ? yes but module needs to wrap to get buffers
    //
    // when creating past in buffer manager,
}

 //buffers: [& mut [u8] ;3])

pub trait IntoBox<A: ?Sized>
 {
    /// Convert self into the appropriate boxed form.
    fn into_box(self) -> Box<A>;
}




pub trait NeuronBlock <  W: Num , O: Num  >
{
    fn getid(&self) -> BlockId ; // clumsy does not belong but need iblock
    fn process_input(& self , weights: & [W] , inputs: & [O] , outputs: & mut [O]);
}


// impl <T , W:Num  , O:Num >  IBlock for T
// where T:NeuronBlock<W,O> //<W,O>
// {
//     fn getid(&self) -> i8 { self.get_id() }
// }

// interface for T
// impl <T >  IBlock for T
// where T:NeuronBlock<f32,f32>
// {
//     fn behaviour(&self) -> BlockBehaviour  { BlockBehaviour::Immutable }
//
// // not needed ?
//     fn get_id(&self) -> BlockId { self.getid() }
//
//     fn process(&self , data: & mut [u8] , inputs: &[& [u8]] , outputs: & mut [u8])
//     {
//          if outputs.len() == 0 { println!("warning 0 length output " );}
//         unsafe
//         {
//             let weight_size = mem::size_of::<f32>();
//             let weights: & [f32] = slice::from_raw_parts( data.as_ptr() as *const f32, data.len()/ weight_size);
//             let input_o = inputs [0];
//             let input: & [f32] = slice::from_raw_parts( input_o.as_ptr() as *const f32, input_o.len()/ mem::size_of::<f32>());
//             let outputs: & mut [f32] = slice::from_raw_parts_mut( outputs.as_ptr() as *mut f32, outputs.len()/ mem::size_of::<f32>());
//
//             println!("process , triplet w{:?} : i{:?} : o{:?}",weights , input , outputs );
//             self.process_input( weights , input , outputs);
//             println!("process new O {:?}", outputs );
//         }
//     }
//
//
//     fn process_self_copy_output(& mut self) ->  Vec<u8>
//     {
//             panic!("no buffers setup or NeuronBlocks cant be  mutable ");
//     }
//
// }


// impl <O,W, T >  IBlock for T<O,W>
// where T:NeuronBlock<O,W>
// {
//     fn behaviour(&self) -> BlockBehaviour  { BlockBehaviour::Immutable }
//
// // not needed ?
//     fn get_id(&self) -> BlockId { self.getid() }
//
//     fn process(&self , data: & mut [u8] , inputs: &[& [u8]] , outputs: & mut [u8])
//     {
//          if outputs.len() == 0 { println!("warning 0 length output " );}
//         unsafe
//         {
//             let weight_size = mem::size_of::<W>();
//             let weights: & [W] = slice::from_raw_parts( data.as_ptr() as *const W, data.len()/ weight_size);
//             let input_o = inputs [0];
//             let input: & [O] = slice::from_raw_parts( input_o.as_ptr() as *const O, input_o.len()/ mem::size_of::<O>());
//             let outputs: & mut [O] = slice::from_raw_parts_mut( outputs.as_ptr() as *mut O, outputs.len()/ mem::size_of::<O>());
//
//             println!("process , triplet w{:?} : i{:?} : o{:?}",weights , input , outputs );
//             self.process_input( weights , input , outputs);
//             println!("process new O {:?}", outputs );
//         }
//     }
//
//
//     fn process_self_copy_output(& mut self) ->  Vec<u8>
//     {
//             panic!("no buffers setup or NeuronBlocks cant be  mutable ");
//     }
//
// }

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



// // not sure if inputs should be mutable , the buffer may be mutable but not for this function

//
//
//
// pub trait MutBlock :IBlock
// {
//     fn process(&mut self , mut_data: & mut  [u8] , inputs: & [u8] , outputs: & mut [u8]) ; // or return slice
// }
//
//
// pub trait FunctionBlock :IBlock
// {
//     fn process(&self , inputs: & [u8] , outputs: & mut [u8]) ; // or return slice
// }
//
//
//
// //#[derive(Debug, Copy, Clone)]
// pub enum BlockType<'a>  {
//     MutBlock(Box<MutBlock + 'a>),
//     Block (Box<Block  + 'a>),
//     FunctionBlock (Box<FunctionBlock  + 'a> )
// }



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

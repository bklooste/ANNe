use std::marker::PhantomData;
use std::fmt::Debug;
//use std::{mem ,slice};

use num::traits::Num;

use core::*;

use super::BlockData;

// fn twice<W, O>(x: i32) -> i32 {
//     x + x
// }

// use FN
pub struct FunctionBlock<W, O>
where W: Num   , O: Num
{
    block: BlockData,
    w: PhantomData<W>,
    o: PhantomData<O>,
    //neural_behaviour: PhantomData<N>,
}


impl<W,O>  FunctionBlock<W,O>
where W: Num  , O: Num
{

    pub fn new (newid: BlockId , ncount: u32 , scount: u32) -> FunctionBlock<  W , O >
    {
        let block_data = BlockData { id : newid , neuron_count: ncount , synapse_count: scount , ..Default::default() } ;
        FunctionBlock::new_b(block_data )
    }

    pub fn new_b(block_data: BlockData )  -> FunctionBlock<  W , O >
    {
        if block_data.neuron_count == 0 || block_data.synapse_count == 0 {  panic!("neuron or synapse_count cannot be 0"); };
        FunctionBlock { block : block_data ,    w: ::std::marker::PhantomData , o: ::std::marker::PhantomData     }
    }
}







impl<W  ,O > IBlock for FunctionBlock< W ,O >
where W: Num + Debug  , O: Num + Debug
//where T:NeuronBlock<f32,f32>
{
    fn behaviour(&self) -> BlockBehaviour  { BlockBehaviour::Immutable }

// not needed ?
    fn get_id(&self) -> BlockId { self.block.id }

    fn process(&self , _data: & mut [u8] , _inputs: &[& [u8]] , outputs: & mut [u8])
    {
         if outputs.len() == 0 { println!("warning 0 length output " );}
        // unsafe
        // {
        //     let weight_size = mem::size_of::<W>();
        //     let weights: & [W] = slice::from_raw_parts( data.as_ptr() as *const W, data.len()/ weight_size);
        //     let input_o = inputs [0];
        //     let input: & [O] = slice::from_raw_parts( input_o.as_ptr() as *const O, input_o.len()/ mem::size_of::<O>());
        //     let outputs: & mut [O] = slice::from_raw_parts_mut( outputs.as_ptr() as *mut O, outputs.len()/ mem::size_of::<O>());
        //
        //     self.process_input( weights , input , outputs);
        //
        // }
    }

    fn process_self_copy_output(& mut self) ->  Vec<u8>
    {
            panic!("no buffers setup or NeuronBlocks cant be  mutable ");
    }

}



// full mesh checks
#[test]
fn functionblock_create_bloc ()
{
    let _block  =  FunctionBlock::<f32,f32>::new(5 , 5, 5);
}




//

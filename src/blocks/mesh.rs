
use std::marker::PhantomData;
use std::fmt::Debug;
use std::{mem ,slice};

use num::traits::Num;

use core::*;
use super::neural::neuron::*;
#[allow(unused_imports)]
use blocks::neural::defaultweight::DefaultNeuron;
#[allow(unused_imports)]
use blocks::neural::activation::Logistic;

use super::BlockData;

//test
// dont enhance it build new ones this is a basic impl.
//#[derive(Default)]
pub struct MeshBlock<W, O, N>
where W: Num   , O: Num   , N: Neuron <W,O >
{
    block: BlockData,
    w: PhantomData<W>,
    o: PhantomData<O>,
    neural_behaviour: PhantomData<N>,
}


impl<W,O,N>  MeshBlock<W,O,N>
where W: Num  , O: Num   , N: Neuron <W,O>
{

    pub fn new (newid: BlockId , ncount: u32 , scount: u32) -> MeshBlock<  W , O , N>
    {
        let block_data = BlockData { id : newid , neuron_count: ncount , synapse_count: scount , ..Default::default() } ;
        MeshBlock::new_b(block_data )
    }

    pub fn new_b(block_data: BlockData )  -> MeshBlock<  W , O , N>
    {
        if block_data.neuron_count == 0 || block_data.synapse_count == 0 {  panic!("neuron or synapse_count cannot be 0"); };
        MeshBlock { block : block_data , neural_behaviour:  ::std::marker::PhantomData ,   w: ::std::marker::PhantomData , o: ::std::marker::PhantomData     }
    }
}


// impl< W  ,O ,N>  IBlock  for MeshBlock< W ,O ,N>
// where W:  Num + Debug  , O:  Num  +Debug, N: Neuron <W,O>
// {
//     //        fn as_blocktype (&self) -> BlockType {   BlockType::Block( self)  }
//     fn get_id(&self) -> BlockId { self.block.id  }
//
//     fn process(&self , data: &  [u8] , inputs: & [u8] , output_u8: & mut [u8])
//     {
//
//         println!("starting process buffer");
//         println!("{:?}", self.block.synapse_count  );
//
//         unsafe
//         {
//             let weight_size = mem::size_of::<W>();
//             let weights: & [W] = slice::from_raw_parts( data.as_ptr() as *const W, data.len()/ weight_size);
//             let inputs: & [O] = slice::from_raw_parts( inputs.as_ptr() as *const O, inputs.len()/ mem::size_of::<O>());
//             let outputs: & mut [O] = slice::from_raw_parts_mut( output_u8.as_ptr() as *mut O, output_u8.len()/ mem::size_of::<O>());
//
//             if  (self.block.synapse_count * self.block.neuron_count) as usize != weights.len()  {
//                 panic!("weights does not equal synapse * neurons")
//             }
//
//             self.process_input( weights , inputs , outputs);
//
//             //
//             // // could use a pair itterator this seems fragile
//             // for weights_for_neuron in weights.chunks( self.block.synapse_count as usize )
//             // {
//             //
//             //         println!("weights_for_neuron {:?}", weights_for_neuron );
//             //
//             //
//             //     // for nc in 0..self.block.neuron_count as usize
//             //     // {
//             //         let activated:O =  { N::eval( inputs ,   weights_for_neuron  )};
//             //         outputs[nc] = activated;
//             //         println!("O {:?}", outputs );
//             //
//             //     //}
//             //     nc = nc + 1;
//             // }
//             println!("O {:?}", outputs );
//         }
//     }// unsafe
// }





impl<W  ,O ,N>  NeuronBlock<W , O>  for MeshBlock< W ,O ,N>
where W: Num + Debug  , O: Num + Debug  , N: Neuron <W,O>
{
    fn getid(&self) -> BlockId {  self.block.id}
    fn process_input(& self , weights: & [W] , inputs: & [O] , outputs: & mut [O])
    {
        let mut nc = 0;
        // could use a pair itterator this seems fragile
        for weights_for_neuron in weights.chunks( self.block.synapse_count as usize )
        {

            println!("weights_for_neuron {:?}", weights_for_neuron );

            let activated:O =  { N::eval( inputs ,   weights_for_neuron  )};
            outputs[nc] = activated;
            println!("O {:?}", outputs );
            nc = nc + 1;
        }
    }
}

impl<W  ,O ,N> IBlock for MeshBlock< W ,O ,N>
where W: Num + Debug  , O: Num + Debug  , N: Neuron <W,O>
//where T:NeuronBlock<f32,f32>
{
    fn behaviour(&self) -> BlockBehaviour  { BlockBehaviour::Immutable }

// not needed ?
    fn get_id(&self) -> BlockId { self.getid() }

    fn process(&self , data: & mut [u8] , inputs: &[& [u8]] , outputs: & mut [u8])
    {
         if outputs.len() == 0 { println!("warning 0 length output " );}
        unsafe
        {
            let weight_size = mem::size_of::<W>();
            let weights: & [W] = slice::from_raw_parts( data.as_ptr() as *const W, data.len()/ weight_size);
            let input_o = inputs [0];
            let input: & [O] = slice::from_raw_parts( input_o.as_ptr() as *const O, input_o.len()/ mem::size_of::<O>());
            let outputs: & mut [O] = slice::from_raw_parts_mut( outputs.as_ptr() as *mut O, outputs.len()/ mem::size_of::<O>());

            println!("process , triplet w{:?} : i{:?} : o{:?}",weights , input , outputs );
            self.process_input( weights , input , outputs);
            println!("process new O {:?}", outputs );
        }
    }

    fn process_self_copy_output(& mut self) ->  Vec<u8>
    {
            panic!("no buffers setup or NeuronBlocks cant be  mutable ");
    }

        fn get_prop_info(& self) -> Option<Box<ErrorInfo>> { None}

}


// impl<W, O, N>  NeuronBlockBehaviour <W, O, N>  for MeshBlock<W, O, N>
// where W: Num + 'a , O: Num +'a , N: Neuron <W,O>
// {
//     // full mesh returns all inputs for every neuron
//     fn get_input_for_neuron (&self  , _neuron_num : u32 ) -> &[O] { self.inputs }
//     fn get_weights_for_neuron (&self  , neuron_num : u32 ) -> &[W] { self.weights_for_neuron(neuron_num)}
// }


pub fn add_four(a: i32) -> i32 {
    a + 4
}


// full mesh checks
#[test]
fn fullmesh_create_fullmesh_bloc ()
{
    let _block  =  MeshBlock::<f32,f32,DefaultNeuron<f32,f32,Logistic>>::new(5 , 5, 5);
}




//

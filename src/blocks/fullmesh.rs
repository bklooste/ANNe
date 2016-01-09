
use std::marker::PhantomData;
use std::fmt::Debug;
use std::cell::{RefCell};
use std::borrow::BorrowMut;


use num::traits::Num;

use core::*;
use super::neural::neuron::*;
#[allow(unused_imports)]
use blocks::neural::defaultweight::DefaultNeuron;
#[allow(unused_imports)]
use blocks::neural::activation::Logistic;

use super::BlockData;


// dont enhance it build new ones this is a basic impl.
//#[derive(Default)]
pub struct FullMeshBlock< W, O, N>
where W: Num, O: Num, N: Neuron <W,O>
{
    weights: Vec<W>,
    inputs: Vec<O>,
    outputs: RefCell<Vec<O>>,

    block: BlockData,
    neural_behaviour: PhantomData<N>,
}


impl<W,O,N>  FullMeshBlock<W,O,N>
where W: Num  +Debug +Copy, O: Num  +Debug +Copy  , N: Neuron <W,O>
{
    pub fn new_late(block_data: BlockData )  -> FullMeshBlock<  W , O , N>
    {
        let mut output = Vec::with_capacity(block_data.neuron_count as usize);
        unsafe { output.set_len(block_data.neuron_count as usize); }

        if block_data.neuron_count == 0 || block_data.synapse_count == 0 {  panic!("neuron or synapse_count cannot be 0"); };
        FullMeshBlock { block : block_data , weights:  Vec::new(),  outputs: RefCell::new(  output) ,inputs: Vec::new() , neural_behaviour:  ::std::marker::PhantomData   }
    }

    pub fn new_vec(block_data: BlockData , all_weights: Vec<W> , output_buf: Vec<O>, input_buf: Vec<O>)  -> FullMeshBlock<  W , O , N>
    {
        if block_data.neuron_count == 0 || block_data.synapse_count == 0 {  panic!("neuron or synapse_count cannot be 0"); };
        if block_data.neuron_count as usize != output_buf.len()  {  panic!("output must be the same size as neuron count"); };

        FullMeshBlock { block : block_data , weights: all_weights ,  outputs: RefCell::new(output_buf) ,inputs: input_buf  , neural_behaviour:  ::std::marker::PhantomData   }
    }

    pub fn new<'a>(block_data: BlockData , all_weights: & 'a [W] , output_buf: & 'a [O], input_buf: & 'a [O])  -> FullMeshBlock<  W , O , N>
    {
        if block_data.neuron_count == 0 || block_data.synapse_count == 0 {  panic!("neuron or synapse_count cannot be 0"); };
        if block_data.neuron_count as usize != output_buf.len()  {  panic!("output must be the same size as neuron count"); };

        FullMeshBlock { block : block_data , weights: all_weights.to_vec()  ,  outputs: RefCell::new(output_buf.to_vec() ) ,inputs: input_buf.to_vec()  , neural_behaviour:  ::std::marker::PhantomData   }
    }


    pub fn process_buffers(& mut self )
    {
        //process_block(self);
        let mut output = self.outputs.borrow_mut();
        let mut nc = 0;
        // could use a pair itterator this seems fragile
        for weights_for_neuron in self.weights.chunks( self.block.synapse_count as usize )
        {

            println!("weights_for_neuron {:?}", weights_for_neuron );

            let activated:O =  { N::eval( & self.inputs[..] ,   weights_for_neuron  )};
            output[nc] = activated;
            println!("O {:?}", self.outputs );
            nc = nc + 1;
        }
    }


} //impl


pub fn process_block<W,O,N> (block: & mut FullMeshBlock<W,O,N> )
where W: Num  +Debug +Copy, O: Num  +Debug +Copy  , N: Neuron <W,O>
{
    block.process_buffers();
    //block.outputs
    //let out:&mut [O] = ;  // should only need 1 but deref not working
   //self.process_input(&self.weights[..] , &self.inputs[..] , & mut self.outputs.borrow_mut()[..]);


}

impl< W ,O ,N>  MutableBlock <O ,W> for FullMeshBlock<W ,O ,N>
where W: Num + Debug + Copy , O: Num + Debug + Copy, N: Neuron <W,O>
{
    fn set_buffers(& mut self , weights: Vec<W>,  inputs: Vec<O> , outputs: Vec<O>)
    {
        self.inputs = inputs;
        self.outputs = RefCell::new(outputs);
        self.weights = weights;
    }

    fn add_data<'a>(& mut self , weights: & 'a [W] , inputs: & 'a [O])
    {
        self.inputs = inputs.to_vec() ;
    //    self.outputs = RefCell::new(outputs);
        self.weights = weights.to_vec() ;
    }

    fn get_output(&self ) -> Vec<O>
    {

        self.outputs.borrow().to_vec()
    }
}




impl< W ,O ,N>  IBlock for FullMeshBlock<W ,O ,N>
where W: Num  +Debug +Copy, O: Num  +Debug +Copy  , N: Neuron <W,O>
{

// not needed ?
    fn get_id(&self) -> BlockId { self.block.id }

fn process_mut_and_copy_output(& mut self , outputs: & mut [u8])
{
    process_block(self);
}

    fn process(&self , data: & mut [u8] , inputs: &[& [u8]] , outputs: & mut [u8])
    {
        panic!("not supported for mutable blocks");
    }
}




        // unsafe
        // {
        //     let weight_size = mem::size_of::<f32>();
        //     let weights: & [f32] = slice::from_raw_parts( data.as_ptr() as *const f32, data.len()/ weight_size);
        //     let inputs: & [f32] = slice::from_raw_parts( inputs[0].as_ptr() as *const f32, inputs.len()/ mem::size_of::<f32>());
        //     let outputs: & mut [f32] = slice::from_raw_parts_mut( outputs.as_ptr() as *mut f32, outputs.len()/ mem::size_of::<f32>());
        //
        //     // if  (self.block.synapse_count * self.block.neuron_count) as usize != weights.len()  {
        //     //     panic!("weights does not equal synapse * neurons")
        //     // }
        //
        //     self.process_input( weights , inputs , outputs);
        //
        //     println!("O {:?}", outputs );
        // }





//we dont impliment neuronblock as default iblock  doesnt work
// impl< W  ,O ,N>  NeuronBlock<W, O>  for FullMeshBlock< W ,O ,N>
// where W: Num + Debug  , O: Num + Debug , N: Neuron <W,O>
// {
//
//     fn getid(&self) -> BlockId {  self.block.id}
//
//     fn process_input(& self , weights: & [W] , inputs: & [O] , outputs: & mut [O])
//     {
//         let mut nc = 0;
//         // could use a pair itterator this seems fragile
//         for weights_for_neuron in weights.chunks( self.block.synapse_count as usize )
//         {
//
//             println!("weights_for_neuron {:?}", weights_for_neuron );
//
//             let activated:O =  { N::eval( inputs ,   weights_for_neuron  )};
//             outputs[nc] = activated;
//             println!("O {:?}", outputs );
//             nc = nc + 1;
//         }
//     }
// }


// impl< W  ,O ,N>  IBlock  for FullMeshBlock< W ,O ,N>
// where W: 'a + Num + Debug  , O: 'a + Num  +Debug, N: Neuron <W,O>
// {
//     //    fn as_blocktype (&self) -> BlockType {   BlockType::Block(  self)  }
//         fn get_id(&self) -> BlockId { self.block.id  }
//
//         fn process(&self , data: &  [u8] , inputs: & [u8] , output_u8: & mut [u8])
//
// //    fn process(& mut self)
//     {
//
//         println!("starting process buffer");
//         println!("{:?}", self.block.synapse_count  );
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




pub fn add_four(a: i32) -> i32 {
    a + 4
}


// full mesh checks
#[test]
fn fullmesh_create_fullmesh_bloc ()
{

         let input: & [f32] = &[1f32, 2f32, 3f32, 4f32, 5f32];
        let mut output: & mut [f32] = & mut [1f32, 2f32, 3f32, 4f32, 5f32];
          let weights: &  [f32] = & [0f32; 500];

        let _block  =  FullMeshBlock::<f32,f32,DefaultNeuron<f32,f32,Logistic>>::new(BlockData::new(5 , 5, 5)
                , weights
                , output
                , input
        );
}


#[test]
fn fullmesh_create_fullmesh_bloc_3 ()
{

         let input: & [f32] = &[1f32, 2f32, 3f32, 4f32, 5f32];
        let mut output: & mut [f32] = & mut [1f32, 2f32, 3f32, 4f32, 5f32];
          let weights: &  [f32] = & [0f32; 500];

        let _block  =  FullMeshBlock::<f32,f32,DefaultNeuron<f32,f32,Logistic>>::new(BlockData::new(5 , 5, 5)
                , weights
                , output
                , input
        );

//anne::blocks::fullmesh::FullMeshBlock<'_, f32, f32, anne::blocks::neural::defaultweight::DefaultNeuron<f32, f32, anne::blocks::neural::activation::Logistic>>
        let iblock : & IBlock  = &_block;
        //let iblock: Box<IBlock> = Box::new(& *neuronblock);
}



//

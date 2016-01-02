
use std::marker::PhantomData;
use std::{slice , mem};
use std::fmt::Debug;
use std::cell::RefCell;
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
pub struct FullMeshBlock<'a, W, O, N>
where W: Num + 'a  , O: Num + 'a  , N: Neuron <W,O >
{
    weights: & 'a [W],
    inputs: & 'a [O],
    outputs: RefCell<& 'a mut  [O]>,

    block: BlockData,
    neural_behaviour: PhantomData<N>,
}


impl<'a,W,O,N>  FullMeshBlock<'a,W,O,N>
where W: Num +'a +Debug , O: Num +'a +Debug  , N: Neuron <W,O>
{
    pub fn new_late(block_data: BlockData )  -> FullMeshBlock< 'a, W , O , N>
    {
        if block_data.neuron_count == 0 || block_data.synapse_count == 0 {  panic!("neuron or synapse_count cannot be 0"); };
        FullMeshBlock { block : block_data , weights:  &[],  outputs: RefCell::new(& mut []) ,inputs: &[]  , neural_behaviour:  ::std::marker::PhantomData   }
    }

     pub fn new(block_data: BlockData , all_weights: & 'a [W] , output_buf: & 'a mut [O], input_buf: & 'a  [O])  -> FullMeshBlock< 'a, W , O , N>
     {
         if block_data.neuron_count == 0 || block_data.synapse_count == 0 {  panic!("neuron or synapse_count cannot be 0"); };
         FullMeshBlock { block : block_data , weights: all_weights ,  outputs: RefCell::new(output_buf) ,inputs: input_buf  , neural_behaviour:  ::std::marker::PhantomData   }
     }

     pub fn process_buffers(& self )
     {
         //let out:&mut [O] = ;  // should only need 1 but deref not working
         self.process_input(self.weights , self.inputs , (self.outputs.borrow_mut()).borrow_mut());
     }


}



impl<'a,W ,O ,N>  BlockBehaviour <'a, O ,W> for FullMeshBlock<'a,W ,O ,N>
where W: Num + Debug + 'a , O: Num + Debug+  'a , N: Neuron <W,O>
{
    fn set_buffers(& mut self , weights: & 'a [W],  inputs: & 'a [& [O]] , outputs: & 'a mut [O])
    {
        self.inputs = inputs[0];
        self.outputs = RefCell::new(outputs);
        self.weights = weights;

    }
}

impl< 'b, W  ,O ,N>  NeuronBlock<O,W>  for FullMeshBlock< 'b,W ,O ,N>
where W: Num + Debug+'b  , O: Num + Debug +'b, N: Neuron <W,O>
{
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


impl<'a, W  ,O ,N>  IBlock  for FullMeshBlock<'a, W ,O ,N>
where W: 'a + Num + Debug  , O: 'a + Num  +Debug, N: Neuron <W,O>
{
        fn as_blocktype (&self) -> BlockType {   BlockType::Block(  self)  }
        fn get_id(&self) -> BlockId { self.block.id  }
}


impl< 'a, W ,O ,N>  Block  for FullMeshBlock< 'a, W ,O ,N>
where W:  Num + 'a + Debug  , O:  Num + 'a  +Debug, N: Neuron <W,O>
{
    fn process(&self , data: &  [u8] , inputs: & [u8] , output_u8: & mut [u8])

//    fn process(& mut self)
    {

        println!("starting process buffer");
        println!("{:?}", self.block.synapse_count  );
        unsafe
        {
            let weight_size = mem::size_of::<W>();
            let weights: & [W] = slice::from_raw_parts( data.as_ptr() as *const W, data.len()/ weight_size);
            let inputs: & [O] = slice::from_raw_parts( inputs.as_ptr() as *const O, inputs.len()/ mem::size_of::<O>());
            let outputs: & mut [O] = slice::from_raw_parts_mut( output_u8.as_ptr() as *mut O, output_u8.len()/ mem::size_of::<O>());

            if  (self.block.synapse_count * self.block.neuron_count) as usize != weights.len()  {
                panic!("weights does not equal synapse * neurons")
            }


            self.process_input( weights , inputs , outputs);

            //
            // // could use a pair itterator this seems fragile
            // for weights_for_neuron in weights.chunks( self.block.synapse_count as usize )
            // {
            //
            //         println!("weights_for_neuron {:?}", weights_for_neuron );
            //
            //
            //     // for nc in 0..self.block.neuron_count as usize
            //     // {
            //         let activated:O =  { N::eval( inputs ,   weights_for_neuron  )};
            //         outputs[nc] = activated;
            //         println!("O {:?}", outputs );
            //
            //     //}
            //     nc = nc + 1;
            // }
            println!("O {:?}", outputs );
        }
    }// unsafe
}




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


//

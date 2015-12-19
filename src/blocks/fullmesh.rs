
use std::marker::PhantomData;

use num::traits::Num;

use core::*;
use super::neural::neuron::*;
use super::block::*;


// dont enhance it build new ones this is a basic impl.
//#[derive(Default)]
pub struct FullMeshBlock<W, O, N>
where W: Num + 'static , O: Num + 'static , N: NeuralNetParameters <W,O>
{
    weights: & 'static [W],
    inputs: & 'static [O],
    outputs: & 'static mut  [O],

    block: BlockData,
    neural_behaviour: PhantomData<N>,
}


impl<W,O,N>  FullMeshBlock<W,O,N>
where W: Num + 'static , O: Num + 'static , N: NeuralNetParameters <W,O>
{
     pub fn new(block_data: BlockData , all_weights: & 'static [W] , output_buf: & 'static mut [O], input_buf: & 'static  [O])  -> FullMeshBlock< W , O , N>
     {
         FullMeshBlock { block : block_data , weights: all_weights ,  outputs: output_buf ,inputs: input_buf  , neural_behaviour:  ::std::marker::PhantomData   }
     }

//obviously faILTU
     // this could change if we have dimensional support
    fn weights_for_neuron(&self , neuron_num:u32 ) -> &[W] { self.weights}

}

impl<W ,O ,N>  BlockBehaviour < O > for FullMeshBlock<W ,O ,N>
where W: Num + 'static, O: Num + 'static, N: NeuralNetParameters <W,O>
{
    fn set_buffers(& mut self , inputs: &[& 'static [O]] , outputs: & 'static mut [O])
    {
        self.inputs = inputs[0];
        self.outputs = outputs;
    }
//    fn get_input_for_neuron (&self  , neuron_num : u32 ) -> &[Self::Output];
}

impl<W ,O ,N>  Block  for FullMeshBlock<W ,O ,N>
where W: Num + 'static , O: Num + 'static, N: NeuralNetParameters <W,O>
{
    fn process_buffers(& mut self)
    {
        let mut nc = 0;
        for weights_for_neuron in self.weights.chunks( self.block.synapse_count as usize )
        {
            for nc in 0..self.block.neuron_count as usize
            {
                let activated:O =
                 {
                     let in_vec_for_neuron = self.get_input_for_neuron( nc as u32);
                     standard_calc::<W,O,N>( weights_for_neuron, in_vec_for_neuron )
                 };

                self.outputs[nc] = activated;
            }
            nc = nc + 1;
        }
    }
}

impl<W, O, N>  NeuronBlockBehaviour <W, O, N>  for FullMeshBlock<W, O, N>
where W: Num + 'static , O: Num +'static , N: NeuralNetParameters <W,O>
{
    // full mesh returns all inputs for every neuron
    fn get_input_for_neuron (&self  , _neuron_num : u32 ) -> &[O] { self.inputs }
    fn get_weights_for_neuron (&self  , neuron_num : u32 ) -> &[W] { self.weights_for_neuron(neuron_num)}
}


pub fn add_four(a: i32) -> i32 {
    a + 4
}


// full mesh checks
#[test]
fn fullmesh_create_fullmesh_bloc ()
{
    unsafe
    {
        static input_buf: &'static [f32] = &[1f32, 2f32, 3f32, 4f32, 5f32];
        static mut output_buf: & 'static mut [f32] = & mut [1f32, 2f32, 3f32, 4f32, 5f32];
        static  weights: & 'static  [f32] = & [0f32; 500];

        let block  =  FullMeshBlock::<f32,f32,super::neural::Sigmoid>::new(BlockData::new(5)
                , weights
                , output_buf
                , input_buf
        );
    }// unsafe
}


//

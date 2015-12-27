
use std::marker::PhantomData;
use std::fmt::Debug;

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
pub struct FullMeshBlock<W, O, N  >
where W: Num + 'static , O: Num + 'static , N: Neuron <W,O >
{
    weights: & 'static [W],
    inputs: & 'static [O],
    outputs: & 'static mut  [O],

    block: BlockData,
    neural_behaviour: PhantomData<N>,
}


impl<W,O,N>  FullMeshBlock<W,O,N>
where W: Num + 'static , O: Num + 'static , N: Neuron <W,O>
{
     pub fn new(block_data: BlockData , all_weights: & 'static [W] , output_buf: & 'static mut [O], input_buf: & 'static  [O])  -> FullMeshBlock< W , O , N>
     {
         if block_data.neuron_count == 0 || block_data.synapse_count == 0 {  panic!("neuron or synapse_count cannot be 0"); };
         // if block_data.neuron_count != block_data.synapse_count  {  panic!("neuron should = synapse_count"); };
         FullMeshBlock { block : block_data , weights: all_weights ,  outputs: output_buf ,inputs: input_buf  , neural_behaviour:  ::std::marker::PhantomData   }
     }

     // not needed for simple chunking
     // this could change if we have dimensional support
    //fn weights_for_neuron(&self , neuron_num:u32 ) -> &[W] { self.weights}

}

impl<W ,O ,N>  BlockBehaviour < O > for FullMeshBlock<W ,O ,N>
where W: Num + 'static, O: Num + 'static, N: Neuron <W,O>
{
    fn set_buffers(& mut self , inputs: &[& 'static [O]] , outputs: & 'static mut [O])
    {
        self.inputs = inputs[0];
        self.outputs = outputs;
    }
//    fn get_input_for_neuron (&self  , neuron_num : u32 ) -> &[Self::Output];
}

impl<W ,O ,N>  Block  for FullMeshBlock<W ,O ,N>
where W: Num + Debug +  'static , O: Num + 'static +Debug, N: Neuron <W,O>
{
    fn process(& mut self)
    {
        println!("starting process buffer");
        println!("{:?}", self.block.synapse_count  );
        let mut nc = 0;

        for weights_for_neuron in self.weights.chunks( self.block.synapse_count as usize )
        {
                println!("W {:?}", self.weights );
                println!("I {:?}", self.inputs );

            for nc in 0..self.block.neuron_count as usize
            {
                let activated:O =  { N::eval( self.inputs ,   weights_for_neuron  )};
                self.outputs[nc] = activated;
                println!("O {:?}", self.outputs );

            }
            nc = nc + 1;
        }
    }
}

// impl<W, O, N>  NeuronBlockBehaviour <W, O, N>  for FullMeshBlock<W, O, N>
// where W: Num + 'static , O: Num +'static , N: Neuron <W,O>
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
    unsafe
    {
        static INPUT_BUF: &'static [f32] = &[1f32, 2f32, 3f32, 4f32, 5f32];
        static mut OUTPUT_BUF: & 'static mut [f32] = & mut [1f32, 2f32, 3f32, 4f32, 5f32];
        static  WEIGHTS: & 'static  [f32] = & [0f32; 500];

        let mut block_data = BlockData::new(5);
        block_data.neuron_count = 5;
        block_data.synapse_count = 5;

        let _block  =  FullMeshBlock::<f32,f32,DefaultNeuron<f32,f32,Logistic>>::new(block_data
                , WEIGHTS
                , OUTPUT_BUF
                , INPUT_BUF
        );
    }// unsafe
}


//

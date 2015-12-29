
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
pub struct FullMeshTBlock<'a, W, O, N>
where W: Num + 'a  , O: Num + 'a  , N: Neuron <W,O >
{
    weights: & 'a [W],
    inputs: & 'a [O],
    outputs: & 'a mut  [O],

    block: BlockData,
    neural_behaviour: PhantomData<N>,
}


impl<'a,W,O,N>  FullMeshTBlock<'a,W,O,N>
where W: Num +'a , O: Num +'a  , N: Neuron <W,O>
{
    pub fn new_late(block_data: BlockData )  -> FullMeshTBlock< 'a, W , O , N>
    {
        if block_data.neuron_count == 0 || block_data.synapse_count == 0 {  panic!("neuron or synapse_count cannot be 0"); };
        // if block_data.neuron_count != block_data.synapse_count  {  panic!("neuron should = synapse_count"); };
        FullMeshTBlock { block : block_data , weights:  &[],  outputs: & mut [] ,inputs: &[]  , neural_behaviour:  ::std::marker::PhantomData   }
    }

     pub fn new(block_data: BlockData , all_weights: & 'a [W] , output_buf: & 'a mut [O], input_buf: & 'a  [O])  -> FullMeshTBlock< 'a, W , O , N>
     {
         if block_data.neuron_count == 0 || block_data.synapse_count == 0 {  panic!("neuron or synapse_count cannot be 0"); };
         // if block_data.neuron_count != block_data.synapse_count  {  panic!("neuron should = synapse_count"); };
         FullMeshTBlock { block : block_data , weights: all_weights ,  outputs: output_buf ,inputs: input_buf  , neural_behaviour:  ::std::marker::PhantomData   }
     }

     // not needed for simple chunking
     // this could change if we have dimensional support
    //fn weights_for_neuron(&self , neuron_num:u32 ) -> &[W] { self.weights}

}

// pub trait BlockBehaviour <'a, O: Num + 'a>
// {
//
//     fn set_buffers(& mut self , weights: & 'a [O] , inputs: & 'a [& [O]] , outputs: & 'a mut [O]);
//
// //    fn set_buffers(& mut self , inputs: &[& 'a  [O]] , outputs: & 'a mut [O]);
// //    fn get_input_for_neuron (&self  , neuron_num : u32 ) -> &[Self::Output];
// }



impl<'a,W ,O ,N>  BlockBehaviour <'a, O ,W> for FullMeshTBlock<'a,W ,O ,N>
where W: Num + 'a , O: Num + 'a , N: Neuron <W,O>
{
    fn set_buffers(& mut self , weights: & 'a [W],  inputs: & 'a [& [O]] , outputs: & 'a mut [O])
    {
        self.inputs = inputs[0];
        self.outputs = outputs;
        self.weights = weights;

    }

    // fn set_buffers(& mut self , weights: & 'static [W] , inputs: & 'static [ & [O]] , outputs: & 'static mut [O])
    // {
    //     self.inputs = inputs[0];
    //     self.outputs = outputs;
    // }
//    fn get_input_for_neuron (&self  , neuron_num : u32 ) -> &[Self::Output];
}

impl<'a, W ,O ,N>  Block  for FullMeshTBlock<'a, W ,O ,N>
where W: 'a + Num + Debug  , O: 'a + Num  +Debug, N: Neuron <W,O>
{
    fn process(& mut self)
    {

        println!("starting process buffer");
        println!("{:?}", self.block.synapse_count  );
        println!("W {:?}", self.weights );   println!("I {:?}", self.inputs );
        let mut nc = 0;

        if  (self.block.synapse_count * self.block.neuron_count) as usize != self.weights.len()  {
            panic!("weights does not equal synapse * neurons")
        }


        // could use a pair itterator this seems fragile
        for weights_for_neuron in self.weights.chunks( self.block.synapse_count as usize )
        {

                println!("weights_for_neuron {:?}", weights_for_neuron );


            // for nc in 0..self.block.neuron_count as usize
            // {
                let activated:O =  { N::eval( self.inputs ,   weights_for_neuron  )};
                self.outputs[nc] = activated;
                println!("O {:?}", self.outputs );

            //}
            nc = nc + 1;
        }
        println!("O {:?}", self.outputs );
    }
}

// impl<W, O, N>  NeuronBlockBehaviour <W, O, N>  for FullMeshTBlock<W, O, N>
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

         let input: & [f32] = &[1f32, 2f32, 3f32, 4f32, 5f32];
        let mut output: & mut [f32] = & mut [1f32, 2f32, 3f32, 4f32, 5f32];
          let weights: &  [f32] = & [0f32; 500];

        let _block  =  FullMeshTBlock::<f32,f32,DefaultNeuron<f32,f32,Logistic>>::new(BlockData::new(5 , 5, 5)
                , weights
                , output
                , input
        );
}


//

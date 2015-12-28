
use std::marker::PhantomData;
use std::fmt::Debug;

use num::traits::Num;

use core::*;
#[allow(unused_imports)]
use blocks::neural::defaultweight::DefaultNeuron;
#[allow(unused_imports)]
use blocks::neural::activation::Logistic;
use super::neural::neuron::*;
use super::BlockData;


// dont enhance it build new ones this is a basic impl.
//#[derive(Default)]
pub struct BlockwWeightHardening<W, O, N>
where W: Num + 'static , O: Num + 'static , N: Neuron <W,O>
{
    weights: & 'static [W],
    #[allow(dead_code)]
    weights_hardness: Vec<i8> , // = size of weights
    inputs: & 'static [O],
    outputs: & 'static mut  [O],

    block: BlockData,
    neural_behaviour: PhantomData<N>,
}


impl<W,O,N>  BlockwWeightHardening<W,O,N>
where W: Num + 'static , O: Num + 'static , N: Neuron <W,O>
{
     pub fn new(block_data: BlockData , all_weights: & 'static [W] , output_buf: & 'static mut [O], input_buf: & 'static  [O])  -> BlockwWeightHardening< W , O , N>
     {
         BlockwWeightHardening { block : block_data , weights: all_weights , weights_hardness: Vec::new() ,  outputs: output_buf ,inputs: input_buf  , neural_behaviour:  ::std::marker::PhantomData   }
     }
}

impl<W ,O ,N>  BlockBehaviour < O > for BlockwWeightHardening<W ,O ,N>
where W: Num + 'static, O: Num + 'static + Debug , N: Neuron <W,O>
{
    fn set_buffers(& mut self , inputs: &[& 'static [O]] , outputs: & 'static mut [O])
    {
        self.inputs = inputs[0];
        self.outputs = outputs;
    }
//    fn get_input_for_neuron (&self  , neuron_num : u32 ) -> &[Self::Output];
}

impl<W ,O ,N>  Block  for BlockwWeightHardening<W ,O ,N>
where W: Num + Debug + 'static , O: Num + Debug +  'static, N: Neuron <W,O>
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

// impl<W, O, N>  NeuronBlockBehaviour <W, O, N>  for BlockwWeightHardening<W, O, N>
// where W: Num + 'static , O: Num +'static , N: Neuron <W,O>
// {
//     fn get_input_for_neuron (&self  , _neuron_num : u32 ) -> &[O] { self.inputs }
//     fn get_weights_for_neuron (&self  , neuron_num : u32 ) -> &[W] { self.weights_for_neuron(neuron_num)}
// }


pub fn add_four(a: i32) -> i32 {
    a + 4
}


// full mesh checks
#[test]
fn block_w_hardening_create_bloc ()
{
    unsafe
    {
        static INPUT_BUF: &'static [f32] = &[1f32, 2f32, 3f32, 4f32, 5f32];
        static mut OUTPUT_BUF: & 'static mut [f32] = & mut [1f32, 2f32, 3f32, 4f32, 5f32];
        static  WEIGHTS: & 'static  [f32] = & [0f32; 500];


        let _block  =  BlockwWeightHardening::<f32,f32,DefaultNeuron<f32,f32,Logistic>>::new(BlockData::new(5 , 5, 9)
                , WEIGHTS
                , OUTPUT_BUF
                , INPUT_BUF
        );
    }// unsafe
}


//

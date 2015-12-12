
use num::traits::Num;
use std::marker::PhantomData;
use core::*;
use super::neuron::*;

fn standard_calc <W, O, N> (weights: &[W] , inputs: &[O] ) -> O
where W: Num , O: Num , N: NeuralNetParameters <W,O>
{
    let result =  N::WeightFunction::calc_weight( inputs ,  weights  ) ;
    N::ActivationFunction::activation(result)
}

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

pub struct BlockwWeightHardening<'a , W: Num + 'a>
{
    weights: &'a [W],
    weights_hardness: Vec<u8>,
    block: BlockData
}

#[derive(Default , Clone)]
pub struct BlockData
{
    pub name: String,
    pub connections: Vec<Connection>,
    pub next_run_sequence: Vec<BlockId>,
    id: BlockId,
    neuron_count: u32,
    synapse_count: u32
}

impl BlockData
{
    pub fn new (newid: BlockId) -> BlockData { BlockData { id : newid , ..Default::default() } }
}

impl<W,O,N>  FullMeshBlock<W,O,N>
where W: Num + 'static , O: Num + 'static , N: NeuralNetParameters <W,O>
{
    //  pub fn new(id: BlockId , neurons: NeuronNum , input_size: u32 , allWeights: & 'static [W])  -> FullMeshBlock< W , O , N>
    //  {
    //      FullMeshBlock { block : BlockData::new(id) , weights: allWeights ,  outputs: output_buf  }
    //  }

     pub fn new(block_data: BlockData , all_weights: & 'static [W] , output_buf: & 'static mut [O], input_buf: & 'static  [O])  -> FullMeshBlock< W , O , N>
     {
         FullMeshBlock { block : block_data , weights: all_weights ,  outputs: output_buf ,inputs: input_buf  , neural_behaviour:  ::std::marker::PhantomData   }
     }

    //
    //  fn value(&self) -> u32 { 0 }
    //
    //  fn offset(&self, neuron: u32) -> u32
    //  {
    //      neuron / self.block.synapse_count
    //  }

     // this could change if we have dimensional support
    fn weights_for_neuron(&self , neuron_num:u32 ) -> &[W] { self.weights}

}



// now we add neuron behaviour which block will use


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


        // for nc in 0..self.block.neuron_count as usize
        // {
        //     let activated:O =
        //     {
        //         let in_vec_for_neuron = self.get_input_for_neuron( nc as u32);
        //         let weights_for_neuron = self.get_weights_for_neuron(nc as u32);
        //         let result =  N::WeightFunction::calc_weight( in_vec_for_neuron, weights_for_neuron  ) ;
        //         N::ActivationFunction::activation(result)
        //     };
        //
        //     self.outputs[nc] = activated;
        // }

    }
}

impl<W, O, N>  NeuronBlockBehaviour <W, O, N>  for FullMeshBlock<W, O, N>
where W: Num + 'static , O: Num +'static , N: NeuralNetParameters <W,O>
{
    fn get_input_for_neuron (&self  , neuron_num : u32 ) -> &[O] { self.inputs }
    fn get_weights_for_neuron (&self  , neuron_num : u32 ) -> &[W] { self.weights_for_neuron(neuron_num)}
}



// impl<'a> BlockBehaviour for FullMeshBlock<'a, f32>
// {
//     // type Output = f32;
//     // fn save_input(&self , data: &[Self::Output] , port: BlockPort  )   {}
// //    fn get_input_for_neuron (&self  , neuron_num : u32 ) -> &[Self::Output] {panic!()}
// }

// impl<'a> NeuronBlockBehaviour<f32> for FullMeshBlock<'a, f32>
// {
//     fn get_neuron_behaviour (&self) -> Neuron<f32 , Output = Self::Output>
//     {
//
//     }
//
//     //type Output = f32;
//     //fn save_input(&self , data: &[Self::Output] , port: BlockPort  )   {}
//     fn get_input_for_neuron (&self  , neuron_num : u32 ) -> &[Self::Output] {panic!()}
// }
//
//
// impl<'a, T:  Neuron<f32, Output=f32>>  Block<f32 ,T> for FullMeshBlock<'a, f32 >
// {
//     fn process_neuron(&self) -> Vec<f32>
//     {
//         let mut vec  =  Vec::<f32>::with_capacity(self.block.neuron_count as usize);
//         for nc in 0..self.block.neuron_count as usize
//         {
//             let in_vec_for_neuron : &[f32] = self.get_input_for_neuron( nc as u32);
//             let weights : &[f32] = self.get_weights_for_neuron(nc as u32);
//             vec[nc] =  T::calc(weights , in_vec_for_neuron);
//
//         }
//         vec
//     }
// }


// try just for FullMesh

// full mesh does not communicate the neuron...
// a better option maybe to place this in the struct and use a get Neuron interface
// that way it does not affect blocks that dont use it
//
// impl<'a, T:  Neuron<f32, Output=f32>>  BBlock for FullMeshBlock<'a, f32 >
// {
//
//     fn process(&self) -> Vec<f32>
//     {
//         Self::process_neuron(&self)
//     }
//
// }


// impl< 'a, W: Num, O: Num ,  T:  Neuron<W, Output=O>>  BBlock for Block<W ,T ,Output=O >
// // where & 'a Block<W ,T ,Output=O> : BlockBehaviour< Output=O> ,
// // & 'a Block<W ,T ,Output=O> : Block<W ,T>
// {
//     fn process(&self) -> Vec<O>
//     {
//         Self::process_neuron(&self)
//     }
// }


// standard impl for all neuron blocks
//impl<N: Neuron<f32 , Output = f32>> Blockf32<N> for Block<f32,N , Output = f32> {}


pub fn add_three(a: i32) -> i32 {
    a + 3
}



// impl<'a, T:  Neuron<f32, Output=f32> , B: BlockBehaviour<Output=f32> > Block<B>
// for FullMeshBlock<'a, f32, f32 , T>
// {
//     //type Output = f32;
//
//     fn load_vector(&self , data: &[f32] , port: BlockPort )  { self.load_vector(data); }
//
//     fn process(&self) -> Vec<f32>
//     {
//         let mut vec  =  Vec::<f32>::with_capacity(self.neuron_count as usize);
//         for nc in 0..self.neuron_count as usize
//         {
//             let in_vec_for_neuron : &[f32] = self.get_weights_for_neuron(nc as u32);
//             vec[nc] =  self.behaviour.process(self.weights);
//
//         }
//         vec
//     }
//
// }

// can we use hardware port as offset = 16 , 32 etc
// we can impliment full mesh here

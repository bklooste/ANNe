extern crate num;

use self::num::traits::Num;
use core::*;

#[derive(Default , Clone)]
pub struct FullMeshBlock<'a , W: Num + 'a >  {
    weights: &'a [W],
    block: BlockData,
}

pub struct BlockwWeightHardening<'a , W: Num + 'a> //num::traits::Num
{
    weights: &'a [W],
    weights_hardness: Vec<u8>,
    block: BlockData
}

#[derive(Default , Clone)]
struct BlockData //num::traits::Num
{
    pub name: String,
    pub connections: Vec<Connection>,
    pub next_run_sequence: Vec<BlockId>,
    id: BlockId,
    neuron_count: u32,
    synapse_count: u32
    // weights: &'a [W],
    // behaviour: T //we dont need self here , neuron behaviour  we can just add to impl

//block has the load and neuron has the neuron processing
}

impl BlockData
{
    pub fn new (newid: BlockId) -> BlockData { BlockData { id : newid , ..Default::default() } }
}

impl<'a , W: Num + Default > FullMeshBlock<'a , W  >
{
     pub fn new(id: BlockId , neurons: NeuronNum , input_size: u32 , allWeights: & 'a [W])  -> FullMeshBlock<'a , W>
     {
         FullMeshBlock { block : BlockData::new(id) , weights: allWeights ,  ..Default::default() }
     }

     fn value(&self) -> u32 { 0 }

     fn offset(&self, neuron: u32) -> u32
     {
         self.block.synapse_count
     }

    fn get_weights_for_neuron(&self , neuron_num:u32 ) -> &[W] { self.weights}

}

impl<'a> BlockBehaviour for FullMeshBlock<'a, f32>
{
    type Output = f32;
    fn save_input(&self , data: &[Self::Output] , port: BlockPort  )   {}
//    fn get_input_for_neuron (&self  , neuron_num : u32 ) -> &[Self::Output] {panic!()}
}

impl<'a> NeuronBlockBehaviour for FullMeshBlock<'a, f32>
{
    //type Output = f32;
    //fn save_input(&self , data: &[Self::Output] , port: BlockPort  )   {}
    fn get_input_for_neuron (&self  , neuron_num : u32 ) -> &[Self::Output] {panic!()}
}

//type f32 neuron
impl<'a, T:  Neuron<f32, Output=f32>>  Block<f32 ,T> for FullMeshBlock<'a, f32 >
{
    //type Output = f32;
//    fn save_input(&self , data: &[Self::Output] , port: BlockPort  );
//    fn save_vector(&self , data: &[f32] , port: BlockPort )  { self.save_input(data , port); }

//after all factors saved
// need a dirty
    fn process(&self) -> Vec<f32>
    {
        let mut vec  =  Vec::<f32>::with_capacity(self.block.neuron_count as usize);
        for nc in 0..self.block.neuron_count as usize
        {
            let in_vec_for_neuron : &[f32] = self.get_input_for_neuron( nc as u32);
            let weights : &[f32] = self.get_weights_for_neuron(nc as u32);
            vec[nc] =  T::calc(weights , in_vec_for_neuron);

        }
        vec
    }
}

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

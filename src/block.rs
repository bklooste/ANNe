extern crate num;

use self::num::traits::Num;

use core::*;

struct FullMeshBlock<'a , W: Num + 'a > //) //, O, T:  Neuron<W, Output=O>> //num::traits::Num
{
    weights: &'a [W],
    block: BlockData<'a>,
//    behaviour: T //we dont need self here , neuron behaviour  we can just add to impl

//block has the load and neuron has the neuron processing
}

pub struct BlockwWeightHardening<'a , W: Num + 'a , O, T:  Neuron<W, Output=O>> //num::traits::Num
{
    weights: &'a [W],
    weightsHardness: Vec<u8>,
    block: BlockData<'a>,
    behaviour: T //we dont need self here , neuron behaviour  we can just add to impl

//block has the load and neuron has the neuron processing
}


struct BlockData<'a> //num::traits::Num
{
    pub name: &'a str,
    pub connections: Vec<Connection>,
    pub next_run_sequence: Vec<BlockId>,
    id: BlockId,
    neuron_count: u32,
    synapse_count: u32
    // weights: &'a [W],
    // behaviour: T //we dont need self here , neuron behaviour  we can just add to impl

//block has the load and neuron has the neuron processing
}

impl<'a , W: Num > FullMeshBlock<'a , W  >
{
     fn value(&self) -> u32 { 0 }
     fn offset(&self, neuron: u32) -> u32 {
         self.block.synapse_count
    }
    fn get_weights_for_neuron(&self , neuron_num:u32 ) -> &[W] { self.weights}

}


impl<'a> BlockBehaviour
for FullMeshBlock<'a, f32>
{
    type Output = f32;
    fn save_input(&self , data: &[Self::Output] , port: BlockPort  )   {}

    fn get_weights_for_neuron (&self  , neuron_num : u32 ) -> &[Self::Output] {panic!()}
}

//type f32 neuron
impl<'a, T:  Neuron<f32, Output=f32>> Block<f32 ,T> for FullMeshBlock<'a, f32 >
{
    //type Output = f32;
//    fn save_input(&self , data: &[Self::Output] , port: BlockPort  );
    fn load_vector(&self , data: &[f32] , port: BlockPort )  { self.save_input(data , port); }

    fn process(&self) -> Vec<f32>
    {
        let mut vec  =  Vec::<f32>::with_capacity(self.block.neuron_count as usize);
        for nc in 0..self.block.neuron_count as usize
        {
            let in_vec_for_neuron : &[f32] = self.get_weights_for_neuron(nc as u32);
            vec[nc] =  T::calc(self.weights , in_vec_for_neuron);

        }
        vec
    }

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

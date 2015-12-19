use num::traits::Num;

use core::*;
use super::neural::neuron::*;

pub fn standard_calc <W, O, N> (weights: &[W] , inputs: &[O] ) -> O
where W: Num , O: Num , N: NeuralNetParameters <W,O>
{
    let result =  N::WeightFunction::calc_weight( inputs ,  weights  ) ;
    N::ActivationFunction::activation(result)
}

#[derive(Default , Clone)]
pub struct BlockData
{
    pub id: BlockId,
    pub name: String,
    pub connections: Vec<Connection>,
    pub next_run_sequence: Vec<BlockId>,
    pub neuron_count: u32,
    pub synapse_count: u32
}

impl BlockData
{
    pub fn new (newid: BlockId) -> BlockData { BlockData { id : newid , ..Default::default() } }
}





pub fn add_three(a: i32) -> i32 {
    a + 3
}

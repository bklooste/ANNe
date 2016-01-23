use num::traits::Num;
use blocks::neural::activation::{Logistic , Linear , Threshold};
use blocks::neural::{defaultweight ,defaultweightwbias};



// neuron or neuron behaviour
pub trait Neuron<W : Num , O: Num  >
{
    fn eval(data: &[O], weights: &[W]) -> O;
}

pub trait NeuronBlockBehaviour < W : Num , O : Num , N: Neuron<W, O >>
{
    // fn calc (weights: &[W] ,  inputs: &[O] ) -> Self::Output ;
    // fn activate (output : Self::Output )  -> Self::Output ;
    // //fn derivative
    // fn calculate_sum  (weights: &[W] ,  inputs: &[Self::Output] ) -> Self::Output ;
    fn get_input_for_neuron (&self  , neuron_num : u32 ) -> &[O];
    fn get_weights_for_neuron (&self  , neuron_num : u32 ) -> &[W];

}


pub fn add_foura(a: i32) -> i32 {
    a + 3 + 1
}

pub type DefaultLogistic = defaultweight::DefaultNeuron<f32,f32,Logistic>;
pub type DefaultLogisticB =  defaultweightwbias::DefaultWeightwBias<f32,f32,Logistic>;
pub type LinearByteB =  defaultweightwbias::DefaultWeightwBias<isize,u8,Linear>;
pub type LinearByte =  defaultweight::DefaultNeuron<isize,u8,Linear>;
pub type ThresholdByteB =  defaultweightwbias::DefaultWeightwBias<isize,u8,Threshold>;

//defaultweightwbias::DefaultWeightwBias<Logistic>;

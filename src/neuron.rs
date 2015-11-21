extern crate num;

use self::num::traits::Num;
use core::Neuron;

// we use diffirent behaviours with dif impl.
struct BehaviourData<O: Num > //num::traits::Num
{
    data : Vec<O>
}


impl< W: Num> Neuron <W> for BehaviourData<f32>
{
    type Output = f32;

    // fn process (&self ,weights: &[W]  ) -> f32 { self.data[0]}
    // fn load_vector(&self , data: &[f32] ) { }

    fn calc (weights: &[W] ,  inputs: &[Self::Output] ) -> Self::Output { inputs[0]}
    fn activate (output : Self::Output )  -> Self::Output  { output }
    fn calculate_sum  (weights: &[W] ,  inputs: &[Self::Output] ) -> Self::Output{ inputs[0]}

}

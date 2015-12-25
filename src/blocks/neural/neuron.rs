use num::traits::Num;
use blocks::neural::activation::Logistic;
use blocks::neural::defaultweight;



// neuron or neuron behaviour
pub trait Neuron<W : Num , O: Num  >
{
    fn eval(data: &[O], weights: &[W]) -> O;
}


pub fn add_foura(a: i32) -> i32 {
    a + 4
}

pub type DefaultLogistic = defaultweight::DefaultNeuron<Logistic>;

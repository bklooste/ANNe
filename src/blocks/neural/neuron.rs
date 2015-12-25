use num::traits::Num;
use blocks::neural::activation::Logistic;
use blocks::neural::defaultweight;

pub trait ActivationFunction<I : Num , O : Num>
{
   fn activate(x: I) -> O;
}

// neuron or neuron behaviour
pub trait Neuron<W : Num , O: Num  >
{
    fn eval(data: &[O], weights: &[W]) -> O;
}


pub fn add_foura(a: i32) -> i32 {
    a + 4
}

pub type DefaultLogistic = defaultweight::DefaultNeuron<Logistic>;

// impl<f32,f32> Neuron<f32,f32> for DefaultLogistic;
// #[derive(Copy, Clone)]
// pub struct DefaultNeuron<T> where T:ActivationFunction<f32,f32> {    _m: PhantomData<T> }

// pub struct DefaultNeuron<T> where T:ActivationFunction<f32,f32>

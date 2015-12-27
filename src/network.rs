extern crate num;

// public enum NeuronType
//     {
//         UNKNOWN,
//         Relay, //(!:!)
//         Pyramid, //..mMan
//         PyramidPull,
//         Dampner,
//         Function,
//         //Disperser,
//         Periodic,
//         Limit, ///Train
//         LimitwTrain
//     }


//collection of modules
trait Network
{
//use num::traits::Num;

//pub use log;
    fn new(name: &'static str) -> Self;

}

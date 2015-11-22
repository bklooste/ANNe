extern crate num;

use self::num::traits::Num;

pub type BlockId = u32;
pub type BlockPort = u32;
pub type NeuronNum = u32;


pub struct ConnectionDestination
{
        destination : BlockId,
        port : i32
    ////  type : ConnectionType
}

pub enum Connection {
    Connector { destination: ConnectionDestination},
    Loom { destination: ConnectionDestination , size:u32 },
    Mesh { destination: ConnectionDestination , interval:u32 , size:u32 },
    RandomMesh { destination: ConnectionDestination , intervalrate:f32 , size:u32 },
    FullMesh { destination: ConnectionDestination , size:u32},
    Output { destination: ConnectionDestination}
}


pub trait Neuron<W: Num> {
    type Output: Num;
    //type Weights: Num;
    //fn requires_data if not we can just use static method on the input and skip the allocation
    fn calc (weights: &[W] ,  inputs: &[Self::Output] ) -> Self::Output ;
    fn activate (output : Self::Output )  -> Self::Output ;
    fn calculate_sum  (weights: &[W] ,  inputs: &[Self::Output] ) -> Self::Output ;
//    fn process  (&self, weights: &[W] ) -> Self::Output ;
//    fn load_vector(&self , data: &[Self::Output] );
}

//manages input data
pub trait BlockBehaviour {
    type Output: Num;
    //type Weights: Num;
    //fn requires_data if not we can just use static method on the input and skip the allocation
//    fn process (weights: &[W] ,  inputs: &[Self::Output] ) -> Self::Output ;
//    fn process  (&self, weights: &[W] ) -> Self::Output ;

    // we need to add connection ? eg can you have diffirent full mesh and loom port ?
    // that will in effect move this behaviour to the port.
    fn save_input(&self , data: &[Self::Output] , port: BlockPort  );
    fn get_input_for_neuron (&self  , neuron_num : u32 ) -> &[Self::Output];
}


pub trait Block<W: Num , T: Neuron< W>> : BlockBehaviour {
    //type Output: Num;
    //type Behaviour: BlockBehaviour;
    fn process(&self) -> Vec<Self::Output>;
    fn save_vector(&self , data: &[Self::Output] , port: BlockPort );
}






// fn main() {
//
// }


pub use core::{ConnectionDestination, Connection, Neuron, Block  , BlockBehaviour , Blockf32 , Blocki8 , Blockf64};

pub use blocks::neuron::Sigmoid;
pub use blocks::block::{FullMeshBlock , BlockwWeightHardening};


//fixme move this and the use somewhere more appropiate eg prelude or lib
trait Blockf32Sigmoid  : Blockf32< Sigmoid> {}

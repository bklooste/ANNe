//! Provides the fundamental units of computation for the [Network][1].
//! [1]: ../network/index.html
//!
//! These layers provide different type of operations to the data Blobs

#[allow(unused_import_braces)]
pub use blocks::neural::neuron::*;
pub use core::*;

pub mod neural;
pub mod fullmesh;
pub mod mesh;
pub mod function;




#[cfg(test)]
mod block_tst;

#[derive(Default , Clone)]
pub struct BlockData
{
    pub id: BlockId,
    pub name: String,
    pub connections: Vec<Connection>,
    pub next_run_sequence: Vec<BlockId>,
    pub neuron_count: u32,
    /// includes bias per neuron
    pub synapse_count: u32
}

impl BlockData
{
    pub fn new (newid: BlockId , ncount: u32 , scount: u32) -> BlockData { BlockData { id : newid , neuron_count: ncount , synapse_count: scount , ..Default::default() } }
}

pub fn add_three(a: i32) -> i32 {
    a + 3
}

 // revist names
pub type LinearByteMutBlock = ::blocks::fullmesh::FullMeshBlock<i8,u8 ,LinearByteB>;
pub type LogisticMutBlock = ::blocks::fullmesh::FullMeshBlock< f32,f32,DefaultLogistic>;
pub type LogisticMutBiasBlock = ::blocks::fullmesh::FullMeshBlock< f32,f32,DefaultLogisticB>;
pub type LinearByteBlock = ::blocks::mesh::MeshBlock<i8,u8 ,LinearByte>;
pub type LinearByteBiasBlock = ::blocks::mesh::MeshBlock<i8,u8 ,LinearByteB>;
pub type LogisticBlock = ::blocks::mesh::MeshBlock<f32,f32,DefaultLogistic>;
pub type LogisticBiasBlock = ::blocks::mesh::MeshBlock<f32,f32,DefaultLogisticB>;
pub type ThreshholdByteBiasBlock = ::blocks::mesh::MeshBlock<i8,u8 ,ThresholdByteB>;

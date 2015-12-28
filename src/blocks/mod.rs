//! Provides the fundamental units of computation for the [Network][1].
//! [1]: ../network/index.html
//!
//! These layers provide different type of operations to the data Blobs
//! that flow through them.
//! The operations provided by the layers can be
//! roughly grouped into four categories:
//!
//! * __Activation__</br>
//! Activation Layers provide element-wise operations and produce one top Blob
//! of the same size as the bottom Blob.
//! It can be seen as a synonym to nonlinear [Activation Functions][2].

#[allow(unused_import_braces)]
pub use blocks::neural::neuron::*;

pub use core::*;


pub mod neural;
pub mod fullmesh; pub mod blockwweighthardening;
pub mod fullmesht;

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
    /// includes bias
    pub synapse_count: u32
}

impl BlockData
{
    pub fn new (newid: BlockId , ncount: u32 , scount: u32) -> BlockData { BlockData { id : newid , neuron_count: ncount , synapse_count: scount , ..Default::default() } }
}

pub fn add_three(a: i32) -> i32 {
    a + 3
}

pub type LinearByteBlock = ::blocks::fullmesh::FullMeshBlock<i8,u8,LinearByteB>;
pub type LogisticBBlock = ::blocks::fullmesh::FullMeshBlock<f32,f32,DefaultLogisticB>;
pub type LogisticBlock = ::blocks::fullmesh::FullMeshBlock<f32,f32,DefaultLogistic>;
pub type LogisticBlockwLifetime<'a> = ::blocks::fullmesht::FullMeshTBlock<'a,f32,f32,DefaultLogistic>;

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
    pub synapse_count: u32
}

impl BlockData
{
    pub fn new (newid: BlockId) -> BlockData { BlockData { id : newid , ..Default::default() } }
}

pub fn add_three(a: i32) -> i32 {
    a + 3
}

pub type LogisticBBlock = ::blocks::fullmesh::FullMeshBlock<f32,f32,DefaultLogisticB>;
pub type LogisticBlock = ::blocks::fullmesh::FullMeshBlock<f32,f32,DefaultLogistic>;

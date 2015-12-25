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
pub use self::neural::neuron::Neuron;

pub mod block;
pub mod neural;

pub mod fullmesh; pub mod blockwweighthardening;

#[cfg(test)]
mod block_tst;

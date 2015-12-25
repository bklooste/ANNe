//! Provides nonlinear activate methods.
//!
//! Activation Layers take a bottom Blob, provide the activate operation and
//! produce a top Blob.
//! Thanks to the nonlinearity of the activate methods, we can 'learn' and
//! detect nonlinearities
//! in our (complex) datasets.
//!
//! The activate operation used should depend on the task at hand. For binary
//! classification a
//! step function might be very useful. For more complex tasks continious
//! activate functions such
//! as Sigmoid, TanH, Softmax or ReLU should be used. In most cases ReLU might
//! prove the best
//! results.
//!
//! The activate function is also sometimes called transfer function.
//pub use self::sigmoid::Sigmoid;

pub mod testdata;
pub mod activation;
pub mod training;

pub mod unit;
pub mod neuron;

pub mod defaultweight;
pub mod defaultweightwbias;

#[cfg(test)]
mod neural_tst;

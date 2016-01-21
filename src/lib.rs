//! The `aNNe` crate
//!
//! # Examples
//!
//! ```
//! assert_eq!(4, anne::add_two(2));
//! ```

// #![feature(plugin)]
// #![feature(augmented_assignments)]
// #![plugin(clippy)]
// #![allow(dead_code)]
// #![allow(unused_variables)]
// #![deny(missing_docs,
//         missing_debug_implementations, missing_copy_implementations,
//         trivial_casts, trivial_numeric_casts,
//         unsafe_code,
//         unused_import_braces, unused_qualifications)]
//

extern crate num;
extern crate rand;
extern crate rustc_serialize;

#[macro_use]
extern crate log;

mod network;
pub mod util;

pub mod buffer_manager;
pub mod blocks;
pub mod graph;
pub mod module;
pub mod core;
pub mod prelude;


/// This function adds two to its argument.
///
/// # Examples
///
/// ```
/// use anne::add_two;
///
/// assert_eq!(14, add_two(12));
/// ```
pub fn add_two(a: i32) -> i32 {
    a + 2
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(4, add_two(2));
    }
}


// //! Leaf is a open, fast and a well-designed, modular Framework for distributed
// //! Deep Learning on {C, G}PUs.
// //!
// //! ## Overview
// //!
// //! To build a Deep Neural Network you first need to create a
// //! [Network][network] which is a container for all different types of
// //! [Layers][layers]. These layers are grouped in different types such as
// //! [Activation Layers][activate] and [Loss Layers][loss] (these state the
// //! characteristics of the layer).
// //!
// //! Now to train your network you can use one of the [Solvers][solvers]. The
// //! Solver defines the [Optimization Method][optimization] and keeps track on
// //! the learning progress.
// //!
// //! The operations can run on different Backends {CPU, GPU} and doesn't have
// //! to be defined at compile time, which allows for easy backend swapping.
// //!
// //! ## Philosophy
// //!
// //! We are strong believers in the technology of Machine Learning.
// //!
// //! We put our experience in software engineering into Leaf, to solve our own
// //! need for a modern, performant and easy-to-use Deep Learning Framework.
// //! These principles direct our decisions on Leaf and related projects.
// //!
// //! * __Cutting Edge Performance__:</br>
// //! For research and industry speed and efficency are curcial for
// //! state-of-the-art machine learning over massive data and networks.
// //! * __Open and Expressive Architecture__:</br>
// //! Designing an open architecture that follows best practices and concepts in
// //! Engineering such as modularity, flexibility and expressiveness is critical
// //! to stimulate future innovation.
// //! * __Clear and Transparent Documentation__:</br>
// //! A well-written documentation that addresses both concepts and
// //! implementations, empowers developers and researchers to contribute their
// //! unique experience to the project for the benefit of everyone.
// //!
// //! ## Examples
// //!
// //! ```
// //! # extern crate leaf;
// //! # use leaf::network::{NetworkConfig};
// //! # fn main() {
// //! # }
// //! ```
// //!
// //! ## Development
// //!
// //! The implementation of various Layers is pretty scarce at the moment.<br/>
// //! There are around a dozen layers, which are really important and would
// //! increase the value and functionality of Leaf tremendously.<br/>
// //! Progress is tracked at<br/>
// //!
// //! - [Issue #18 for Loss Layers][issue-loss]
// //! - [Issue #19 for Activation Layers][issue-activate]
// //! - [Issue #20 for Common Layers][issue-common]
// //!
// //! [network]: ./network/index.html
// //! [layers]: ./layers/index.html
// //! [activate]: ./layers/activate/index.html
// //! [loss]: ./layers/loss/index.html
// //! [solvers]: ./solvers/index.html
// //! [optimization]: https://en.wikipedia.org/wiki/Stochastic_optimization
// //!
// //! [issue-loss]: https://github.com/autumnai/leaf/issues/18
// //! [issue-activate]: https://github.com/autumnai/leaf/issues/19
// //! [issue-common]: https://github.com/autumnai/leaf/issues/20

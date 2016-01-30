# ANNe • [![Join the chat at https://gitter.im/bklooste/anne](https://img.shields.io/badge/gitter-join%20chat-brightgreen.svg)](https://gitter.im/bklooste/anne?utm_source=badge&utm_medium=badge&utm_campaign=pr-badge&utm_content=badge) [![Build Status](https://travis-ci.org/bklooste/ANNe.svg?branch=master)](https://travis-ci.org/bklooste/ANNe) [![Coverage Status](https://coveralls.io/repos/bklooste/ANNe/badge.svg?branch=master&service=github)](https://coveralls.io/github/bklooste/ANNe?branch=master) [![License](https://img.shields.io/apm/l/vim-mode.svg)](LICENSE)



## Introduction

aNNe  is a open source framework for machine decision making focusing on practical and generic applications such as robotics.
To do this aNNe uses a combination of Artificial Neural Networks and traditional code and shares concepts with TensorFlow and Caffe.

aNNe is written in Rust, a language which is well suited for state-of-the-art machine learning. It allows for the performance, memory-security, and extensibility, that other frameworks (TensorFlow, Caffe, Theano) only gain by combining high-level languages (Python) with low-level languages (C, C++).

However these frameworks have the goal to produce the best solutions for a given training set , aNNe takes a very different approach  from a philosophy point of view aNNe takes the view of having more small human designed reusable components is more desirable even if a lower result is obtained and in the long term these components will give a better result in a more flexible manner.   The goal of the framework is to research this as it will be highly valuable in the automation and robotics field.

Here are some of the key differences.
- Modules are drop in independent deployable units that are deployable and can be wired into any network. Think of them as a shared library with associated data with improved support for working within a larger neural net or as part of a traditional application .
- Blocks may contain traditional code.
- More general approach a system can be very interconnected , eg control system  rather than produce a result.
- Support for hard limits.
- Support for immediate training while running to support hard limits.   
- No use of CUDA and GPUs . These are not suitable for embedded environments. Instead optional SIMD is used and its worth noting SIMD with bytes can give CUDA GPU like performance , with a lot more flexibility in scheduling and when dealing with lots of small blocks performance will be much higher.
- Support for many types eg signed byte weights , f64 or bit input / outputs and these can be combined in a single module.
- While stochastic back-propagation is supported a lot more emphasizes is given to  investigation of other methods which may not give a better solution but will work better as a large complex unit. Specifically Hebbian learning with weight handling.
- It will features a higher level and likely graphical design component which will work with higher level logical types and a lower level run time .  We see this as allowing users to use pre made components without having a high skill level.

The architecture  is an application is composed of a number of independent / re-usable  module components each consisting of a number of connected  blocks, blocks  represent operations over n-dimensional numerical inputs into the network. Blocks usually implement mathematical operations, but can be used for many more such as feeding in data, logging, or returning results. You can use the  Blocks that ship with aNNe (e.g. Convolutional, ReLU  etc.) or thanks to Rust, easily extend aNNe with your own blocks.

A list of current blocks:
  FunctionBlock (run any rust function)
  MeshBlock standard mesh layer block , can be used with a variety of weight / activation behaviour.
  FullMeshBlock  as Mesh block but manages   its own mutable data.


aNNe strives for leading-edge performance, while providing a flexible architecture that creates - as we hope - an innovative and active community around machine intelligence and fuels future research.

## License

Anne is released under the [MIT License][license].

[license]: LICENSE

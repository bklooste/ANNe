//! Provides the generics and interfaces for the specific [Solvers][solvers].
//! [solvers]: ../solvers/index.html
// use co::backend::*;
// use co::framework::*;
// use co::frameworks::Native;
// use co::libraries::blas::IBlas;
// use shared_memory::*;
// use network::*;
use std::ops::Deref;

use std::cell::RefCell;
use core::IBlock;
use module::*;
use solver::*;
use solvers::*;
use solvers::sgd::momentum::*;// should be exported from solvers
use core::ErrorInfo;
//use HaltCondition::{ Epochs, MSE, Timer };

use time::{ Duration, PreciseTime };

pub use self::HaltCondition::*;

use std::rc::Rc;




#[derive(Debug)]
/// Solver that optimizes a [Network][1].
/// [1]: ../network/struct.Network.html
pub struct NewSolver<S> {
    net: Module,
    /// The implementation of the Solver
    pub worker: S,

    config: SolverConfig,

    /// The current iteration / number of times weights have been updated
    iter: usize,
}

#[derive(Debug, Copy, Clone)]
pub enum HaltCondition {
    /// Stop training after a certain number of epochs
    Epochs(u32),
    /// Train until a certain error rate is achieved
    MSE(f32),
    /// Train for some fixed amount of time and then halt
    Timer(Duration),
}


#[derive(Debug)]
pub struct SolverRun<'a> {
    examples: &'a [(Vec<f32>, Vec<f32>)],
    halt_condition: HaltCondition,
    log_interval: Option<u32>
//    nn: &'a mut Module,
}

impl<S> NewSolver<S> {
    // /// Create Solver from [SolverConfig][1]
    // /// [1]: ./struct.SolverConfig.html
    // ///
    // /// This is the **preferred method** to create a Solver for training a neural network.
    // ///
    // /// ## Example
    // ///
    // /// ```
    // /// # extern crate leaf;
    // /// # extern crate collenchyma;
    // /// # use leaf::solver::*;
    // /// # use collenchyma::backend::Backend;
    // /// # use collenchyma::frameworks::Native;
    // ///
    // /// # fn main() {
    // /// let cfg = SolverConfig{
    // ///             solver: SolverKind::SGD(SGDKind::Momentum),
    // ///             ..SolverConfig::default()};
    // /// let solver = Solver::<Box<ISolver<Backend<Native>>>, Backend<Native>>::from_config(&cfg);
    // /// # }
    // /// ```
    // pub fn from_config(config: &SolverConfig) -> Solver<Box<ISolver> {
    //     // let framework = Native::new();
    //     // let hardwares = framework.hardwares();
    //     // let backend_config = BackendConfig::new(framework, hardwares);
    //     // let backend = Rc::new(Backend::new(backend_config).unwrap());
    //
    //     let worker = config.solver.with_config(backend.clone(), &config);
    //     Solver {
    //         worker: worker,
    //         net: Network::from_config(backend, &config.train_net),
    //         iter: 0,
    //
    //         config: config.clone(),
    //     }
    // }

}

impl<S: ISolver> NewSolver<S>{
    fn init(&mut self, config: SolverConfig) {
        // Caffe
        //   CHECK(Caffe::root_solver() || root_solver_)
        //       << "root_solver_ needs to be set for all non-root solvers";
        info!("Initializing solver from configuration: {:?}", config);
        self.config = config;
        assert!(self.config.average_loss > 1);

        Solver::<S>::init_train_net(&mut self.config, &mut self.net);
        // if (Caffe::root_solver()) {
        {
            // self.init_test_nets();
            info!("Solver scaffolding done.");
        }
    }


    /// Initialize the training net
    fn init_train_net(config: &mut SolverConfig, net: &mut Module , run: &SolverRun) {
        // Caffe
        // Set the correct NetState.  We start with the solver defaults (lowest
        // precedence); then, merge in any NetState specified by the net_param itself;
        // finally, merge in any NetState specified by the train_state (highest
        // precedence).
        // NetState net_state;
        // net_state.set_phase(TRAIN);
        // net_state.MergeFrom(net_param.state());
        // net_state.MergeFrom(param_.train_state());
        // net_param.mutable_state()->CopyFrom(net_state);

        // TODO: there currently is no merging; we probably only need solver_default ||
        // net_param
        //let solver_default = NetworkState { mode: NetworkMode::Train, ..NetworkState::default() };

        //puhes into module
        //param.train_net.state = solver_default;

        // Caffe
        // if (Caffe::root_solver()) {
        //     net_.reset(new Net<Dtype>(net_param));
        // } else {
        //     net_.reset(new Net<Dtype>(net_param, root_solver_->net_.get()));
        // }
    //    *net = Network::from_config(backend, &param.train_net);

        unimplemented!();
    }

    // might take a solver state as argument in the future to resume a stopped
    // solver
    // pub fn solve(&mut self,  run: SolverRun ) {
    //     info!("Solving {}", self.net.name);
    //
    //     let num_iter = 100;
    //     self.step(num_iter , run);
    // }

    fn train_details(&mut self, config: &mut SolverConfig, net: &mut Module , run: &SolverRun) -> f32 {

        // check that input and output sizes are correct
        //fixme cleanup
        let input_layer_size =  net.buffer_mgr.get_buffer(*net.buffer_mgr.module_in_buffers.first().unwrap()).borrow().len();
        let output_layer_size = net.buffer_mgr.get_buffer(*net.buffer_mgr.module_out_buffers.first().unwrap()).borrow().len();
        for &(ref inputs, ref outputs) in run.examples.iter() {
            if inputs.len()  != input_layer_size {
                panic!("input has a different length than the network's input layer");
            }
            if outputs.len() != output_layer_size {
                panic!("output has a different length than the network's output layer");
            }
        }

        self.train_incremental(config , net , run )
    }

    fn train_incremental(&mut self,config: &mut SolverConfig, net: &mut Module , run: &SolverRun) -> f32 {

        let mut prev_deltas = self.make_weights_tracker(net.blocks , 0.0f32);
        let mut epochs = 0u32;
        let mut training_error_rate = 0f32;
        let start_time = PreciseTime::now();

        loop {

            if epochs > 0 {
                // log error rate if necessary
                match run.log_interval {
                    Some(interval) if epochs % interval == 0 => {
                        println!("error rate: {}", training_error_rate);
                    },
                    _ => (),
                }

                // check if we've met the halt condition yet
                match run.halt_condition {
                    Epochs(epochs_halt) => {
                        if epochs == epochs_halt { break }
                    },
                    MSE(target_error) => {
                        if training_error_rate <= target_error { break }
                    },
                    Timer(duration) => {
                        let now = PreciseTime::now();
                        if start_time.to(now) >= duration { break }
                    }
                }
            }

            training_error_rate = 0f32;

//TODO NOW
            // for &(ref inputs, ref targets) in run.examples.iter() {
            //     let results = self.do_run(&inputs);
            //     let weight_updates = self.calculate_weight_updates(&results, &targets);
            //     training_error_rate += calculate_error(&results, &targets);
            //     self.update_weights(&weight_updates, &mut prev_deltas, config.get_learning_rate(epochs), config.momentum)
            // }

            epochs += 1;
        }

        training_error_rate
    }
    //
    // fn do_run(&self, inputs: &[f64]) -> Vec<Vec<f64>> {
    //     let mut results = Vec::new();
    //     results.push(inputs.to_vec());
    //     for (layer_index, layer) in self.layers.iter().enumerate() {
    //         let mut layer_results = Vec::new();
    //         for node in layer.iter() {
    //             layer_results.push( sigmoid(modified_dotprod(&node, &results[layer_index])) )
    //         }
    //         results.push(layer_results);
    //     }
    //     results
    // }
    //
    // // updates all weights in the network
    // fn update_weights(&mut self, network_weight_updates: &Vec<Vec<Vec<f64>>>, prev_deltas: &mut Vec<Vec<Vec<f64>>>, rate: f64, momentum: f64) {
    //     for layer_index in 0..self.layers.len() {
    //         let mut layer = &mut self.layers[layer_index];
    //         let layer_weight_updates = &network_weight_updates[layer_index];
    //         for node_index in 0..layer.len() {
    //             let mut node = &mut layer[node_index];
    //             let node_weight_updates = &layer_weight_updates[node_index];
    //             for weight_index in 0..node.len() {
    //                 let weight_update = node_weight_updates[weight_index];
    //                 let prev_delta = prev_deltas[layer_index][node_index][weight_index];
    //                 let delta = (rate * weight_update) + (momentum * prev_delta);
    //                 node[weight_index] += delta;
    //                 prev_deltas[layer_index][node_index][weight_index] = delta;
    //             }
    //         }
    //     }
    //
    // }
    //
    // // calculates all weight updates by backpropagation
    // fn calculate_weight_updates(&self, results: &Vec<Vec<f64>>, targets: &[f64]) -> Vec<Vec<Vec<f64>>> {
    //     let mut network_errors:Vec<Vec<f64>> = Vec::new();
    //     let mut network_weight_updates = Vec::new();
    //     let layers = &self.layers;
    //     let network_results = &results[1..]; // skip the input layer
    //     let mut next_layer_nodes: Option<&Vec<Vec<f64>>> = None;
    //
    //     for (layer_index, (layer_nodes, layer_results)) in iter_zip_enum(layers, network_results).rev() {
    //         let prev_layer_results = &results[layer_index];
    //         let mut layer_errors = Vec::new();
    //         let mut layer_weight_updates = Vec::new();
    //
    //
    //         for (node_index, (node, &result)) in iter_zip_enum(layer_nodes, layer_results) {
    //             let mut node_weight_updates = Vec::new();
    //             let mut node_error;
    //
    //             // calculate error for this node
    //             if layer_index == layers.len() - 1 {
    //                 node_error = result * (1f64 - result) * (targets[node_index] - result);
    //             } else {
    //                 let mut sum = 0f64;
    //                 let next_layer_errors = &network_errors[network_errors.len() - 1];
    //                 for (next_node, &next_node_error_data) in next_layer_nodes.unwrap().iter().zip((next_layer_errors).iter()) {
    //                     sum += next_node[node_index+1] * next_node_error_data; // +1 because the 0th weight is the threshold
    //                 }
    //                 node_error = result * (1f64 - result) * sum;
    //             }
    //
    //             // calculate weight updates for this node
    //             for weight_index in 0..node.len() {
    //                 let mut prev_layer_result;
    //                 if weight_index == 0 {
    //                     prev_layer_result = 1f64; // threshold
    //                 } else {
    //                     prev_layer_result = prev_layer_results[weight_index-1];
    //                 }
    //                 let weight_update = node_error * prev_layer_result;
    //                 node_weight_updates.push(weight_update);
    //             }
    //
    //             layer_errors.push(node_error);
    //             layer_weight_updates.push(node_weight_updates);
    //         }
    //
    //         network_errors.push(layer_errors);
    //         network_weight_updates.push(layer_weight_updates);
    //         next_layer_nodes = Some(&layer_nodes);
    //     }
    //
    //     // updates were built by backpropagation so reverse them
    //     network_weight_updates.reverse();
    //
    //     network_weight_updates
    // }
    //
    fn make_weights_tracker<T: Clone>(&self, layers: Vec<RefCell<Box<IBlock>>>,    place_holder: T) -> Vec<Vec<Vec<T>>> {
        let mut network_level = Vec::new();
        for layer in layers.iter() {
            let mut layer_level = Vec::new();
            for node in layer.borrow().deref().iter() {
                let mut node_level = Vec::new();
                for _ in node.iter() {
                    node_level.push(place_holder.clone());
                }
                layer_level.push(node_level);
            }
            network_level.push(layer_level);
        }

        network_level
    }

    // fn step(&mut self, iters: usize ,  run: SolverRun) {
    //     let start_iter = self.iter;
    //     let stop_iter = start_iter + iters;
    //     // int average_loss = this->param_.average_loss(); // Caffe
    //     let mut losses = Vec::<f32>::new();
    //     let mut smoothed_loss = 0f32;
    //
    //     while self.iter < stop_iter {
    //         let mut loss = 0f32;
    //
    //     //    self.net.clear_weight_diffs();
    //         // if self.param.test_interval.is_some() && self.iter % self.param
    //
    //         // run tests all `test_interval` iterations
    //         // unless it's the first iteration and we are not testing on initialization
    //         if let Some(test_interval) = self.config.test_interval {
    //             if self.iter % test_interval == 0 && (self.iter > 0 || self.config.test_initialization) {
    //                 // && Caffe::root_solver()) { // Caffe
    //
    //                 // TODO
    //                 //   TestAll();
    //                 //   if (requested_early_exit_) {
    //                 //     // Break out of the while loop because stop was requested while testing.
    //                 //     break;
    //                 //   }
    //             }
    //         }
    //         // Caffe
    //         // for (int i = 0; i < callbacks_.size(); ++i) {
    //         //   callbacks_[i]->on_start();
    //         // }
    //
    //         // Caffe : display info every .display() iterations
    //         // const bool display = param_.display() && iter_ % param_.display() == 0;
    //         // net_->set_debug_info(display && param_.debug_info());
    //
    //         //let noop_bottom : Vec<Box<[u8]>> = Vec::new();
    //         for _ in 0..self.config.minibatch_size - 1 {
    //
    //             get samples
    //
    //             // run forward
    //             self.net.process_blocks();
    //             // now inspect buffers
    //
    //             for block in &self.net.blocks
    //             {
    //
    //                 match block.borrow().get_prop_info()
    //                  {
    //                      Some(ref block_info) => loss +=  calc_loss(block_info.deref()), //fixme solver specific
    //                      None => continue,
    //                  }
    //
    //                 //
    //                 // println!( "block {}" , block.get_id() );
    //                 //
    //                 // if block.get_prop_info() == None { continue ;}
    //                 // let learn_data = block.get_prop_info().unwrap();
    //
    //             }
    //
    //
    //         //    loss += self.net.forward_backward(&noop_bottom);
    //         }
    //         // average the loss across iterations of minibatch
    //         loss /= self.config.minibatch_size as f32;
    //
    //         // average the loss across iterations for smoothed reporting
    //         if losses.len() < self.config.average_loss {
    //             losses.push(loss);
    //             let size = losses.len() as f32;
    //             smoothed_loss = (smoothed_loss * (size - 1f32) + loss) / size;
    //         } else {
    //             let idx = (self.iter - start_iter) % self.config.average_loss;
    //             smoothed_loss += (loss - losses[idx]) / self.config.average_loss as f32;
    //             losses[idx] = loss;
    //         }
    //
    //         // Caffe
    //         // if (display) {
    //         //   LOG_IF(INFO, Caffe::root_solver()) << "Iteration " << iter_
    //         //       << ", loss = " << smoothed_loss;
    //         //   const vector<Blob<Dtype>*>& result = net_->output_blobs();
    //         //   int score_index = 0;
    //         //   for (int j = 0; j < result.size(); ++j) {
    //         //     const Dtype* result_vec = result[j]->cpu_data();
    //         //     const string& output_name =
    //         //         net_->blob_names()[net_->output_blob_indices()[j]];
    //         //     const Dtype loss_weight =
    //         //         net_->blob_loss_weights()[net_->output_blob_indices()[j]];
    //         //     for (int k = 0; k < result[j]->count(); ++k) {
    //         //       ostringstream loss_msg_stream;
    //         //       if (loss_weight) {
    //         //         loss_msg_stream << " (* " << loss_weight
    //         //                         << " = " << loss_weight * result_vec[k] << " loss)";
    //         //       }
    //         //       LOG_IF(INFO, Caffe::root_solver()) << "    Train net output #"
    //         //           << score_index++ << ": " << output_name << " = "
    //         //           << result_vec[k] << loss_msg_stream.str();
    //         //     }
    //         //   }
    //         // }
    //         // for (int i = 0; i < callbacks_.size(); ++i) {
    //         //   callbacks_[i]->on_gradients_ready();
    //         // }
    //
    //         // Caffe / Display
    //         //   if (this->param_.display() && this->iter_ % this->param_.display() == 0) {
    //         //     LOG(INFO) << "Iteration " << this->iter_ << ", lr = " << rate;
    //         //   }
    //         //self.worker.apply_update(&self.config, &mut self.net, self.iter);
    //
    //         // Increment the internal iter counter -- its value should always indicate
    //         // the number of times the weights have been updated.
    //         self.iter += 1;
    //
    //         // Caffe
    //         // SolverAction::Enum request = GetRequestedAction();
    //         //
    //         // // Save a snapshot if needed.
    //         // if ((param_.snapshot()
    //         //      && iter_ % param_.snapshot() == 0
    //         //      && Caffe::root_solver()) ||
    //         //      (request == SolverAction::SNAPSHOT)) {
    //         //   Snapshot();
    //         // }
    //         // if (SolverAction::STOP == request) {
    //         //   requested_early_exit_ = true;
    //         //   // Break out of training loop.
    //         //   break;
    //         // }
    //
    //     }
    // }
}

// calculates MSE of output layer
fn calculate_error(results: &Vec<Vec<f32>>, targets: &[f32]) -> f32 {
    let ref last_results = results[results.len()-1];
    let mut total:f32 = 0f32;
    for (&result, &target) in last_results.iter().zip(targets.iter()) {
        total += (target - result).powi(2);
    }
    total / (last_results.len() as f32)
}

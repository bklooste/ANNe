
// not sure why needed ....
extern crate anne;
use anne::module::{Module};
use anne::blocks::{BlockData, LogisticMutBlock , LogisticBlock , LogisticBiasBlock , LinearByteBlock , ThreshholdByteBiasBlock};
pub use anne::blocks::neural::neuron::*;
use anne::core::{ MutableBlock };
// , BlockData};


// start with https://github.com/autumnai/leaf/blob/89c31a93e5b4550a03c24caa68973598aee46c9f/src/network.rs

#[test]
fn module_build_rerun_i8_xor()
{
    let mut module = Module::new_from_input(vec! [1u8 , 1u8] ,1);
    let blk1 = module.add_block(Box::new(ThreshholdByteBiasBlock::new(1, 2, 6)));
    let blk2 = module.add_block_w_data(Box::new(ThreshholdByteBiasBlock::new(2 , 1, 6)) );  // no output as we use module output

    // we dont have output on block 1 and weights...
    module.add_simple_connections( blk1, blk2, &[ (blk1, blk2)] );

    module.process_blocks();

    let train_set = [
    (&[0u8, 0u8],  &[0u8]),
    (&[1u8, 0u8],  &[1u8]),
    (&[0u8, 1u8],  &[1u8]),
    (&[1u8, 1u8],  &[0u8])
    ]
    module.train ( train_set )

    // train the network on the examples of the XOR function
// all methods seen here are optional except go() which must be called to begin training
// see the documentation for the Trainer struct for more info on what each method does


    let mod_output: Vec<u8>  =  module.copy_output();
    assert_eq!(mod_output, & [0u8]);

    // set to 0 , 0
    module.set_input( &[0u8, 0u8]);
    module.process_blocks();
    let mod_output: Vec<u8>  =  module.copy_output();
    assert_eq!(mod_output, & [0u8]);

    module.set_input( &[0u8, 1u8]);
    module.process_blocks();
    let mod_output: Vec<u8>  =  module.copy_output();
    assert_eq!(mod_output, & [1u8]);

    module.set_input( &[1u8, 0u8]);
    module.process_blocks();
    let mod_output: Vec<u8>  =  module.copy_output();
    assert_eq!(mod_output, & [1u8]);

    assert_eq!(    module.get_stats().blocks_processed, 8);
}

#[test]
fn module_build_rerun_xor_change_input()
{
    let mut module = Module::new_from_input(vec! [1f32 ;2] ,4);
    let blk1 = module.add_block_w_data_and_output(Box::new(LogisticBlock::new(1, 2, 2)), vec![ 1f32, 1f32, 2f32, 2f32] , 8);
    let blk2 = module.add_block_w_data(Box::new(LogisticBlock::new(2 , 1, 2))  , vec![-1000f32, 850f32] );  // no output as we use module output
    module.add_simple_connections( blk1, blk2, &[ (blk1, blk2)] );
    module.process_blocks();

    let mod_output: Vec<f32>  =  module.copy_output();

    module.train(&examples)
        .halt_condition( HaltCondition::Epochs(10000) )
        .log_interval( Some(100) )
        .momentum( 0.1 )
        .rate( 0.3 )
        .go();

    assert!(mod_output[0] < 0.01f32 && mod_output[0] > -0.01f32  );

    // set to 0 , 0
    module.set_input( &[0f32, 0f32]);
    module.process_blocks();
    let mod_output: Vec<f32>  =  module.copy_output();
    assert!(mod_output[0] < 0.01f32 && mod_output[0] > -0.01f32  );

    module.set_input( &[0f32, 1f32]);
    module.process_blocks();
    let mod_output: Vec<f32>  =  module.copy_output();
    assert!(mod_output[0] < 1.01f32 && mod_output[0] > 0.099f32  );
    assert_eq!(    module.get_stats().blocks_processed, 6);
}

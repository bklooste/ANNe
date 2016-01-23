
// not sure why needed ....
extern crate anne;



use anne::module::{Module};
//use std::ops::Range;
use anne::blocks::{BlockData, LogisticMutBlock , LogisticBlock , LogisticBiasBlock , LinearByteBlock , ThreshholdByteBiasBlock};

//fixme remove
pub use anne::blocks::neural::neuron::*;
use anne::core::{ MutableBlock };
// , BlockData};



#[test]
fn module_new()
{
    Module::new();
}

// externally management data rather module looks dodgy
// need to use static .. we will then get module to create
// #[test]
// fn module_build_add_node()
// {
//     unsafe
//     {
//         static INPUT_BUF: &'static [f32] = &[1f32, 2f32, 3f32, 4f32, 5f32];
//         static mut OUTPUT_BUF: & 'static mut [f32] = & mut [0f32; 5];
//         static  WEIGHTS: & 'static  [f32] = & [ 1f32, 2f32, 3f32, 4f32, 5f32, 11f32, 12f32, 13f32, 14f32, 15f32, 0.1f32, 0.2f32, 0.3f32, 0.4f32, 0.5f32 ];
//
//         let block = LogisticMutBlock::new(BlockData::new(2 , 5, 5), WEIGHTS, OUTPUT_BUF, INPUT_BUF);
//         let mut module = Module::new();
//         module.add_block(Box::new(block) );
//     }
// }


// let mut block2 = LogisticMutBlock::new_late(BlockData::new(2 , 5, 5));
// block2.set_buffers(weights2 , inputs2 , output2 );
//
// let mut module = Module::new();
// module.add_block( &(block1.as_blocktype()) );
// module.add_block( &(block2.as_blocktype()) );
// module.process_blocks();

#[test]
fn fullmesh_create_fullmesh_bloc1_set_bufs ()
{
    let input: & [f32] = &[1f32, 2f32, 3f32, 4f32, 5f32];
    let output: & mut [f32] = & mut [1f32, 2f32, 3f32, 4f32, 5f32];
    let weights: &  [f32] = & [0f32; 500];


//pub type LogisticMutBlock<'a> = ::blocks::fullmesh::FullMeshBlock<'a, f32,f32,DefaultLogistic>;
    let _block  =  ::anne::blocks::fullmesh::FullMeshBlock::<f32,f32,DefaultLogistic>::new(BlockData::new(5 , 5, 5)
                , weights
                , output
                , input
        );
}

#[test]
fn fullmesh_create_fullmesh_bloc1_set_bufs_late ()
{

    let input: & [f32] = &[1f32, 2f32, 3f32, 4f32, 5f32];
    let weights =  [0f32; 500];

    let mut _block  =   ::anne::blocks::fullmesh::FullMeshBlock::<f32,f32,DefaultLogistic>::new_late(BlockData::new(5 , 5, 5));
    _block.add_data(&weights[..] , input  );
}



#[test]
fn module_build_add_block_w_buffers()
{
    let input: & [f32] = &[1f32, 2f32, 3f32, 4f32, 5f32];
    let weights  =  [0f32; 500];

    let mut block = LogisticMutBlock::new_late(BlockData::new(2 , 5, 5));
    {
        {
            block.add_data(&weights[..] , input );
        }
        {
            let mut module = Module::new();
            module.add_block(Box::new(block) );
        }
    }
    //assert_eq!(output,& [0f32; 5]);
}

#[test]
fn module_build_add_2blocks()
{
    let weights =   vec![ 0.5f32  ; 25];
    let output = vec!  [0f32 ;5];
    let output2 = vec!  [0f32 ;5];

    let input  = vec! [1f32 ;5];
    {
        let mut block1 = LogisticMutBlock::new_late(BlockData::new(2 , 5, 5));
        block1.set_buffers(weights.to_vec() , input.to_vec() , output );
        let mut block2 = LogisticMutBlock::new_late(BlockData::new(2 , 5, 5));
        block2.set_buffers(weights , input , output2 );
        {
            let mut module = Module::new();
            module.add_block(Box::new(block1) );
            module.add_block(Box::new(block2) );
        }
    }

}


//these types of tests are not
//allowed for now.. do we reuse ?
#[test]
fn module_build_add_2blocks_same_id()
{
    let weights =   vec! [ 0.5f32  ; 25];
    let output2 = vec!  [0f32 ;5];

    let input  = vec! [1f32 ;5];
    //let inputs  = vec! [ &input[..]];
    {
        let block1 = LogisticMutBlock::new_late(BlockData::new(2 , 5, 5));
        let mut block2 = LogisticMutBlock::new_late(BlockData::new(2 , 5, 5));
        block2.set_buffers(weights , input , output2 );
        let mut module = Module::new();
        module.add_block(Box::new(block1) );
        module.add_block(Box::new(block2) );

        //assert_eq!( block2.get_output()  ,& [0f32; 5]);

    }
    //assert_eq!(output,& [0f32; 5]);
}


// change this

#[test]
fn module_build_add_2_imm_blocks()
{
    let weights =   vec![ 0.5f32  ; 25];
    let input  = vec! [1f32 ;5];
    let output = vec!  [0f32 ;5];
    {
        let block1 = LogisticBlock::new(2 , 5, 5);
        let mut block2 = LogisticMutBlock::new_late(BlockData::new(2 , 5, 5));
        block2.set_buffers(weights , input , output  );
        let mut module = Module::new();
        println!("created module1");
        module.add_block(Box::new(block1) );
        module.add_block(Box::new(block2) );
        assert_eq!(    module.get_stats().block_count, 2);
    //    module.process_blocks();
    //    assert_eq!(block1.get_output(), & [0.9241418; 5]);
    }
}

#[test]
fn module_build_from_input()
{
    let input  = vec! [1f32 ;5];
    Module::new_from_input(input , 20);
}

#[test]
fn module_build_from_input_add_block()
{
    let input  = vec! [1f32 ;5];
    Module::new_from_input(input , 20);
    LogisticBlock::new(2 , 5, 5);

}

#[test]
fn module_build_from_input_add_block2()
{
    let input  = vec! [1f32 ;5];
     let weights =   vec![ 0.5f32  ; 25];


     let block1 = LogisticBlock::new(2 , 5, 5);
     let mut module = Module::new_from_input(input , 20);

     module.add_block_w_data(Box::new(block1)  , weights );

}


// key  test wire up a single module.
#[test]
fn module_build_add_block_w_module_process()
{
    let weights =   vec![ 0.5f32  ; 25];
    let input  = vec! [1f32 ;5];

    let block1 = LogisticBlock::new(2 , 5, 5);
    let mut module = Module::new_from_input(input , 20);
    let blk_index = module.add_block_w_data(Box::new(block1)  , weights );
    module.add_simple_connections( blk_index, blk_index , &[] );

    assert_eq!(    module.get_stats().blocks_processed, 0);
    module.process_blocks();
    assert_eq!(    module.get_stats().blocks_processed, 1); // there is no link so only 1 processed
    let mod_output: Vec<f32>  = module.copy_output();
    assert_eq!(mod_output, & [0.9241418; 5]);
}

#[test]
fn module_build_add_2imm_module_process()
{
    let block1 = LogisticBlock::new(2 , 5, 5);
    let block2 = LogisticBlock::new(2 , 5, 5);
    let mut module = Module::new_from_input(vec! [1f32 ;5] ,20);
    let blk1 = module.add_block_w_data_and_output(Box::new(block1)  , vec![ 0.5f32  ; 25]  , 20);
    let blk2 = module.add_block_w_data(Box::new(block2)  , vec![ 0.5f32  ; 25] );  // no output as we use module output
    module.add_simple_connections( blk1, blk2 , &[ (blk1, blk2)] );

    assert_eq!(    module.get_stats().blocks_processed, 0);
    module.process_blocks();
    assert_eq!(    module.get_stats().blocks_processed, 2); // there is no link so only 1 processed

    let mod_output: Vec<f32>  =  module.copy_output();
    assert_eq!(mod_output, & [0.9097309; 5]);  // single process would be 0.9241418
}

#[test]
fn module_build_add_4imm_module_process()
{
    let mut module = Module::new_from_input(vec! [1f32 ;5] ,20);

    let blk1 = module.add_block_w_data_and_output(Box::new(LogisticBlock::new(2, 5, 5)), vec![ 0.5f32; 25], 20);
    let blk2 = module.add_block_w_data_and_output(Box::new(LogisticBlock::new(2, 5, 5)), vec![ 0.5f32; 25], 20);
    let blk3 = module.add_block_w_data_and_output(Box::new(LogisticBlock::new(2, 5, 5)), vec![ 0.5f32; 25], 20);
    let blk4 = module.add_block_w_data(Box::new(LogisticBlock::new(2 , 5, 5))  , vec![ 0.5f32  ; 25] );  // no output as we use module output
    module.add_simple_connections( blk1, blk4, &[ (blk1, blk2), (blk2, blk3), (blk3, blk4) ] );

    assert_eq!(    module.get_stats().blocks_processed, 0);
    module.process_blocks();
    assert_eq!(    module.get_stats().blocks_processed, 4);

    let mod_output: Vec<f32>  =  module.copy_output();
    assert_eq!(mod_output, & [0.90609163; 5]);  // single process would be 0.9241418
}


#[test]
fn module_build_bias_irregular()
{
    let mut module = Module::new_from_input(vec! [1f32 ;2] ,8);

    // probably need to imrpove this synapse_ count inmcludes bias =3 , weights = weight1 ,weight2 , bias ,  8 is 4 bytes output ie 2 floats
    let blk1 = module.add_block_w_data(Box::new(LogisticBiasBlock::new(1, 2, 3)), vec![ 1f32 , 1f32 , 0f32 , 1f32, 1f32, -1f32]);
    module.add_simple_connections( blk1, blk1, &[ ] );

    assert_eq!(module.get_stats().blocks_processed, 0);
    module.process_blocks();
    assert_eq!(    module.get_stats().blocks_processed, 1);

    let mod_output: Vec<f32>  =  module.copy_output();
    assert_eq!(mod_output, & [0.880797, 0.95257413]);  // single process would be 0.9241418
}

#[test]
fn module_build_run_xor()
{
    let mut module = Module::new_from_input(vec! [1f32 ;2] ,4);
    let blk1 = module.add_block_w_data_and_output(Box::new(LogisticBlock::new(1, 2, 2)), vec![ 1f32, 1f32, 2f32, 2f32] , 8);
    let blk2 = module.add_block_w_data(Box::new(LogisticBlock::new(2 , 1, 2))  , vec![-1000f32, 850f32] );  // no output as we use module output
    module.add_simple_connections( blk1, blk2, &[ (blk1, blk2)] );
    assert_eq!(    module.get_stats().blocks_processed, 0);
    module.process_blocks();
    assert_eq!(    module.get_stats().blocks_processed, 2);
    let mod_output: Vec<f32>  =  module.copy_output();
    assert!(mod_output[0] < 0.01f32 && mod_output[0] > -0.01f32  )
}

#[test]
fn module_build_run_i8()
{
    let mut module = Module::new_from_input(vec! [1u8 , 1u8] ,1);
    let blk1 = module.add_block_w_data_and_output(Box::new(LinearByteBlock::new(1, 2, 2)), vec![ 2i8 , -1i8 , -1i8 , 2i8 ] , 2);
    let blk2 = module.add_block_w_data(Box::new(LinearByteBlock::new(2 , 1, 2))  , vec![2i8, 2i8] );  // no output as we use module output
    module.add_simple_connections( blk1, blk2, &[ (blk1, blk2)] );
    module.process_blocks();
    let mod_output: Vec<u8>  =  module.copy_output();
    assert_eq!(mod_output, & [4u8]);
}



//FIX BIAS
#[test]
fn module_build_run_i8_bias()
{
    let mut module = Module::new_from_input(vec! [1u8 , 1u8] ,1);
    let blk1 = module.add_block_w_data_and_output(Box::new(ThreshholdByteBiasBlock::new(1, 2, 6)), vec![ 1i8 , 1 ,0 , 0 , 0 , 0, 1 , 1  ,0 , 0 , 0 , -1] , 2);
    let blk2 = module.add_block_w_data(Box::new(ThreshholdByteBiasBlock::new(2 , 1, 6))  , vec![3i8, -2 , 0 , 0 , 0 , -2] );  // no output as we use module output
    module.add_simple_connections( blk1, blk2, &[ (blk1, blk2)] );
    module.process_blocks();
    let mod_output: Vec<u8>  =  module.copy_output();
    assert_eq!(mod_output, & [1u8]);
}


#[test]
fn module_build_rerun_i8_xor()
{
    let mut module = Module::new_from_input(vec! [1u8 , 1u8] ,1);
    let blk1 = module.add_block_w_data_and_output(Box::new(ThreshholdByteBiasBlock::new(1, 2, 6)), vec![ 1i8 , 1 ,0 , 0 , 0 , 1, 1 , 1  ,0 , 0 , 0 , 0] , 2);
    let blk2 = module.add_block_w_data(Box::new(ThreshholdByteBiasBlock::new(2 , 1, 6))  , vec![-1i8, 1, 0 , 0 , 0 , 0] );  // no output as we use module output
    module.add_simple_connections( blk1, blk2, &[ (blk1, blk2)] );
    module.process_blocks();

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




// // this should fail
// #[test]
// fn module_build_add_1_imm_1_mut_blocks_process()
// {
//     let weights_bytes =   vec![ 5  ; 100];
//     let weights =   vec![ 0.5f32  ; 25];
//     let input  = vec! [1f32 ;5];
//     let mut output = vec!  [0f32 ;5];
//     {
//         let mut block1 = LogisticBlock::new(2 , 5, 5);
//         let mut block2 = LogisticMutBlock::new_late(BlockData::new(2 , 5, 5));
//         block2.set_buffers(weights , vec! [0.2f32 ;5] , output  );
//         let mut module = Module::new_from_inputs(input);
//         module.add_block_w_static_data(Box::new(block1)  , weights_bytes );
//         module.add_block(Box::new(block2) );
//         assert_eq!(    module.get_stats().blocks_processed, 0);
//
//         // we must link blocks else no input
//
//         //fails buffers not setup
//         module.process_blocks();
//         assert_eq!(    module.get_stats().blocks_processed, 1); // there is no link so only 1 processed
//     //    assert_eq!(block1.get_output(), & [0.9241418; 5]);
//     }
// }

// Logistic_mut

//
// #[test]
// fn module_build_add_2blocks_diff_data_process()
// {
//     let weights =   & [ 0.5f32  ; 25];
//     let weights2 =   & [ 0.2f32  ; 25];
//
//     let mut output = & mut  [0f32 ;5];
//     let mut output2 = & mut  [0f32 ;5];
//
//     let inputs  =  &[1f32 ;5];
//     let inputs2 =  &[0.3f32 ;5];
//
//     {
//         let mut block1 = LogisticBlock::new_late(BlockData::new(2 , 5, 5));
//         block1.set_buffers(weights , inputs , output );
//         let mut block2 = LogisticMutBlock::new_late(BlockData::new(2 , 5, 5));
//         block2.set_buffers(weights2 , inputs2 , output2 );
//
//         let mut module = Module::new();
//         let from = module.add_block( Box::new(block1)  );
//         let to = module.add_block( Box::new(block2) );
//         module.add_link(from, to);
//         module.process_blocks();
//
//     }
//     assert_eq!(output, & [0.9241418; 5]);
//     assert_eq!(output2, & [0.5744425; 5]);
// }



// TODO add blocks with output buf is input
// TODO  add blocks with output buf is input process




//
//
// //fn module_build_add_node_w_module_allocation()
//
//
// // add node with module doing allocation
//
//
//
// // xor document test

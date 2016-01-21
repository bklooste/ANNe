use anne::module::{Module};
// not sure why needed ....
extern crate anne;

//use std::ops::Range;
use anne::blocks::{BlockData, LogisticMutBlock , LogisticBlock};

//fixme remove
pub use anne::blocks::neural::neuron::*;
// use anne::blocks::neural::neuron::DefaultLogistic;
use anne::core::{ MutableBlock ,IBlock  };
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
        let mut output: & mut [f32] = & mut [1f32, 2f32, 3f32, 4f32, 5f32];
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
        let mut output: & mut [f32] = & mut [1f32, 2f32, 3f32, 4f32, 5f32];
          let weights =  [0f32; 500];

//pub type LogisticMutBlock<'a> = ::blocks::fullmesh::FullMeshBlock<'a, f32,f32,DefaultLogistic>;
        let mut _block  =   ::anne::blocks::fullmesh::FullMeshBlock::<f32,f32,DefaultLogistic>::new_late(BlockData::new(5 , 5, 5));
        //         , weights
        //         , output
        //         , input
        // );
        _block.add_data(&weights[..] , input  );
}



#[test]
fn module_build_add_block_w_buffers()
{
//     let weights =    [ 0.5f32  ; 25];
//     let mut outvec = vec! [0f32 ;5];
// //    let mut output =;
//     let input  = vec! [1f32 ;5];

let input: & [f32] = &[1f32, 2f32, 3f32, 4f32, 5f32];
let mut output: & mut [f32] = & mut [1f32, 2f32, 3f32, 4f32, 5f32];
 let weights  =  [0f32; 500];

    let mut block = LogisticMutBlock::new_late(BlockData::new(2 , 5, 5));
    {
        {
        block.add_data(&weights[..] , input );
        }
        //block.set_buffers(&weights , &input ,  & mut outvec [..] );
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
    //let mut outvec = vec! [0f32 ;5];
    let mut output = vec!  [0f32 ;5];
    let mut output2 = vec!  [0f32 ;5];

    let input  = vec! [1f32 ;5];
    //let inputs  = vec! [ &input[..]];
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
        //assert_eq!( block2.get_output()  ,& [0f32; 5]);


    }

}


//these types of tests are not
//allowed for now.. do we reuse ?
#[test]
fn module_build_add_2blocks_same_id()
{
    let weights =   vec! [ 0.5f32  ; 25];
    //let mut outvec = vec! [0f32 ;5];
    let mut output = vec!  [0f32 ;5];
    let mut output2 = vec!  [0f32 ;5];

    let input  = vec! [1f32 ;5];
    //let inputs  = vec! [ &input[..]];
    {
        let mut block1 = LogisticMutBlock::new_late(BlockData::new(2 , 5, 5));
        //block1.set_buffers(weights.to_vec() , input.to_vec() , output );
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
    let mut output = vec!  [0f32 ;5];

    {
        let mut block1 = LogisticBlock::new(2 , 5, 5);
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
    let mut module = Module::new_from_input(input , 20);
}

#[test]
fn module_build_from_input_add_block()
{
    let input  = vec! [1f32 ;5];
    let mut module = Module::new_from_input(input , 20);

     let mut block1 = LogisticBlock::new(2 , 5, 5);

}

#[test]
fn module_build_from_input_add_block2()
{
    let input  = vec! [1f32 ;5];
     let weights =   vec![ 0.5f32  ; 25];


     let mut block1 = LogisticBlock::new(2 , 5, 5);
     let mut module = Module::new_from_input(input , 20);

     let blk_index = module.add_block_w_data(Box::new(block1)  , weights );

}

#[test]
fn module_build_add_block_w_data_process()
{
    let weights =   vec![ 0.5f32  ; 25];
    let input  = vec! [1f32 ;5];
    //let mut output = vec!  [0f32 ;5];
    {
        let mut block1 = LogisticBlock::new(2 , 5, 5);
        let mut module = Module::new_from_input(input , 20);
        let blk_index = module.add_block_w_data(Box::new(block1)  , weights );
        module.add_simple_connections( blk_index, blk_index , &[] );

        assert_eq!(    module.get_stats().blocks_processed, 0);
        module.process_blocks();


        println!("block processed");
        assert_eq!(    module.get_stats().blocks_processed, 1); // there is no link so only 1 processed
        let mod_output: Vec<f32>  = module.get_output();
        //
        assert_eq!(mod_output, & [0.9341418; 5]);
    }

        println!("unwind ");
}

// #[test]
// fn module_build_add_1imm_blocks_hook_to_module_process()
// {
// //    let weights_bytes =   vec![ 5  ; 100];
//     let weights1 =   vec![ 0.5f32  ; 25];
//     let weights2 =   vec![ 0.5f32  ; 25];
//
//     let input  = vec! [1f32 ;5];
//     let mut output = vec!  [0f32 ;5];
//     {
//         let mut block1 = LogisticBlock::new(2 , 5, 5);
//         let mut block2 = LogisticBlock::new(2 , 5, 5);
//         let mut module = Module::new_from_inputs(input);
//         let blk1 = module.add_block_w_data(Box::new(block1)  , weights1 );
//         let blk2 = module.add_block_w_data(Box::new(block2)  , weights2 );
//
//         module.add_simple_connections( blk1, blk2 , &[ (blk1, blk2)] );
//         // module.add_link( blk1 , blk2);
//         // module.receive_module_input(blk1);
//         // module.use_module_output(blk2);
//
//
//         assert_eq!(    module.get_stats().blocks_processed, 0);
//
//         //fails buffers not setup
//         module.process_blocks();
//         assert_eq!(    module.get_stats().blocks_processed, 1); // there is no link so only 1 processed
//
//         let mod_output: Vec<f32>  = module.get_output();
//         assert_eq!(mod_output, & [0.9241418; 5]);
//     }
// }



// #[test]
// fn module_build_add_2_imm_blocks_process()
// {
// //    let weights_bytes =   vec![ 5  ; 100];
//     let weights1 =   vec![ 0.5f32  ; 25];
//     let weights2 =   vec![ 0.5f32  ; 25];
//
//     let input  = vec! [1f32 ;5];
//     let mut output = vec!  [0f32 ;5];
//     {
//         let mut block1 = LogisticBlock::new(2 , 5, 5);
//         let mut block2 = LogisticBlock::new(2 , 5, 5);
//         let mut module = Module::new_from_inputs(input);
//         let blk1 = module.add_block_w_data(Box::new(block1)  , weights1 );
//         let blk2 = module.add_block_w_data(Box::new(block2)  , weights2 );
//
//         module.add_simple_connections( blk1, blk2 , &[ (blk1, blk2)] );
//         // module.add_link( blk1 , blk2);
//         // module.receive_module_input(blk1);
//         // module.use_module_output(blk2);
//
//
//         assert_eq!(    module.get_stats().blocks_processed, 0);
//
//         //fails buffers not setup
//         module.process_blocks();
//         assert_eq!(    module.get_stats().blocks_processed, 1); // there is no link so only 1 processed
//
//         let mod_output: Vec<f32>  = module.get_output();
//         assert_eq!(mod_output, & [0.9241418; 5]);
//     }
// }

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

// #[test]
// fn module_build_add_2blocks_diff_data_process_no_link()
// {
//     let weights =   & [ 0.5f32  ; 25];
//     let weights2 =   & [ 0.2f32  ; 25];
//
//     //let mut outvec = vec! [0f32 ;5];
//     let mut output = & mut  [0f32 ;5];
//     let mut output2 = & mut  [0f32 ;5];
//
//     let inputs  =  &[1f32 ;5][..];
//     let inputs2 =  &[0.3f32 ;5][..];
//
//     {
//         let mut block1 = LogisticBlock::new_late(BlockData::new(2 , 5, 5));
//         block1.set_buffers(weights , inputs , output );
//         let mut block2 = LogisticMutBlock::new_late(BlockData::new(2 , 5, 5));
//         block2.set_buffers(weights2 , inputs2 , output2 );
//
//         let mut module = Module::new();
//         module.add_block(Box::new(block1) );
//         module.add_block(Box::new(block2) );
//         module.process_blocks();
//
//     }
//     assert_eq!(output, & [0.9241418; 5]);
//     assert_eq!(output2, & [0.0; 5]);  // should not process as no link
// }
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

// #[test]
// fn module_build_add_2blocks_staged()
// {
//     let weights =   & [ 0.5f32  ; 25];
//     let weights2 =   & [ 0.2f32  ; 25];
//
//     let mut output_buf =  vec! [0f32 ;5];
//     let mut output = & mut  output_buf[..];
// //    let inputs2 = &[ output];
//     let mut output2 = & mut  [0f32 ;5];
//
//     let mut inputs2data  = vec!  [ & mut output_buf[..] ];
//     let mut inputs2 = & inputs2data [..] as &[&[f32]];
//
//     let inputs  = &[ &[1f32 ;5][..]];
//
//
//     {
//         let mut block1 = LogisticMutBlock::new_late(BlockData::new(2 , 5, 5));
//         block1.set_buffers(weights , inputs , output );
//         let mut block2 = LogisticMutBlock::new_late(BlockData::new(2 , 5, 5));
//         block2.set_buffers(weights2 , inputs2 , output2 );
//
//         let mut module = Module::new();
//         let from = module.add_block( &(block1.as_blocktype()) );
//         let to = module.add_block( &(block2.as_blocktype()) );
//         module.add_link(from, to);
//         module.process_blocks();
//
//     }
//     assert_eq!(output, & [0.9241418; 5]);
//
// }
//
// ///TODO add blocks with output buf is input
// ///TODO  add blocks with output buf is input process
//
// #[test]
// fn module_build_add_node_process()
// {
//
//     let weights =   & [ 0.5f32  ; 25];
//     let mut outvec = vec! [0f32 ;5];
//     let mut output = & mut outvec [..];
//     let input  = vec! [2f32 ;5];
//     let inputs  = vec! [ &input[..]];
//
//     {
//         let mut block = LogisticMutBlock::new_late(BlockData::new(2 , 5, 5));
//         block.set_buffers(weights , &inputs[..] , output );
//
//         let mut module = Module::new();
//         module.add_block(Box::new( block) );
//         module.process_blocks();
//     }
//
//     assert_eq!(output, & [0.9933072f32; 5]);
// }


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

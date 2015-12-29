use anne::module::{Module};
// not sure why needed ....
extern crate anne;

//use std::ops::Range;
use anne::blocks::{LogisticBlock ,LogisticBBlock , BlockData , LinearByteBlock,LogisticBlockwLifetime};
use anne::blocks::neural::neuron::DefaultLogistic;
use anne::core::{Block , BlockBehaviour };
// , BlockData};



#[test]
fn module_new()
{
    Module::new();
}

// externally management data rather module looks dodgy
// need to use static .. we will then get module to create
#[test]
fn module_build_add_node()
{
    let mut module = Module::new();
    // let input =  ::anne::util::to_floats(1..6);
    // println!("input {:?}", input );
    // let mut output = vec! [0f32 ;3];
    //   static  WEIGHTS: & 'static  [f32] = & [ 0.5f32  ; 25];
    unsafe
        {
        static INPUT_BUF: &'static [f32] = &[1f32, 2f32, 3f32, 4f32, 5f32];
        static mut OUTPUT_BUF: & 'static mut [f32] = & mut [0f32; 3];
        static  WEIGHTS: & 'static  [f32] = & [ 1f32, 2f32, 3f32, 4f32, 5f32, 11f32, 12f32, 13f32, 14f32, 15f32, 0.1f32, 0.2f32, 0.3f32, 0.4f32, 0.5f32 ];

        let mut block = LogisticBlockwLifetime::new(BlockData::new(2 , 5, 5), WEIGHTS, OUTPUT_BUF, INPUT_BUF);
        //let weights = vec! [1f32, 2f32, 3f32, 4f32, 5f32, 11f32, 12f32, 13f32, 14f32, 15f32, 0.1f32, 0.2f32, 0.3f32, 0.4f32, 0.5f32 ];
        //    let mut block = ::anne::blocks::fullmesht::FullMeshTBlock::<f32,f32,DefaultLogistic>::new(BlockData::new(5 , 3, 5), &weights, & mut output, &input);
        //let mut block = LogisticBlockwLifetime::new(BlockData::new(5 , 3, 5), &weights, & mut output, &input);
        let mut box_block =  Box::new( block);
        module.add_box_block(box_block );
        //    block.process();
        assert_eq!(OUTPUT_BUF,& mut [0f32; 3]);
        //assert_eq!(OUTPUT_BUF, & [1f32, 1f32, 0.9959299f32]);
    }
}

#[test]
fn module_build_add_block()
{
    let weights =   & [ 0.5f32  ; 25];
    let mut outvec = vec! [0f32 ;5];
    let mut output = & mut outvec [..];
    let mut input  = vec! [1f32 ;5];
    let mut inputs  = vec! [ &input[..]];
    {
        let mut block = LogisticBlockwLifetime::new_late(BlockData::new(2 , 5, 5));
        block.set_buffers(weights , &inputs[..] , output );
        let mut module = Module::new();
        module.add_box_block(Box::new( block) );
    }
    assert_eq!(output,& [0f32; 5]);
}

#[test]
fn module_build_add_2blocks()
{
    let weights =   & [ 0.5f32  ; 25];
    //let mut outvec = vec! [0f32 ;5];
    let mut output = & mut  [0f32 ;5];
    let mut output2 = & mut  [0f32 ;5];

    let mut input  = vec! [1f32 ;5];
    let mut inputs  = vec! [ &input[..]];
    {
        let mut block1 = LogisticBlockwLifetime::new_late(BlockData::new(2 , 5, 5));
        block1.set_buffers(weights , &inputs[..] , output );
        let mut block2 = LogisticBlockwLifetime::new_late(BlockData::new(2 , 5, 5));
        block2.set_buffers(weights , &inputs[..] , output2 );
        let mut module = Module::new();
        module.add_box_block(Box::new( block1) );
        module.add_box_block(Box::new( block2) );

    }
    assert_eq!(output,& [0f32; 5]);
}

// add blocks with output buf is input
// add blocks with output buf is input process

#[test]
fn module_build_add_node_process()
{

    let weights =   & [ 0.5f32  ; 25];
    let mut outvec = vec! [0f32 ;5];
    let mut output = & mut outvec [..];
    let mut input  = vec! [2f32 ;5];
    let mut inputs  = vec! [ &input[..]];

    {

        //  let mut box_block =  Box::new( block);
        //  module.add_box_block(box_block );
        let mut block = LogisticBlockwLifetime::new_late(BlockData::new(2 , 5, 5));
        block.set_buffers(weights , &inputs[..] , output );
        let mut module = Module::new();
        module.add_box_block(Box::new( block) );
        module.process_blocks();
    }

    assert_eq!(output, & [0.9933072f32; 5]);

}


//fn module_build_add_node_w_module_allocation()


// add node with module doing allocation


//add node and prcocess

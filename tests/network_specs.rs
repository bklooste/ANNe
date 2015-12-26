// not sure why needed ....
extern crate anne;

use anne::blocks::{LogisticBlock , BlockData};
use anne::core::{Block};
// , BlockData};


    #[test]
     fn fullmesh_integration ()
     {
         unsafe
         {
             static input_buf: &'static [f32] = &[1f32, 2f32, 3f32, 4f32, 5f32];
             static mut output_buf: & 'static mut [f32] = & mut [1f32, 2f32, 3f32, 4f32, 5f32];
             static  weights: & 'static  [f32] = & [ 0f32  ; 5];

             let mut block_data = BlockData::new(5);
             block_data.neuron_count = 5;
             block_data.synapse_count = 5;

             let mut block  =  LogisticBlock::new(block_data
                     , weights
                     , output_buf
                     , input_buf
             );

             block.process_buffers();
             // assert output is correct

             assert_eq!(output_buf, & [0.5f32 ;5]);



         }// unsafe
     }

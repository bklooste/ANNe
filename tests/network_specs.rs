// not sure why needed ....
extern crate anne;

use anne::blocks::{LogisticBlock , BlockData};
use anne::core::{Block};
// , BlockData};


    #[test]
     fn fullmesh_integration_w0 ()
     {
         unsafe
         {
             static INPUT_BUF: &'static [f32] = &[1f32, 2f32, 3f32, 4f32, 5f32];
             static mut OUTPUT_BUF: & 'static mut [f32] = & mut [1f32, 2f32, 3f32, 4f32, 5f32];
             static  WEIGHTS: & 'static  [f32] = & [ 0f32  ; 5];

             let mut block_data = BlockData::new(5);
             block_data.neuron_count = 5;
             block_data.synapse_count = 5;

             let mut block = LogisticBlock::new(block_data, WEIGHTS, OUTPUT_BUF, INPUT_BUF);
             block.process_buffers();

             assert_eq!(OUTPUT_BUF, & [0.5f32 ;5]);
         }// unsafe
     }

     #[test]
      fn fullmesh_integration_w05 ()
      {
          unsafe
          {
              static INPUT_BUF: &'static [f32] = &[1f32, 2f32, 3f32, 4f32, 5f32];
              static mut OUTPUT_BUF: & 'static mut [f32] = & mut [1f32, 2f32, 3f32, 4f32, 5f32];
              static  WEIGHTS: & 'static  [f32] = & [ 0.5f32  ; 5];

              let mut block_data = BlockData::new(5);
              block_data.neuron_count = 5;
              block_data.synapse_count = 5;

              let mut block = LogisticBlock::new(block_data, WEIGHTS, OUTPUT_BUF, INPUT_BUF);
              block.process_buffers();

              assert_eq!(OUTPUT_BUF, & [0.99944717f32 ;5]);
          }// unsafe
      }

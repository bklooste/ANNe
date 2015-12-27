// not sure why needed ....
extern crate anne;

use anne::blocks::{LogisticBlock ,LogisticBBlock , BlockData , LinearByteBlock};
use anne::core::{Block};
// , BlockData};


// block tests 

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
        block.process();

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
      block.process();

      assert_eq!(OUTPUT_BUF, & [0.99944717f32 ;5]);
  }// unsafe
}

#[test]
fn fullmesh_bias_integration_w0 ()
{
   unsafe
   {
       static INPUT_BUF2: &'static [f32] = &[1f32, 2f32, 3f32, 4f32, 5f32];
       static mut OUTPUT_BUF2: & 'static mut [f32] = & mut [1f32, 2f32, 3f32, 4f32, 5f32];
       static  WEIGHTS2: & 'static  [f32] = & [ 0f32  ; 6];

       let mut block_data = BlockData::new(5);
       block_data.neuron_count = 5;
       block_data.synapse_count = 6;  //include bias

       let mut block = LogisticBBlock::new(block_data, WEIGHTS2, OUTPUT_BUF2, INPUT_BUF2);
       block.process();

       assert_eq!(OUTPUT_BUF2, & [0.5f32 ;5]);
   }// unsafe
}

#[test]
fn fullmesh_bias_integration_w05 ()
{
    unsafe
    {
        static INPUT_BUF: &'static [f32] = &[1f32, 2f32, 3f32, 4f32, 5f32];
        static mut OUTPUT_BUF: & 'static mut [f32] = & mut [1f32, 2f32, 3f32, 4f32, 5f32];
        static  WEIGHTS: & 'static  [f32] = & [ 0.5f32  ; 6];

        let mut block_data = BlockData::new(5);
        block_data.neuron_count = 5;
        block_data.synapse_count = 6;

        let mut block = LogisticBBlock::new(block_data, WEIGHTS, OUTPUT_BUF, INPUT_BUF);
        block.process();

        assert_eq!(OUTPUT_BUF, & [0.999089f32 ;5]);
    }// unsafe
}

#[test]
fn fullmesh_bias_weighti8_integration_w0_w_largebias ()
{
    unsafe
    {
         static INPUT_BUF2: &'static [u8] = &[1u8 ;5];
         static mut OUTPUT_BUF2: & 'static mut [u8] = & mut [0u8 ;5];
         static  WEIGHTS2: & 'static  [i8] = & [ 1i8  ; 9];

         let mut block_data = BlockData::new(5);
         block_data.neuron_count = 5;
         block_data.synapse_count = 9;

         let mut block = LinearByteBlock::new(block_data, WEIGHTS2, OUTPUT_BUF2, INPUT_BUF2);
         block.process();

         assert_eq!(OUTPUT_BUF2, & [0u8 ;5]);
    }// unsafe
}

#[test]
fn fullmesh_bias_weighti8_integration_w0_w_0bias ()
{
  unsafe
  {
      static INPUT_BUF2: &'static [u8] = &[1u8 ;5];
      static mut OUTPUT_BUF2: & 'static mut [u8] = & mut [0u8 ;5];
      static  WEIGHTS2: & 'static  [i8] = & [1i8 ,1i8,1i8,1i8,1i8,0i8,0i8,0i8,0i8];

      let mut block_data = BlockData::new(5);
      block_data.neuron_count = 5;
      block_data.synapse_count = 9;

      let mut block = LinearByteBlock::new(block_data, WEIGHTS2, OUTPUT_BUF2, INPUT_BUF2);
      block.process();

      assert_eq!(OUTPUT_BUF2, & [5u8 ;5]);
  }// unsafe
}


#[test]
fn fullmesh_bias_weighti8_integration_w0_w_1bias ()
{
    unsafe
    {
        static INPUT_BUF2: &'static [u8] = &[1u8 ;5];
        static mut OUTPUT_BUF2: & 'static mut [u8] = & mut [ 1u8  ; 5];
        static  WEIGHTS2: & 'static  [i8] = & [ 1i8 , 1i8, 1i8, 1i8, 1i8 , 0i8 , 0i8 , 0i8 , 1i8];

        let mut block_data = BlockData::new(5);
        block_data.neuron_count = 5;
        block_data.synapse_count = 9;

        let mut block = LinearByteBlock::new(block_data, WEIGHTS2, OUTPUT_BUF2, INPUT_BUF2);
        block.process();

        assert_eq!(OUTPUT_BUF2, & [4u8 ;5]);
    }// unsafe
}

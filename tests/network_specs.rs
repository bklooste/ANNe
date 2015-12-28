// not sure why needed ....
extern crate anne;

use std::ops::Range;
use anne::blocks::{LogisticBlock ,LogisticBBlock , BlockData , LinearByteBlock,LogisticBlockwLifetime};
use anne::core::{Block};
// , BlockData};


fn to_floats ( range : Range<i32>) -> Vec<f32>
{
        println!("range {:?}", range);
        let mut val:Vec<f32> = Vec::new();
        for i in range {
            val.push(i as f32);
            println!("val {:?}", val);

        }
        val
}

// block tests

#[test]
fn fullmesh_integration_w0 ()
{
    unsafe
    {
        static INPUT_BUF: &'static [f32] = &[1f32, 2f32, 3f32, 4f32, 5f32];
        static mut OUTPUT_BUF: & 'static mut [f32] = & mut [1f32, 2f32, 3f32, 4f32, 5f32];
        static  WEIGHTS: & 'static  [f32] = & [ 0f32  ; 25];

        let mut block = LogisticBlock::new(BlockData::new(5 , 5, 5), WEIGHTS, OUTPUT_BUF, INPUT_BUF);
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
      static  WEIGHTS: & 'static  [f32] = & [ 0.5f32  ; 25];

      let mut block = LogisticBlock::new(BlockData::new(2 , 5, 5), WEIGHTS, OUTPUT_BUF, INPUT_BUF);
      block.process();

      assert_eq!(OUTPUT_BUF, & [0.99944717f32 ;5]);
  }// unsafe
}

#[test]
fn fullmesh_integration_w05_5x3 ()
{
  unsafe
  {
      static INPUT_BUF: &'static [f32] = &[1f32, 2f32, 3f32, 4f32, 5f32];
      static mut OUTPUT_BUF: & 'static mut [f32] = & mut [1f32, 2f32, 3f32];
      static  WEIGHTS: & 'static  [f32] = & [ 0.5f32  ; 15];

      let mut block = LogisticBlock::new(BlockData::new(5 , 3, 5), WEIGHTS, OUTPUT_BUF, INPUT_BUF);
      block.process();

      assert_eq!(OUTPUT_BUF, & [0.99944717f32 ;3]);
  }// unsafe
}

#[test]
fn fullmesh_integration_w15x05_5x3 ()
{
  unsafe
  {
      static INPUT_BUF: &'static [f32] = &[1f32, 2f32, 3f32, 4f32, 5f32];
      static mut OUTPUT_BUF: & 'static mut [f32] = & mut [1f32, 2f32, 3f32];
      static  WEIGHTS: & 'static  [f32] = & [ 0.5f32  ; 15];

      let mut block = LogisticBlock::new(BlockData::new(5 , 3, 5), WEIGHTS, OUTPUT_BUF, INPUT_BUF);
      block.process();

      assert_eq!(OUTPUT_BUF, & [0.99944717f32 ;3]);
  }// unsafe
}


#[test]
fn fullmesht_w15_5x3_wstatic ()
{
    unsafe
    {
      static INPUT_BUF: &'static [f32] = &[1f32, 2f32, 3f32, 4f32, 5f32];
      static mut OUTPUT_BUF: & 'static mut [f32] = & mut [0f32, 0f32, 0f32];
      static  WEIGHTS: & 'static  [f32] = & [ 1f32, 2f32, 3f32, 4f32, 5f32, 11f32, 12f32, 13f32, 14f32, 15f32, 0.1f32, 0.2f32, 0.3f32, 0.4f32, 0.5f32 ];


      let mut block = LogisticBlockwLifetime::new(BlockData::new(5 , 3, 5), WEIGHTS, OUTPUT_BUF, INPUT_BUF);
      block.process();

      assert_eq!(OUTPUT_BUF, & [1f32, 1f32, 0.9959299f32]);
  }
}

#[test]
fn fullmesht_w15_5x3()
{

      let input = & to_floats(1..6);
      println!("input {:?}", input );
      let mut output = & mut [0f32 ;3];
      let weights = & [ 1f32, 2f32, 3f32, 4f32, 5f32, 11f32, 12f32, 13f32, 14f32, 15f32, 0.1f32, 0.2f32, 0.3f32, 0.4f32, 0.5f32 ];
      {
          let mut block = LogisticBlockwLifetime::new(BlockData::new(5 , 3, 5), weights, output, input);
          block.process();
      }
      assert_eq!(output, & [1f32, 1f32, 0.9959299f32]);
}


#[test]
fn fullmesh_integration_w15_5x3 ()
{
  unsafe
  {
      static INPUT_BUF: &'static [f32] = &[1f32, 2f32, 3f32, 4f32, 5f32];
      static mut OUTPUT_BUF: & 'static mut [f32] = & mut [0f32, 0f32, 0f32];
      static  WEIGHTS: & 'static  [f32] = & [ 1f32, 2f32, 3f32, 4f32, 5f32, 11f32, 12f32, 13f32, 14f32, 15f32, 0.1f32, 0.2f32, 0.3f32, 0.4f32, 0.5f32 ];

      let mut block = LogisticBlock::new(BlockData::new(5 , 3, 5), WEIGHTS, OUTPUT_BUF, INPUT_BUF);
      block.process();

      assert_eq!(OUTPUT_BUF, & [1f32, 1f32, 0.9959299f32]);
  }// unsafe
}

#[test]
fn fullmesh_bias_integration_w0 ()
{
   unsafe
   {
       static INPUT_BUF2: &'static [f32] = &[1f32, 2f32, 3f32, 4f32, 5f32];
       static mut OUTPUT_BUF2: & 'static mut [f32] = & mut [1f32, 2f32, 3f32, 4f32, 5f32];
       static  WEIGHTS2: & 'static  [f32] = & [ 0f32  ; 30];

       let mut block = LogisticBBlock::new(BlockData::new(5 , 5, 6), WEIGHTS2, OUTPUT_BUF2, INPUT_BUF2);
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
        static  WEIGHTS: & 'static  [f32] = & [ 0.5f32  ; 30];

        let mut block = LogisticBBlock::new(BlockData::new(5 , 5, 6), WEIGHTS, OUTPUT_BUF, INPUT_BUF);
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
         static  WEIGHTS2: & 'static  [i8] = & [ 1i8  ; 45];

         let mut block = LinearByteBlock::new(BlockData::new(5 , 5, 9), WEIGHTS2, OUTPUT_BUF2, INPUT_BUF2);
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
      static  WEIGHTS2: & 'static  [i8] = & [1i8 ,1i8,1i8,1i8,1i8,0i8,0i8,0i8,0i8 , 1i8 ,1i8,1i8,1i8,1i8,0i8,0i8,0i8,0i8 , 1i8 ,1i8,1i8,1i8,1i8,0i8,0i8,0i8,0i8 , 1i8 ,1i8,1i8,1i8,1i8,0i8,0i8,0i8,0i8 , 1i8 ,1i8,1i8,1i8,1i8,0i8,0i8,0i8,0i8];

      let mut block = LinearByteBlock::new(BlockData::new(5 , 5, 9), WEIGHTS2, OUTPUT_BUF2, INPUT_BUF2);
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
        static  WEIGHTS2: & 'static  [i8] = & [1i8,1i8,1i8,1i8,1i8,0i8,0i8,0i8,1i8 , 1i8,1i8,1i8,1i8,1i8,0i8,0i8,0i8,1i8, 1i8,1i8,1i8,1i8,1i8,0i8,0i8,0i8,1i8 , 1i8,1i8,1i8,1i8,1i8,0i8,0i8,0i8,1i8 , 1i8,1i8,1i8,1i8,1i8,0i8,0i8,0i8,1i8];

        let mut block = LinearByteBlock::new(BlockData::new(5 , 5, 9), WEIGHTS2, OUTPUT_BUF2, INPUT_BUF2);
        block.process();

        assert_eq!(OUTPUT_BUF2, & [4u8 ;5]);
    }// unsafe
}

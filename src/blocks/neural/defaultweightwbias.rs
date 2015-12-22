use num::traits::Num;
use num::traits::ToPrimitive;
use num::{Float};

use blocks::neural::neuron::*;
use blocks::neural::testdata::*;

 fn get_bias_from_end<W: ToPrimitive>( array : &[W]  ) -> u32
 {
     // it is length 1 ..
     ((array[array.len() -4 ].to_u8().unwrap() as u32) << 24)
     + ((array[array.len() -3].to_u8().unwrap()  as u32) << 16)
     + ((array[array.len() -2 ].to_u8().unwrap()  as u32) << 8)
     + (array[array.len()].to_u8().unwrap()  as u32)
 // return (*pbyte << 24) | (*(pbyte + 1) << 16) | (*(pbyte + 2) << 8) | (*(pbyte + 3));
 }

// basic non performant weights good for debugging and comparisons
#[derive(Copy, Clone)]
pub struct DefaultWeightwBiasFunction;

// fixme DefaultWeightwBiasFunction should return usize for integer types.
//enum_primitive crate,
// we will need custom version of toprimative which checks ranges and never fails nor construct a option
impl <W:Num + ToPrimitive> WeightFunction<W , f32 > for DefaultWeightwBiasFunction
{
    #[inline]
    fn calc_weight(v: &[f32], weights: &[W]) -> f32
    {
        if  v.len() +1 != weights.len() {
            if  v.len() == weights.len() {
                panic!("weight does not have bias");
            }

            panic!("weight length  not the same as input vector");
        }


        let mut sum = 0f32;
        for (vn , weight) in v.iter().zip(weights.iter()) {
                sum = sum + vn * weight.to_f32().unwrap();
        }
        sum -  weights[weights.len()].to_f32().unwrap()
    }
}

impl <W:Num + ToPrimitive> WeightFunction<W , f64 > for DefaultWeightwBiasFunction
{
    #[inline]
    fn calc_weight(v: &[f64], weights: &[W]) -> f64
    {
        if  v.len() +1 != weights.len()         {
            if  v.len() == weights.len()         {
                panic!("weight does not have bias");}

                panic!("weight length  not the same as input vector");
        }


        let mut sum = 0f64;

        for (vn , weight) in v.iter().zip(weights.iter()) {
                sum = sum + vn * weight.to_f64().unwrap();
        }
        sum - weights[weights.len()].to_f64().unwrap()
    }
}

impl <W:Num+ ToPrimitive> WeightFunction<W , u8 > for DefaultWeightwBiasFunction
{
    #[inline]
    fn calc_weight(v: &[u8], weights: &[W]) -> u8
    {
        if  v.len() +4 != weights.len()         {
            if  v.len() == weights.len()         {
                panic!("weight does not have 4 byte bias");}


                panic!("weight length  not the same as input vector");
        }

        let bias = get_bias_from_end::<W>(weights);
        let mut sum = 0usize;
        for (vn , weight) in v.iter().zip(weights.iter()) {
                sum = sum + (vn * weight.to_u8().unwrap()) as usize;
        }
        // if ( sum <= bias )
        //     return 0;
        sum = sum - bias as usize;
        if sum > 255{ return 255u8; }
        sum.to_u8().unwrap()
    }
}

impl <W:Num+ ToPrimitive> WeightFunction<W , i8 > for DefaultWeightwBiasFunction
{
    #[inline]
    fn calc_weight(v: &[i8], weights: &[W]) -> i8
    {
        if  v.len() +4 != weights.len()         {
            if  v.len() == weights.len()         {
                panic!("weight does not have 4 byte bias");}

                panic!("weight length  not the same as input vector");
        }

        let bias = get_bias_from_end(weights);
        let mut sum = 0isize;
        for (vn , weight) in v.iter().zip(weights.iter()) {
             println!("sum bef {}",  sum);
             let add = weight.to_i8().unwrap() as isize;
             println!("sum add {}",  add);
             println!("sum vn {}",  vn);
             let mult = *vn as isize * add;
                sum = sum + (mult) as isize;
                     println!("sum aft {}",  sum);
        }
        sum = sum - bias as isize;
        if  sum > 128 {
                 println!("sum exceeded {}",  sum);
            return 127i8;
        }
        if  sum < -127 { return -128i8;}
        sum.to_i8().unwrap()
    }
}

impl <W:Num+ ToPrimitive> WeightFunction<W , i32 > for DefaultWeightwBiasFunction
{
    #[inline]
    fn calc_weight(v: &[i32], weights: &[W]) -> i32
    {
        if  v.len() != weights.len()         {
            panic!("weight length not the same as input vector");
        }

        let mut sum = 0isize;
        for (vn , weight) in v.iter().zip(weights.iter()) {
                sum = sum + (vn * weight.to_i32().unwrap()) as isize ;
        }
        sum.to_i32().unwrap()
    }
}

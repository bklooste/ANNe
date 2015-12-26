use std::marker::PhantomData;
use num::traits::Num;
use num::traits::ToPrimitive;

use blocks::neural::neuron::*;
use blocks::neural::activation::*;
#[allow(unused_imports)]
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

#[derive(Copy, Clone)]
pub struct DefaultWeightwBiasFunction<I, O, T> where T:ActivationFunction<I,O> {    _m: PhantomData<T>  , _i: PhantomData<I> , _o: PhantomData<O> }


// fixme DefaultWeightwBiasFunction should return isize for integer types.
//enum_primitive crate,
// we will need custom version of toprimative which checks ranges and never fails nor construct a option

// f32 could be more efficient if bias was inluded in v.

impl <W:Num + ToPrimitive , N: ActivationFunction<f32,f32>> Neuron<W , f32 > for DefaultWeightwBiasFunction<f32,f32,N>
{
    #[inline]
    fn eval(v: &[f32], weights: &[W]) -> f32
    {
        if  v.len() +1 != weights.len() {
            if  v.len() == weights.len() {
                panic!("weight does not have bias");
            }

            panic!("weight length  not the same as input vector");
        }

        if v.len() == 0 { return 0f32;}


        let mut sum = 0f32;
        for (vn , weight) in v.iter().zip(weights.iter()) {
                sum = sum + vn * weight.to_f32().unwrap();
        }
        sum = sum - weights[weights.len() -1].to_f32().unwrap();;

        N::activate(sum)
    }
}


impl <W:Num + ToPrimitive , N: ActivationFunction<f64,f64>> Neuron<W , f64 > for DefaultWeightwBiasFunction<f64,f64,N>
{
    #[inline]
    fn eval(v: &[f64], weights: &[W]) -> f64
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
        sum = sum - weights[weights.len() -1].to_f64().unwrap();

        N::activate(sum)
    }
}

impl <W:Num + ToPrimitive , N: ActivationFunction<isize,u8>> Neuron<W , u8 > for DefaultWeightwBiasFunction<isize,u8,N>
{
    #[inline]
    fn eval(v: &[u8], weights: &[W]) -> u8
    {
        if  v.len() +4 != weights.len()         {
            if  v.len() == weights.len()         {
                panic!("weight does not have 4 byte bias");}


                panic!("weight length  not the same as input vector");
        }

        let bias = get_bias_from_end::<W>(weights);
        let mut sum = 0isize;
        for (vn , weight) in v.iter().zip(weights.iter()) {
                sum = sum + (vn * weight.to_u8().unwrap()) as isize;
        }
        // if ( sum <= bias )
        //     return 0;
        sum = sum - bias as isize;
        N::activate(sum)
    }
}

impl <W:Num + ToPrimitive , N: ActivationFunction<isize,i8>> Neuron<W , i8 > for DefaultWeightwBiasFunction<isize,i8,N>
{
    #[inline]
    fn eval(v: &[i8], weights: &[W]) -> i8
    {
        if  v.len() +4 != weights.len()         {
            if  v.len() == weights.len()         {
                panic!("weight does not have 4 byte bias");}

                panic!("weight length  not the same as input vector");
        }

        let bias = get_bias_from_end(weights);
        let mut sum = 0isize;
        for (vn , weight) in v.iter().zip(weights.iter()) {
             let add = weight.to_i8().unwrap() as isize;
             let mult = *vn as isize * add;
             sum = sum + (mult) as isize;
        }
        sum = sum - bias as isize;

        N::activate(sum)
        }
}
//
// impl <W:Num+ ToPrimitive> Neuron<W , i32 > for DefaultWeightwBiasFunction
// {
//     #[inline]
//     fn eval(v: &[i32], weights: &[W]) -> i32
//     {
//         if  v.len() != weights.len()         {
//             panic!("weight length not the same as input vector");
//         }
//
//         let mut sum = 0isize;
//         for (vn , weight) in v.iter().zip(weights.iter()) {
//                 sum = sum + (vn * weight.to_i32().unwrap()) as isize ;
//         }
//         sum.to_i32().unwrap()
//     }
// }


    //    vec.push( (F32_VECTOR8 , F32_VECTOR8_1, 26f32)  );

    // #[test]
    // #[should_panic(expected = "weight length")]
    // fn test_default_weight_function_dif_len_input()
    // {
    //     let sum = DefaultNeuron::eval(I8_VECTOR1 , I8_VECTOR3 ) ;
    // }
    //
    // #[test]
    // fn test_default_weight_function_i8() {
    //     //let weightFunction : &Neuron<f32 ,f32 > = &DefaultNeuron;
    //
    //     info!("running default_weight_tests");
    //
    //
    //     let sum = DefaultNeuron::eval(I8_VECTOR1 , I8_VECTOR1 ) ;
    //     assert_eq!(1, sum);
    //         let sum = DefaultNeuron::eval(&[1 ,2, 3 ] , &[1,2,3] ) ;
    //         assert_eq!(14, sum);
    //         let sum = DefaultNeuron::eval(&[1f32 ; 3 ] , &[1f32 ; 3 ] ) ;
    //         assert_eq!(3f32, sum);
    // }

#[test]
fn test_default_weight_function_w_bias_f32_many() {
    info!("running default_weight_tests");

    for (v1, v2,result) in getf32datawbias()
    {
        //println!("{:?}",testdata );
        let sum = DefaultWeightwBiasFunction::<f32, f32,Linear>::eval(v1 , v2 ) ;
        if sum != result {
            //let str1 = ;
             println!("{}", format! ( "test fail v {:?} w {:?} expected {:?}" , v1 ,v2 , result ));
        }
        assert_eq!(result, sum);
    }
}

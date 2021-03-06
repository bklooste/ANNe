use std::marker::PhantomData;
use std::fmt::Debug;

use num::traits::Num;
use num::traits::ToPrimitive;

use blocks::neural::neuron::*;
use blocks::neural::activation::*;
#[allow(unused_imports)]
use blocks::neural::testdata::*;

 fn get_bias_from_end<W: ToPrimitive + Debug>( array : &[W]  ) -> i32
 {
     // it is length 1 ..
    let bias = ((array[array.len() -4 ].to_i8().unwrap() as i32) << 24)
     + ((array[array.len() -3].to_i8().unwrap()  as i32) << 16)
     + ((array[array.len() -2 ].to_i8().unwrap()  as i32) << 8)
     + (array[array.len() -1 ].to_i8().unwrap()  as i32);
 // return (*pbyte << 24) | (*(pbyte + 1) << 16) | (*(pbyte + 2) << 8) | (*(pbyte + 3));

    println!("bias {:?} w {:?} " ,bias ,array );
    bias
 }

#[derive(Copy, Clone)]
pub struct DefaultWeightwBias<I, O, T>
where O: Num  +Debug +Copy, I: Num  +Debug +Copy , T:ActivationFunction<I,O>
{    _m: PhantomData<T>  , _i: PhantomData<I> , _o: PhantomData<O> }


// fixme DefaultWeightwBias should return isize for integer types.
//enum_primitive crate,
// we will need custom version of toprimative which checks ranges and never fails nor construct a option

// f32 could be more efficient if bias was inluded in v.

impl <W:Num + Debug + ToPrimitive , N: ActivationFunction<f32,f32>> Neuron<W , f32 > for DefaultWeightwBias<f32,f32,N>
{
    #[inline]
    fn eval(v: &[f32], weights: &[W]) -> f32
    {
        if  v.len() +1 != weights.len() {
            if  v.len() == weights.len() {
                println!("v {:?} ", v );
                println!("weights {:?} ", weights );
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


impl <W:Num + ToPrimitive , N: ActivationFunction<f64,f64>> Neuron<W , f64 > for DefaultWeightwBias<f64,f64,N>
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

impl <W:Num + ToPrimitive +Debug , N: ActivationFunction<isize,u8>> Neuron<W , u8 > for DefaultWeightwBias<isize,u8,N>
{
    #[inline]
    fn eval(v: &[u8], weights: &[W]) -> u8
    {
        if  v.len() +4 != weights.len()         {
            println!("v{:?}",v );
            println!("weights{:?}",weights );

            if  v.len()+1 == weights.len()         {
                panic!("weight does not have 4 byte bias");}



                panic!("weight length  not the same as input vector");
        }

        let bias = get_bias_from_end::<W>(weights);
                println!("bias {:?}", bias );
        let mut sum = 0isize;
        for (vn , weight) in v.iter().zip(weights.iter()) {
                //sum = sum + (*vn as isize * weight.to_isize().unwrap()) ;
                let add = weight.to_isize().unwrap();

                let mult = *vn as isize * add;
                sum = sum + (mult) as isize;
                println!("add  {:?} mult {:?} sum {:?}", add , mult , sum );

        }
        // if ( sum <= bias )
        //     return 0;
        sum = sum - bias as isize;
        let res = N::activate(sum);
        println!("result {:?} sum {:?} bias {:?}", res , sum , bias );

        res
    }
}

impl <W:Num + ToPrimitive +Debug , N: ActivationFunction<isize,i8>> Neuron<W , i8 > for DefaultWeightwBias<isize,i8,N>
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
             let add = weight.to_isize().unwrap();

             let mult = *vn as isize * add;
             sum = sum + (mult) as isize;
             println!("add  {:?} mult {:?} sum {:?}", add , mult , sum );
        }
        sum = sum - bias as isize;
                println!("sum {:?} bias {:?}", sum , bias );
        N::activate(sum)
        }
}

///TODO
#[test]
fn test_large_bias()
{
    assert_eq!(0, 0);
}

///TODO
#[test]
fn test_large_negative_bias()
{
    assert_eq!(0, 0);
}

#[test]
fn test_default_weight_function_w_bias_f32_many()
{
    info!("running default_weight_tests");

    for (v1, v2,result) in getf32datawbias()
    {
        //println!("{:?}",testdata );
        let sum = DefaultWeightwBias::<f32, f32,Linear>::eval(v1 , v2 ) ;
        if sum != result {
            //let str1 = ;
             println!("{}", format! ( "test fail v {:?} w {:?} expected {:?}" , v1 ,v2 , result ));
        }
        assert_eq!(result, sum);
    }
}

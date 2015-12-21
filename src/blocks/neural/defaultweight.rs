use num::traits::Num;
use num::traits::ToPrimitive;

use blocks::neural::neuron::*;
use blocks::neural::testdata::*;


// basic non performant weights good for debugging and comparisons
#[derive(Copy, Clone)]
pub struct DefaultWeightFunction;

// fixme DefaultWeightFunction should return usize for integer types.
//enum_primitive crate,
// we will need custom version of toprimative which checks ranges and never fails nor construct a option
impl <W:Num + ToPrimitive> WeightFunction<W , f32 > for DefaultWeightFunction
{
    #[inline]
    fn calc_weight(v: &[f32], weights: &[W]) -> f32
    {
        if  v.len() != weights.len()         {
            panic!("weight length not the same as input vector");
        }

        let mut sum = 0f32;
        for (vn , weight) in v.iter().zip(weights.iter()) {
                sum = sum + vn * weight.to_f32().unwrap();
        }
        sum
//unsafe {
//     let a = [0u8, 0u8, 0u8, 0u8];
//
//     let b = mem::transmute::<[u8; 4], u32>(a);
    }
}

impl <W:Num + ToPrimitive> WeightFunction<W , f64 > for DefaultWeightFunction
{
    #[inline]
    fn calc_weight(v: &[f64], weights: &[W]) -> f64
    {
        if  v.len() != weights.len()         {
            panic!("weight length not the same as input vector");
        }

        let mut sum = 0f64;

        for (vn , weight) in v.iter().zip(weights.iter()) {
                sum = sum + vn * weight.to_f64().unwrap();
        }
        sum
    }
}

impl <W:Num+ ToPrimitive> WeightFunction<W , u8 > for DefaultWeightFunction
{
    #[inline]
    fn calc_weight(v: &[u8], weights: &[W]) -> u8
    {
        if  v.len() != weights.len()         {
            panic!("weight length not the same as input vector");
        }

        let mut sum = 0usize;
        for (vn , weight) in v.iter().zip(weights.iter()) {
            let add = weight.to_u8().unwrap() as usize;
            let mult = *vn as usize * add;
            sum = sum + (mult) as usize;
        }
        if sum > 255{ return 255u8; }
        sum.to_u8().unwrap()
    }
}

impl <W:Num+ ToPrimitive> WeightFunction<W , i8 > for DefaultWeightFunction
{
    #[inline]
    fn calc_weight(v: &[i8], weights: &[W]) -> i8
    {
        if  v.len() != weights.len()         {
            panic!("weight length not the same as input vector");
        }

        let mut sum = 0isize;
        for (vn , weight) in v.iter().zip(weights.iter()) {

             let add = weight.to_i8().unwrap() as isize;
             let mult = *vn as isize * add;
             sum = sum + (mult) as isize;

        }
        if  sum > 128 {
                 println!("sum exceeded {}",  sum);
            return 127i8;
        }
        if  sum < -127 { return -128i8;}
        sum.to_i8().unwrap()
    }
}

impl <W:Num+ ToPrimitive> WeightFunction<W , i32 > for DefaultWeightFunction
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




#[test]
#[should_panic(expected = "weight length")]
fn test_default_weight_function_dif_len_input()
{
    let sum = DefaultWeightFunction::calc_weight(I8_VECTOR1 , I8_VECTOR3 ) ;
}

#[test]
fn test_default_weight_function_i8() {
    //let weightFunction : &WeightFunction<f32 ,f32 > = &DefaultWeightFunction;

    info!("running default_weight_tests");


    let sum = DefaultWeightFunction::calc_weight(I8_VECTOR1 , I8_VECTOR1 ) ;
    assert_eq!(1, sum);
        let sum = DefaultWeightFunction::calc_weight(&[1 ,2, 3 ] , &[1,2,3] ) ;
        assert_eq!(14, sum);
        let sum = DefaultWeightFunction::calc_weight(&[1f32 ; 3 ] , &[1f32 ; 3 ] ) ;
        assert_eq!(3f32, sum);
}

#[test]
fn test_default_weight_function_i8_many() {
    //let weightFunction : &WeightFunction<f32 ,f32 > = &DefaultWeightFunction;
    info!("running default_weight_tests");

    for (v1, v2,result) in geti8data()
    {
        //println!("{:?}",testdata );
        let sum = DefaultWeightFunction::calc_weight(v1 , v2 ) ;
        if sum != result {
            //let str1 = ;
             println!("{}", format! ( "test fail v {:?} w {:?} expected {:?}" , v1 ,v2 , result ));
        }
        assert_eq!(result, sum);
    }
}


#[test]
fn test_default_weight_function_u8_many() {
    //let weightFunction : &WeightFunction<f32 ,f32 > = &DefaultWeightFunction;
    info!("running default_weight_tests");

    for (v1, v2,result) in getu8data()
    {
        //println!("{:?}",testdata );
        let sum = DefaultWeightFunction::calc_weight(v1 , v2 ) ;
        if sum != result {
            //let str1 = ;
             println!("{}", format! ( "test fail v {:?} w {:?} expected {:?}" , v1 ,v2 , result ));
        }
        assert_eq!(result, sum);
    }
}

#[test]
fn test_default_weight_function_f32_many() {
    //let weightFunction : &WeightFunction<f32 ,f32 > = &DefaultWeightFunction;
    info!("running default_weight_tests");

    for (v1, v2,result) in getf32data()
    {
        //println!("{:?}",testdata );
        let sum = DefaultWeightFunction::calc_weight(v1 , v2 ) ;
        if sum != result {
            //let str1 = ;
             println!("{}", format! ( "test fail v {:?} w {:?} expected {:?}" , v1 ,v2 , result ));
        }
        assert_eq!(result, sum);
    }
}

//uper::block::
// #[derive(Copy, Cltion = DefaultWeightFunction;
//   type BiasWeightFunction = PositiveOneBiasFunction;
// }
//one)] pub struct DefaultErrorGradient;
//
// impl ErrorGradient for DefaultErrorGradient {
//   #[inline(always)]
//   fn errhidden<A>(act: f64, sum: f64) -> f64 where A : ActivationFunction {
//     A::derivative(act) * sum
//   }
//   #[inline(always)]
//   fn erroutput<A>(exp: f64, act: f64) -> f64 where A : ActivationFunction {
//     A::derivative(act) * (exp - act)
//   }
// }
//
//
// /// Bias function that returns a random weight between -0.5 and 0.5.
// ///
// #[derive(Copy, Clone)] pub struct RandomBiasWeightFunction;
//,2f32,3f32{
//     let range = Range::new(-0.5f64, 0.5f64);
//     range.ind_sample(&mut thread_rng())
//   }
// }
//
//
// /// Returns -1 for each bias node.
// ///
// #[derive(Copy, Clone)] pub struct NegativeOneBiasFunction;
//
// impl BiasWeightFunction for NegativeOneBiasFunction {
//   #[inline] fn biasw() -> f64 { -1f64 }
// }
//
//
// /// Returns 1 for each bias node.
// ///
// #[derive(Copy, Clone)] pub struct PositiveOneBiasFunction;
//
// impl BiasWeightFunction for PositiveOneBiasFunction {
//   #[inline] fn biasw() -> f64 { 1f64 }
// }
//
//
// /// MSE Error function.
// ///
// #[derive(Copy, Clone)] pub struct MSEFunction;
//
// impl ErrorFunction for MSEFunction {
//   fn error<'a, I>(predictions: I, expected: I) -> f64
//     where I : Iterator<Item = &'a f64>
//   {
//     let mut n = 0f64;
//     let sum = predictions
//       .zip(expected)
//       .fold(0f64, |acc, (act, exp)| { n += 1f64; acc + num::pow((act - exp), 2) });
//     (1f64 / n) * sum
//   }
// }
//
//
// /// Cross Entropy error function.
// ///
// #[derive(Copy, Clone)] pub struct CEFunction;
//
// impl ErrorFunction for CEFunction {
//   fn error<'a, I>(predictions: I, expected: I) -> f64
//     where I : Iterator<Item = &'a f64>
//   {
//     let mut n = 0f64;
//     let sum = predictions
//       .zip(expected)
//       .fold(0f64, |acc, (act, exp)| { n += 1f64; acc + act.ln() * exp });
//     -(1f64 / n) * sum
//   }
// }uper::block::

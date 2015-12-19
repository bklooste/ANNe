use num::traits::Num;
use num::traits::ToPrimitive;
use num::{Float};
//use core::marker::Reflect;


//sigmoid activation & SIMD weights

/// Activation Function
///
pub trait ActivationFunction<O : Num>
{
   fn activation(x: O) -> O;
}

pub trait WeightFunction<W : Num , O: Num >
{
   fn calc_weight(ins: &[O], outs: &[W]) -> O;
}

pub trait NeuralNetParameters<W : Num , O: Num >
{
   type ActivationFunction : ActivationFunction<O>;
   type WeightFunction : WeightFunction<W, O>;
}


//common activations
#[derive(Copy, Clone, RustcEncodable, RustcDecodable)]
pub struct LogisticNeuralNet;

impl ActivationFunction<u8> for LogisticNeuralNet {
  #[inline(always)] fn activation(x: u8) -> u8 { 1  }
}

impl ActivationFunction<f32> for LogisticNeuralNet {
  #[inline(always)] fn activation(x: f32) -> f32 { 1f32 / (1f32 + (-x).exp()) }
}

impl ActivationFunction<f64> for LogisticNeuralNet {
  #[inline(always)] fn activation(x: f64) -> f64 { 1f64 / (1f64 + (-x).exp()) }
}

#[derive(Copy, Clone, RustcEncodable, RustcDecodable)]
pub struct TanhNeuralNet;

impl ActivationFunction<u8> for TanhNeuralNet {
  #[inline(always)] fn activation(x: u8) -> u8 { 1  }
}

impl ActivationFunction<f32> for TanhNeuralNet {
  #[inline(always)] fn activation(x: f32) -> f32 { x.tanh() }
}

impl ActivationFunction<f64> for TanhNeuralNet {
  #[inline(always)] fn activation(x: f64) -> f64 { x.tanh() }
  //derivative #[inline(always)] fn derivative(x: f64) -> f64 { 1f64 - x.tanh().powi(2) }
}

pub trait Convert<T : Num > :Num
{
   //fn mul(x: O , y: W) -> O;
   fn mulslice(Self ,  T ) -> T;
}
//
// impl<f32> Multiply for f32
// {
//http://starsautohost.org/sahforum2/index.php?t=msg&th=5494&start=75&rid=0
// }

// basic non performant weights good for debugging and comparisons
#[derive(Copy, Clone)]
pub struct DefaultWeightFunction;


// we will need custom version of toprimative for extreme cases

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
// }
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

        let mut sum = 0u8;
        for (vn , weight) in v.iter().zip(weights.iter()) {
                sum = sum + vn * weight.to_u8().unwrap();
        }
        sum
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

        let mut sum = 0i8;
        for (vn , weight) in v.iter().zip(weights.iter()) {
                sum = sum + vn * weight.to_i8().unwrap();
        }
        sum
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

        let mut sum = 0i32;
        for (vn , weight) in v.iter().zip(weights.iter()) {
                sum = sum + vn * weight.to_i32().unwrap();
        }
        sum
    }
}

pub fn add_foura(a: i32) -> i32 {
    a + 4
}

///TODO do the same for floats
static I8_VECTOR0   :& 'static[i8] = &[];
static I8_VECTOR1_N1   :& 'static[i8] = &[-1];
static I8_VECTOR1   :& 'static[i8] = &[1];
static I8_VECTOR1_0 :& 'static[i8] = &[0];
static I8_VECTOR3   :& 'static[i8] = &[1 ,2, 3 ];
static I8_VECTOR8   :& 'static[i8] = &[1 ,2, 3 ,4 ,5 ,6 ,7 ,8 ];
static I8_VECTOR8_0   :& 'static[i8] = &[0 ;8 ];
static I8_VECTOR8_1   :& 'static[i8] = &[1 ;8 ];
static I8_VECTOR4096_0   :& 'static[i8] = &[0 ;4096 ];
static I8_VECTOR4096_1   :& 'static[i8] = &[1 ;4096 ];
static I8_VECTOR4096_N1   :& 'static[i8] = &[-1 ;4096 ];
static I8_VECTOR4096_100   :& 'static[i8] = &[100 ;4096 ];

pub fn getdata<'a>() -> Vec<(& 'a [i8] , & 'a [i8] ,i8)>
{
    let mut vec = Vec::new();
    //same size
    vec.push( (I8_VECTOR0 , I8_VECTOR0, 0)  );
    vec.push( (I8_VECTOR1_N1 , I8_VECTOR1_N1, 0)  );
    vec.push( (I8_VECTOR1 , I8_VECTOR1, 0)  );
    vec.push( (I8_VECTOR1_0 , I8_VECTOR1_0, 0)  );
    vec.push( (I8_VECTOR3 , I8_VECTOR3, 0)  );
    vec.push( (I8_VECTOR8 , I8_VECTOR8, 0)  );
    vec.push( (I8_VECTOR8_0 , I8_VECTOR8_0, 0)  );
    vec.push( (I8_VECTOR8_1 , I8_VECTOR8_1, 0)  );

    vec.push( (I8_VECTOR4096_0 , I8_VECTOR4096_0, 0)  );
    vec.push( (I8_VECTOR4096_1 , I8_VECTOR4096_1, 0)  );
    vec.push( (I8_VECTOR4096_N1 , I8_VECTOR4096_N1, 0)  );
    vec.push( (I8_VECTOR4096_100 , I8_VECTOR4096_100, 0)  );

    //dif
    vec.push( (I8_VECTOR8_0 , I8_VECTOR8_1, 0)  );
    vec.push( (I8_VECTOR8 , I8_VECTOR8_1, 0)  );


    //Vec
    // array of tupples
    vec
}

#[test]
#[should_panic(expected = "assertion failed")]
fn test_default_weight_function_dif_len_input()
{
    let sum = DefaultWeightFunction::calc_weight(I8_VECTOR1 , I8_VECTOR3 ) ;
}

#[test]
fn test_default_weight_function() {
    //let weightFunction : &WeightFunction<f32 ,f32 > = &DefaultWeightFunction;

    info!("running default_weight_tests");

    for (v1, v2,result) in getdata()
    {
        //println!("{:?}",testdata );
        let sum = DefaultWeightFunction::calc_weight(v1 , v2 ) ;
        assert_eq!(result, sum);
    }

    let sum = DefaultWeightFunction::calc_weight(I8_VECTOR1 , I8_VECTOR1 ) ;
    assert_eq!(6, sum);
        let sum = DefaultWeightFunction::calc_weight(&[1 ,2, 3 ] , &[1,2,3] ) ;
        assert_eq!(6, sum);
        let sum = DefaultWeightFunction::calc_weight(&[1f32 ; 3 ] , &[1f32 ; 3 ] ) ;
        assert_eq!(6f32, sum);

        let x = [0..10];

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

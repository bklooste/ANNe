use blocks::neural::neuron::*;

// logistic

#[derive(Debug, Copy, Clone)]  // RustcEncodable, RustcDecodable
pub struct Logistic;  // logistic

impl ActivationFunction<f32, f32> for Logistic {
    #[inline(always)] fn activate(x: f32) -> f32 { 1f32 / (1f32 + (-x).exp()) }
}

impl ActivationFunction<f64, f64> for Logistic {
    #[inline(always)] fn activate(x: f64) -> f64 { 1f64 / (1f64 + (-x).exp()) }
}




//
// impl ActivationFunction<f64> for LogisticNeuralNet {
//   #[inline(always)] fn activate(x: f64) -> f64 { 1f64 / (1f64 + (-x).exp()) }
// }
//
#[derive(Copy, Clone, RustcEncodable, RustcDecodable)]
pub struct TanhNeuralNet;

impl ActivationFunction<u8,u8> for TanhNeuralNet {
  #[inline(always)] fn activate(x: u8) -> u8 { 1  }
}

impl ActivationFunction<f32,f32> for TanhNeuralNet {
  #[inline(always)] fn activate(x: f32) -> f32 { x.tanh() }
}

impl ActivationFunction<f64,f64> for TanhNeuralNet {
  #[inline(always)] fn activate(x: f64) -> f64 { x.tanh() }
  //derivative #[inline(always)] fn derivative(x: f64) -> f64 { 1f64 - x.tanh().powi(2) }
}


// waiting on specialization
// impl <W : Num , O: Num + Default > NeuralNetParameters<W, O> for Sigmoid
// {
//     type ActivationFunction = LogisticNeuralNet;
//     type Neuron = DefaultNeuron;
// }



    //    The functions are described with functions where
    //x is the input to the activate function,
    //y is the output,
    //s is the steepness and
    //d is the derivation.
    //FANN_LINEAR Linear activate function.
    //span: -inf<y<inf
    //y = x*s, d = 1*s
    //Can NOT be used in fixed point.
    //FANN_THRESHOLD  Threshold activate function.
    //x< 0 -> y = 0, x >= 0 -> y = 1
    //Can NOT be used during training.
    //FANN_THRESHOLD_SYMMETRIC    Threshold activate function.
    //x< 0 -> y = 0, x >= 0 -> y = 1
    //Can NOT be used during training.
    //FANN_SIGMOID    Sigmoid activate function.
    //One of the most used activate functions.
    //span: 0 < y< 1
    //y = 1/(1 + exp(-2*s*x))
    //d = 2* s* y*(1 - y)
    //FANN_SIGMOID_STEPWISE Stepwise linear approximation to sigmoid.
    //Faster than sigmoid but a bit less precise.
    //FANN_SIGMOID_SYMMETRIC Symmetric sigmoid activate function, aka.tanh.
    //One of the most used activate functions.
    //span: -1 < y< 1
    //y = tanh(s*x) = 2/(1 + exp(-2*s* x)) - 1
    //d = s*(1-(y* y))
    //FANN_SIGMOID_SYMMETRIC_STEPWISE Stepwise linear approximation to symmetric sigmoid.
    //Faster than symmetric sigmoid but a bit less precise.
    //FANN_GAUSSIAN Gaussian activate function.
    //0 when x = -inf, 1 when x = 0 and 0 when x = inf
    //span: 0 < y< 1
    //y = exp(-x*s*x*s)
    //d = -2* x* s* y* s
    //FANN_GAUSSIAN_SYMMETRIC Symmetric gaussian activate function.
    //-1 when x = -inf, 1 when x = 0 and 0 when x = inf
    //span: -1 < y< 1
    //y = exp(-x* s* x* s)*2-1
    //d = -2* x* s*(y+1)* s
    //FANN_ELLIOT Fast(sigmoid like) activate function defined by David Elliott
    //span: 0 < y< 1
    //y = ((x* s) / 2) / (1 + |x* s|) + 0.5
    //d = s*1/(2*(1+|x* s|)*(1+|x* s|))
    //FANN_ELLIOT_SYMMETRIC Fast(symmetric sigmoid like) activate function defined by David Elliott
    //span: -1 < y< 1
    //y = (x* s) / (1 + |x* s|)
    //d = s*1/((1+|x* s|)*(1+|x* s|))
    //FANN_LINEAR_PIECE Bounded linear activate function.
    //span: 0 < y< 1
    //y = x*s, d = 1*s
    //FANN_LINEAR_PIECE_SYMMETRIC Bounded Linear activate function.
    //span: -1 < y< 1
    //y = x*s, d = 1*s
    //FANN_SIN_SYMMETRIC  Periodical sinus activate function.
    //span: -1 <= y <= 1
    //y = sin(x* s)
    //d = s* cos(x* s)
    //FANN_COS_SYMMETRIC Periodical cosinus activate function.
    //span: -1 <= y <= 1
    //y = cos(x* s)
    //d = s*-sin(x* s)


        // note int and byte functions are always quant. to 0 and 1 or -1 to1.



        //        atan(pi* x/2)*2/pi   24.1 ns
        // atan(x)             23.0 ns
        //1/(1+exp(-x))       20.4 ns
        //1/sqrt(1+x^2)       13.4 ns  // great for parralel
        //erf(sqrt(pi)* x/2)    6.7 ns
        // tanh(x)              5.5 ns
        //x/(1+|x|)            5.5 ns // elliot

        // there are really 3 forms of each case
        // standard eg 0-1 output  where 0 = 0.5
        // Symmetric eg -1 to 1 output  where 0 = 0
        // Positive eg symetric where -1 output is 0 . Can use Symetric if input positive


//         public static double Tanh(double weightCalculation)
//         {
//             return Math.Tanh(weightCalculation);
//         }
//
//         // 0.5 * (x * alpha / (1 + abs(x*alpha)) + 0.5 is eqivalent to logistic
//         public static float ElliotSigmoid(float value)
//         {
//             return ( (value/2) / (1 + Math.Abs(value))) +0.5f;
//         }
//
//         public static float ElliotSigmoidPositive(float value)
//         {
//             if (value <= 0)
//                 return 0;
//             return SymmetricElliotSigmoid(value);
//         }
//
//         public static float SymmetricElliotSigmoid(float value)
//         {
//             return ((value / 2) / (1 + Math.Abs(value))) + 0.5f;
//         }
//
//         //        f(x) = x / (1 + abs(x))
//         //g(x) = ax / (a + abs(x) + 1)
//         //h(x) = ax / (a + abs(x) – 1)
//
//         // eg 1 = 1/2  , 2 = 2/3 etc max output is lim 1.
//
//
//             //0 -255 pos input
//         public static int ElliotSigmoidPositive(int value , int maxOut )
//     }
//
//         public static int ElliotSigmoidSymmetric(int value, int maxOut)
//         {
//             // quite a few optomizations can be made
//             if (value > maxOut * 64)
//                 return maxOut;
//             return (maxOut * value) / (maxOut + abs(value));  //  eg
//         }
//
//         public static int ElliotSigmoid(int value, int maxOut)
//         {
//             if (value > maxOut * 64)
//                 return maxOut;
//             return (maxOut * value /2) / (maxOut + abs(value)) + maxOut/2;  //  eg
//         }
//
//         //candidate to replace fast sig.
//         public static float FastSigmoid3(float signal)
//         {
//             if (signal >= 4f) return 1f;
//             float tmp = 1f - 0.25f * signal;
//             tmp *= tmp;
//             tmp *= tmp;
//             tmp *= tmp;
//             tmp *= tmp;
//             return 1f / (1f + tmp);
//         }
//
//
//
//         //1/(1 + 0.3678749025^x)
//
//         static int abs(int weightCalculation)
//         {
//             var mask = weightCalculation >> 31;
//             weightCalculation ^= mask;
//             weightCalculation += mask & 1;
//             return weightCalculation;
//         }
//
//         public static float SigmoidSymmetric(float weightCalculation)
//         {
//             return (float)(2.0f / (1.0f + Math.Exp(-2.0f * weightCalculation)) - 1.0f );
//         }
//
//         public static float LogisticFunction(float weightCalculation)
//         {
//             return (float) (1.0f / (1.0f + Math.Exp(-2.0f * weightCalculation)));
//         }
//
//
//
//
//     }
// }




// impl ILayer for Sigmoid {
//     impl_neuron_layer!();
//
//     fn forward_cpu(&self, bottom: &[ReadBlob], top: &mut Vec<&mut WriteBlob>) {
//         let bottom_data = bottom[0].cpu_data();
//         let top_data = top[0].mutable_cpu_data();
//
//         for (i, _) in bottom_data.iter().enumerate() {
//             top_data[i] = Sigmoid::sigmoid(bottom_data[i])
//         }
//     }
//
//     fn backward_cpu(&self, top: &[HeapBlob], propagate_down: &[bool], bottom: &mut Vec<HeapBlob>) {
//         if propagate_down[0] {
//             let top_data = top[0].cpu_data();
//             let top_diff = top[0].cpu_diff();
//             let count = bottom[0].len();
//             let bottom_diff = bottom[0].mutable_cpu_diff();
//
//             for i in 0..count {
//                 let sigmoid_x = top_data[i];
//                 // bottom_diff[i] = top_diff[i] * sigmoid_x * (1f32 - sigmoid_x);
//                 bottom_diff[i] = top_diff[i] * Sigmoid::sigmoid_prime_precalc(sigmoid_x)
//             }
//         }
//     }
// }
//
// impl Sigmoid {
//     fn sigmoid(z: f32) -> f32 {
//         1f32 / ,2f32,3f32(1f32 + (-z).exp())
//     }
//
//     fn sigmoid_prime(z: f32) -> f32 {
//         Sigmoid::sigmoid_prime_precalc(Sigmoid::sigmoid(z))
//     }
//
//     fn sigmoid_prime_precalc(sigmoid_z: f32) -> f32 {
//         sigmoid_z * (1f32 - sigmoid_z)
//     }
// }

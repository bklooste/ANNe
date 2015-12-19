use num::traits::Num;
use core::*;

use super::neural::Sigmoid;
use super::fullmesh::FullMeshBlock;
use super::block::BlockData;

    //use super::block;

    //static input_buf: Vec<f32> = Vec::new();

    //  input_buf: &'static [f32] = &'static[1f32, 2f32, 3f32, 4f32, 5f32];
    // output_buf: & mut 'static [f32] = & mut 'static [1f32, 2f32, 3f32, 4f32, 5f32];

    // let input_buf  : Vec<f32> = Vec::new();
    // let mut output_buf  : Vec<f32> = Vec::new();

    #[test]
    fn it_works3() {
        assert_eq!(5, super::block::add_three(2));
    }



    #[test]
    fn create_fullmesh_bloc ()
    {
        unsafe {
        let id = 5;
        // let input_buf  : Vec<f32> = Vec::new();
        // let mut output_buf  : Vec<f32> = Vec::new();

        //static x: &'static [u8] = &[1,2,3];

        // static input_buf: &'static [f32] = &[1f32, 2f32, 3f32, 4f32, 5f32];
        // static mut output_buf: & 'static mut [f32] = & mut [1f32, 2f32, 3f32, 4f32, 5f32];
        static input_buf: &'static [f32] = &[1f32, 2f32, 3f32, 4f32, 5f32];
        static mut output_buf: & 'static mut [f32] = & mut [1f32, 2f32, 3f32, 4f32, 5f32];
        static  weights: & 'static  [f32] = & [0f32; 500];

        //let weights  : Vec<f32> ,2f32,3f32= Vec::new();
        //let block  =  FullMeshBlock::<f32,f32,Sigmoid>::qnew(1 , 5, 2, &weights[..]) ;
        let block  =  FullMeshBlock::<f32,f32,Sigmoid>::new(BlockData::new(id)
                , weights
                , output_buf
                , input_buf
        );
    }; //unsafe
}


    //
    // [Theory, MemberData("WeightDataOne")]
    // public void simple_process_tests(byte[] value, uint numneurons, sbyte[][] weights, byte[] expected)
    // {
    //     var options = new NeuronBlockOptions() { inputSize = (uint)value.Length, numNeurons = numneurons, behaviour = new TestNeuron<U>(1) };
    //     va    assert_eq!(6, super::neuron::add_foura(2));r result = SetAndProcess(  options, value, weights);
    //     //var result = SetAndRetrieve(value, size, offset);
    //     Assert.Equal (expected , result );
    //     //AssertOffsetArrayEqual(value, offset, result);
    //
    // }
    //
    // [Theory
    // , InlineData(new byte[] { 1 }, 1, 1, 1, new byte[] { 1 })
    // , InlineData(new byte[] { 1 }, 1, 1, 0, new byte[] { 0 })
    // ]
    // public void test_activation_is_returned(byte[] inputs, uint numneurons, sbyte weightsValue, float activationValue, byte[] expected)
    // {
    //     var weights = GetWeightsAndSetValue(inputs.Length, numneurons, weightsValue);
    //     var options = new NeuronBlockOptions() { inputSize = (uint)inputs.Length, numNeurons = numneurons, behaviour = new TestNeuron<U>(activationValue) };
    //     var result = SetAndProcess(options, inputs, weights);
    //     Assert.Equal(expected, result);
    //
    // }
    //
    // // this willl only work with an sbyte block not all
    // [Theory
    // //, InlineData(new byte[] { 1 }, 1, 1, 1)
    // //, InlineData(new byte[] { 1 }, 1, 1, 0)
    //     , InlineData(new byte[] { 1 }, 1, 1, -1)
    // ]
    // public void test_activation_signed_is_returned(byte[] inputs, uint numneurons, sbyte weightsValue, float activationValue)
    // {
    //
    //     if (typeof(U) == typeof(byte))
    //         return;
    //
    //     byte[] expected = new byte[1];
    //     if (activationValue.GetType() == typeof(float))
    //         expected[0] =  (byte) Convert.ToSByte((float)activationValue);  // fixme crimp ?
    //     else
    //         expected[0] = (byte)activationValue;
    //
    //     var weights = GetWeightsAndSetValue(inputs.Length, numneurons, weightsValue);
    //     var options = new NeuronBlockOptions() { inputSize = (uint)inputs.Length, numNeurons = numneurons, behaviour = new TestNeuron<U>(activationValue) };
    //     var result = SetAndProcess(options, inputs, weights);
    //     Assert.Equal(expected, result);
    //
    // }
    //
    // sbyte[][] GetWeightsAndSetValue(int inputLength, uint numneurons, sbyte weightsValue)
    // {
    //     sbyte[][] weights = new sbyte[numneurons][];
    //     for (int i = 0; i < numneurons; i++)
    //     {
    //         weights[i] = new sbyte[inputLength + 4];
    //         for (int j = 0; j < inputLength; j++)
    //             weights[i][j] = weightsValue;
    //     }
    //     return weights;
    // }
    //
    // [Theory ,MemberData("WeightData")]
    // public void simple_process_w_weights_tests(byte[] value, uint numneurons, sbyte[][] weights, byte[] expected)
    // {
    //     var options = new NeuronBlockOptions() { inputSize = (uint)value.Length, numNeurons = numneurons, behaviour = new TestNeuronwWeights<U>() };
    //     var result = SetAndProcess(options, value, weights);
    //     Assert.Equal(expected, result);
    // }

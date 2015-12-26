use super::fullmesh::FullMeshBlock;
use blocks::BlockData;
#[allow(unused_imports)]
use blocks::neural::defaultweight::DefaultNeuron;
use blocks::neural::activation::Logistic;

    //use super::block;

    //static input_buf: Vec<f32> = Vec::new();

    //  input_buf: &'static [f32] = &'static[1f32, 2f32, 3f32, 4f32, 5f32];
    // output_buf: & mut 'static [f32] = & mut 'static [1f32, 2f32, 3f32, 4f32, 5f32];

    // let input_buf  : Vec<f32> = Vec::new();
    // let mut output_buf  : Vec<f32> = Vec::new();

    #[test]
    fn it_works3() {
        assert_eq!(5, ::blocks::add_three(2));
    }



    #[test]
    fn create_fullmesh_bloc ()
    {

        unsafe
        {
            static input_buf: &'static [f32] = &[1f32, 2f32, 3f32, 4f32, 5f32];
            static mut output_buf: & 'static mut [f32] = & mut [1f32, 2f32, 3f32, 4f32, 5f32];
            static  weights: & 'static  [f32] = & [0f32; 500];

            let mut block_data = BlockData::new(5);
            block_data.neuron_count = 5;
            block_data.synapse_count = 5;

            let block  =  FullMeshBlock::<f32,f32,DefaultNeuron<Logistic>>::new(block_data
                    , weights
                    , output_buf
                    , input_buf
            );
        }// unsafe
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
    // public void test_activate_is_returned(byte[] inputs, uint numneurons, sbyte weightsValue, float activateValue, byte[] expected)
    // {
    //     var weights = GetWeightsAndSetValue(inputs.Length, numneurons, weightsValue);
    //     var options = new NeuronBlockOptions() { inputSize = (uint)inputs.Length, numNeurons = numneurons, behaviour = new TestNeuron<U>(activateValue) };
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
    // public void test_activate_signed_is_returned(byte[] inputs, uint numneurons, sbyte weightsValue, float activateValue)
    // {
    //
    //     if (typeof(U) == typeof(byte))
    //         return;
    //
    //     byte[] expected = new byte[1];
    //     if (activateValue.GetType() == typeof(float))
    //         expected[0] =  (byte) Convert.ToSByte((float)activateValue);  // fixme crimp ?
    //     else
    //         expected[0] = (byte)activateValue;
    //
    //     var weights = GetWeightsAndSetValue(inputs.Length, numneurons, weightsValue);
    //     var options = new NeuronBlockOptions() { inputSize = (uint)inputs.Length, numNeurons = numneurons, behaviour = new TestNeuron<U>(activateValue) };
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

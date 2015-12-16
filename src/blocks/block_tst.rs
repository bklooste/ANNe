

use num::traits::Num;
use core::*;
use super::activation::Sigmoid;
use super::block::FullMeshBlock;
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

        static  weights: & 'static  [f32] = & [1f32, 2f32, 3f32, 4f32, 5f32];

        //let weights  : Vec<f32> = Vec::new();
        //let block  =  FullMeshBlock::<f32,f32,Sigmoid>::qnew(1 , 5, 2, &weights[..]) ;
        let block  =  FullMeshBlock::<f32,f32,Sigmoid>::new(BlockData::new(id)
                , weights
                , output_buf
                , input_buf
        );

    };

        //process2::<FullMeshBlock> ( block);
        // process::<f32 , f32 , Sigmoid <f32 , Output = f32>> ( block);
        //http://stackoverflow.com/questions/27567849/what-makes-something-a-trait-object

        assert_eq!(5, super::block::add_three(2));
    }

    // we can alias these !
//Neuron<f32, Output=f32>
    // fn process<W : Num, O : Num ,  T : Neuron< W , Output = O>> ( block : Block<W , T , Output = O>)
    // {
    //
    // }


//IoResult<T> type is an alias for the Result<T, IoError> type.
//type Inch = u64;

// redefine type
    // fn process2< T : Block> ( block : T)
    // {
    //     block.process();
    // }

    // struct C;
    //
    // struct B<'b> {
    //     c: &'b C,2
    // struct A<'a> {
    //     b: B<'a>,
    //     c: &'a C
    // }
    //
    // fn main1() {
    //     let c1 = C;
    //     let _ = A::new(&c1);
    // }



//     [Fact]
// public void size_0_is_illegal()
// {
//     Assert.Throws<System.ArgumentException>(() => new FullMeshNeuronProcessingBehaviour(0));
// }
//
// [Fact]
// public void get_vector_with_no_set_illegal()
// {
//     Assert.Throws<System.InvalidOperationException>(() =>
//     {
//         var mesh = new FullMeshNeuronProcessingBehaviour(10);
//         mesh.GetVectorForNeuron(1);
//
//     });
// }
//
//
// [Fact]
// public void get_vector_with_set()
// {
//     Assert.Throws<System.InvalidOperationException>(() =>
//     {
//         var mesh = new FullMeshNeuronProcessingBehaviour(10);
//         mesh.GetVectorForNeuron(1);
//
//     });
// }
//
//
// [Theory
//     , InlineData(new byte[] {1, 2, 3} ,  0 )
//     , InlineData(new byte[] {  }, 0)
//     , InlineData(new byte[] { 6, 7 , 8, 1, 2, 3 }, 0)
//     , InlineData(new byte[] { 0 }, 0)
//     , InlineData(new byte[] { 1,1,1,1,1,1,1,1,1,1,1,1 }, 0)
//
// ]
// public void set_vector_0_offset(byte[] value , uint offset)
// {2
//     SetAndRetrieve(value, (uint) value.Length, offset);
//
// }
//
// [Theory
// , InlineData(new byte[] { 1, 2, 3 }, 3, 0)
// , InlineData(new byte[] { }, 5, 0)
// , InlineData(new byte[] { 6, 7, 8, 1, 2, 3 }, 20, 0)
// ]
// public void set_vector_0_offset_bigger_size(byte[] value, uint size, uint offset)
// {
//
//     SetAndRetrieve(value, size, offset);
//
// }
//
// [Theory
//     , InlineData(new byte[] {1, 2, 3} , 3,  1 )
//     , InlineData(new byte[] {  }, 5 , 2)
//     , InlineData(new byte[] { 6, 7 , 8, 1, 2, 3 },  20 ,3)
//     , InlineData(new byte[] { 0 }, 1, 4)
//     , InlineData(new byte[] { 1,1,1,1,1,1,1,1,1,1,1,1 }, 100, 5)
//
// ]
// public void set_vector_n_offset(byte[] value , uint size , uint offset)
// {
//
//     SetAndRetrieve(value, size,  offset);
//
// }
//
// void SetAndRetrieve(byte[] value, uint initialLength ,  uint offset)
// {
//     var mesh = new FullMeshNeuronProcessingBehaviour(initialLength);
//     mesh.LoadVector(value, offset);
//     var result = mesh.GetVectorForNeuron(1);
//
//     Assert.Equal(value , result);
// }

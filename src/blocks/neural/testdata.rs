
///TODO do the same for floats
pub static I8_VECTOR0   :& 'static[i8] = &[];
pub static I8_VECTOR1_N1   :& 'static[i8] = &[-1];
pub static I8_VECTOR1   :& 'static[i8] = &[1];
pub static I8_VECTOR1_0 :& 'static[i8] = &[0];
pub static I8_VECTOR3   :& 'static[i8] = &[1 ,2, 3 ];
pub static I8_VECTOR8   :& 'static[i8] = &[1 ,2, 3 ,4 ,5 ,6 ,7 ,8 ];
pub static I8_VECTOR8_M   :& 'static[i8] = &[1 ,-2, 3 ,-4 ,5 ,-6 ,7 ,-8 ];
pub static I8_VECTOR8_0   :& 'static[i8] = &[0 ;8 ];
pub static I8_VECTOR8_1   :& 'static[i8] = &[1 ;8 ];
pub static I8_VECTOR4096_0   :& 'static[i8] = &[0 ;4096 ];
pub static I8_VECTOR4096_1   :& 'static[i8] = &[1 ;4096 ];
pub static I8_VECTOR4096_N1   :& 'static[i8] = &[-1 ;4096 ];
pub static I8_VECTOR4096_100   :& 'static[i8] = &[100 ;4096 ];  // large for Over flow
pub static I8_VECTOR4096_N100   :& 'static[i8] = &[-100 ;4096 ];

pub fn geti8data<'a>() -> Vec<(& 'a [i8] , & 'a [i8] ,i8)>
{
    let mut vec = Vec::new();
    //same size
    vec.push( (I8_VECTOR0 , I8_VECTOR0, 0)  );
    vec.push( (I8_VECTOR1_N1 , I8_VECTOR1_N1, 1)  );
    vec.push( (I8_VECTOR1 , I8_VECTOR1, 1)  );
    vec.push( (I8_VECTOR1_0 , I8_VECTOR1_0, 0)  );
    vec.push( (I8_VECTOR3 , I8_VECTOR3, 14)  );
    vec.push( (I8_VECTOR8 , I8_VECTOR8, 127)  );
    vec.push( (I8_VECTOR8_1 , I8_VECTOR8_1, 8)  );

    vec.push( (I8_VECTOR4096_0 , I8_VECTOR4096_0, 0)  );
    vec.push( (I8_VECTOR4096_1 , I8_VECTOR4096_1, 127)  );
    vec.push( (I8_VECTOR4096_N1 , I8_VECTOR4096_N1, 127)  );
    vec.push( (I8_VECTOR4096_100 , I8_VECTOR4096_100, 127)  );
    vec.push( (I8_VECTOR4096_N100 , I8_VECTOR4096_100, -128)  );

    // mixed
    vec.push( (I8_VECTOR8_1 , I8_VECTOR8_M, -4)  );
    vec.push( (I8_VECTOR8 , I8_VECTOR8_M, -36)  );
    vec.push( (I8_VECTOR8_M , I8_VECTOR8_M, 127)  );


    //dif
    vec.push( (I8_VECTOR8_0 , I8_VECTOR8_1, 0)  );
    vec.push( (I8_VECTOR8 , I8_VECTOR8_1, 36)  );
    vec.push( (I8_VECTOR1 , I8_VECTOR1_N1, -1)  );


    //Vec
    // array of tupples
    vec
}

    ///TODO do the same for floats
    pub static U8_VECTOR0   :& 'static[u8] = &[];
    pub static U8_VECTOR1   :& 'static[u8] = &[1];
    pub static U8_VECTOR1_0 :& 'static[u8] = &[0];
    pub static U8_VECTOR3   :& 'static[u8] = &[1 ,2, 3 ];
    pub static U8_VECTOR8   :& 'static[u8] = &[1 ,2, 3 ,4 ,5 ,6 ,7 ,8 ];
    pub static U8_VECTOR8_M   :& 'static[u8] = &[1 ,2, 1 , 3 ,1  ,2 ,1,1 ];
    pub static U8_VECTOR8_0   :& 'static[u8] = &[0 ;8 ];
    pub static U8_VECTOR8_1   :& 'static[u8] = &[1 ;8 ];
    pub static U8_VECTOR4096_0   :& 'static[u8] = &[0 ;4096 ];
    pub static U8_VECTOR4096_1   :& 'static[u8] = &[1 ;4096 ];
    pub static U8_VECTOR4096_100   :& 'static[u8] = &[100 ;4096 ];  // large for Over flow
    pub static U8_VECTOR4096_255   :& 'static[u8] = &[255 ;4096 ];

    pub fn getu8data<'a>() -> Vec<(& 'a [u8] , & 'a [u8] ,u8)>
    {
        let mut vec = Vec::new();
        //same size
        vec.push( (U8_VECTOR0 , U8_VECTOR0, 0)  );
        vec.push( (U8_VECTOR1 , U8_VECTOR1, 1)  );
        vec.push( (U8_VECTOR1_0 , U8_VECTOR1_0, 0)  );
        vec.push( (U8_VECTOR3 , U8_VECTOR3, 14)  );
        vec.push( (U8_VECTOR8 , U8_VECTOR8, 204)  );
        vec.push( (U8_VECTOR8_1 , U8_VECTOR8_1, 8)  );

        vec.push( (U8_VECTOR4096_0 , U8_VECTOR4096_0, 0)  );
        vec.push( (U8_VECTOR4096_1 , U8_VECTOR4096_1, 255)  );
        vec.push( (U8_VECTOR4096_100 , U8_VECTOR4096_100, 255)  );
        vec.push( (U8_VECTOR4096_255 , U8_VECTOR4096_100, 255)  );

        // mixed
        vec.push( (U8_VECTOR8_1 , U8_VECTOR8_M, 12)  );
        vec.push( (U8_VECTOR8 , U8_VECTOR8_M, 52)  );
        vec.push( (U8_VECTOR8_M , U8_VECTOR8_M, 22)  );


        //dif
        vec.push( (U8_VECTOR8_0 , U8_VECTOR8_1, 0)  );
        vec.push( (U8_VECTOR8 , U8_VECTOR8_1, 36)  );


        //Vec
        // array of tupples
        vec
    }

    ///TODO do the same for floats
    pub static I32_VECTOR0   :& 'static[i32] = &[];
    pub static I32_VECTOR1_N1   :& 'static[i32] = &[-1];
    pub static I32_VECTOR1   :& 'static[i32] = &[1];
    pub static I32_VECTOR1_0 :& 'static[i32] = &[0];
    pub static I32_VECTOR3   :& 'static[i32] = &[1 ,2, 3 ];
    pub static I32_VECTOR8   :& 'static[i32] = &[1 ,2, 3 ,4 ,5 ,6 ,7 ,8 ];
    pub static I32_VECTOR8_M   :& 'static[i32] = &[1 ,-2, 3 ,-4 ,5 ,-6 ,7 ,-8 ];
    pub static I32_VECTOR8_0   :& 'static[i32] = &[0 ;8 ];
    pub static I32_VECTOR8_1   :& 'static[i32] = &[1 ;8 ];
    pub static I32_VECTOR4096_0   :& 'static[i32] = &[0 ;4096 ];
    pub static I32_VECTOR4096_1   :& 'static[i32] = &[1 ;4096 ];
    pub static I32_VECTOR4096_N1   :& 'static[i32] = &[-1 ;4096 ];
    pub static I32_VECTOR4096_100   :& 'static[i32] = &[100 ;4096 ];  // large for Over flow
    pub static I32_VECTOR4096_N100   :& 'static[i32] = &[-100 ;4096 ];

    pub fn geti32data<'a>() -> Vec<(& 'a [i32] , & 'a [i32] ,i32)>
    {
        let mut vec = Vec::new();
        //same size
        vec.push( (I32_VECTOR0 , I32_VECTOR0, 0)  );
        vec.push( (I32_VECTOR1_N1 , I32_VECTOR1_N1, 1)  );
        vec.push( (I32_VECTOR1 , I32_VECTOR1, 1)  );
        vec.push( (I32_VECTOR1_0 , I32_VECTOR1_0, 0)  );
        vec.push( (I32_VECTOR3 , I32_VECTOR3, 14)  );
        vec.push( (I32_VECTOR8 , I32_VECTOR8, 204)  );
        vec.push( (I32_VECTOR8_1 , I32_VECTOR8_1, 8)  );

        vec.push( (I32_VECTOR4096_0 , I32_VECTOR4096_0, 0)  );
        vec.push( (I32_VECTOR4096_1 , I32_VECTOR4096_1, 127)  );
        vec.push( (I32_VECTOR4096_N1 , I32_VECTOR4096_N1, 127)  );
        vec.push( (I32_VECTOR4096_100 , I32_VECTOR4096_100, 127)  );
        vec.push( (I32_VECTOR4096_N100 , I32_VECTOR4096_100, -128)  );

        // mixed
        vec.push( (I32_VECTOR8_1 , I32_VECTOR8_M, -4)  );
        vec.push( (I32_VECTOR8 , I32_VECTOR8_M, -36)  );
        vec.push( (I32_VECTOR8_M , I32_VECTOR8_M, 127)  );


        //dif
        vec.push( (I32_VECTOR8_0 , I32_VECTOR8_1, 0)  );
        vec.push( (I32_VECTOR8 , I32_VECTOR8_1, 36)  );
        vec.push( (I32_VECTOR1 , I32_VECTOR1_N1, -1)  );


        //Vec
        // array of tupples
        vec
    }

    ///TODO do the same for floats
    pub static F32_VECTOR0   :& 'static[f32] = &[];
    pub static F32_VECTOR1_N1   :& 'static[f32] = &[-1f32];
    pub static F32_VECTOR1   :& 'static[f32] = &[1f32];
    pub static F32_VECTOR1_0 :& 'static[f32] = &[0f32];
    pub static F32_VECTOR3   :& 'static[f32] = &[1f32 ,2f32, 3f32 ];
    pub static F32_VECTOR8   :& 'static[f32] = &[1f32 ,2f32 , 3f32 ,4f32 ,5f32 ,6f32 ,7f32 ,8f32 ];
    pub static F32_VECTOR8_M   :& 'static[f32] = &[1f32 ,-2f32, 3f32 ,-4f32 ,5f32 ,-6f32 ,7f32 ,-8f32 ];
    pub static F32_VECTOR8_0   :& 'static[f32] = &[0f32 ;8 ];
    pub static F32_VECTOR8_1   :& 'static[f32] = &[1f32 ;8 ];
    pub static F32_VECTOR4096_0   :& 'static[f32] = &[0f32 ;4096 ];
    pub static F32_VECTOR4096_1   :& 'static[f32] = &[1f32 ;4096 ];
    pub static F32_VECTOR4096_N1   :& 'static[f32] = &[-1f32 ;4096 ];
    pub static F32_VECTOR4096_100   :& 'static[f32] = &[100f32 ;4096 ];  // large for Over flow
    pub static F32_VECTOR4096_N100   :& 'static[f32] = &[-100f32 ;4096 ];

    pub fn getf32data<'a>() -> Vec<(& 'a [f32] , & 'a [f32] ,f32)>
    {
        let mut vec = Vec::new();
        //same size
        vec.push( (F32_VECTOR0 , F32_VECTOR0, 0f32)  );
        vec.push( (F32_VECTOR1_N1 , F32_VECTOR1_N1, 1f32)  );
        vec.push( (F32_VECTOR1 , F32_VECTOR1, 1f32)  );
        vec.push( (F32_VECTOR1_0 , F32_VECTOR1_0, 0f32)  );
        vec.push( (F32_VECTOR3 , F32_VECTOR3, 14f32)  );
        vec.push( (F32_VECTOR8 , F32_VECTOR8, 204f32)  );
        vec.push( (F32_VECTOR8_1 , F32_VECTOR8_1, 8f32)  );

        vec.push( (F32_VECTOR4096_0 , F32_VECTOR4096_0, 0f32)  );
        vec.push( (F32_VECTOR4096_1 , F32_VECTOR4096_1, 4096f32)  );
        vec.push( (F32_VECTOR4096_N1 , F32_VECTOR4096_N1, 4096f32)  );
        vec.push( (F32_VECTOR4096_100 , F32_VECTOR4096_100, 40960000f32)  );
        vec.push( (F32_VECTOR4096_N100 , F32_VECTOR4096_100, -40960000f32)  );

        // mixed
        vec.push( (F32_VECTOR8_1 , F32_VECTOR8_M, -4f32)  );
        vec.push( (F32_VECTOR8 , F32_VECTOR8_M, -36f32)  );
        vec.push( (F32_VECTOR8_M , F32_VECTOR8_M, 204f32)  );


        //dif
        vec.push( (F32_VECTOR8_0 , F32_VECTOR8_1, 0f32)  );
        vec.push( (F32_VECTOR8 , F32_VECTOR8_1, 36f32)  );
        vec.push( (F32_VECTOR1 , F32_VECTOR1_N1, -1f32)  );


        //Vec
        // array of tupples
        vec
    }


    pub static F32_WEIGHT0   :& 'static[f32] = &[10f32];
    pub static F32_WEIGHT1_N1   :& 'static[f32] = &[-1f32, 10f32];
    pub static F32_WEIGHT1   :& 'static[f32] = &[1f32 , 10f32];
    pub static F32_WEIGHT1_0 :& 'static[f32] = &[0f32, 10f32];
    pub static F32_WEIGHT3   :& 'static[f32] = &[1f32 ,2f32, 3f32 , 10f32];
    pub static F32_WEIGHT8   :& 'static[f32] = &[1f32 ,2f32 , 3f32 ,4f32 ,5f32 ,6f32 ,7f32 ,8f32 , 10f32];
    pub static F32_WEIGHT8_M   :& 'static[f32] = &[1f32 ,-2f32, 3f32 ,-4f32 ,5f32 ,-6f32 ,7f32 ,-8f32 , 10f32];
    pub static F32_WEIGHT8_0   :& 'static[f32] = &[0f32 ;9 ];
    pub static F32_WEIGHT8_1   :& 'static[f32] = &[1f32 ;9 ];
    pub static F32_WEIGHT4096_0   :& 'static[f32] = &[0f32 ;4097 ];
    pub static F32_WEIGHT4096_1   :& 'static[f32] = &[1f32 ;4097 ];
    pub static F32_WEIGHT4096_N1   :& 'static[f32] = &[-1f32 ;4097 ];
    pub static F32_WEIGHT4096_100   :& 'static[f32] = &[100f32 ;4097 ];  // large for Over flow
    pub static F32_WEIGHT4096_N100   :& 'static[f32] = &[-100f32 ;4097 ];

    pub fn getf32datawbias<'a>() -> Vec<(& 'a [f32] , & 'a [f32] ,f32)>
    {
        let mut vec = Vec::new();
        //same size
        vec.push( (F32_VECTOR0 , F32_WEIGHT0, 0f32)  );
        vec.push( (F32_VECTOR1_N1 , F32_WEIGHT1_N1, -9f32)  );
        vec.push( (F32_VECTOR1 , F32_WEIGHT1, -9f32)  );
        vec.push( (F32_VECTOR1_0 , F32_WEIGHT1_0, -10f32)  );
        vec.push( (F32_VECTOR3 , F32_WEIGHT3, 4f32)  );
        vec.push( (F32_VECTOR8 , F32_WEIGHT8, 194f32)  );
        vec.push( (F32_VECTOR8_1 , F32_WEIGHT8_1, 7f32)  );

        vec.push( (F32_VECTOR4096_0 , F32_WEIGHT4096_0, 0f32)  );
        vec.push( (F32_VECTOR4096_1 , F32_WEIGHT4096_1, 4095f32)  );
        vec.push( (F32_VECTOR4096_N1 , F32_WEIGHT4096_N1, 4097f32)  );
        vec.push( (F32_VECTOR4096_100 , F32_WEIGHT4096_100, 40959900f32)  );
        vec.push( (F32_VECTOR4096_N100 , F32_WEIGHT4096_100, -40960100f32)  );

        // mixed
        vec.push( (F32_VECTOR8_1 , F32_WEIGHT8_M, -14f32)  );
        vec.push( (F32_VECTOR8 , F32_WEIGHT8_M, -46f32)  );
        vec.push( (F32_VECTOR8_M , F32_WEIGHT8_M, 194f32)  );


        //dif
        vec.push( (F32_VECTOR8_0 , F32_WEIGHT8_1, -1f32)  );
        vec.push( (F32_VECTOR8 , F32_WEIGHT8_1, 35f32)  );
        vec.push( (F32_VECTOR1 , F32_WEIGHT1_N1, -11f32)  );


        //Vec
        // array of tupples
        vec
    }

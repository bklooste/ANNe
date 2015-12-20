


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

    pub fn getdata<'a>() -> Vec<(& 'a [i8] , & 'a [i8] ,i8)>
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

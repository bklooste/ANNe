use std::ops::Range;
// pub use self::flatten::Flatten;
//
// pub mod flatten;

//pub use self::convolution::Convolution;

pub mod buffer;
//pub mod logger;

#[allow(dead_code)]
pub fn to_floats ( range : Range<i32>) -> Vec<f32>
{
        println!("range {:?}", range);
        let mut val:Vec<f32> = Vec::new();
        for i in range {
            val.push(i as f32);
            println!("val {:?}", val);

        }
        val
}

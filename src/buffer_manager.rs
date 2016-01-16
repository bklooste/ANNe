use std::collections::HashMap;
use std::cell::RefCell;
use std::{mem, slice };
use core::{BlockIndex};

pub type BufferIndex = u32;

#[derive(Default , Clone)]
pub struct BuffersForBlock
{
    // manages_own_data: bool,
    // copy_output_to_buffer: bool,
    pub data_buffer_id : BufferIndex, // eg weights
    pub output_buffer_id : BufferIndex,
    pub inputs_buffer_ids:  Vec<BufferIndex>
}


    //let input2 = self.buffers[ buffer_ids[INPUT2_DATA_INDEX]].borrow();

// all heap data
#[derive(Default , Clone)]
pub struct BufferManager
{

    buffers_for_block: HashMap< BlockIndex, BuffersForBlock>,
    buffers: Vec<RefCell<Vec<u8>>>,
    module_in_buffers: Vec<BufferIndex>,
    module_out_buffers: Vec<BufferIndex>,
}

impl BufferManager
{
    pub fn new() -> BufferManager
    {
        let mut  bm = BufferManager{  ..Default::default()}  ;

        //this needs to be better
        bm.module_in_buffers.push( 0);
        bm.module_out_buffers.push( 1);
        bm.buffers.push(RefCell::new( Vec::new() ));
        bm.buffers.push(RefCell::new(  Vec::new() ));

        bm
    }

    // pub fn get_data_for_block (&self , block_index: BlockIndex ) -> & mut [u8]
    // {
    //     & mut self.get_buffer(self.get_buffer_block_data(block_index).data_buffer_id).borrow_mut()[..]
    // }
    //
    // pub fn get_module_inputs (&self  ) -> Vec< &[u8] >
    // {
    //         let ref buf_ids = self.module_in_buffers;
    //         let mut buffs  = buf_ids.into_iter().map(|x| & self.get_buffer(*x).borrow() [..]).collect::<Vec<& [u8]>>();
    //         buffs //as  &[ &[u8]]
    // }
    //
    // pub fn get_module_ouputs (&self  ) -> Vec< &[u8]>
    // {
    //         let ref buf_ids = self.module_out_buffers;
    //         let mut buffs  = buf_ids.into_iter().map(|x| &self.get_buffer(*x).borrow() [..]).collect::<Vec<& [u8]>>();
    //         buffs //as  &[ &[u8]]
    // }
    //
    // pub fn get_inputs_for_block (&self , block_index: BlockIndex ) -> Vec<&[u8]>
    // {
    //         let ref buf_ids = self.get_buffer_block_data(block_index).inputs_buffer_ids;
    //         let mut buffs  = buf_ids.into_iter().map(|x| &self.get_buffer(*x).borrow() [..]).collect::<Vec<& [u8]>>();
    //         buffs //as  &[ &[u8]]
    // }
    //
    //
    //
    // pub fn get_output_for_block (&self , block_index: BlockIndex ) -> & mut [u8]
    // {
    //     & mut self.get_buffer(self.get_buffer_block_data(block_index).output_buffer_id).borrow_mut()[..]
    // }
    //
    // pub fn get_common_buffers_for_block (& mut self , block_index: BlockIndex ) -> ( & mut [u8] ,& mut [u8] ,& mut [u8])
    // {
    //     let ref bufs =
    //         {self.get_buffer_block_data(block_index)};
    //
    //     //let inputs = bufs.inputs_buffer_ids.iter().map(|&x| self.get_buffer(x) as &[u8]).collect::<Vec<& [u8]>>();
    //
    //     (
    //         & mut self.get_buffer(bufs.data_buffer_id).borrow_mut()[..] ,
    //         & mut self.get_buffer(* bufs.inputs_buffer_ids.first().unwrap()).borrow_mut() [..] ,
    //         //bufs.inputs_buffer_ids.iter().map(|&x| self.get_buffer(x) as &[u8]).collect::<Vec<& [u8]>>() ,
    //         & mut self.get_buffer(bufs.output_buffer_id).borrow_mut() [..]
    //     )
    // }

//     self.buffers.push(RefCell::new(static_data));
// buffers_for_node.insert(blkid as u32,  vec!(self.buffers.len() -1)  );
    pub fn set_data_for_block (&mut self , block_index: BlockIndex , buffer: Vec<u8> )
    {
        self.buffers.push(RefCell::new(buffer));
        self.get_mut_buffer_block_data(block_index).data_buffer_id = self.buffers.len() as BufferIndex - 1 ;
    }

    pub fn set_inputs_block (&mut self , block_index: BlockIndex , buffer: Vec<u8> )
    {
        let  mut next_buf: BufferIndex = 0;
        {
            next_buf = self.buffers.len() as BufferIndex - 1 ;
        }
        self.buffers.push(RefCell::new(buffer));
        self.get_mut_buffer_block_data(block_index).inputs_buffer_ids.push(  next_buf) ;
    }

    pub fn set_output_for_block (&mut self , block_index: BlockIndex , buffer: Vec<u8>)
    {
        self.buffers.push(RefCell::new(buffer));
        self.get_mut_buffer_block_data(block_index).output_buffer_id =   self.buffers.len() as BufferIndex -1 ;
    }

    fn get_mut_buffer_block_data(&mut self, block_index: BlockIndex ) -> & mut BuffersForBlock
    {
        match self.buffers_for_block.get_mut(&block_index )
        {
            Some(x) => x ,
            None =>  panic!("no buffers for node") ,
        }
    }

    fn copy_buffer_block_data(&self, block_index: BlockIndex ) -> BuffersForBlock
    {
        match self.buffers_for_block.get(&block_index )
        {
            Some(x) => x.clone() ,
            None =>  panic!("no buffers for node") ,
        }
        // if buffer_option.is_none()
        // &mut buffer_option.unwrap()
    }

    pub fn get_buffer_block_data(&self, block_index: BlockIndex ) -> &BuffersForBlock
    {
        match self.buffers_for_block.get(&block_index )
        {
            Some(x) => x ,
            None =>  panic!("no buffers for node") ,
        }
        // if buffer_option.is_none()
        // &mut buffer_option.unwrap()
    }
    //
    // pub fn get_buffer_block_data(&self , block_index: BlockIndex ) -> &BuffersForBlock
    // {
    //     let buffer_option = self.buffers_for_block.get(&block_index );
    //     if buffer_option.is_none() { panic!("no buffers for node")}
    //     &buffer_option.unwrap()
    // }

    pub fn get_mod_output_buffer_as_type <T:Sized+Copy>(&self  ) -> Vec<T>
    {
        self.get_buffer_copy_as_type::<T>(0 )

    }


    pub fn get_buffer_copy_as_type<T:Sized+Copy>(&self , index: BufferIndex ) -> Vec<T>
    {

        let byte_vec = self.buffers[index as usize].borrow().to_vec();
        let data_size = byte_vec.len() /  mem::size_of::<T>();
        unsafe
        {
            let output: Vec<T> = Vec::from_raw_parts( byte_vec.as_ptr() as *mut T, data_size , data_size);
            output
        }
    }


    // fn get_mut_buffer(&self, buffer_index: BufferIndex ) -> & mut Vec<u8>
    // {
    //     &mut *self.buffers[buffer_index as usize].borrow_mut()
    // }

    pub fn get_buffer(&self, buffer_index: BufferIndex ) -> &RefCell<Vec <u8>>
    {
        &self.buffers[buffer_index as usize]
    }

    pub fn link_output_to_input(&mut self, from: BlockIndex , to: BlockIndex )
    {
        let in_bfb =  self.copy_buffer_block_data(from);
        let mut  out_bfb = self.get_mut_buffer_block_data(to);

        // if 0
        //if ( in_bfb.output_buffer_id )

        out_bfb.inputs_buffer_ids.push( in_bfb.output_buffer_id);

    // let buffer_option = self.buffers_for_node.get(&blockfrom_index );
    // if buffer_option == None
    // {
    //     self.buffers_for_node.insert(from, Vec::new());
    //     return self.buffers_for_node.get(&blockfrom_index ).unwrap() // wont work for mutable
    // }
    // let buffer_ids = buffer_option.unwrap();
    // buffer_ids
    }

    // self.buffers.push(RefCell::new(static_data));
    // self.buffers_for_node.insert(blkid as u32,  vec!(self.buffers.len())  );



            //let input1 = self.buffers[ buffer_ids[INPUT_DATA_INDEX]].borrow();


}

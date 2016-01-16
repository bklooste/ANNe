use std::collections::HashMap;
use std::cell::RefCell;
use std::{mem, slice};
//use std::slice;
// use num::traits::Num;

use core::{IBlock , BlockBehaviour , BlockIndex};
use graph::{Graph , NodeIndex};
use buffer_manager::BufferManager;

//externals should use node index.



// pub struct Buffer
// {
//
// }

// we could pull the vector graph out .. eg NodeData<T> and have module hold a graph ..
#[derive(Default)]
pub struct ModuleStats
{
    pub blocks_processed: u32,
    pub block_count: u32
}

pub struct Module
{
    //data mutable on creation ..eg add block , edges
    graph: Graph<BlockIndex>,
    //buffers_for_node: HashMap< BlockIndex, Vec<usize>>, // vector is staticdata , return/output , inputs..

    // data mutable durring run  (note ...Vec should not change ) , with these changes self should be immutable
    blocks:  Vec<RefCell<Box<IBlock>>>, // we may be able to remove refcell here ...

    //eventually we can do something like variable/ flow anylysis on this.
    //buffers: Vec<RefCell<Vec<u8>>>,
    buffer_mgr: BufferManager,

    stats: ModuleStats

}

impl Module
{
    pub fn new() -> Module {
        let mut  buffers =  Vec::new();
        buffers.push( RefCell::new( Vec::<u8>::new())); //module input
        buffers.push( RefCell::new( Vec::<u8>::new())); //module output
        let module = Module { graph: Graph::<BlockIndex>::new() , blocks: Vec::new()
            , buffer_mgr: BufferManager::new() , stats : ModuleStats{ ..Default::default()} };

        // block input 1
        module
    }

    pub fn new_from_inputs(bytes_size : usize) -> Module {
        let mut  buffers =  Vec::new();

        let output = vec![0; bytes_size];
        //unsafe { output.set_len(bytes_size as usize); }
        let input = vec![0; bytes_size];
        //  Vec::with_capacity(bytes_size as usize);
        // unsafe { input.set_len(bytes_size as usize); }

        buffers.push( RefCell::new(  output )); //module output
        buffers.push( RefCell::new(  input)); //module input

        let module = Module { graph: Graph::<BlockIndex>::new() , blocks: Vec::new() ,
             buffer_mgr: BufferManager::new() , stats : ModuleStats{ ..Default::default()} };

        // block input 1
        module
    }

    pub fn get_stats(&self) -> &ModuleStats  {
         & self.stats
    }

    //TODO NodeIndex replace with nodeid IF unique
//    pub fn add_block(& mut self, block: & 'b mut BlockType<'b>) -> NodeIndex {  self.graph.add_node(block) }

// some examples
// pub fn insert<T: IntoBox<A>>(&mut self, value: IBlock) -> Option<T> {
//         unsafe {
//             self.raw.insert(TypeId::of::<T>(), value.into_box())
//                 .map(|any| *any.downcast_unchecked::<T>())
//         }
// pub unsafe fn insert(&mut self, key: TypeId, value: Box<A>) -> Option<Box<A>> {
//     self.inner.insert(key, value)
// pub fn insert(&mut self, value: V) -> V {
//     unsafe { *self.inner.insert(value.into_box()).downcast_unchecked() }
// }

    pub fn add_block(& mut self, box_block: Box<IBlock>) -> NodeIndex
    {
        self.stats.block_count = self.stats.block_count+ 1;
        self.blocks.push(RefCell::new(box_block));
        self.graph.add_node((self.blocks.len() - 1) as u32)
    }


    //fixme should index be exposed
    pub fn add_block_w_data<T:Sized+Copy>(& mut self, box_block: Box<IBlock> , data: Vec<T>) -> NodeIndex
    {
        let new_size = data.len() * mem::size_of::<T>();

        // fn from_raw_parts(ptr: *mut T, length: usize, capacity: usize) -> Vec<T>
        unsafe
        {
        let weights: Vec<u8> = Vec::from_raw_parts( data.as_ptr() as *mut u8, new_size , new_size);
        self. add_block_w_static_data( box_block , weights)
        }
    }

    pub fn add_block_w_static_data(& mut self, box_block: Box<IBlock> , static_data: Vec<u8>) -> NodeIndex
    {
        let nodeid = self.add_block(box_block);
        let block_index = self.graph.get_node(nodeid);

//        println!("static_data {:?} : {:?}", self.buffers.len() , buffer_ids  );
//TODO fixme add buffer function

    //block_index
        self.buffer_mgr.set_data_for_block(*block_index ,static_data);


        nodeid
    }

    fn link_buffers(& mut self, from: BlockIndex , to: BlockIndex)
    {
        self.buffer_mgr.link_output_to_input(from , to );
    }

    pub fn add_link(& mut self, from: NodeIndex , to: NodeIndex)
    {
        let mut blockfrom_index  = 0;
        let mut blockto_index  = 0;

        {
            blockfrom_index = *self.graph.get_node(from);
            blockto_index = *self.graph.get_node(to);
        }

        // check if valid
        self.graph.add_edge( blockfrom_index as usize, blockto_index as usize );

    }

    //todo borrow output as slice

    // pretty crap
    pub fn get_output<T:Sized+Copy>(&self ) -> Vec<T>
    {
        self.buffer_mgr.get_mod_output_buffer_as_type::<T>()
    }

    pub fn process_blocks(&mut self)
    {
        self.process_rec(0);
    }

    pub fn process_rec(&mut self , index: NodeIndex )
    {

        {
            debug!("DEBUG: foo");
            let block_index  =  self.graph.get_node(index);
            self.process_block(*block_index);
            self.stats.blocks_processed = self.stats.blocks_processed + 1;
        }
        let successor_ids: Vec<NodeIndex>  = { self.graph.successors(index).collect()};

        for i in successor_ids { self.process_rec(i);}
    }

    fn get_block_behaviour(&self ,block_index: BlockIndex) -> BlockBehaviour
    {
        let block = self.blocks[block_index as usize].borrow();
        block.behaviour()
    }

    fn process_block<'a>(& self ,block_index: BlockIndex)
    {

        match self.get_block_behaviour(block_index)
        {
            BlockBehaviour::Immutable => {
                let block_ref = self.blocks[block_index as usize].borrow();

                // let tupple  = self.buffer_mgr.get_common_buffers_for_block(block_index);
                // process(  &*block_ref ,  tupple.0 , & [tupple.1 as &[u8]][..],  tupple.2)
                // let stat_data = self.buffer_mgr.get_data_for_block(block_index);
                // let output  = self.buffer_mgr.get_output_for_block(block_index);
                //
                // let inputs  = self.buffer_mgr.get_inputs_block(block_index);

                let block_buffer_data = self.buffer_mgr.get_buffer_block_data(block_index);

                let mut stat_data = self.buffer_mgr.get_buffer(block_buffer_data.data_buffer_id).borrow_mut();
                println!("stat_data {:?} : {:?}", stat_data.len() , stat_data  );
                let mut output  = self.buffer_mgr.get_buffer(block_buffer_data.output_buffer_id).borrow_mut();

                let inputs  =self.buffer_mgr.get_buffer(*block_buffer_data.inputs_buffer_ids.first().unwrap()).borrow();
                process(  &*block_ref ,  & mut stat_data[..], &[ & inputs[..]][..],  & mut output[..])
            } ,
            BlockBehaviour::Mutable {copy_out} => {
                let mut mut_block_ref = self.blocks[block_index as usize].borrow_mut();
                let out = mut_block_ref.process_self_copy_output( );
                // todo  copy
            },
            _ => println!("something else"),
        }





    }

        //if buffer_ids.len() == 0  { panic!("There is no buffers for this block ") };



        // let mut static_data = self.buffers[ buffer_ids[ STATIC_DATA_INDEX ]].borrow_mut();
        // if buffer_ids.len() < 3 {
        //     let mut module_output = self.buffers[MODULE_OUTPUT].borrow_mut();
        //     let module_input = self.buffers[MODULE_INPUT].borrow();
        //     println!(" less than 3 {:?} : {:?}", module_input , module_output  );
        //     return process(  &*block_ref , & mut static_data[..], &[&module_input[..]][..] , & mut module_output[..])
        // }
        //
        // let mut output = self.buffers[buffer_ids[OUTPUT_DATA_INDEX]].borrow_mut();
        //
        // let input1 = self.buffers[ buffer_ids[INPUT_DATA_INDEX]].borrow();
        // if buffer_ids.len() == 3 {
        //         return process(  &*block_ref , & mut static_data[..], &[&input1[..]][..] , & mut output[..])
        // }
        //
        // let input2 = self.buffers[ buffer_ids[INPUT2_DATA_INDEX]].borrow();
        // if buffer_ids.len() == 4 {
        //     return process(  &*block_ref , & mut static_data[..], &[&input1[..] , &input2[..]][..] , & mut output[..])
        // }
        //
        // let input3 = self.buffers[ buffer_ids[INPUT3_DATA_INDEX]].borrow();
        // if buffer_ids.len() == 5 {
        //     return process(  &* block_ref ,&mut static_data[..] ,&[&input1[..],&input2[..],&input3[..]  ][..] , & mut output[..])
        // }

        //process(  &block , & mut static_data[..], &inputs[..] , & mut output[..]);
    //     panic!("Only 3 inputs per block supported")
    // }
} //impl


fn process( block: &Box<IBlock>   , static_data: & mut [u8]  , input : &[&  [u8]]   , output: & mut [u8]    )
{
    block.process( static_data , input  , output);
}





// we dont really need an interface , maybe for network
pub trait IModule
{
    //fn process(<Vec<O>>) -> Vec<O>;
    //(args: &[&str])
    //fn process(&[ &[O] ]) -> Vec<O>;
    fn process_blocks(&mut self) ; // or return slice
    // void LoadInputsIntoModules();
    //  void ProcessFirstBlock();
    //  INeuronBlock GetEntryBlock();
    //  INeuronBlock GetBlock(uint id);
    //  void AddRunningTasks(Task task);
    //  void RemoveRunningTasks(Task task);
}

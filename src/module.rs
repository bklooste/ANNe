use std::collections::HashMap;
use std::cell::RefCell;
// use num::traits::Num;

use core::{IBlock , BlockIndex};
use graph::{Graph , NodeIndex};

const  STATIC_DATA_INDEX: usize = 0;
const  OUTPUT_DATA_INDEX: usize = 1;
const  INPUT_DATA_INDEX: usize = 2;
const  INPUT2_DATA_INDEX: usize = 3;
const  INPUT3_DATA_INDEX: usize = 4;

const  MODULE_INPUT: usize = 0;
const  MODULE_OUTPUT: usize = 1;

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
    buffers_for_node: HashMap< BlockIndex, Vec<usize>>, // vector is staticdata , return/output , inputs..

    // data mutable durring run  (note ...Vec should not change ) , with these changes self should be immutable
    blocks:  Vec<RefCell<Box<IBlock>>>, // we may be able to remove refcell here ...
    buffers: Vec<RefCell<Vec<u8>>>,

    stats: ModuleStats

}

impl Module
{
    pub fn new() -> Module {
        let mut  buffers =  Vec::new();
        buffers.push( RefCell::new( Vec::<u8>::new())); //module input
        buffers.push( RefCell::new( Vec::<u8>::new())); //module output
        let module = Module { graph: Graph::<BlockIndex>::new() , blocks: Vec::new() , buffers: buffers , buffers_for_node: HashMap::new() , stats : ModuleStats{ ..Default::default()} };

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

        let module = Module { graph: Graph::<BlockIndex>::new() , blocks: Vec::new() , buffers: buffers , buffers_for_node: HashMap::new() , stats : ModuleStats{ ..Default::default()} };

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

    pub fn add_block_w_static_data(& mut self, box_block: Box<IBlock> , static_data: Vec<u8>) -> NodeIndex
    {
        let blkid = self.add_block(box_block);

//        println!("static_data {:?} : {:?}", self.buffers.len() , buffer_ids  );
//TODO fixme add buffer function
        self.buffers.push(RefCell::new(static_data));
        self.buffers_for_node.insert(blkid as u32,  vec!(self.buffers.len() -1)  );

        blkid
    }

    pub fn add_link(& mut self, from: NodeIndex , to: NodeIndex)
    {
        self.graph.add_edge(from, to);

        // self.buffers.push(RefCell::new(static_data));
        // self.buffers_for_node.insert(blkid as u32,  vec!(self.buffers.len())  );
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

    fn process_block<'a>(&self ,block_index: BlockIndex)
    {

        let buffer_option = self.buffers_for_node.get(&block_index );
        if buffer_option == None
        {
            let mut mut_block_ref = self.blocks[block_index as usize].borrow_mut();
            // if no buffers just process assume its an independent block  ...
            return mut_block_ref.process_mut_and_copy_output(  & mut []) ;  // ,&mut [] ,&[] , &mut []);
        }
        let buffer_ids = buffer_option.unwrap();


        let block_ref = self.blocks[block_index as usize].borrow();

        println!("static_data {:?} : {:?}", self.buffers.len() , buffer_ids  );


        if buffer_ids.len() == 0  { panic!("There is no buffers for this block ") };



        let mut static_data = self.buffers[ buffer_ids[ STATIC_DATA_INDEX ]].borrow_mut();
        if buffer_ids.len() < 3 {
            let mut module_output = self.buffers[MODULE_OUTPUT].borrow_mut();
            let module_input = self.buffers[MODULE_INPUT].borrow();
            println!(" less than 3 {:?} : {:?}", module_input , module_output  );
            return process(  &*block_ref , & mut static_data[..], &[&module_input[..]][..] , & mut module_output[..])
        }

        let mut output = self.buffers[buffer_ids[OUTPUT_DATA_INDEX]].borrow_mut();

        let input1 = self.buffers[ buffer_ids[INPUT_DATA_INDEX]].borrow();
        if buffer_ids.len() == 3 {
                return process(  &*block_ref , & mut static_data[..], &[&input1[..]][..] , & mut output[..])
        }

        let input2 = self.buffers[ buffer_ids[INPUT2_DATA_INDEX]].borrow();
        if buffer_ids.len() == 4 {
            return process(  &*block_ref , & mut static_data[..], &[&input1[..] , &input2[..]][..] , & mut output[..])
        }

        let input3 = self.buffers[ buffer_ids[INPUT3_DATA_INDEX]].borrow();
        if buffer_ids.len() == 5 {
            return process(  &* block_ref ,&mut static_data[..] ,&[&input1[..],&input2[..],&input3[..]  ][..] , & mut output[..])
        }

//         //let mut inputs = Vec::new();
//         match buffer_ids.len() {
//             0 => panic!("There is no buffers for this block "),  // may be allowed later
//             1 => inputs.push  ( & (* self.buffers[ buffer_ids[INPUT_DATA_INDEX]].borrow()) [..] ) ,
//             // 2 => &[&self.buffers[ buffer_ids[INPUT_DATA_INDEX]].borrow()[..]
//             //         ,&self.buffers[ buffer_ids[3]].borrow()[..]
//             //      ][..],
//             // 3 => &[&self.buffers[ buffer_ids[INPUT_DATA_INDEX]].borrow()[..]
//             //         ,&self.buffers[ buffer_ids[3]].borrow()[..]
//             //         ,&self.buffers[ buffer_ids[4]].borrow()[..]
//             //      ][..],
//     _ => panic!("Only 3 inputs per block supported"),
// };
//     //   ^ Don't

        //process(  &block , & mut static_data[..], &inputs[..] , & mut output[..]);
        panic!("Only 3 inputs per block supported")
    }
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

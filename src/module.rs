use std::collections::HashMap;
use std::cell::RefCell;
// use num::traits::Num;

use core::{IBlock , BlockIndex};
use graph::{Graph , NodeIndex};

// pub struct Buffer
// {
//
// }

// we could pull the vector graph out .. eg NodeData<T> and have module hold a graph ..

pub struct Module
{
    // graph: Graph<Box<IBlock>>, //block live as long as module
    // buffers: Vec<RefCell<Vec<u8>>>,   ///TODO try a single buffer .
    // buffers_for_node: HashMap< usize, Vec<usize>>

    //data mutable on creation ..eg add block , edges
 // graphid
    graph: Graph<BlockIndex>,
   // graph: RefCell<Graph<Box<BlockType<'b>>>>,
      ///TODO try a single buffer .
    buffers_for_node: HashMap< BlockIndex, Vec<usize>>,

    // data mutable durring run  (note ...Vec should not change ) , with these changes self should be immutable
    blocks:  Vec<RefCell<Box<IBlock>>>, // we may be able to remove refcell here ...
    buffers: Vec<RefCell<Vec<u8>>>,

}

impl Module
{
    pub fn new() -> Module {
        Module { graph: Graph::<BlockIndex>::new() , blocks: Vec::new() , buffers: Vec::new() , buffers_for_node: HashMap::new() }
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

    pub fn add_block(& mut self, box_block: Box<IBlock>) -> NodeIndex {


        self.blocks.push(RefCell::new(box_block));
        self.graph.add_node((self.blocks.len() - 1) as u32)
        //
        // unsafe {
        //
        //     let ref_mut: &mut BlockType = & mut *Box::into_raw(box_block);
        //     self.add_block(ref_mut)
        //
        // }



    }

    pub fn add_link(& mut self, from: NodeIndex , to: NodeIndex) {  self.graph.add_edge(from, to); }

    pub fn process_blocks(&mut self)
{
    self.process_rec(0);

}

pub fn process_rec(&mut self , index: NodeIndex )
{

    {
        let block_index  =  self.graph.get_node(index);
        self.process_block(*block_index);
    }

    let successor_ids: Vec<NodeIndex>  = { self.graph.successors(index).collect()};

    for i in successor_ids { self.process_rec(i);}

}

fn process_block<'a>(&self ,block_index: BlockIndex)
 {
 // get block
   let block = self.blocks[block_index as usize].borrow_mut();
   let buffer_ids = self.buffers_for_node.get(&block_index ).unwrap();
 // get buffers

 let mut static_data = self.buffers[ buffer_ids[0]].borrow_mut();
 let mut input  = self.buffers[ buffer_ids[1]].borrow_mut();
 let mut output = self.buffers[buffer_ids[2]].borrow_mut();

//let bufs_for_block  = [& mut static_data[..], & mut input[..] , & mut output[..]];

    process(  &block , & mut static_data[..], & mut input[..] , & mut output[..]);
 }




//https://play.rust-lang.org/?gist=8f5b81caf75d90233590&version=stable




    //    fn process(& mut self ,  block: & 'b IBlock<'b>)
    //    {
    //        match block {
    //            &BlockType::Block(ref b) => self.process_block (b ),
    //            &BlockType::MutBlock(b) => self.process_mut_block( b ),
    //            &BlockType::FunctionBlock(_) => {}
    //        }
    //    }

       // some major changes

       //https://play.rust-lang.org/?gist=3157795d100a6c964a82&version=stable

    // fn get_buffer_ids(&self ,  block_id: BlockId) -> Vec< (usize , usize , bool)>
    // {
    //
    //     self.buffers_for_node.get(block_id).map( )
    // }


    // fn get_buffers(& self ,  block_id: BlockId) -> Vec<& 'b [u8]>
    // {
    //
    //     let bufs = self.buffers_for_node.get(& (block_id as usize) ).iter( )
    //         .map(|x| & (self.buffers[block_id as usize] ) [..] );
    //
    //     bufs.collect::<Vec<_>>()
    //     //a.iter().map(|&x| 2 * x);
    //     //buffer_for_node: HashMap< usize, Vec< (usize , usize , bool)>>
    // }

    // fn get_buffers(& 'b mut self,  block_id: BlockId) -> Vec<& mut [u8]>
    //  {
    //
    //     //  let bufs = self.buffers_for_node[&(block_id as usize)].iter( )
    //     //      .map(|&x| & mut self.buffers[x as usize][..] );
    //      //
    //     //  bufs.collect::<Vec<_>>()
    //  }






} //impl


fn process( block: &Box<IBlock>   , static_data: & mut [u8]  , input : & mut [u8]   , output: & mut [u8]    )
{
    // let blockid = block.get_id() as usize;
    // let buffers = self.buffers_for_node.get(&blockid).unwrap();


    // let static_data = self.buffers[ buffers[0]].borrow();
    // let input  = self.buffers[ buffers[1]].borrow();
    // let mut output = self.buffers[buffers[2]].borrow_mut();


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

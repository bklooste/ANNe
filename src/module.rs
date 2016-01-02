use std::collections::HashMap;
use std::cell::RefCell;
// use num::traits::Num;

use core::{Block , BlockType , MutBlock }; 
use graph::{Graph , NodeIndex};

// pub struct Buffer
// {
//
// }

// we could pull the vector graph out .. eg NodeData<T> and have module hold a graph ..

pub struct Module<'b>
{
    graph: Graph<& 'b BlockType<'b>>,
    buffers: Vec<RefCell<Vec<u8>>>,   ///TODO try a single buffer .
    buffers_for_node: HashMap< usize, Vec<usize>>

}

impl<'b> Module<'b>
{
    pub fn new() -> Module<'b> {
        Module { graph: Graph::<& 'b BlockType<'b>>::new() , buffers: Vec::new() , buffers_for_node: HashMap::new() }
    }

    //TODO NodeIndex replace with nodeid IF unique
    pub fn add_box_block(& mut self, block: & 'b mut BlockType<'b>) -> NodeIndex {

        self.graph.add_node(block)
    }


    pub fn add_link(& mut self, from: NodeIndex , to: NodeIndex) {  self.graph.add_edge(from, to); }

    /// this will get WAY more complicated ..
    /// we need to handle loops which never finish , re run on completion , threading , timed and priority etc  1
    pub fn process_blocks(& 'b mut self)
    {
        self.process_rec(0);

    }


    pub fn process_rec(&mut self , index: NodeIndex )
    {
        let successor_ids: Vec<NodeIndex> = { self.graph.successors(index).collect()};

        {
            let node: & BlockType = { self.graph.get_node(index)};
            self.process( node);

        }

        for i in successor_ids {
            self.process_rec ( i);
        }

    }

    fn process(& mut self ,  block: & 'b BlockType)
    {
        match *block {
            BlockType::Block(b) => self.process_block (b ),
            BlockType::MutBlock(ref b) => process_mut_block( *b ),
            BlockType::FunctionBlock(_) => {}
        }
    }

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





    //    Block (Box<Block + 'b> ),
    fn process_block(& mut self ,  block: & 'b Block)
    {
        let blockid = block.get_id() as usize;
        let buffers = self.buffers_for_node.get(&blockid).unwrap();


        let static_data = self.buffers[ buffers[0]].borrow();
        let input  = self.buffers[ buffers[1]].borrow();
        let mut output = self.buffers[buffers[2]].borrow_mut();


        block.process( &static_data[..] ,&input[..] , & mut output[..]);


        // .iter( )
        //     .map(|&x| & mut self.buffers[x as usize][..] ).collect::<Vec<_>>();
        //let buffers = self.get_buffers(block.get_id());
        //let mut_buffers = get_mut_buffers(blockType.get_id());
    }
} //impl



fn process_mut_block( _block_type: &MutBlock)
{

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

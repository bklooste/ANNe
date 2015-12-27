use core::Block;
use graph::{Graph , NodeIndex};

// we could pull the vector graph out .. eg NodeData<T> and have module hold a graph ..

pub struct Module
{
    graph: Graph<Box<Block>>,
}

impl Module
{
    pub fn new() -> Module {
        Module { graph: Graph::<Box<Block>>::new() }
    }

//TODO
//    pub fn add_block(&self, block: &Block) { self.add_box_block( Box::new( *block))  }

    pub fn add_box_block(& mut self, block: Box<Block>) {  self.graph.add_node(block); }

    /// this will get WAY more complicated ..
    /// we need to handle loops which never finish , re run on completion , threading etc
    pub fn process_blocks(& mut self)
    {
        let successors: Vec<NodeIndex> = self.graph.successors(0).collect();

        for i in successors { self.graph.get_node(i).process(); }
    }
}


// we dont really need an interface
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



// mod test {
//     use super::*;

    // #[test]
    // pub fn example() {
    //
    //     // N0 ---E0---> N1 ---E1---> 2
    //     // |                         ^
    //     // E2                        |
    //     // |                         |
    //     // v                         |
    //     // N3 ----------E3-----------+
    //
    //     let mut graph = Module::new();
    //
    //     let n0 = graph.add_node();
    //     let n1 = graph.add_node();
    //     let n2 = graph.add_node();
    //     let n3 = graph.add_node();
    //
    //     graph.add_edge(n0, n1); // e0
    //     graph.add_edge(n1, n2); // e1
    //     graph.add_edge(n0, n3); // e2
    //     graph.add_edge(n3, n2); // e3
    //
    //     let successors: Vec<NodeIndex> = graph.successors(n0).collect();
    //     assert_eq!(&successors[..], &[n3, n1]);
    // }
//}

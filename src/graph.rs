

pub struct Graph<T>  {
    nodes: Vec<NodeData<T>>,
    edges: Vec<EdgeData>,
}


// we dont really need an interface
// pub trait IModule
// {
//     //fn process(<Vec<O>>) -> Vec<O>;
//     //(args: &[&str])
//     //fn process(&[ &[O] ]) -> Vec<O>;
//     fn process(&mut self) ; // or return slice
//     // void LoadInputsIntoModules();
//     //  void ProcessFirstBlock();
//     //  INeuronBlock GetEntryBlock();
//     //  INeuronBlock GetBlock(uint id);
//     //  void AddRunningTasks(Task task);
//     //  void RemoveRunningTasks(Task task);
//
//
// }

pub type NodeIndex = usize;

pub struct NodeData<T> where T: Sized {
    first_outgoing_edge: Option<EdgeIndex>,
    node: T
}

pub type EdgeIndex = usize;

pub struct EdgeData {
    target: NodeIndex,
    next_outgoing_edge: Option<EdgeIndex>
}

impl <T> Graph<T>
where T: Sized
{
    pub fn new() -> Graph<T> {
        Graph::<T> { nodes: Vec::new(),
                edges: Vec::new(), }
    }

    pub fn add_node(&mut self , blk: T) -> NodeIndex {
        let index = self.nodes.len();
        self.nodes.push(NodeData { first_outgoing_edge: None , node: blk });
        index
    }

    pub fn add_edge(&mut self, source: NodeIndex, target: NodeIndex) {
        let edge_index = self.edges.len();
        let node_data = &mut self.nodes[source];
        self.edges.push(EdgeData {
            target: target,
            next_outgoing_edge: node_data.first_outgoing_edge
        });
        node_data.first_outgoing_edge = Some(edge_index);
    }

    pub fn get_node(&mut self, source: NodeIndex) -> & mut T {
        println!("get node {:?} , num nodes{:?} ", source , self.nodes.len());
        & mut self.nodes[source].node
    }

    pub fn successors(&self, source: NodeIndex) -> Successors<T> {
        let first_outgoing_edge = self.nodes[source].first_outgoing_edge;
        Successors { graph: self, current_edge_index: first_outgoing_edge }
    }
}


pub struct Successors<'graph,T> where T: 'graph{
    graph: &'graph Graph<T> ,
    current_edge_index: Option<EdgeIndex>,
}

impl<'graph,T> Iterator for Successors<'graph,T> {
    type Item = NodeIndex;

    fn next(&mut self) -> Option<NodeIndex> {
        match self.current_edge_index {
            None => None,
            Some(edge_num) => {
                let edge = &self.graph.edges[edge_num];
                self.current_edge_index = edge.next_outgoing_edge;
                Some(edge.target)
            }
        }
    }
}


// mod test {
//     use super::*;

    #[test]
    pub fn example() {

        // N0 ---E0---> N1 ---E1---> 2
        // |                         ^
        // E2                        |
        // |                         |
        // v                         |
        // N3 ----------E3-----------+

        let mut graph = Graph::<i32>::new();

        let n0 = graph.add_node(1i32);
        let n1 = graph.add_node(1i32);
        let n2 = graph.add_node(1i32);
        let n3 = graph.add_node(1i32);

        graph.add_edge(n0, n1); // e0
        graph.add_edge(n1, n2); // e1
        graph.add_edge(n0, n3); // e2
        graph.add_edge(n3, n2); // e3

        let successors: Vec<NodeIndex> = graph.successors(n0).collect();
        assert_eq!(&successors[..], &[n3, n1]);
    }
//}

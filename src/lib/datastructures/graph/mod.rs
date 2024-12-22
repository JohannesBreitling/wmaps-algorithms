use std::fmt;

pub trait Node {
    fn get_id(&self) -> usize;
}
pub trait Edge {
    fn from(&self) -> usize;
    fn to(&self) -> usize;
}

pub trait Graph<NodeT: Node, EdgeT: Edge> {
    fn new() -> Self;

    fn add_node(&mut self, node: NodeT);
    fn add_edge(&mut self, edge: EdgeT);

    fn get_node(&self, id: usize) -> &NodeT;
    fn get_edge(&self, from: usize, to: usize) -> &EdgeT;

    fn get_degree(&self, id: usize) -> usize;
}

pub struct GraphListImpl<NodeT: Node, EdgeT: Edge> {
    nodes: Vec<NodeT>,
    neighbours: Vec<Vec<EdgeT>>,
}

impl<NodeT: Node, EdgeT: Edge> Graph<NodeT, EdgeT> for GraphListImpl<NodeT, EdgeT> {
    fn new() -> Self {
        GraphListImpl {
            nodes: Vec::new(),
            neighbours: Vec::new(),
        }
    }

    fn add_node(&mut self, node: NodeT) {
        self.nodes.push(node);
        self.neighbours.push(Vec::new());
    }

    fn add_edge(&mut self, edge: EdgeT) {
        self.neighbours[edge.from()].push(edge);
    }

    fn get_node(&self, id: usize) -> &NodeT {
        &self.nodes[id]
    }

    fn get_edge(&self, from: usize, to: usize) -> &EdgeT {
        self.neighbours[from].iter().find(|x| x.to() == to).unwrap()
    }

    fn get_degree(&self, id: usize) -> usize {
        self.neighbours[id].len()
    }
}

impl<NodeT: Node, EdgeT: Edge> fmt::Display for GraphListImpl<NodeT, EdgeT> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result = String::new();
        for node in self.nodes.iter() {
            result.push_str(&format!("<Node {} ", node.get_id()));

            for edge in self.neighbours[node.get_id()].iter() {
                result.push_str("Neigbours<");
                result.push_str(&format!("{} ", edge.to()));
                result.push_str(">");
            }
            result.push_str("> ");
        }

        write!(f, "Graph<{}>", result)
    }
}

pub struct BasicNode {
    id: usize,
}

impl Node for BasicNode {
    fn get_id(&self) -> usize {
        self.id
    }
}

impl BasicNode {
    pub fn new(id: usize) -> BasicNode {
        BasicNode { id }
    }
}

pub struct BasicEdge {
    from: usize,
    to: usize,
}

impl Edge for BasicEdge {
    fn from(&self) -> usize {
        self.from
    }

    fn to(&self) -> usize {
        self.to
    }
}

impl BasicEdge {
    pub fn new(from: usize, to: usize) -> BasicEdge {
        BasicEdge { from, to }
    }
}

pub struct OSMNode {
    id: usize,
    lat: f64,
    lon: f64,
}

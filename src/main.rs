use wmaps_algorithms::datastructures::graph::BasicEdge;
use wmaps_algorithms::datastructures::graph::BasicNode;
use wmaps_algorithms::datastructures::graph::Edge;
use wmaps_algorithms::datastructures::graph::Graph;
use wmaps_algorithms::datastructures::graph::GraphListImpl;

fn main() {
    println!("Hello, wmaps!!");

    let mut graph = GraphListImpl::<BasicNode, BasicEdge>::new();

    graph.add_node(BasicNode::new(0));
    graph.add_node(BasicNode::new(1));
    graph.add_node(BasicNode::new(2));
    graph.add_node(BasicNode::new(3));
    graph.add_node(BasicNode::new(4));
    graph.add_node(BasicNode::new(5));

    graph.add_edge(BasicEdge::new(1, 2));
    graph.add_edge(BasicEdge::new(2, 3));
    graph.add_edge(BasicEdge::new(3, 1));

    println!("{}", graph);
}

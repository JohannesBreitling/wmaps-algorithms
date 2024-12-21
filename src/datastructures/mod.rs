trait Node {}
trait Edge {}

trait Graph {
    fn add_node(&mut self, node: Node);
    fn add_edge(&mut self, edge: Edge);
}

// Implementation of a graph with adjaceny list

struct OSMNode {
    id: i32,
    lat: f64,
    lon: f64,
}

struct OSMEdge {
    start: i32,
    end: i32,
    length: f64,
    road_type: RoadType,
}

enum RoadType {}

impl Node for OSMNode {}

struct OsmGraphListImpl {
    edges: Vec<Vec<OSMNode>>,
    nodes: Vec<OSMNode>,
}

// Implementation of a graph with adjacency array
struct OsmGraphImpl {
    offset: i32,
    nodes: Vec<i32>,
}


use diagrams_rs::aws::alb::Alb;
use diagrams_rs::graph::{Diagram, Node};

fn main() {
    let mut nodes = Vec::<Box<dyn Node>>::new();

    nodes.push(Box::new(Alb::new("aaa")));
    nodes.push(Box::new(Alb::new("bbb")));
    nodes.push(Box::new(Alb::new("ccc")));
    nodes.push(Box::new(Alb::new("ddd")));
    let edges = vec![(0, 1), (0, 2), (1, 3), (2, 3)];
    let diagram = Diagram { nodes, edges };

    diagram.render_to("mydot.dot");
}

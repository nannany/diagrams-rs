use std::env;
use std::path::Path;

use diagrams_rs::aws::alb;
use diagrams_rs::aws::alb::Alb;
use diagrams_rs::global::LARGE_TEXT;
use diagrams_rs::graph::{Graph, Node};

fn main() {
    use std::fs::File;
    let mut nodes = Vec::<Box<dyn Node>>::new();

    nodes.push(Box::new(Alb::new("aaa")));
    nodes.push(Box::new(Alb::new("bbb")));
    nodes.push(Box::new(Alb::new("ccc")));
    nodes.push(Box::new(Alb::new("ddd")));
    let edges = vec![(0, 1), (0, 2), (1, 3), (2, 3)];
    let graph = Graph { nodes, edges };

    graph.render_to("mydot.dot");
}

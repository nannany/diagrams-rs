use std::env;
use std::path::Path;

use diagrams_rs::global::LARGE_TEXT;
use diagrams_rs::graph::{Graph, Node};

fn main() {
    use std::fs::File;
    let mut f = File::create("example1.dot").unwrap();

    let mut nodes = Vec::<Box<dyn Node>>::new();

    nodes.push(Box::new(Alb::new("aaa")));
    nodes.push(Box::new(Alb::new("bbb")));
    nodes.push(Box::new(Alb::new("ccc")));
    nodes.push(Box::new(Alb::new("ddd")));
    let edges = vec![(0, 1), (0, 2), (1, 3), (2, 3)];
    let graph = Graph { nodes, edges };

    graph.render_to(&mut f);
}

struct Alb<'a> {
    label: &'a str,
    path: &'a str,
}

impl Alb<'_> {
    fn new(label: &str) -> Alb {
        Alb {
            label,
            path: "assets/aws/network/elb-application-load-balancer.png",
        }
    }
}

impl Node for Alb<'_> {
    fn label(&self) -> &str {
        self.label
    }

    fn image_path(&self) -> &str {
        self.path
    }
}

use diagrams_rs::graph::{Graph, Node};

fn main() {
    use std::fs::File;
    let mut f = File::create("example1.dot").unwrap();

    let mut nodes = Vec::<Box<dyn Node>>::new();

    nodes.push(Box::new(Alb {
        label: "aaa",
        path: ("path1"),
    }));
    nodes.push(Box::new(Alb {
        label: "bbb",
        path: ("path2"),
    }));
    nodes.push(Box::new(Alb {
        label: "ccc",
        path: ("path3"),
    }));
    nodes.push(Box::new(Alb {
        label: "ddd",
        path: ("path4"),
    }));
    let edges = vec![(0, 1), (0, 2), (1, 3), (2, 3)];
    let graph = Graph { nodes, edges };

    graph.render_to(&mut f);
}

struct Alb<'a> {
    label: &'a str,
    path: &'a str,
}

impl Node for Alb<'_> {
    fn label(&self) -> &str {
        &self.label
    }

    fn image_path(&self) -> &str {
        &self.path
    }
}

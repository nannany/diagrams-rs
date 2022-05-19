use diagrams_rs::graph::{Graph, Node};

fn main() {
    use std::fs::File;
    let mut f = File::create("example1.dot").unwrap();

    let nodes = vec![
        Box::new(Alb {
            label: "aaa",
            path: ("path1"),
        }),
        Box::new(Alb {
            label: "bbb",
            path: ("path2"),
        }),
        Box::new(Alb {
            label: "ccc",
            path: ("path3"),
        }),
        Box::new(Alb {
            label: "ddd",
            path: ("path4"),
        }),
    ];
    let edges = vec![(0, 1), (0, 2), (1, 3), (2, 3)];
    let graph = Graph { nodes, edges };

    graph.render_to(&mut f);
}

struct Alb {
    label: &'static str,
    path: &'static str,
}

impl Node for Alb {
    fn label(&self) -> &str {
        &self.label
    }

    fn image_path(&self) -> &str {
        &self.path
    }
}

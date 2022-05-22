use std::env;
use std::path::Path;

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

    let path = Path::new("assets/aws/network/elb-application-load-balancer.png");
    let pwd = env::current_dir().unwrap();
    let absolute_path = pwd.join(path);
    // パスを文字列のスライスに変換する。
    match absolute_path.to_str() {
        None => panic!("new path is not a valid UTF-8 sequence"),
        Some(s) => println!("new path is {}", s),
    }

    graph.render_to(&mut f);
}

struct Alb<'a> {
    label: &'a str,
    path: &'a str,
}

impl Alb<'_> {
    fn init(&mut self, label: &str) -> Self {
        let path = Path::new("assets/aws/network/elb-application-load-balancer.png");
        let pwd = env::current_dir().unwrap();
        let absolute_path = pwd.join(path);
        let temp_path = match absolute_path.to_str() {
            None => panic!("new path is not a valid UTF-8 sequence"),
            Some(s) => s,
        };

        let path: &str = temp_path;

        Alb { label, path }
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

use diagrams_rs::graph::Graph;

fn main() {
    use std::fs::File;
    let mut f = File::create("example1.dot").unwrap();

    let nodes = vec!("{x,y}", "{x}", "{y}", "{}");
    let edges = vec!((0, 1), (0, 2), (1, 3), (2, 3));
    let graph = Graph { nodes, edges };

    graph.render_to( &mut f);
}

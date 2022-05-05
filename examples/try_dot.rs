use std::io::Write;

type Nd<'a> = (usize, &'a str);
type Ed<'a> = (Nd<'a>, Nd<'a>);
struct Graph { nodes: Vec<&'static str>, edges: Vec<(usize,usize)> }

pub fn render_to<W: Write>(output: &mut W) {
    let nodes = vec!("{x,y}","{x}","{y}","{}");
    let edges = vec!((0,1), (0,2), (1,3), (2,3));
    let graph = Graph { nodes: nodes, edges: edges };

    dot::render(&graph, output).unwrap()
}

impl<'a> dot::Labeller<'a, Nd<'a>, Ed<'a>> for Graph {
    fn graph_id(&'a self) -> dot::Id<'a> { dot::Id::new("example3").unwrap() }
    fn node_id(&'a self, n: &Nd<'a>) -> dot::Id<'a> {
        dot::Id::new(format!("N{}", n.0)).unwrap()
    }
    fn node_label<'b>(&'b self, n: &Nd<'b>) -> dot::LabelText<'b> {
        let &(i, _) = n;
        dot::LabelText::LabelStr(self.nodes[i].into())
    }
    fn edge_label<'b>(&'b self, _: &Ed<'b>) -> dot::LabelText<'b> {
        dot::LabelText::LabelStr("&sube;".into())
    }
}

impl<'a> dot::GraphWalk<'a, Nd<'a>, Ed<'a>> for Graph {
    fn nodes(&'a self) -> dot::Nodes<'a,Nd<'a>> {
        self.nodes.iter().map(|s| &s[..]).enumerate().collect()
    }
    fn edges(&'a self) -> dot::Edges<'a,Ed<'a>> {
        self.edges.iter()
            .map(|&(i,j)|((i, &self.nodes[i][..]),
                          (j, &self.nodes[j][..])))
            .collect()
    }
    fn source(&self, e: &Ed<'a>) -> Nd<'a> { e.0 }
    fn target(&self, e: &Ed<'a>) -> Nd<'a> { e.1 }
}

fn main() {
    use std::fs::File;
    let mut f = File::create("example1.dot").unwrap();
    render_to(&mut f)

}

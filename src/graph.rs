use std::io::Write;

// nodeを表現
type Nd<'a> = (usize, &'a str, &'a str);
// edgeを表現
type Ed<'a> = (Nd<'a>, Nd<'a>);

trait Node {
    fn label(&self) -> &str;

    fn image_path(&self) -> &str;
}

pub struct Graph {
    pub nodes: Vec<dyn Node>,
    pub edges: Vec<(usize, usize)>,
}

/// 識別子を振ったり、ラベルを貼り付けたりするLabellerを実装
impl<'a> dot::Labeller<'a, Nd<'a>, Ed<'a>> for Graph {
    fn graph_id(&'a self) -> dot::Id<'a> {
        dot::Id::new("my_graph").unwrap()
    }
    fn node_id(&'a self, n: &Nd<'a>) -> dot::Id<'a> {
        dot::Id::new(format!("N{}", n.0)).unwrap()
    }
    fn node_label<'b>(&'b self, n: &Nd<'b>) -> dot::LabelText<'b> {
        let &(i, _, _) = n;
        dot::LabelText::LabelStr(self.nodes[i].into())
    }
    fn edge_label<'b>(&'b self, _: &Ed<'b>) -> dot::LabelText<'b> {
        dot::LabelText::LabelStr("aaaa".into())
    }
}

/// nodeとedgeの位置関係を把握するために必要なGraphWalkを実装
impl<'a> dot::GraphWalk<'a, Nd<'a>, Ed<'a>> for Graph {
    fn nodes(&'a self) -> dot::Nodes<'a, Nd<'a>> {
        self.nodes.iter().map(|s| &s[..]).enumerate().collect()
    }
    fn edges(&'a self) -> dot::Edges<'a, Ed<'a>> {
        self.edges
            .iter()
            .map(|&(i, j)| ((i, &self.nodes[i][..]), (j, &self.nodes[j][..])))
            .collect()
    }
    /// どっちからどっちの矢印を出すかという話
    fn source(&self, e: &Ed<'a>) -> Nd<'a> {
        e.0
    }
    fn target(&self, e: &Ed<'a>) -> Nd<'a> {
        e.1
    }
}

impl Graph {
    pub fn render_to<W: Write>(&self, output: &mut W) {
        dot::render(self, output).unwrap()
    }
}

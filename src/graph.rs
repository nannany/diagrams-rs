use std::fs::File;

use crate::global::LARGE_TEXT;

// nodeを表現
type Nd<'a> = (u32, (&'a str, &'a str));
// edgeを表現
type Ed<'a> = (Nd<'a>, Nd<'a>);

pub trait Node {
    fn label(&self) -> &str;

    fn image_path(&self) -> &str;

    fn id(&self) -> u32;
}

pub struct Diagram<'a> {
    pub nodes: Vec<&'a dyn Node>,
    pub edges: Vec<(u32, u32)>,
}

/// Identify and label
impl<'a> dot::Labeller<'a, Nd<'a>, Ed<'a>> for Diagram<'_> {
    fn graph_id(&'a self) -> dot::Id<'a> {
        dot::Id::new("my_graph").unwrap()
    }
    fn node_id(&'a self, n: &Nd<'a>) -> dot::Id<'a> {
        dot::Id::new(format!("N{}", n.0)).unwrap()
    }
    fn node_label<'b>(&'b self, n: &Nd<'b>) -> dot::LabelText<'b> {
        let &(_, (label, image_path)) = n; // nodeのIDとlistの順番が同じ前提
                                           // dot::LabelText::LabelStr(self.nodes[i].image_path().into())
        let mut absolute_image_path = String::new();
        absolute_image_path.push_str(LARGE_TEXT.as_str());
        absolute_image_path.push_str("/");
        absolute_image_path.push_str(image_path);
        let html_string = build_html_string(absolute_image_path.as_str(), label);
        dot::LabelText::HtmlStr(html_string.into())
    }
    fn edge_label<'b>(&'b self, _: &Ed<'b>) -> dot::LabelText<'b> {
        dot::LabelText::LabelStr("edge".into())
    }
}

fn build_html_string(path: &str, text: &str) -> String {
    format!(
        "<TABLE><TR><TD><IMG SRC=\"{}\"/></TD></TR><TR><TD>{}</TD></TR></TABLE>",
        path, text
    )
}

/// Implement GraphWalk required to understand the relationship between node and edge location
impl<'a> dot::GraphWalk<'a, Nd<'a>, Ed<'a>> for Diagram<'_> {
    // impl dot::Nodes which lists nodes
    fn nodes(&'a self) -> dot::Nodes<'a, Nd<'a>> {
        self.nodes
            .iter()
            .map(|node| (node.id(), (node.label(), node.image_path())))
            .collect()
    }
    // impl dot::Edges which lists edges
    fn edges(&'a self) -> dot::Edges<'a, Ed<'a>> {
        self.edges
            .iter()
            .map(|&(i, j)| {
                (
                    self.nodes
                        .iter()
                        .find(|&node| node.id() == i)
                        .map(|node| (node.id(), (node.label(), node.image_path())))
                        .unwrap(),
                    self.nodes
                        .iter()
                        .find(|&node| node.id() == j)
                        .map(|node| (node.id(), (node.label(), node.image_path())))
                        .unwrap(),
                )
            })
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

impl<'a> Diagram<'a> {
    pub fn render_to(&self, file_name: &str) {
        let mut f = File::create(file_name).unwrap();
        dot::render(self, &mut f).unwrap()
    }

    pub fn connect(&mut self, src_node: &'a (dyn Node + 'a), target_node: &'a (dyn Node + 'a)) {
        self.nodes.push(src_node);
        self.nodes.push(target_node);
        self.edges.push((src_node.id(), target_node.id()))
    }

    pub fn new() -> Diagram<'static> {
        Diagram {
            nodes: vec![],
            edges: vec![],
        }
    }
}

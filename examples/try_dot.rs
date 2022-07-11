use diagrams_rs::aws::alb::Alb;
use diagrams_rs::aws::ecs::Ecs;
use diagrams_rs::graph::{Diagram, Node};

fn main() {
    let mut diagram = Diagram::new();

    let alb1 = Alb::new("abc");
    let alb2 = Alb::new("dce");

    diagram.connect(&alb1, &alb2);

    diagram.render_to("mydot.dot");
}

use crate::graph::Node;

pub struct Ecs<'a> {
    label: &'a str,
    path: &'a str,
}

impl Ecs<'_> {
    pub fn new(label: &str) -> Ecs {
        Ecs {
            label,
            path: "assets/aws/compute/elastic-container-service.png",
        }
    }
}

impl Node for Ecs<'_> {
    fn label(&self) -> &str {
        self.label
    }

    fn image_path(&self) -> &str {
        self.path
    }
}

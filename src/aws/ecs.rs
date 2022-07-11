use crate::global::LARGE_ID;
use crate::graph::Node;

pub struct Ecs<'a> {
    label: &'a str,
    path: &'a str,
    id: u32,
}

impl Ecs<'_> {
    pub fn new(label: &str) -> Ecs {
        unsafe {
            *LARGE_ID += 1;
            new_with_id(label, *LARGE_ID as u32)
        }
    }
}

fn new_with_id(label: &str, id: u32) -> Ecs {
    Ecs {
        label,
        path: "assets/aws/compute/elastic-container-service.png",
        id,
    }
}

impl Node for Ecs<'_> {
    fn label(&self) -> &str {
        self.label
    }

    fn image_path(&self) -> &str {
        self.path
    }

    fn id(&self) -> u32 {
        self.id
    }
}

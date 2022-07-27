use crate::global::LARGE_ID;
use crate::graph::Node;

pub struct Rds<'a> {
    label: &'a str,
    path: &'a str,
    id: u32,
}

impl Rds<'_> {
    pub fn new(label: &str) -> Rds {
        unsafe {
            *LARGE_ID += 1;
            new_with_id(label, *LARGE_ID as u32)
        }
    }
}

fn new_with_id(label: &str, id: u32) -> Rds {
    Rds {
        label,
        path: "assets/aws/database/rds.png",
        id,
    }
}

impl Node for Rds<'_> {
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

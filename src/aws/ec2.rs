use crate::global::LARGE_ID;
use crate::graph::Node;

pub struct Ec2<'a> {
    label: &'a str,
    path: &'a str,
    id: u32,
}

impl Ec2<'_> {
    pub fn new(label: &str) -> Ec2 {
        unsafe {
            *LARGE_ID += 1;
            new_with_id(label, *LARGE_ID as u32)
        }
    }
}

fn new_with_id(label: &str, id: u32) -> Ec2 {
    Ec2 {
        label,
        path: "assets/aws/compute/ec2.png",
        id,
    }
}

impl Node for Ec2<'_> {
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

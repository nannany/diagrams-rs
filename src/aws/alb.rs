use crate::graph::Node;

pub struct Alb<'a> {
    label: &'a str,
    path: &'a str,
}

impl Alb<'_> {
    pub fn new(label: &str) -> Alb {
        Alb {
            label,
            path: "assets/aws/network/elb-application-load-balancer.png",
        }
    }
}

impl Node for Alb<'_> {
    fn label(&self) -> &str {
        self.label
    }

    fn image_path(&self) -> &str {
        self.path
    }

    fn id(&self) -> &str {
        todo!()
    }
}

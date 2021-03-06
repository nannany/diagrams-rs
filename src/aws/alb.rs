use crate::global::LARGE_ID;
use crate::graph::Node;

pub struct Alb<'a> {
    label: &'a str,
    path: &'a str,
    id: u32,
}

impl Alb<'_> {
    pub fn new(label: &str) -> Alb {
        unsafe {
            *LARGE_ID += 1;
            new_with_id(label, *LARGE_ID as u32)
        }
    }
}

fn new_with_id(label: &str, id: u32) -> Alb {
    Alb {
        label,
        path: "assets/aws/network/elb-application-load-balancer.png",
        id,
    }
}

impl Node for Alb<'_> {
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

#[cfg(test)]
mod tests {
    use crate::aws::alb::Alb;

    #[test]
    fn check_global_id() {
        let alb_1 = Alb::new("label1");
        let alb_2 = Alb::new("label2");
        assert_eq!(alb_1.id + 1, alb_2.id);
    }
}

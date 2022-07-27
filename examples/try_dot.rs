use diagrams_rs::aws::alb::Alb;
use diagrams_rs::aws::ec2::Ec2;
use diagrams_rs::aws::rds::Rds;
use diagrams_rs::graph::Diagram;

fn main() {
    let mut diagram = Diagram::new();

    let alb1 = Alb::new("lb");
    let worker1 = Ec2::new("worker1");
    let worker2 = Ec2::new("worker2");
    let worker3 = Ec2::new("worker3");
    let worker4 = Ec2::new("worker4");
    let rds1 = Rds::new("Grouped Workers");

    diagram.connect(&alb1, &worker1);
    diagram.connect(&alb1, &worker2);
    diagram.connect(&alb1, &worker3);
    diagram.connect(&alb1, &worker4);

    diagram.connect(&worker1, &rds1);
    diagram.connect(&worker2, &rds1);
    diagram.connect(&worker3, &rds1);
    diagram.connect(&worker4, &rds1);

    diagram.render_to("mydot.dot");
}

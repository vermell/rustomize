use stage_promotion;
use stage_promotion::StagePromotion;

fn main() {
    let s = stage_promotion::Stage { name: String::from("dev-nbg"), deployment: 3 };

    let e = stage_promotion::Stage { name: String::from("prod-nbg"), deployment: 2 };

    let d = stage_promotion::MockingDeployment;
    let foo = d.promote(&s, &e);
    println!("Hello, world from {:?}", foo);
}

use std::string::ToString;

struct Circle {
    radius: i32
}

impl ToString for Circle {
    fn to_string(&self) -> String {
        format!("Circle of radius {:?}", self.radius)
    }
}

fn main1() {
    let circle = Circle { radius: 6 };
    println!("{}", circle.to_string());
}
fn main2() {
    let parsed: i32 = "5".parse().unwrap();
    let turbo_parsed = "10".parse::<i32>().unwrap();

    let sum = parsed + turbo_parsed;
    println!{"Sum: {:?}", sum};
}
pub fn main(){
    main1();
    println!("++++++++++++++++++++ main1 结束 +++++++++++++++++++++++++");
    main2();
    println!("++++++++++++++++++++ main2 结束 +++++++++++++++++++++++++");
}

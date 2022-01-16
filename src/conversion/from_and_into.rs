use std::convert::From;

#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

fn main1() {
    let num = Number::from(30);
    println!("My number is {:?}", num);
}

fn main2() {
    let int = 5;
    // 试试删除类型说明
    let num: Number = int.into();
    println!("My number is {:?}", num);
}

pub fn main(){
    main1();
    println!("++++++++++++++++++++ main1 结束 +++++++++++++++++++++++++");
    main2();
    println!("++++++++++++++++++++ main2 结束 +++++++++++++++++++++++++");

}

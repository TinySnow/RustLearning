#[allow(dead_code)]
fn main1() {
    // 语句
    // 语句
    // 语句
}


fn main2() {
    // 变量绑定
    #[allow(unused_variables)]
    let x = 5;

    // 表达式;
    // x;
    // x + 1;
    // 15;
}

fn main3() {
    let x = 5u32;

    let y = {
        let x_squared = x * x;
        let x_cube = x_squared * x;

        // 将此表达式赋给 `y`
        x_cube + x_squared + x
    };

    let z = {
        // 分号结束了这个表达式，于是将 `()` 赋给 `z`
        // 2 * x;
        2 * x
    };

    println!("x is {:?}", x);
    println!("y is {:?}", y);
    println!("z is {:?}", z);
}

pub fn main() {
    main1();
    println!("++++++++++++++++++++ main1 结束 +++++++++++++++++++++++++");
    main2();
    println!("++++++++++++++++++++ main2 结束 +++++++++++++++++++++++++");
    main3();
    println!("++++++++++++++++++++ main3 结束 +++++++++++++++++++++++++");
}
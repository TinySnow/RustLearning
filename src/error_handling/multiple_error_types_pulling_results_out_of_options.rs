use std::num::ParseIntError;

fn double_first1(vec: Vec<&str>) -> Option<Result<i32, ParseIntError>> {
    vec.first().map(|first| {
        first.parse::<i32>().map(|n| 2 * n)
    })
}

fn main1() {
    let numbers = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    println!("The first doubled is {:?}", double_first1(numbers));

    println!("The first doubled is {:?}", double_first1(empty));
    // Error 1: the input vector is empty

    println!("The first doubled is {:?}", double_first1(strings));
    // Error 2: the element doesn't parse to a number
}


fn double_first2(vec: Vec<&str>) -> Result<Option<i32>, ParseIntError> {
    let opt = vec.first().map(|first| {
        first.parse::<i32>().map(|n| 2 * n)
    });

    opt.map_or(Ok(None), |r| r.map(Some))
}

fn main2() {
    let numbers = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    println!("The first doubled is {:?}", double_first2(numbers));
    println!("The first doubled is {:?}", double_first2(empty));
    println!("The first doubled is {:?}", double_first2(strings));
}

pub fn main(){
    main1();
    println!("++++++++++++++++++++ main1 结束 +++++++++++++++++++++++++");
    main2();
    println!("++++++++++++++++++++ main2 结束 +++++++++++++++++++++++++");
}
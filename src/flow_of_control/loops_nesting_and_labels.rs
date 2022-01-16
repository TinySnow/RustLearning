#![allow(unreachable_code)]

pub fn main() {
    'outer: loop {
        println!("Entered the outer loop");
        #[allow(unused_labels)]
        'inner: loop {
            println!("Entered the inner loop");

            // 这只是中断内部的循环
            //break;

            // 这会中断外层循环
            break 'outer;
        }

        println!("This point will never be reached");
    }

    println!("Exited the outer loop");
}

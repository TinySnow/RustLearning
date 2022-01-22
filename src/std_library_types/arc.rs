///
/// 请注意，此篇章内容为英文版更新内容，暂无中文翻译
/// 以及，代码中涉及多线程操作，可能出现结果不一致的情况
///
use std::sync::Arc;
use std::thread;

pub fn main() {
    // This variable declaration is where its value is specified.
    let apple = Arc::new("the same apple");

    for _ in 0..10 {
        // Here there is no value specification as it is a pointer to a reference
        // in the memory heap.
        let apple = Arc::clone(&apple);

        thread::spawn(move || {
            // As Arc was used, threads can be spawned using the value allocated
            // in the Arc variable pointer's location.
            println!("{:?}", apple);
        });
    }
}


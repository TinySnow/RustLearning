///
/// 注意：
/// 该文件并未被包含在整个项目结构中，所以不会被编译
/// 如需编译该文件，按照官方教程进行编译，需要用到教程中已经编译好的 library.rlib 文件
/// 命令如下：
/// rustc src/use_crate/executable.rs --extern rary=library.rlib --edition=2018
/// 参考链接：
/// https://rustwiki.org/zh-CN/rust-by-example/crates/using_lib.html 或者
/// https://rustwiki.org/en/rust-by-example/crates/using_lib.html
///

// extern crate rary; // 在 Rust 2015 版或更早版本需要这个导入语句

fn main() {
    rary::public_function();

    // 报错！ `private_function` 是私有的
    //rary::private_function();

    rary::indirect_access();
}

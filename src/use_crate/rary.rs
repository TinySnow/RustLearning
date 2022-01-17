///
/// 注意：
/// 该文件并未被包含在整个项目结构中，所以不会被编译
/// 因为加了层级目录，编译时请使用新目录命令，命令如下：
/// rustc --crate-type=lib src/use_crate/rary.rs
/// 生成的 lib 文件在 src 同级目录
///
///
// // 这个 crate 是一个库文件
// #![crate_type = "lib"]
// // 库的名称为 “rary”
// #![crate_name = "rary"]

pub fn public_function() {
    println!("called rary's `public_function()`");
}

fn private_function() {
    println!("called rary's `private_function()`");
}

pub fn indirect_access() {
    print!("called rary's `indirect_access()`, that\n> ");

    private_function();
}

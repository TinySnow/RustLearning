///
/// 注意：
/// 该文件并未被包含在项目结构中
///
#[cfg(some_condition)]
fn conditional_function() {
    println!("condition met!")
}

pub fn main() {
    conditional_function();
}

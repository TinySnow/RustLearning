#![allow(dead_code)]

// 位于模块 common
pub fn setup() {
    // 一些配置代码，比如创建文件/目录，开启服务器等等。
}
// 假设这个 crate 叫做 adder，我们需要在集成测试中用 extern 说明。
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
// 声明被测试的外部 crate，就像其他使用该 crate 的程序要声明的那样。
// 被测试的外部 crate。
// extern crate adder;

// 导入共用模块。
// mod common;

#[test]
fn test_add() {
    // 使用共用模块。
    // common::setup();
    assert_eq!(adder::add(3, 2), 5);
}

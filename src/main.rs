mod hello_world;
mod origin;

fn main() {
    hello_world::hello_world::main();
    println!("----------------------- hello_world::hello_world::main(); 结束 --------------------------");
    hello_world::ignore::main();
    println!("----------------------- hello_world::ignore::main(); 结束 --------------------------");
    hello_world::format::main();
    println!("----------------------- hello_world::format::main(); 结束 --------------------------");
    hello_world::format_debug::main();
    println!("----------------------- hello_world::format_debug::main(); 结束 --------------------------");
    hello_world::format_display::main();
    println!("----------------------- hello_world::format_display::main(); 结束 --------------------------");
    hello_world::format_list::main();
    println!("----------------------- hello_world::format_list::main(); 结束 --------------------------");
    hello_world::format_fmt::main();
    println!("----------------------- hello_world::format_fmt::main(); 结束 --------------------------");
    origin::origin::main();
    println!("----------------------- origin::origin::main(); 结束 --------------------------");
}

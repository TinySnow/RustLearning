static LOREM_IPSUM: &'static str =
    "This is file content of src/std_misc/file_io_create.rs file.";

// use std::error::Error;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

pub fn main() {
    let path = Path::new("output.txt");
    let display = path.display();

    // 以只写模式打开文件，返回 `io::Result<File>`
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    // 将 `LOREM_IPSUM` 字符串写进 `file`，返回 `io::Result<()>`
    match file.write_all(LOREM_IPSUM.as_bytes()) {
        Err(why) => {
            panic!("couldn't write to {}: {}", display, why)
        }
        Ok(_) => println!("successfully wrote to {}", display),
    }
}

trait Person {
    fn name(&self) -> String;
}

// Person 是 Student 的父 trait。
// 实现 Student 需要你也 impl 了 Person。
trait Student: Person {
    fn university(&self) -> String;
}

trait Programmer {
    fn fav_language(&self) -> String;
}

// CompSciStudent (computer science student，计算机科学的学生) 是 Programmer 和 Student 两者的子类。
// 实现 CompSciStudent 需要你同时 impl 了两个父 trait。
trait CompSciStudent: Programmer + Student {
    fn git_username(&self) -> String;
}

#[allow(dead_code)]
fn comp_sci_student_greeting(student: &dyn CompSciStudent) -> String {
    format!(
        "My name is {} and I attend {}. My favorite language is {}. My Git username is {}",
        student.name(),
        student.university(),
        student.fav_language(),
        student.git_username()
    )
}

// struct TestStudent {
//     name: String
// }
//
// impl CompSciStudent for TestStudent {
//     fn git_username(&self) -> String {
//         unimplemented!()
//     }
// }
//
// impl Programmer for TestStudent {
//     fn fav_language(&self) -> String {
//         unimplemented!()
//     }
// }
//
// impl Student for TestStudent {
//     fn university(&self) -> String {
//         unimplemented!()
//     }
// }
//
// impl Person for TestStudent {
//     fn name(&self) -> String {
//         unimplemented!()
//     }
// }

pub fn main() {
    // let testing: &dyn CompSciStudent = TestStudent { name: "杨懿".parse().unwrap() };
    // comp_sci_student_greeting(testing);
}

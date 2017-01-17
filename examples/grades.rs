extern crate htwdresden;

use htwdresden::{Login, Course, Grade};

fn main() {
    let login = Login::new(include_str!("secret_login").trim(),
                           include_str!("secret_password").trim());
    let courses = Course::get(&login).unwrap();
    println!("{:?}", courses);

    let grades = Grade::get(&login, &courses[0]).unwrap();
    println!("{:?}", grades);
}

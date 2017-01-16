extern crate htwdresden;

use htwdresden::{Login, grades};

fn main() {
    let login = Login::new(include_str!("secret_login").trim(),
                           include_str!("secret_password").trim());
    let courses = grades::get_courses(&login).unwrap();
    println!("{:?}", courses);

    let grades = grades::get_grades(&login, &courses[0]).unwrap();
    println!("{:?}", grades);
}

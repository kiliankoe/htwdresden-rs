extern crate htwdresden;

use std::env;
use htwdresden::{Login, Course, Grade};

fn main() {
    let login = env::var("LOGIN").unwrap_or_else(|_| "".into());
    let password = env::var("PASSWORD").unwrap_or_else(|_| "".into());
    let login = Login::new(&login, &password);

    if let Ok(courses) = Course::get(&login) {
        println!("{:?}", courses);

        let grades = Grade::get(&login, &courses[0]).unwrap();
        println!("{:?}", grades);
    } else {
        println!("Couldn't find courses. Wrong login perhaps?");
    }
}

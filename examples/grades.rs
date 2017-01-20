extern crate htwdresden;

use htwdresden::{Login, Course, Grade};

fn main() {
    let login = option_env!("LOGIN").unwrap_or("");
    let password = option_env!("PASSWORD").unwrap_or("");
    let login = Login::new(login, password);

    if let Ok(courses) = Course::get(&login) {
        println!("{:?}", courses);

        let grades = Grade::get(&login, &courses[0]).unwrap();
        println!("{:?}", grades);
    } else {
        println!("Couldn't find courses. Wrong login perhaps?");
    }
}

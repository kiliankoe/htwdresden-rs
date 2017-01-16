extern crate htwdresden;

use htwdresden::{Login, grades};

fn main() {
    let login = Login::new(include_str!("secret_login"),
                           include_str!("secret_password"));
    grades::get_courses(&login);
}

extern crate htwdresden;

use htwdresden::{Login, grades};

fn main() {
    let login = Login::new(include_str!("secret_login").trim(),
                           include_str!("secret_password").trim());
    grades::get_courses(&login);
}

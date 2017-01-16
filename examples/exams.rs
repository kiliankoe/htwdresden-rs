extern crate htwdresden;

use htwdresden::*;

fn main() {
    let e = exams::student_exams(2016, 121, Degree::Bachelor).unwrap();
    println!("{:#?}", e);
    // exams::prof_exams("Rennekamp");
}

extern crate htwdresden;

use htwdresden::*;

fn main() {
    println!("Exams for 16/121/61:");
    let e = exams::student_exams(2016, 121, Degree::Bachelor).unwrap();
    println!("{:?}", e);

    println!("");

    println!("Exams for Prof. Rennekamp:");
    let e = exams::prof_exams("Rennekamp").unwrap();
    println!("{:?}", e);
}

extern crate htwdresden;

use htwdresden::{Degree, Studygroup, exams};

fn main() {
    println!("Exams for 16/121/61:");
    let group = Studygroup {
        year: 2016,
        course: 121,
        group: 61,
        degree: Degree::Bachelor,
    };
    let e = exams::student_exams(&group).unwrap();
    println!("{:?}", e);

    println!();

    println!("Exams for Prof. Rennekamp:");
    let e = exams::prof_exams("Rennekamp").unwrap();
    println!("{:?}", e);
}

extern crate htwdresden;

use htwdresden::{Degree, Studygroup, Exam};

fn main() {
    let group = Studygroup {
        year: 2016,
        course: 121,
        group: 61,
        degree: Degree::Bachelor,
    };
    println!("Exams for {}:", group.identifier());
    let exam = Exam::for_studygroup(&group).unwrap();
    println!("{:?}", exam);

    println!();

    println!("Exams for Prof. Rennekamp:");
    let exam = Exam::for_prof("Rennekamp").unwrap();
    println!("{:?}", exam);
}

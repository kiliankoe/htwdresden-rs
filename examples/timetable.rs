extern crate htwdresden;
use htwdresden::Degree;
use htwdresden::Studygroup;
use htwdresden::Lesson;

fn main() {
    let group = Studygroup {
        year: 2016,
        course: 121,
        group: 61,
        degree: Degree::Bachelor,
    };

    let lessons = Lesson::for_studygroup(&group).unwrap();
    println!("{:?}", lessons);
}

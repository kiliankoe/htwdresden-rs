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

    println!("Lessons for 16/121/61");
    let lessons = Lesson::for_studygroup(&group).unwrap();
    println!("{:?}", lessons);
    println!();

    println!("Lessons for Prof. Sobe");
    let lessons = Lesson::for_prof("Sobe").unwrap();
    println!("{:?}", lessons);
    println!();

    println!("Lessons for Room Z 254");
    let lessons = Lesson::for_room("Z 254").unwrap();
    println!("{:?}", lessons);
}

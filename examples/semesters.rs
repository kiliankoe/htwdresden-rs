extern crate htwdresden;

use htwdresden::SemesterPlan;

fn main() {
    let semesters = SemesterPlan::get().unwrap();
    println!("{:?}", semesters);
}

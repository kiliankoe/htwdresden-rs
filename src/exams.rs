use Degree;

pub fn student_exams(year: u16, course: u16, degree: Degree) {
    println!("Getting data for course {} in {} with degree {}.",
             course,
             year,
             degree.short());
}

pub fn prof_exams(prof: &str) {
    println!("Getting data for prof {}.", prof);
}

#[derive(Debug)]
pub struct Exam {
    pub title: String,
    pub exam_type: String,
    pub study_branch: String,
    pub day: String,
    pub start_time: String,
    pub end_time: String,
    pub examiner: String,
    pub next_chance: String,
    pub rooms: Vec<String>,
}

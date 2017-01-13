pub enum Degree {
    Bachelor,
    Master,
    Diploma,
}

impl Degree {
    fn short(self) -> &'static str {
        match self {
            Degree::Bachelor => "B",
            Degree::Master => "M",
            Degree::Diploma => "D",
        }
    }
}

pub fn student_exams(year: u16, course: u16, degree: Degree) {
    println!("Getting data for course {} in {} with degree {}.",
             course,
             year,
             degree.short());
}

pub fn prof_exams(prof: &str) {
    println!("Getting data for prof {}.", prof);
}

pub mod grades;
pub mod exams;

#[derive(Debug)]
pub struct Studygroup {
    pub year: u16,
    pub course: u16,
    pub group: u16,
    pub degree: Degree,
}

impl Studygroup {
    pub fn identifier(&self) -> String {
        format!("{}/{}/{}", self.year, self.course, self.group)
    }
}

#[derive(Debug)]
pub enum Degree {
    Bachelor,
    Master,
    Diploma,
}

impl Degree {
    pub fn short(self) -> &'static str {
        match self {
            Degree::Bachelor => "B",
            Degree::Master => "M",
            Degree::Diploma => "D",
        }
    }
}

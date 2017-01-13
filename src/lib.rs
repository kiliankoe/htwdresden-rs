pub mod grades;
pub mod exams;

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

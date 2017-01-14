pub mod grades;
pub mod exams;

pub type Year = u16;

#[derive(Debug)]
pub struct Studygroup {
    pub year: Year,
    pub course: u16,
    pub group: u16,
    pub degree: Degree,
}

impl Studygroup {
    pub fn identifier(&self) -> String {
        format!("{}/{}/{}", self.year, self.course, self.group)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Degree {
    Bachelor,
    Master,
    Diploma,
}

impl Degree {
    pub fn short(&self) -> &'static str {
        match *self {
            Degree::Bachelor => "B",
            Degree::Master => "M",
            Degree::Diploma => "D",
        }
    }
}

#[derive(Debug)]
pub enum Semester {
    Winter(Year),
    Summer(Year),
}

pub struct Login {
    snumber: String,
    password: String,
}

impl Login {
    pub fn new(snumber: &str, password: &str) -> Login {
        Login {
            snumber: snumber.to_string(),
            password: password.to_string(),
        }
    }
}

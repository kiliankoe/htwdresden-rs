extern crate reqwest;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod grades;
pub use grades::{Course, Grade};
mod exams;
pub use exams::Exam;
mod lesson;
pub use lesson::Lesson;
mod rooms;
pub use rooms::Room;

pub type Year = u16;
pub type CourseId = u16;

#[derive(Deserialize, Debug)]
pub enum Week {
    Even,
    Odd,
}

#[derive(Deserialize, Debug)]
pub enum Weekday {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

#[derive(Debug)]
pub struct Studygroup {
    pub year: Year,
    pub course: CourseId,
    pub group: u16,
    pub degree: Degree,
}

impl Studygroup {
    pub fn identifier(&self) -> String {
        format!("{}/{}/{}", self.year, self.course, self.group)
    }
}

/// A degree, e.g. something you graduate with.
#[derive(Debug, Clone, Copy)]
pub enum Degree {
    Bachelor = 'B' as isize,
    Master = 'M' as isize,
    Diploma = 'D' as isize,
}

impl Degree {
    // fn short(&self) -> &'static str {
    //     match *self {
    //         Degree::Bachelor => "B",
    //         Degree::Master => "M",
    //         Degree::Diploma => "D",
    //     }
    // }

    // not quite sure yet if this a better solution than above...
    fn short(&self) -> String {
        ((*self as u8) as char).to_string()
    }
}

/// A semester, either Winter or Summer with a corresponding year.
#[derive(Debug)]
pub enum Semester {
    Winter(Year),
    Summer(Year),
}

/// HTW Buildings supported by the API.
#[derive(Debug)]
pub enum Building {
    Z,
    S,
}

/// A login used to authenticate with the server.
pub struct Login {
    snumber: String,
    password: String,
}

impl Login {
    /// Create a new login
    ///
    /// # Arguments
    ///
    /// * `snumber` - The "sNummer", e.g. s12345
    /// * `password` - Password
    pub fn new(snumber: &str, password: &str) -> Login {
        Login {
            snumber: snumber.to_string(),
            password: password.to_string(),
        }
    }
}

/// A set of errors that may occur during execution.
#[derive(Debug)]
pub enum Error {
    Network(reqwest::Error),
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Error::Network(err)
    }
}

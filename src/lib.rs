extern crate reqwest;
extern crate json;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod grades;
pub use grades::{Course, Grade};
mod exams;
pub use exams::Exam;

use std::io::Read;
use std::collections::HashMap;
use json::JsonValue;

pub type Year = u16;
pub type CourseId = u16;

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
    Bachelor,
    Master,
    Diploma,
}

impl Degree {
    fn short(&self) -> &'static str {
        match *self {
            Degree::Bachelor => "B",
            Degree::Master => "M",
            Degree::Diploma => "D",
        }
    }
}

/// A semester, either Winter or Summer with a corresponding year.
#[derive(Debug)]
pub enum Semester {
    Winter(Year),
    Summer(Year),
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
pub enum HTWError {
    Network(reqwest::Error),
    Json(json::Error),
    Decoding(&'static str),
}

impl From<reqwest::Error> for HTWError {
    fn from(err: reqwest::Error) -> Self {
        HTWError::Network(err)
    }
}

impl From<json::Error> for HTWError {
    fn from(err: json::Error) -> Self {
        HTWError::Json(err)
    }
}

// internal stuff

fn json_string(json: &JsonValue, name: &'static str) -> Result<String, HTWError> {
    // not sure if this is a stupid idea or not
    let str = json[name].as_str().ok_or_else(|| HTWError::Decoding(name))?;
    Ok(String::from(str))
}

fn get_json(url: &str) -> Result<JsonValue, HTWError> {
    let mut res = match reqwest::get(url) {
        Ok(res) => res,
        Err(err) => return Err(HTWError::Network(err)),
    };

    let mut response = String::new();
    let _ = res.read_to_string(&mut response);

    match json::parse(&response) {
        Ok(json) => Ok(json),
        Err(err) => Err(HTWError::Json(err)),
    }
}

trait FromJson {
    fn from_json(json: JsonValue) -> Result<Self, HTWError> where Self: std::marker::Sized;

    fn mult_from_json(json: JsonValue) -> Result<Vec<Self>, HTWError>
        where Self: std::marker::Sized
    {
        let arr = match json {
            JsonValue::Array(arr) => arr,
            _ => panic!("Can't instantiate many values from non-array in JSON."),
        };

        // TODO: How does map or whatever is equivalent work in Rust?
        let mut values: Vec<Self> = Vec::new();
        for json_val in arr {
            let val = Self::from_json(json_val)?;
            values.push(val);
        }

        Ok(values)
    }
}

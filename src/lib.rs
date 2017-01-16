extern crate reqwest;
extern crate json;

pub mod grades;
pub mod exams;

use std::io::Read;
use std::collections::HashMap;
use json::JsonValue;

pub type Year = u16;
pub type Course = u16;

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

// internal stuff

fn get_json(url: &str) -> JsonValue {
    let mut res = reqwest::get(url).unwrap();

    let mut response = String::new();
    res.read_to_string(&mut response);

    json::parse(&response).unwrap()
}

fn post_json(url: &str, params: HashMap<&str, String>) -> JsonValue {
    let client = reqwest::Client::new().unwrap();
    let mut res = client.post(url)
        .form(&params)
        .send()
        .unwrap();

    let mut response = String::new();
    res.read_to_string(&mut response);

    json::parse(&response).unwrap()
}

use Login;
use FromJson;
use post_json;

use std::collections::HashMap;
use json::JsonValue;

pub fn get_courses(login: &Login) -> Option<Vec<Course>> {
    let url = "https://wwwqis.htw-dresden.de/appservice/getcourses";
    let mut map = HashMap::new();
    map.insert("sNummer", login.snumber.clone()); // is cloning ok?
    map.insert("RZLogin", login.password.clone());

    let json = post_json(url, map);
    let courses = Course::mult_from_json(json);
    Some(courses)
}

#[derive(Debug)]
pub struct Course {
    pub degree_txt: String,
    pub reg_version: String,
    pub deg_nr: String,
    pub course_nr: String,
    pub course_note: String,
}

impl FromJson for Course {
    fn from_json(json: JsonValue) -> Course {
        Course {
            degree_txt: String::from(json["AbschlTxt"].as_str().unwrap()),
            reg_version: String::from(json["POVersion"].as_str().unwrap()),
            deg_nr: String::from(json["AbschlNr"].as_str().unwrap()),
            course_nr: String::from(json["StgNr"].as_str().unwrap()),
            course_note: String::from(json["StgTxt"].as_str().unwrap()),
        }
    }
}

pub fn get_grades(login: &Login, course: &Course) {}

#[derive(Debug)]
pub struct Grade {
    pub exam_nr: String,
    pub status: String,
    pub ects_credits: String,
    pub exam_txt: String,
    pub semester: String,
    pub try_count: String,
    pub exam_date: String,
    pub grade: String,
    pub vo_date: String,
    pub exam_form: String,
    pub comment: String,
    pub ects_grade: String,
}

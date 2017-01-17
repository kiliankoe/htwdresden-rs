use HTWError;
use Login;
use FromJson;
use post_json;

use std::collections::HashMap;
use json::JsonValue;

/// A course of studies.
#[derive(Debug)]
pub struct Course {
    pub degree_txt: String,
    pub regulation_version: String,
    pub deg_nr: String,
    pub course_nr: String,
    pub course_note: String,
}

impl FromJson for Course {
    fn from_json(json: JsonValue) -> Result<Self, HTWError> {
        let course = Course {
            // FIXME
            degree_txt: String::from(json["AbschlTxt"].as_str().unwrap()),
            regulation_version: String::from(json["POVersion"].as_str().unwrap()),
            deg_nr: String::from(json["AbschlNr"].as_str().unwrap()),
            course_nr: String::from(json["StgNr"].as_str().unwrap()),
            course_note: String::from(json["StgTxt"].as_str().unwrap()),
        };

        Ok(course)
    }
}

impl Course {
    /// Returns a list of courses for a given login.
    ///
    /// # Arguments
    ///
    /// * `login` - Login
    ///
    /// # Example
    ///
    /// ```
    /// use htwdresden::{Login, Course};
    ///
    /// let login = Login::new("s#####", "password");
    /// let courses = Course::get(&login);
    /// ```
    pub fn get(login: &Login) -> Option<Vec<Course>> {
        let url = "https://wwwqis.htw-dresden.de/appservice/getcourses";
        let mut map = HashMap::new();
        map.insert("sNummer", login.snumber.clone()); // is cloning ok?
        map.insert("RZLogin", login.password.clone());

        let json = post_json(url, map);
        let courses = Course::mult_from_json(json).unwrap();
        Some(courses)
    }
}

/// A grade, hopefully a good one.
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
    pub publish_date: String,
    pub exam_form: String,
    pub comment: String,
    pub ects_grade: String,
}

impl FromJson for Grade {
    fn from_json(json: JsonValue) -> Result<Self, HTWError> {
        let grade = Grade {
            // FIXME
            exam_nr: String::from(json["PrNr"].as_str().unwrap()),
            status: String::from(json["Status"].as_str().unwrap()),
            ects_credits: String::from(json["EctsCredits"].as_str().unwrap()),
            exam_txt: String::from(json["PrTxt"].as_str().unwrap()),
            semester: String::from(json["Semester"].as_str().unwrap()),
            try_count: String::from(json["Versuch"].as_str().unwrap()),
            exam_date: String::from(json["PrDatum"].as_str().unwrap()),
            grade: String::from(json["PrNote"].as_str().unwrap()),
            publish_date: String::from(json["VoDatum"].as_str().unwrap()),
            exam_form: String::from(json["PrForm"].as_str().unwrap()),
            comment: String::from(json["Vermerk"].as_str().unwrap()),
            ects_grade: String::from(json["EctsGrade"].as_str().unwrap()),
        };

        Ok(grade)
    }
}

impl Grade {
    /// Returns a list of grades for a given login and course.
    ///
    /// # Arguments
    ///
    /// * `login` - Login
    /// * `course` - Course
    ///
    /// # Example
    ///
    /// ```
    /// use htwdresden::{Login, Course, Grade};
    ///
    /// let login = Login::new("s#####", "password");
    /// let courses = Course::get(&login).unwrap();
    /// let grades = Grade::get(&login, &courses[0]);
    /// ```
    pub fn get(login: &Login, course: &Course) -> Option<Vec<Grade>> {
        let url = "https://wwwqis.htw-dresden.de/appservice/getgrades";
        let mut map = HashMap::new();
        map.insert("sNummer", login.snumber.clone());
        map.insert("RZLogin", login.password.clone());
        map.insert("AbschlNr", course.deg_nr.clone());
        map.insert("StgNr", course.course_nr.clone());
        map.insert("POVersion", course.regulation_version.clone());

        let json = post_json(url, map);
        let grades = Grade::mult_from_json(json).unwrap();
        Some(grades)
    }
}

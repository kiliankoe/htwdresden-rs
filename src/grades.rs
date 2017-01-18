use HTWError;
use Login;
use FromJson;
use json_string;
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
            degree_txt: json_string(&json, "AbschlTxt")?,
            regulation_version: json_string(&json, "POVersion")?,
            deg_nr: json_string(&json, "AbschlNr")?,
            course_nr: json_string(&json, "StgNr")?,
            course_note: json_string(&json, "StgTxt")?,
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
    pub fn get(login: &Login) -> Result<Vec<Course>, HTWError> {
        let url = "https://wwwqis.htw-dresden.de/appservice/getcourses";
        let mut map = HashMap::new();
        map.insert("sNummer", login.snumber.clone()); // is cloning ok?
        map.insert("RZLogin", login.password.clone());

        let json = post_json(url, map)?;
        Ok(Course::mult_from_json(json)?)
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
            exam_nr: json_string(&json, "PrNr")?,
            status: json_string(&json, "Status")?,
            ects_credits: json_string(&json, "EctsCredits")?,
            exam_txt: json_string(&json, "PrTxt")?,
            semester: json_string(&json, "Semester")?,
            try_count: json_string(&json, "Versuch")?,
            exam_date: json_string(&json, "PrDatum")?,
            grade: json_string(&json, "PrNote")?,
            publish_date: json_string(&json, "VoDatum")?,
            exam_form: json_string(&json, "PrForm")?,
            comment: json_string(&json, "Vermerk")?,
            ects_grade: json_string(&json, "EctsGrade")?,
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
    /// if let Ok(courses) = Course::get(&login) {
    ///     let grades = Grade::get(&login, &courses[0]);
    /// }
    /// ```
    pub fn get(login: &Login, course: &Course) -> Result<Vec<Grade>, HTWError> {
        let url = "https://wwwqis.htw-dresden.de/appservice/getgrades";
        let mut map = HashMap::new();
        map.insert("sNummer", login.snumber.clone());
        map.insert("RZLogin", login.password.clone());
        map.insert("AbschlNr", course.deg_nr.clone());
        map.insert("StgNr", course.course_nr.clone());
        map.insert("POVersion", course.regulation_version.clone());

        let json = post_json(url, map)?;
        Ok(Grade::mult_from_json(json)?)
    }
}

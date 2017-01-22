use Error;
use Login;

use std::collections::HashMap;
use reqwest;

/// A course of studies.
#[derive(Deserialize, Debug)]
pub struct Course {
    #[serde(rename = "AbschlTxt")]
    pub degree_txt: String,
    #[serde(rename = "POVersion")]
    pub regulation_version: String,
    #[serde(rename = "AbschlNr")]
    pub deg_nr: String,
    #[serde(rename = "StgNr")]
    pub course_nr: String,
    #[serde(rename = "StgTxt")]
    pub course_note: String,
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
    pub fn get(login: &Login) -> Result<Vec<Course>, Error> {
        let url = "https://wwwqis.htw-dresden.de/appservice/getcourses";
        let mut map = HashMap::new();
        map.insert("sNummer", login.snumber.clone()); // is cloning ok?
        map.insert("RZLogin", login.password.clone());

        let client = reqwest::Client::new().expect("Failed to instantiate reqwest client o.O");
        let courses = client.post(url)
            .form(&map)
            .send()?
            .json()
            .map(|response: Vec<Course>| response)?;

        Ok(courses)
    }
}

/// A grade, hopefully a good one.
#[derive(Deserialize, Debug)]
pub struct Grade {
    #[serde(rename = "PrNr")]
    pub exam_nr: String,
    #[serde(rename = "Status")]
    pub status: String,
    #[serde(rename = "EctsCredits")]
    pub ects_credits: String,
    #[serde(rename = "PrTxt")]
    pub exam_txt: String,
    #[serde(rename = "Semester")]
    pub semester: String,
    #[serde(rename = "Versuch")]
    pub try_count: String,
    #[serde(rename = "PrDatum")]
    pub exam_date: String,
    #[serde(rename = "PrNote")]
    pub grade: String,
    #[serde(rename = "VoDatum")]
    pub publish_date: String,
    #[serde(rename = "PrForm")]
    pub exam_form: String,
    #[serde(rename = "Vermerk")]
    pub comment: String,
    #[serde(rename = "EctsGrade")]
    pub ects_grade: String,
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
    pub fn get(login: &Login, course: &Course) -> Result<Vec<Grade>, Error> {
        let url = "https://wwwqis.htw-dresden.de/appservice/getgrades";
        let mut map = HashMap::new();
        map.insert("sNummer", login.snumber.clone());
        map.insert("RZLogin", login.password.clone());
        map.insert("AbschlNr", course.deg_nr.clone());
        map.insert("StgNr", course.course_nr.clone());
        map.insert("POVersion", course.regulation_version.clone());

        let client = reqwest::Client::new().expect("Failed to instantiate reqwest client o.O");
        let grades = client.post(url)
            .form(&map)
            .send()?
            .json()
            .map(|response: Vec<Grade>| response)?;

        Ok(grades)
    }
}

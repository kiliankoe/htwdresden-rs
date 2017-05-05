use Error;
use Login;
use Year;

use std::fmt;
use std::collections::HashMap;
use reqwest;
use chrono;

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

#[derive(Deserialize, Debug)]
struct RawGrade {
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
    pub annotation: String,
    #[serde(rename = "EctsGrade")]
    pub ects_grade: String,
}

pub enum Status {
    /// AN = Angemeldet
    SignedUp,
    /// BE = bestanden
    Passed,
    /// EN = endgültig nicht bestanden
    Failed,
    /// NB = nicht bestanden
    UltimatelyFailed,
}

impl Status {
    fn via(string: &str) -> Option<Self> {
        match string {
            "AN" => Some(Status::SignedUp),
            "BE" => Some(Status::Passed),
            "NB" => Some(Status::Failed),
            "EN" => Some(Status::UltimatelyFailed),
            _    => None,
        }
    }
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let val = match self {
            SignedUp         => "Angemeldet",
            Passed           => "Bestanden",
            Failed           => "Nicht bestanden",
            UltimatelyFailed => "Endgültig nicht bestanden",
        };
        write!(f, "{}", val)
    }
}

pub enum Annotation {
    /// a = anerkannt
    Accredited,
    /// k = krank
    Sick,
    /// e = abgemeldet
    SignedOut,
    /// g = gesperrt
    Blocked,
    /// nz = nicht zugelassen
    NotPermitted,
    /// 5ue = unentschuldigt gefehlt
    UnexcusedAbsence,
    /// 5na = nicht angetreten - Fristüberschreitung
    NotReported,
    /// kA = kein Antrag 2. Wiederholungsprüfung gestellt
    NoSecondAttemptApplication,
    /// PFV = Freiversuch
    FreeAttempt,
    /// mE = mit Erfolg
    WithSuccess,
    /// N = nicht bestanden (unbenotete Leistung)
    Failed,
    /// VPo = Vorprakt. offen
    OpenPracticalCourse,
    /// f = freiwilliger Termin nicht wahrgenommen
    NoShowOptionalAppointment,
    /// uV = unter Vorbehalt
    WithReservation,
}

impl Annotation {
    fn via(string: &str) -> Option<Self> {
        match string {
            "a"   => Some(Annotation::Accredited),
            "k"   => Some(Annotation::Sick),
            "e"   => Some(Annotation::SignedOut),
            "g"   => Some(Annotation::Blocked),
            "nz"  => Some(Annotation::NotPermitted),
            "5ue" => Some(Annotation::UnexcusedAbsence),
            "5na" => Some(Annotation::NotReported),
            "kA"  => Some(Annotation::NoSecondAttemptApplication),
            "PFV" => Some(Annotation::FreeAttempt),
            "mE"  => Some(Annotation::WithSuccess),
            "N"   => Some(Annotation::Failed),
            "VPo" => Some(Annotation::OpenPracticalCourse),
            "f"   => Some(Annotation::NoShowOptionalAppointment),
            "uV"  => Some(Annotation::WithReservation),
            _     => None,
        }
    }
}

pub enum Semester {
    Winter(Year),
    Summer(Year),
}

impl Semester {
    fn via(string: &str) -> Option<Self> {
        let year_str = &string[0..3];
        let year: Year = year_str.to_owned().parse::<u16>().unwrap_or_default();
        let id = string.chars().nth(4).unwrap_or('0');

        match id {
            '1' => Some(Semester::Winter(year)),
            '2' => Some(Semester::Summer(year)),
            _   => None,
        }
    }
}

impl fmt::Display for Semester {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Semester::Winter(year) => write!(f, "WS {}", year),
            &Semester::Summer(year) => write!(f, "SS {}", year),
        }
    }
}

/// A grade, hopefully a good one.
pub struct Grade {
    pub exam_nr: String,
    pub status: Option<Status>,
    pub ects_credits: f32,
    pub exam_txt: String,
    pub semester: Option<Semester>,
    pub try_count: i8,
    pub exam_date: Option<chrono::DateTime<chrono::UTC>>,
    pub grade: Option<f32>,
    pub publish_date: Option<chrono::DateTime<chrono::UTC>>,
    pub exam_form: String, // should be it's own enum type, not sure about possible values though
    pub annotation: Option<Annotation>,
    pub ects_grade: Option<String>, // not sure what this should be, probably f32
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
            .map(|response: Vec<RawGrade>| response)?
            .into();

        Ok(grades)
    }
}

impl From<RawGrade> for Grade {
    fn from(raw: RawGrade) -> Self {
        Grade {
            exam_nr:      raw.exam_nr,
            status:       Status::via(&raw.status),
            ects_credits: raw.ects_credits.parse::<f32>().unwrap_or(0.0),
            exam_txt:     raw.exam_txt,
            semester:     Semester::via(&raw.semester),
            try_count:    raw.try_count.parse::<i8>().unwrap_or(0),
            exam_date:    chrono::DateTime::parse_from_str(&raw.exam_date, "%d.%m.%Y").ok(),
            grade:        Some(1.0), // TODO
            publish_date: chrono::DateTime::parse_from_str(&raw.publish_date, "%d.%m.%Y").ok(),
            exam_form:    raw.exam_form,
            annotation:   Annotation::via(&raw.annotation),
            ects_grade:   Some(raw.ects_grade),
        }
    }
}

impl fmt::Display for Grade {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {} {}", self.exam_txt, self.grade.unwrap_or(0.0), self.status.unwrap_or(Status::SignedUp)) // FIXME
    }
}

use Error;
use Year;

use reqwest;

#[derive(Debug, Deserialize)]
pub struct SemesterPlan {
    pub year: Year,
    #[serde(rename = "type")]
    pub semester_type: String,
    pub period: Period,
    #[serde(rename = "freeDays")]
    pub holidays: Vec<Holiday>,
    #[serde(rename = "lecturePeriod")]
    pub lecture_period: Period,
    #[serde(rename = "examsPeriod")]
    pub exam_period: Period,
    #[serde(rename = "reregistration")]
    pub reregistration_period: Period,
}

#[derive(Debug, Deserialize)]
pub struct Period {
    #[serde(rename = "beginDay")]
    begin_day: String,
    #[serde(rename = "endDay")]
    end_day: String,
}

#[derive(Debug, Deserialize)]
pub struct Holiday {
    name: String,
    #[serde(rename = "beginDay")]
    begin_day: String,
    #[serde(rename = "endDay")]
    end_day: String,
}

const BASE_URL: &'static str = "https://www2.htw-dresden.de/~app/API/semesterplan.json";

impl SemesterPlan {
    /// Returns all currently available `SemesterPlan`s.
    ///
    /// # Example
    ///
    /// ```
    /// use htwdresden::SemesterPlan;
    ///
    /// fn main() {
    ///     let semesters = SemesterPlan::get();
    /// }
    /// ```
    pub fn get() -> Result<Vec<SemesterPlan>, Error> {
        let semesters = reqwest::get(BASE_URL)?
            .json()
            .map(|response: Vec<SemesterPlan>| response)?;
        Ok(semesters)
    }
}

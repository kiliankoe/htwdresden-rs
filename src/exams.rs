use HTWError;
use Year;
use CourseId;
use Degree;
use Studygroup;

use reqwest;

const BASE_URL: &'static str = "https://www2.htw-dresden.de/~app/API/GetExams.php";

/// An exam, something to study for!
#[derive(Deserialize, Debug)]
pub struct Exam {
    #[serde(rename = "Title")]
    pub title: String,
    #[serde(rename = "ExamType")]
    pub exam_type: String,
    #[serde(rename = "StudyBranch")]
    pub study_branch: String,
    #[serde(rename = "Day")]
    pub day: String,
    #[serde(rename = "StartTime")]
    pub start_time: String,
    #[serde(rename = "EndTime")]
    pub end_time: String,
    #[serde(rename = "Examiner")]
    pub examiner: String,
    #[serde(rename = "NextChance")]
    pub next_chance: String,
    #[serde(rename = "Rooms")]
    pub rooms: Vec<String>,
}

impl Exam {
    /// Returns a list of `Exam`s for a given study group.
    ///
    /// # Arguments
    ///
    /// * `group` - Studygroup
    ///
    /// # Example
    ///
    /// ```
    /// use htwdresden::{Degree, Studygroup, Exam};
    ///
    /// let group = Studygroup { year: 2016, course: 121, group: 61, degree: Degree::Bachelor };
    /// let exams = Exam::for_studygroup(&group);
    /// ```
    pub fn for_studygroup(group: &Studygroup) -> Result<Vec<Exam>, HTWError> {
        Exam::for_student(group.year, group.course, group.degree)
    }

    /// Returns a list of `Exam`s for a given student.
    ///
    /// # Arguments
    ///
    /// * `year` - Year
    /// * `course` - Course identifier
    /// * `degree` - Degree
    ///
    /// # Example
    ///
    /// ```
    /// use htwdresden::{Degree, Exam};
    ///
    /// let exams = Exam::for_student(2016, 121, Degree::Bachelor);
    /// ```
    pub fn for_student(year: Year,
                       course: CourseId,
                       degree: Degree)
                       -> Result<Vec<Exam>, HTWError> {
        let url = format!("{base}?StgJhr={year}&Stg={course}&AbSc={degree}",
                          base = BASE_URL,
                          year = year,
                          course = course,
                          degree = degree.short());

        let exams = reqwest::get(&url)?
            .json()
            .map(|response: Vec<Exam>| response)?;

        Ok(exams)
    }

    /// Returns a list of `Exam`s for a given professor.
    ///
    /// # Arguments
    ///
    /// * `prof` - Professor name
    ///
    /// # Example
    ///
    /// ```
    /// use htwdresden::Exam;
    ///
    /// let exams = Exam::for_prof("Rennekamp");
    /// ```
    pub fn for_prof(prof: &str) -> Result<Vec<Exam>, HTWError> {
        let url = format!("{base}?Prof={prof}", base = BASE_URL, prof = prof);
        let exams = reqwest::get(&url)?
            .json()
            .map(|response: Vec<Exam>| response)?;

        Ok(exams)
    }
}

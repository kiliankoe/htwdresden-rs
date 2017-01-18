use json::JsonValue;

use HTWError;
use Year;
use CourseId;
use Degree;
use Studygroup;
use FromJson;
use json_string;
use get_json;

const BASE_URL: &'static str = "https://www2.htw-dresden.de/~app/API/GetExams.php";

/// An exam, something to study for!
#[derive(Debug)]
pub struct Exam {
    pub title: String,
    pub exam_type: String,
    pub study_branch: String,
    pub day: String,
    pub start_time: String,
    pub end_time: String,
    pub examiner: String,
    pub next_chance: String,
    pub rooms: Vec<String>,
}

impl FromJson for Exam {
    fn from_json(json: JsonValue) -> Result<Self, HTWError> {
        let exam = Exam {
            title: json_string(&json, "Title")?,
            exam_type: json_string(&json, "ExamType")?,
            study_branch: json_string(&json, "StudyBranch")?,
            day: json_string(&json, "Day")?,
            start_time: json_string(&json, "StartTime")?,
            end_time: json_string(&json, "EndTime")?,
            examiner: json_string(&json, "Examiner")?,
            next_chance: json_string(&json, "NextChance")?,
            rooms: vec![String::new()], // TODO
        };

        Ok(exam)
    }
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

        let json = get_json(&url);
        Ok(Exam::mult_from_json(json)?)
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
        let json = get_json(&url);
        Ok(Exam::mult_from_json(json)?)
    }
}

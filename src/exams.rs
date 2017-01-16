use json::JsonValue;

use Studygroup;
use FromJson;
use get_json;

const BASE_URL: &'static str = "https://www2.htw-dresden.de/~app/API/GetExams.php";

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
/// use htwdresden::{Degree, Studygroup, exams};
///
/// let group = Studygroup { year: 2016, course: 121, group: 61, degree: Degree::Bachelor };
/// let all_exams = exams::student_exams(&group);
/// ```
pub fn student_exams(group: &Studygroup) -> Option<Vec<Exam>> {
    let url = format!("{base}?StgJhr={year}&Stg={course}&AbSc={degree}",
                      base = BASE_URL,
                      year = group.year,
                      course = group.course,
                      degree = group.degree.short());

    let json = get_json(&url);
    let exams = Exam::mult_from_json(json);
    Some(exams)
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
/// use htwdresden::exams::prof_exams;
///
/// let exams = prof_exams("Rennekamp");
/// ```
pub fn prof_exams(prof: &str) -> Option<Vec<Exam>> {
    let url = format!("{base}?Prof={prof}", base = BASE_URL, prof = prof);
    let json = get_json(&url);
    let exams = Exam::mult_from_json(json);
    Some(exams)
}

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
    fn from_json(json: JsonValue) -> Exam {
        Exam {
            // wat o.O FIXME
            title: String::from(json["Title"].as_str().unwrap()),
            exam_type: String::from(json["ExamType"].as_str().unwrap()),
            study_branch: String::from(json["StudyBranch"].as_str().unwrap()),
            day: String::from(json["Day"].as_str().unwrap()),
            start_time: String::from(json["StartTime"].as_str().unwrap()),
            end_time: String::from(json["EndTime"].as_str().unwrap()),
            examiner: String::from(json["Examiner"].as_str().unwrap()),
            next_chance: String::from(json["NextChance"].as_str().unwrap()),
            rooms: vec![String::new()], // TODO
        }
    }
}

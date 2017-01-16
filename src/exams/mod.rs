use json::JsonValue;

use Year;
use Degree;
use Course;
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
/// use htwdresden::Degree;
/// use htwdresden::exams::student_exams;
///
/// let exams = student_exams(2016, 121, Degree::Bachelor);
/// ```
pub fn student_exams(year: Year, course: Course, degree: Degree) -> Option<Vec<Exam>> {
    let url = format!("{base}?StgJhr={year}&Stg={course}&AbSc={degree}",
                      base = BASE_URL,
                      year = year,
                      course = course,
                      degree = degree.short());

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
            // wat o.O
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

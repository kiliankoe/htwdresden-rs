use super::get_json;
use json::JsonValue;

use Degree;

const base_url: &'static str = "https://www2.htw-dresden.de/~app/API/GetExams.php";

pub fn student_exams(year: u16, course: u16, degree: Degree) -> Option<Vec<Exam>> {
    let url = format!("{base}?StgJhr={year}&Stg={course}&AbSc={degree}",
                      base = base_url,
                      year = year,
                      course = course,
                      degree = degree.short());

    let json = get_json(&url);
    let arr = match json {
        JsonValue::Array(arr) => arr,
        _ => panic!("No data in json array ¯\\_(ツ)_/¯"),
    };

    let mut exams: Vec<Exam> = Vec::new();
    for value in arr {
        let e = Exam::from_json(value);
        exams.push(e);
    }

    Some(exams)
}

pub fn prof_exams(prof: &str) {
    println!("Getting data for prof {}.", prof);
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

impl Exam {
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
            rooms: vec![String::new()],
        }
    }
}

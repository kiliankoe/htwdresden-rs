use hyper::Client;
use std::io::Read;
// use hyper::status::StatusCode;
use json;
use json::JsonValue;

use Degree;

const base_url: &'static str = "https://www2.htw-dresden.de/~app/API/GetExams.php";

pub fn student_exams(year: u16, course: u16, degree: Degree) {
    // println!("Getting exams for course {} in {} with degree {}.",
    //          course,
    //          year,
    //          degree.short());

    let client = Client::new();
    let url = format!("{base}?StgJhr={year}&Stg={course}&AbSc={degree}",
                      base = base_url,
                      year = year,
                      course = course,
                      degree = degree.short());

    let mut res = client.get(&url).send().unwrap();
    // println!("{:?}", res);

    let mut response = String::new();
    res.read_to_string(&mut response);
    // println!("{}", response);

    let json = json::parse(&response).unwrap();
    // println!("{}", json);
    let arr = match json {
        JsonValue::Array(arr) => arr,
        _ => panic!("No data in json array ¯\\_(ツ)_/¯"),
    };

    for value in arr {
        let e = Exam::from_json(value);
        println!("{:?}", e);
    }
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

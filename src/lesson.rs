use Week;
use Weekday;
use Studygroup;
use HTWError;

use reqwest;

const BASE_URL: &'static str = "https://www2.htw-dresden.de/~app/API/GetTimetable.php";

#[derive(Deserialize, Debug)]
pub struct Lesson {
    #[serde(rename = "lessonTag")]
    pub tag: String,
    pub name: String,
    #[serde(rename = "type")]
    pub lesson_type: String,
    pub week: i32, // FIXME: These should both be enum types, but serde doesn't want to :/
    pub day: i32,
    #[serde(rename = "beginTime")]
    pub begin_time: String,
    #[serde(rename = "endTime")]
    pub end_time: String,
    pub professor: String,
    #[serde(rename = "WeeksOnly")]
    pub weeks_only: String,
    #[serde(rename = "Rooms")]
    pub rooms: Vec<String>,
}

impl Lesson {
    pub fn for_studygroup(group: &Studygroup) -> Result<Vec<Lesson>, HTWError> {
        let url = format!("{}?StgJhr={}&Stg={}&StgGrp={}",
                          BASE_URL,
                          group.year,
                          group.course,
                          group.group);
        let lessons = reqwest::get(&url)?.json().map(|response: Vec<Lesson>| response)?;
        Ok(lessons)
    }

    pub fn for_prof(prof: &str) -> Result<Vec<Lesson>, HTWError> {
        let url = format!("{}?Prof={}", BASE_URL, prof);
        let lessons = reqwest::get(&url)?.json().map(|response: Vec<Lesson>| response)?;
        Ok(lessons)
    }

    pub fn for_room(room: &str) -> Result<Vec<Lesson>, HTWError> {
        let url = format!("{}?Room={}", BASE_URL, room);
        let lessons = reqwest::get(&url)?.json().map(|response: Vec<Lesson>| response)?;
        Ok(lessons)
    }
}

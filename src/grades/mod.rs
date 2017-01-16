use Login;
use get_json;

const base_url: &'static str = "https://wwwqis.htw-dresden.de/appservice/";

#[derive(Debug)]
pub struct Course {
    pub degree_txt: String,
    pub reg_version: String,
    pub deg_nr: String,
    pub course_nr: String,
    pub course_note: String,
}

pub fn get_courses(login: &Login) {}

#[derive(Debug)]
pub struct Grade {
    pub exam_nr: String,
    pub status: String,
    pub etcs_credits: String,
    pub exam_txt: String,
    pub semester: String,
    pub try_count: String,
    pub exam_date: String,
    pub grade: String,
    pub vo_date: String,
    pub exam_form: String,
    pub comment: String,
    pub ects_grade: String,
}

pub fn get_grades(login: &Login, course: &Course) {}

# 📚 htwdresden

[![Travis](https://img.shields.io/travis/kiliankoe/htwdresden-rs.svg?style=flat-square)](https://travis-ci.org/kiliankoe/htwdresden-rs)
![Crates.io](https://img.shields.io/crates/v/htwdresden.svg?style=flat-square)

Rust library for accessing organisational data from the [University of Applied Sciences Dresden](https://www.htw-dresden.de/).

## Examples

### Exams

```rust
use htwdresden::{Studygroup, Degree, Exam};

let group = Studygroup {
    year: 2016,
    course: 121,
    group: 61,
    degree: Degree::Bachelor,
};
let exams = Exam::for_studygroup(&group);

let exams = Exam::for_prof("prof identifier");
```

### Grades

```rust
use htwdresden::{Login, Course, Grade};

let login = Login::new("s#####", "password");
if let Ok(courses) = Course::get(&login) {
    let grades = Grade::get(&login, &courses[0]);
}
```

### Timetable

WIP

### Room Search

WIP

## Contributors

Kilian Koeltzsch, [@kiliankoe](https://github.com/kiliankoe)

## License

htwdresden is available under the MIT license. See the LICENSE file for more info.

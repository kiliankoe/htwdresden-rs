# 📚 htwdresden

[![Travis](https://img.shields.io/travis/kiliankoe/htwdresden-rs.svg?style=flat-square)](https://travis-ci.org/kiliankoe/htwdresden-rs)
[![Crates.io](https://img.shields.io/crates/v/htwdresden.svg?style=flat-square)](https://crates.io/crates/htwdresden)

Rust library for accessing organisational data from the [University of Applied Sciences Dresden](https://www.htw-dresden.de/).

Add the following to your dependencies in your `Cargo.toml`.

```toml
htwdresden = "0.3"
```

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

### Lessons

```rust
use htwdresden::{Degree, Studygroup, Lesson};

let group = Studygroup {
    year: 2016,
    course: 121,
    group: 61,
    degree: Degree::Bachelor,
};
let lessons = Lesson::for_studygroup(&group);

let lessons = Lesson::for_prof("prof identifier");

let lessons = Lesson::for_room("Z 254");
```

### Room Search

```rust
use htwdresden::{Week, Weekday, Room, Building};

let rooms = Room::get_free(Week::Even, Weekday::Monday, "9:30", "10:30", Building::Z);
```

### Semester Plan

```rust
use htwdresden::SemesterPlan;

let semesters = SemesterPlan::get();
```

## Contributors

Kilian Koeltzsch, [@kiliankoe](https://github.com/kiliankoe)

## License

htwdresden is available under the MIT license. See the LICENSE file for more info.

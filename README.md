# 📚 htwdresden

Rust library for accessing organisational data from the [University of Applied Sciences Dresden](https://www.htw-dresden.de/).

## Examples

### Exams

```rust
use htwdresden::{Degree, Exam};

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
use htwdresden::{Login, grades};

let login = Login::new("s#####", "password");
let courses = grades::get_courses(&login).unwrap();
let all_grades = grades::get_grades(&login, &courses[0]);
```

### Timetable

WIP

### Room Search

WIP

## Contributors

Kilian Koeltzsch, [@kiliankoe](https://github.com/kiliankoe)

## License

htwdresden is available under the MIT license. See the LICENSE file for more info.

# 📚 htwdresden

Rust library for accessing organisational data from the [University of Applied Sciences Dresden](https://www.htw-dresden.de/).

## Examples

### Exams

```rust
use htwdresden::{Degree, exams};

let e = exams::student_exams(2016, 121, Degree::Bachelor).unwrap();
let p = exams::prof_exams("prof identifier").unwrap();
```

### Grades

```rust
use htwdresden::{Login, grades};

let login = Login::new("s#####", "password");
let courses = grades::get_courses(&login).unwrap();
let all_grades = grades::get_grades(&login, &courses[0]).unwrap();
```

### Timetable

WIP

### Room Search

WIP

## Contributors

Kilian Koeltzsch, [@kiliankoe](https://github.com/kiliankoe)

## License

htwdresden is available under the MIT license. See the LICENSE file for more info.

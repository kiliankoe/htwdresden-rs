# 📚 htwdresden

Rust library for accessing organisational data from the [University of Applied Sciences Dresden](https://www.htw-dresden.de/).

## Examples

### Exams

```rust
use htwdresden::Degree;
use htwdresden::exams;

let e = exams::student_exams(2016, 121, Degree::Bachelor);
```

### Grades

WIP

### Timetable

WIP

### Room Search

WIP

## Contributors

Kilian Koeltzsch, [@kiliankoe](https://github.com/kiliankoe)

## License

htwdresden is available under the MIT license. See the LICENSE file for more info.

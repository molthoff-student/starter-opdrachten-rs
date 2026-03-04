/// Contains basic information about the student.
/// 
/// Lifetime specifier `<'a>` derives the lifetime from the given input.
/// 
/// Can be used to display the information of a student.
struct StudentInfo<'a> {
    first_name: &'a str,
    last_name: &'a str,
    class_code: &'a str,
    student_id: usize,
}

impl<'a> StudentInfo<'a> {
    /// Creates a new StudentInfo instance.
    fn new(
        first_name: &'a str,
        last_name: &'a str,
        class_code: &'a str,
        student_id: usize,
    ) -> Self {
        Self {
            first_name: first_name,
            last_name: last_name,
            class_code: class_code,
            student_id: student_id,
        }
    }
}

// Implement the formatting trait Display from the standart library.
// This allows any formatter to automatically format the struct to a String output.
impl<'a> std::fmt::Display for StudentInfo<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {}\n{}\n{}",
            self.first_name, self.last_name, self.student_id, self.class_code
        )
    }
}

fn main() {
    let student = StudentInfo::new(
        "Mick", 
        "Olthoff", 
        "XSDX1", 
        97132891
    );

    println!("{}", student);
}

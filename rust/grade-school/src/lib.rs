use itertools::Itertools;

#[allow(clippy::new_without_default)]
pub struct School {
    student_grade: Vec<(String, u32)>,
}

impl School {
    pub fn new() -> School {
        Self { student_grade: Vec::new() }
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        let student = student.to_string();
        for i in 0.. {
            if let Some((s, g)) = self.student_grade.get(i) {
                if *g < grade || (*g == grade && *s < student) {
                    continue;
                }
            }

            self.student_grade.insert(i, (student, grade));
            break;
        }
    }

    pub fn grades(&self) -> Vec<u32> {
        self.student_grade.iter()
            .map(|(_, g)| *g)
            .unique()
            .collect()
    }

    pub fn grade(&self, grade: u32) -> Vec<String> {
        self.student_grade.iter()
            .filter(|(_, g)| *g == grade)
            .map(|(s, _)| s.clone())
            .collect()
    }
}

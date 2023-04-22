const KIDS: [&str; 12] = [
    "Alice", "Bob", "Charlie", "David", "Eve", "Fred", "Ginny", "Harriet", "Ileana", "Joseph",
    "Kincaid", "Larry",
];
pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    let index = KIDS
        .iter()
        .position(|name| *name == student)
        .expect("Student's name not known")
        * 2;

    diagram
        .lines()
        .flat_map(|line| {
            line[index..=(index + 1)].chars().map(|plant| match plant {
                'V' => "violets",
                'R' => "radishes",
                'C' => "clover",
                'G' => "grass",
                _ => panic!("Plant type not known"),
            })
        })
        .collect()
}

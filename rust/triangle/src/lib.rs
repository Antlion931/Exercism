use std::ops::Add;

pub struct Triangle<T: Add<Output = T> + PartialOrd + Copy> {
    sides: [T; 3],
}

impl<T: Add<Output = T> + PartialOrd + Copy> Triangle<T> {
    pub fn build(sides: [T; 3]) -> Option<Self> {
        let mut sides = sides.clone();
        sides.sort_unstable_by(|a, b| a.partial_cmp(b).expect("Partial Compare error"));

        (sides[0] + sides[1] > sides[2]).then_some(Self { sides } )
    }

    pub fn is_equilateral(&self) -> bool {
        self.sides[0] == self.sides[2]
    }

    pub fn is_isosceles(&self) -> bool {
        self.sides[0] == self.sides[1] || self.sides[1] == self.sides[2]
    }

    pub fn is_scalene(&self) -> bool {
        !self.is_equilateral() && !self.is_isosceles()
    }
}

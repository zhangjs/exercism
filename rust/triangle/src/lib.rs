use std::ops::Add;

pub struct Triangle<T> {
    sorted_sides: [T; 3],
}

impl<T: PartialOrd + Copy + From<i32> + Add<Output = T>> Triangle<T> {
    pub fn build(sides: [T; 3]) -> Option<Triangle<T>> {
        let mut sorted_sides = sides;

        sorted_sides.sort_by(|a, b| a.partial_cmp(b).unwrap());

        if sorted_sides[0] > T::from(0) && sorted_sides[2] <= sorted_sides[0] + sorted_sides[1] {
            Some(Triangle { sorted_sides })
        } else {
            None
        }
    }

    pub fn is_equilateral(&self) -> bool {
        self.sorted_sides[0] == self.sorted_sides[2]
    }

    pub fn is_isosceles(&self) -> bool {
        self.sorted_sides[0] == self.sorted_sides[1] || self.sorted_sides[1] == self.sorted_sides[2]
    }

    pub fn is_scalene(&self) -> bool {
        self.sorted_sides[0] != self.sorted_sides[1] && self.sorted_sides[1] != self.sorted_sides[2]
    }
}

use std::collections::HashSet;

pub struct Triangle {
    sides: HashSet<u64>,
}

impl Triangle {
    pub fn build(mut sides: [u64; 3]) -> Option<Triangle> {
        let set: HashSet<u64> = sides.iter().cloned().collect();
        sides.sort();

        if set.contains(&0) || (sides[0] + sides[1]) < sides[2] {
            return None;
        }

        Some(Triangle { sides: set })
    }

    pub fn is_equilateral(&self) -> bool {
        self.sides.len() == 1
    }

    pub fn is_isosceles(&self) -> bool {
        self.sides.len() == 2
    }
    pub fn is_scalene(&self) -> bool {
        self.sides.len() == 3
    }
}

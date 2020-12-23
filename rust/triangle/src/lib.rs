pub struct Triangle {
    sides: [u64; 3]
}

impl Triangle {
    pub fn build(sides: [u64; 3]) -> Option<Triangle> {
        if sides.iter().any(|x| *x <= 0) {
            return None
        }

        for i in 0..3 {
            if sides[i] + sides[(i+1)%3] < sides[(i+2)%3] {
                return None
            }
        }

        Some(Triangle{sides})
    }

    pub fn is_equilateral(&self) -> bool {
        self.sides[0] == self.sides[1] && self.sides[1] == self.sides[2] && self.sides[2] == self.sides[0]
    }

    pub fn is_scalene(&self) -> bool {
        self.sides[0] != self.sides[1] && self.sides[1] != self.sides[2] && self.sides[2] != self.sides[0]
    }

    pub fn is_isosceles(&self) -> bool {
        !self.is_scalene()
    }
}

use std::ops::{Add, Sub};

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq)]
pub struct Dim {
    s: i64,
    m: i64,
    kg: i64,
    a: i64,
    k: i64,
    mol: i64,
    cd: i64,
}

impl Dim {
    pub const fn new(s: i64, m: i64, kg: i64, a: i64, k: i64, mol: i64, cd: i64) -> Dim {
        Dim {
            s,
            m,
            kg,
            a,
            k,
            mol,
            cd,
        }
    }
}

impl const Add for Dim {
    type Output = Dim;
    fn add(self, rhs: Self) -> Self::Output {
        Dim {
            s: self.s + rhs.s,
            m: self.m + rhs.m,
            kg: self.kg + rhs.kg,
            a: self.a + rhs.a,
            k: self.k + rhs.k,
            mol: self.mol + rhs.mol,
            cd: self.cd + rhs.cd,
        }
    }
}

impl const Sub for Dim {
    type Output = Dim;
    fn sub(self, rhs: Self) -> Self::Output {
        Dim {
            s: self.s - rhs.s,
            m: self.m - rhs.m,
            kg: self.kg - rhs.kg,
            a: self.a - rhs.a,
            k: self.k - rhs.k,
            mol: self.mol - rhs.mol,
            cd: self.cd - rhs.cd,
        }
    }
}

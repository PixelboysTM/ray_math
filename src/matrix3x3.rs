use std::ops::Mul;

use crate::{equal, matrix2x2::Matrix2x2, Num};

#[derive(Clone, Copy)]
pub struct Matrix3x3 {
    m: [Num; 3 * 3],
}

impl Matrix3x3 {
    pub fn new<T>(data: [T; 3 * 3]) -> Matrix3x3
    where
        T: Into<Num>,
    {
        Matrix3x3 {
            m: data.map(|f| f.into()),
        }
    }
    pub fn identity() -> Matrix3x3 {
        Matrix3x3::new([1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0])
    }
}

impl Matrix3x3 {
    pub fn at(&self, row: usize, col: usize) -> Num {
        self.m[self.index(row, col)]
    }
    pub fn set(&mut self, row: usize, col: usize, val: Num) {
        self.m[self.index(row, col)] = val;
    }

    pub fn transpose(&self) -> Self {
        Matrix3x3::new([
            self.at(0, 0),
            self.at(1, 0),
            self.at(2, 0),
            self.at(0, 1),
            self.at(1, 1),
            self.at(2, 1),
            self.at(0, 2),
            self.at(1, 2),
            self.at(2, 2),
        ])
    }

    pub fn submatrix(&self, row: usize, col: usize) -> Matrix2x2 {
        let mut m = Matrix2x2::new([0.0; 4]);

        let mut ri = 0;
        for r in 0..3 {
            if row == r {
                continue;
            }
            let mut ci = 0;
            for c in 0..3 {
                if col == c {
                    continue;
                }
                m.set(ri, ci, self.at(r, c));

                ci += 1;
            }
            ri += 1;
        }

        m
    }

    pub fn minor(&self, row: usize, col: usize) -> Num {
        self.submatrix(row, col).determinant()
    }

    pub fn cofactor(&self, row: usize, col: usize) -> Num {
        let m = self.minor(row, col);
        if (row + col) % 2 == 0 {
            m
        } else {
            -m
        }
    }

    pub fn determinant(&self) -> Num {
        self.at(0, 0) * self.cofactor(0, 0)
            + self.at(0, 1) * self.cofactor(0, 1)
            + self.at(0, 2) * self.cofactor(0, 2)
    }

    pub fn invertible(&self) -> bool {
        !equal(self.determinant(), 0.0)
    }

    fn index(&self, row: usize, col: usize) -> usize {
        row * 3 + col
    }
}

impl PartialEq<Matrix3x3> for Matrix3x3 {
    fn eq(&self, other: &Matrix3x3) -> bool {
        for i in 0..3 * 3 {
            if !equal(self.m[i], other.m[i]) {
                return false;
            }
        }

        true
    }
}

impl Mul<Matrix3x3> for Matrix3x3 {
    type Output = Matrix3x3;

    fn mul(self, rhs: Matrix3x3) -> Self::Output {
        let mut m = Matrix3x3::new([0.0; 3 * 3]);

        for row in 0..3 {
            for col in 0..3 {
                m.set(
                    row,
                    col,
                    self.at(row, 0) * rhs.at(0, col)
                        + self.at(row, 1) * rhs.at(1, col)
                        + self.at(row, 2) * rhs.at(2, col)
                        + self.at(row, 3) * rhs.at(3, col),
                );
            }
        }

        m
    }
}

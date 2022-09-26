use std::ops::Mul;

use crate::{equal, Num};

#[derive(Clone, Copy)]
pub struct Matrix2x2 {
    m: [Num; 2 * 2],
}

impl Matrix2x2 {
    pub fn new<T>(data: [T; 2 * 2]) -> Matrix2x2
    where
        T: Into<Num>,
    {
        Matrix2x2 {
            m: data.map(|f| f.into()),
        }
    }
    pub fn identity() -> Matrix2x2 {
        Matrix2x2::new([1.0, 0.0, 0.0, 1.0])
    }
}

impl Matrix2x2 {
    pub fn at(&self, row: usize, col: usize) -> Num {
        self.m[self.index(row, col)]
    }
    pub fn set(&mut self, row: usize, col: usize, val: Num) {
        self.m[self.index(row, col)] = val;
    }

    pub fn transpose(&self) -> Self {
        Matrix2x2::new([self.at(0, 0), self.at(1, 0), self.at(0, 1), self.at(1, 1)])
    }

    pub fn determinant(&self) -> Num {
        self.at(0, 0) * self.at(1, 1) - self.at(0, 1) * self.at(1, 0)
    }

    pub fn invertible(&self) -> bool {
        !equal(self.determinant(), 0.0)
    }

    fn index(&self, row: usize, col: usize) -> usize {
        row * 2 + col
    }
}

impl PartialEq<Matrix2x2> for Matrix2x2 {
    fn eq(&self, other: &Matrix2x2) -> bool {
        for i in 0..2 * 2 {
            if !equal(self.m[i], other.m[i]) {
                return false;
            }
        }

        true
    }
}

impl Mul<Matrix2x2> for Matrix2x2 {
    type Output = Matrix2x2;

    fn mul(self, rhs: Matrix2x2) -> Self::Output {
        let mut m = Matrix2x2::new([0.0; 2 * 2]);

        for row in 0..2 {
            for col in 0..2 {
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

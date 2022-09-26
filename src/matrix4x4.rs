use std::ops::Mul;

use crate::{equal, matrix3x3::Matrix3x3, tuple::Tuple, Num};

#[derive(Clone, Copy)]
pub struct Matrix4x4 {
    m: [Num; 4 * 4],
}

impl Matrix4x4 {
    pub fn new<T>(data: [T; 4 * 4]) -> Matrix4x4
    where
        T: Into<Num>,
    {
        Matrix4x4 {
            m: data.map(|f| f.into()),
        }
    }
    pub fn identity() -> Matrix4x4 {
        Matrix4x4::new([
            1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
        ])
    }
}

impl Matrix4x4 {
    pub fn at(&self, row: usize, col: usize) -> Num {
        self.m[self.index(row, col)]
    }
    pub fn set<T>(&mut self, row: usize, col: usize, val: T)
    where
        T: Into<Num>,
    {
        self.m[self.index(row, col)] = val.into();
    }

    pub fn transpose(&self) -> Self {
        Matrix4x4::new([
            self.at(0, 0),
            self.at(1, 0),
            self.at(2, 0),
            self.at(3, 0),
            self.at(0, 1),
            self.at(1, 1),
            self.at(2, 1),
            self.at(3, 1),
            self.at(0, 2),
            self.at(1, 2),
            self.at(2, 2),
            self.at(3, 2),
            self.at(0, 3),
            self.at(1, 3),
            self.at(2, 3),
            self.at(3, 3),
        ])
    }

    pub fn submatrix(&self, row: usize, col: usize) -> Matrix3x3 {
        let mut m = Matrix3x3::new([0.0; 9]);

        let mut ri = 0;
        for r in 0..4 {
            if row == r {
                continue;
            }
            let mut ci = 0;
            for c in 0..4 {
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

    pub fn invertible(&self) -> bool {
        !equal(self.determinant(), 0.0)
    }

    pub fn determinant(&self) -> Num {
        self.at(0, 0) * self.cofactor(0, 0)
            + self.at(0, 1) * self.cofactor(0, 1)
            + self.at(0, 2) * self.cofactor(0, 2)
            + self.at(0, 3) * self.cofactor(0, 3)
    }

    pub fn inverse(&self) -> Result<Matrix4x4, String> {
        if !self.invertible() {
            Err("Matrix not invertible.".to_string())
        } else {
            let mut m2 = Matrix4x4::identity();
            let det = self.determinant();
            for row in 0..4 {
                for col in 0..4 {
                    let c = self.cofactor(row, col);
                    m2.set(col, row, c / det);
                }
            }

            Ok(m2)
        }
    }

    fn index(&self, row: usize, col: usize) -> usize {
        row * 4 + col
    }
}

impl PartialEq<Matrix4x4> for Matrix4x4 {
    fn eq(&self, other: &Matrix4x4) -> bool {
        for i in 0..4 * 4 {
            if !equal(self.m[i], other.m[i]) {
                return false;
            }
        }

        true
    }
}

impl Mul<Matrix4x4> for Matrix4x4 {
    type Output = Matrix4x4;

    fn mul(self, rhs: Matrix4x4) -> Self::Output {
        let mut m = Matrix4x4::new([0.0; 4 * 4]);

        for row in 0..4 {
            for col in 0..4 {
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

impl Mul<Tuple> for Matrix4x4 {
    type Output = Tuple;

    fn mul(self, rhs: Tuple) -> Self::Output {
        Tuple::new(
            self.at(0, 0) * rhs.get_x()
                + self.at(0, 1) * rhs.get_y()
                + self.at(0, 2) * rhs.get_z()
                + self.at(0, 3) * rhs.get_w(),
            self.at(1, 0) * rhs.get_x()
                + self.at(1, 1) * rhs.get_y()
                + self.at(1, 2) * rhs.get_z()
                + self.at(1, 3) * rhs.get_w(),
            self.at(2, 0) * rhs.get_x()
                + self.at(2, 1) * rhs.get_y()
                + self.at(2, 2) * rhs.get_z()
                + self.at(2, 3) * rhs.get_w(),
            self.at(3, 0) * rhs.get_x()
                + self.at(3, 1) * rhs.get_y()
                + self.at(3, 2) * rhs.get_z()
                + self.at(3, 3) * rhs.get_w(),
        )
    }
}

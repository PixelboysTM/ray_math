use std::{ops::{Add, Div, Mul, Neg, Sub}, fmt::Display};

use crate::{equal, Num};

#[derive(Clone, Copy)]
pub struct Tuple {
    x: Num,
    y: Num,
    z: Num,
    w: Num,
}

impl Tuple {
    pub fn new<T1, T2, T3, T4>(x: T1, y: T2, z: T3, w: T4) -> Tuple
    where
        T1: Into<Num>,
        T2: Into<Num>,
        T3: Into<Num>,
        T4: Into<Num>,
    {
        Tuple {
            x: x.into(),
            y: y.into(),
            z: z.into(),
            w: w.into(),
        }
    }
    pub fn point<T1, T2, T3>(x: T1, y: T2, z: T3) -> Tuple
    where
        T1: Into<Num>,
        T2: Into<Num>,
        T3: Into<Num>,
    {
        Self::new(x, y, z, 1.0)
    }
    pub fn vector<T1, T2, T3>(x: T1, y: T2, z: T3) -> Tuple
    where
        T1: Into<Num>,
        T2: Into<Num>,
        T3: Into<Num>,
    {
        Self::new(x, y, z, 0.0)
    }
}

impl Tuple {
    pub fn get_x(&self) -> Num {
        self.x
    }
    pub fn get_y(&self) -> Num {
        self.y
    }
    pub fn get_z(&self) -> Num {
        self.z
    }
    pub fn get_w(&self) -> Num {
        self.w
    }
    pub fn set_x<T>(&mut self, v: T)
    where
        T: Into<Num>,
    {
        self.x = v.into();
    }
    pub fn set_y<T>(&mut self, v: T)
    where
        T: Into<Num>,
    {
        self.y = v.into();
    }
    pub fn set_z<T>(&mut self, v: T)
    where
        T: Into<Num>,
    {
        self.z = v.into();
    }
    pub fn set_w<T>(&mut self, v: T)
    where
        T: Into<Num>,
    {
        self.w = v.into();
    }
    pub fn is_point(&self) -> bool {
        equal(self.w, 1.0)
    }
    pub fn is_vector(&self) -> bool {
        equal(self.w, 0.0)
    }
    pub fn magnitude(&self) -> Num {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2) + self.w.powi(2)).sqrt()
    }
    pub fn normalize(&self) -> Tuple {
        let mag = self.magnitude();
        Tuple::new(self.x / mag, self.y / mag, self.z / mag, self.w / mag)
    }
    pub fn dot(&self, b: &Tuple) -> Num {
        self.x * b.x + self.y * b.y + self.z * b.z + self.w * b.w
    }
    pub fn cross(&self, b: &Tuple) -> Tuple {
        Tuple::vector(
            self.y * b.z - self.z * b.y,
            self.z * b.x - self.x * b.z,
            self.x * b.y - self.y * b.x,
        )
    }
    pub fn reflect(&self, normal: Tuple) -> Tuple {
        *self - normal * 2.0 * self.dot(&normal)
    }
}

impl Add<Tuple> for Tuple {
    type Output = Tuple;

    fn add(self, rhs: Self) -> Self::Output {
        Tuple::new(
            self.x + rhs.x,
            self.y + rhs.y,
            self.z + rhs.z,
            self.w + rhs.w,
        )
    }
}

impl Sub<Tuple> for Tuple {
    type Output = Tuple;

    fn sub(self, rhs: Tuple) -> Self::Output {
        Tuple::new(
            self.x - rhs.x,
            self.y - rhs.y,
            self.z - rhs.z,
            self.w - rhs.w,
        )
    }
}

impl Neg for Tuple {
    type Output = Tuple;

    fn neg(self) -> Self::Output {
        Tuple::new(-self.x, -self.y, -self.z, -self.w)
    }
}

impl Mul<Num> for Tuple {
    type Output = Tuple;

    fn mul(self, rhs: Num) -> Self::Output {
        Tuple::new(self.x * rhs, self.y * rhs, self.z * rhs, self.w * rhs)
    }
}

impl Div<Num> for Tuple {
    type Output = Tuple;

    fn div(self, rhs: Num) -> Self::Output {
        Tuple::new(self.x / rhs, self.y / rhs, self.z / rhs, self.w / rhs)
    }
}

impl PartialEq<Tuple> for Tuple {
    fn eq(&self, other: &Tuple) -> bool {
        equal(self.x, other.x)
            && equal(self.y, other.y)
            && equal(self.z, other.z)
            && equal(self.w, other.w)
    }
}


impl Display for Tuple {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_point() {
            f.write_str(format!("p({}/{}/{})", self.x, self.y, self.z).as_str())
        } else if self.is_vector() {
            f.write_str(format!("v({}/{}/{})", self.x, self.y, self.z).as_str())
        } else {
            f.write_str(format!("t({}/{}/{}/{})", self.x, self.y, self.z, self.w).as_str())
        }
    }
}
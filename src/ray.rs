use crate::{matrix4x4::Matrix4x4, tuple::Tuple, Num};

#[derive(Clone, Copy)]
pub struct Ray {
    origin: Tuple,
    direction: Tuple,
}

impl Ray {
    pub fn new(origin: Tuple, direction: Tuple) -> Ray {
        Ray { origin, direction }
    }
}

impl Ray {
    pub fn origin(&self) -> Tuple {
        self.origin
    }
    pub fn direction(&self) -> Tuple {
        self.direction
    }
    pub fn position<T>(&self, t: T) -> Tuple
    where
        T: Into<Num>,
    {
        self.origin + self.direction * t.into()
    }
    pub fn transform(&self, m: Matrix4x4) -> Ray {
        Ray::new(m * self.origin(), m * self.direction())
    }
}

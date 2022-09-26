use crate::{Pixel, Tuple, equal, Pattern, Matrix4x4};

#[derive(Clone, Copy)]
pub struct CheckersPattern{
    color_a: Pixel,
    color_b: Pixel,
    transform: Matrix4x4,
}

impl CheckersPattern {
    pub(super) fn at(&self, point: Tuple) -> Pixel{
        if equal((point.get_x().floor() + point.get_y().floor() + point.get_z().floor()) % 2.0, 0.0) {
            self.color_a
        } else {
            self.color_b
        }
    }
    
    pub(super) fn transform(&self) -> Matrix4x4{
        self.transform
    }
    pub(super) fn set_transform(&mut self, transform: Matrix4x4){
        self.transform = transform;
    }
}

impl Pattern {
    pub fn checkers(color_a: Pixel, color_b: Pixel) -> Pattern{
        Pattern::Checkers(CheckersPattern { color_a, color_b, transform: Matrix4x4::identity() })
    }
}

impl PartialEq<CheckersPattern> for CheckersPattern{
    fn eq(&self, other: &CheckersPattern) -> bool {
        self.color_a == other.color_a && self.color_b == other.color_b && self.transform == other.transform
    }
}
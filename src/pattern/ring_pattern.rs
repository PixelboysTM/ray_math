use crate::{Pixel, Tuple, equal, Pattern, Matrix4x4};

#[derive(Clone, Copy)]
pub struct RingPattern{
    color_a: Pixel,
    color_b: Pixel,
    transform: Matrix4x4,
}

impl RingPattern {
    pub(super) fn at(&self, point: Tuple) -> Pixel{
        if equal((point.get_x()*point.get_x() + point.get_z()*point.get_z()).sqrt().floor() % 2.0, 0.0) {
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
    pub fn ring(color_a: Pixel, color_b: Pixel) -> Pattern{
        Pattern::Ring(RingPattern { color_a, color_b, transform: Matrix4x4::identity() })
    }
}

impl PartialEq<RingPattern> for RingPattern{
    fn eq(&self, other: &RingPattern) -> bool {
        self.color_a == other.color_a && self.color_b == other.color_b && self.transform == other.transform
    }
}
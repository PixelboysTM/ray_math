use crate::{Pixel, Tuple, equal, Pattern, Matrix4x4};

#[derive(Clone, Copy)]
pub struct StripePattern{
    color_a: Pixel,
    color_b: Pixel,
    transform: Matrix4x4,
}

impl StripePattern {
    pub(super) fn at(&self, point: Tuple) -> Pixel{
        if equal(point.get_x().floor() % 2.0, 0.0) {
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
    pub fn stripe(color_a: Pixel, color_b: Pixel) -> Pattern{
        Pattern::Stripe(StripePattern { color_a, color_b, transform: Matrix4x4::identity() })
    }
}

impl PartialEq<StripePattern> for StripePattern{
    fn eq(&self, other: &StripePattern) -> bool {
        self.color_a == other.color_a && self.color_b == other.color_b && self.transform == other.transform
    }
}
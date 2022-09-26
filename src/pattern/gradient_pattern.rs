use crate::{Pixel, Tuple, Pattern, Matrix4x4};

#[derive(Clone, Copy)]
pub struct GradientPattern{
    color_a: Pixel,
    color_b: Pixel,
    transform: Matrix4x4,
}

impl GradientPattern {
    pub(super) fn at(&self, point: Tuple) -> Pixel{
        let distance = self.color_b.to_tuple() - self.color_a.to_tuple();
        let fraction = point.get_x() - point.get_x().floor();
        let a = distance * fraction;

        self.color_a + Pixel::rgb(a.get_x(), a.get_y(), a.get_z())
    }
    
    pub(super) fn transform(&self) -> Matrix4x4{
        self.transform
    }
    pub(super) fn set_transform(&mut self, transform: Matrix4x4){
        self.transform = transform;
    }
}

impl Pattern {
    pub fn gradient(color_a: Pixel, color_b: Pixel) -> Pattern{
        Pattern::Gradient(GradientPattern { color_a, color_b, transform: Matrix4x4::identity() })
    }
}

impl PartialEq<GradientPattern> for GradientPattern{
    fn eq(&self, other: &GradientPattern) -> bool {
        self.color_a == other.color_a && self.color_b == other.color_b && self.transform == other.transform
    }
}
use crate::{Pixel, Tuple, Pattern, Matrix4x4};

#[derive(Clone, Copy)]
pub struct TestPattern{
    transform: Matrix4x4,
}

impl TestPattern {
    pub(super) fn at(&self, point: Tuple) -> Pixel{
        Pixel::rgb(point.get_x(), point.get_y(), point.get_z())
    }
    
    pub(super) fn transform(&self) -> Matrix4x4{
        self.transform
    }
    pub(super) fn set_transform(&mut self, transform: Matrix4x4){
        self.transform = transform;
    }
}

impl Pattern {
    pub(crate) fn test() -> Pattern{
        Pattern::Test(TestPattern { transform: Matrix4x4::identity() })
    }
}

impl PartialEq<TestPattern> for TestPattern{
    fn eq(&self, other: &TestPattern) -> bool {
        self.transform == other.transform
    }
}
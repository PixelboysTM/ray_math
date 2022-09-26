use crate::{Pixel, Tuple, Matrix4x4, Shape};

mod stripe_pattern;
mod gradient_pattern;
mod ring_pattern;
mod checkers_pattern;
mod test_pattern;
use self::{stripe_pattern::StripePattern, gradient_pattern::GradientPattern, ring_pattern::RingPattern, checkers_pattern::CheckersPattern, test_pattern::TestPattern};


#[derive(Clone, Copy)]
pub enum Pattern{
    Stripe(StripePattern),
    Gradient(GradientPattern),
    Ring(RingPattern),
    Checkers(CheckersPattern),
    Test(TestPattern),
}

impl Pattern {
    pub(crate) fn at(&self, point: Tuple) -> Pixel{
        match self {
            Self::Stripe(item) => item.at(point),
            Self::Gradient(item) => item.at(point),
            Self::Ring(item) => item.at(point),
            Self::Checkers(item) => item.at(point),
            Self::Test(item) => item.at(point),
        }
    }
    pub fn at_object(&self, object: &Shape, world_point: Tuple) -> Pixel{
        let object_point = object.transform().inverse().unwrap() * world_point;
        let pattern_point = self.transform().inverse().unwrap() * object_point;

        self.at(pattern_point)
    }

    pub fn transform(&self) -> Matrix4x4{
        match self {
            Self::Stripe(item) => item.transform(),
            Self::Gradient(item) => item.transform(),
            Self::Ring(item) => item.transform(),
            Self::Checkers(item) => item.transform(),
            Self::Test(item) => item.transform(),
        }
    }
    pub fn set_transform(&mut self, transform: Matrix4x4){
        match self {
            Self::Stripe(item) => item.set_transform(transform),
            Self::Gradient(item) => item.set_transform(transform),
            Self::Ring(item) => item.set_transform(transform),
            Self::Checkers(item) => item.set_transform(transform),
            Self::Test(item) => item.set_transform(transform),
        }
    }
}

impl PartialEq<Pattern> for Pattern{
    fn eq(&self, other: &Pattern) -> bool {
        match (self, other) {
            (Self::Stripe(l0), Self::Stripe(r0)) => l0 == r0,
            (Self::Gradient(l0), Self::Gradient(r0)) => l0 == r0,
            (Self::Ring(l0), Self::Ring(r0)) => l0 == r0,
            (Self::Checkers(l0), Self::Checkers(r0)) => l0 == r0,
            (Self::Test(l0), Self::Test(r0)) => l0 == r0,

            _ => false,
        }
    }
}
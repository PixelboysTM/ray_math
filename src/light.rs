use crate::{img::Pixel, tuple::Tuple};

#[derive(Clone, Copy)]
pub enum Light {
    Point(PointLight),
}

impl Light {
    pub fn position(&self) -> Tuple {
        match self {
            Light::Point(item) => item.position(),
        }
    }
    pub fn intensity(&self) -> Pixel {
        match self {
            Light::Point(item) => item.intensity(),
        }
    }

    pub fn point(position: Tuple, intensity: Pixel) -> Self {
        Light::Point(PointLight::new(intensity, position))
    }
}

impl PartialEq<Light> for Light {
    fn eq(&self, other: &Light) -> bool {
        match (self, other) {
            (Self::Point(l0), Self::Point(r0)) => l0 == r0,
        }
    }
}

//POINTLIGHT

#[derive(Clone, Copy)]
pub struct PointLight {
    intensity: Pixel,
    position: Tuple,
}

impl PointLight {
    fn new(intensity: Pixel, position: Tuple) -> PointLight {
        PointLight {
            intensity,
            position,
        }
    }
}

impl PointLight {
    fn position(&self) -> Tuple {
        self.position
    }

    fn intensity(&self) -> Pixel {
        self.intensity
    }
}

impl PartialEq<PointLight> for PointLight {
    fn eq(&self, other: &PointLight) -> bool {
        self.intensity == other.intensity && self.position == other.position
    }
}

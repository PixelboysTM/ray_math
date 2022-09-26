mod camera;
mod intersection;
mod light;
mod material;
mod matrix2x2;
mod matrix3x3;
mod matrix4x4;
mod ray;
mod shape;
mod transformation;
mod tuple;
mod world;
mod pattern;

pub use crate::{
    camera::Camera, img::*, intersection::*, light::Light, material::Material,
    matrix2x2::Matrix2x2, matrix3x3::Matrix3x3, matrix4x4::Matrix4x4, ray::Ray, shape::Shape,
    transformation::TransformationBuilder, tuple::Tuple, world::World, pattern::Pattern,
};

#[cfg(test)]
mod tests;

const EPSILON: Num = 0.0001;
pub const PI: Num =
    3.141592653589793238462643383279502884197169399375105820974944592307816406286208998628034825342;
pub type Num = f64;

fn equal(a: Num, b: Num) -> bool {
    (a - b).abs() < EPSILON
}

pub fn rad_to_deg<T>(rad: T) -> Num
where
    T: Into<Num>,
{
    rad.into() * 180.0 / PI
}
pub fn deg_to_rad<T>(deg: T) -> Num
where
    T: Into<Num>,
{
    deg.into() * PI / 180.0
}

mod img {
    use std::ops::{Add, Mul};

    use crate::{equal, tuple::Tuple, Num};

    #[derive(Clone)]
    pub struct Canvas {
        width: u32,
        height: u32,
        data: Vec<Pixel>,
    }

    impl Canvas {
        pub fn with_dimesnions(width: u32, height: u32) -> Canvas {
            let mut c = Canvas {
                width,
                height,
                data: Vec::new(),
            };
            for _ in 0..(width * height) {
                c.data.push(Pixel::black());
            }
            c
        }
    }

    impl Canvas {
        pub fn fill(&mut self, color: Pixel) {
            for pixel in self.data.iter_mut() {
                *pixel = color;
            }
        }
        pub fn get(&self, x: u32, y: u32) -> Pixel {
            let index = self.index(x, y);
            self.data[index]
        }
        pub fn set(&mut self, x: u32, y: u32, color: Pixel) {
            let index = self.index(x, y);
            self.data[index] = color;
        }

        pub fn width(&self) -> u32 {
            self.width
        }
        pub fn height(&self) -> u32 {
            self.height
        }

        fn index(&self, x: u32, y: u32) -> usize {
            (x * self.height() + y) as usize
        }
    }

    #[derive(Debug, Clone, Copy)]
    pub struct Pixel {
        r: Num,
        g: Num,
        b: Num,
    }

    impl Pixel {
        pub fn red() -> Pixel {
            Pixel {
                r: 1.0,
                g: 0.0,
                b: 0.0,
            }
        }
        pub fn green() -> Pixel {
            Pixel {
                r: 0.0,
                g: 1.0,
                b: 0.0,
            }
        }
        pub fn blue() -> Pixel {
            Pixel {
                r: 0.0,
                g: 0.0,
                b: 1.0,
            }
        }
        pub fn black() -> Pixel {
            Pixel {
                r: 0.0,
                g: 0.0,
                b: 0.0,
            }
        }
        pub fn white() -> Pixel {
            Pixel {
                r: 1.0,
                g: 1.0,
                b: 1.0,
            }
        }

        pub fn rgb(r: Num, g: Num, b: Num) -> Pixel {
            Pixel { r, g, b }
        }

        pub fn to_tuple(&self) -> Tuple {
            Tuple::new(self.r, self.g, self.b, -1.0)
        }
    }

    impl Pixel {
        pub fn as_array_u8(&self) -> [u8; 3] {
            let r: u8 = (self.r * 255.0) as u8;
            let g: u8 = (self.g * 255.0) as u8;
            let b: u8 = (self.b * 255.0) as u8;
            [r, g, b]
        }
    }

    impl Mul<Pixel> for Pixel {
        type Output = Pixel;

        fn mul(self, rhs: Pixel) -> Self::Output {
            Pixel::rgb(self.r * rhs.r, self.g * rhs.g, self.b * rhs.b)
        }
    }

    impl Mul<Num> for Pixel {
        type Output = Pixel;

        fn mul(self, rhs: Num) -> Self::Output {
            Pixel::rgb(self.r * rhs, self.g * rhs, self.b * rhs)
        }
    }

    impl Add<Pixel> for Pixel {
        type Output = Self;

        fn add(self, rhs: Pixel) -> Self::Output {
            Pixel::rgb(self.r + rhs.r, self.g + rhs.g, self.b + rhs.b)
        }
    }

    impl PartialEq<Pixel> for Pixel {
        fn eq(&self, other: &Pixel) -> bool {
            equal(self.r, other.r) && equal(self.g, other.g) && equal(self.b, other.b)
        }
    }
}

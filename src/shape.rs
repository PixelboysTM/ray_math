use crate::{
    intersection::Intersection, material::Material, matrix4x4::Matrix4x4, ray::Ray, tuple::Tuple,
};

mod sphere;
pub use sphere::Sphere;
mod test_shape;
pub(crate) use test_shape::{TestShape, SAVED_RAY};
mod plane;
pub use plane::Plane;

#[derive(Clone, Copy)]
pub enum Shape {
    Sphere(Sphere),
    Plane(Plane),
    Test(TestShape),
}

impl Shape {
    pub fn intersect(&self, ray: Ray) -> Vec<Intersection> {
        let local_ray = ray.transform(self.transform().inverse().unwrap());
        match self {
            Shape::Sphere(item) => item.intersect(local_ray, *self),
            Shape::Test(item) => item.intersect(local_ray, *self),
            Shape::Plane(item) => item.intersect(local_ray, *self),
        }
    }
    pub fn set_transform(&mut self, transform: Matrix4x4) {
        match self {
            Shape::Sphere(item) => item.set_transform(transform),
            Shape::Test(item) => item.set_transform(transform),
            Shape::Plane(item) => item.set_transform(transform),
        }
    }
    pub fn transform(&self) -> Matrix4x4 {
        match self {
            Shape::Sphere(item) => item.transform(),
            Shape::Test(item) => item.transform(),
            Shape::Plane(item) => item.transform(),
        }
    }
    pub fn normal_at(&self, point: Tuple) -> Tuple {
        let inv = self.transform().inverse().unwrap();

        let local_point = inv * point;
        let local_normal = match self {
            Shape::Sphere(item) => item.normal_at(local_point),
            Shape::Test(item) => item.normal_at(local_point),
            Shape::Plane(item) => item.normal_at(local_point),
        };

        let mut world_normal = inv.transpose() * local_normal;
        world_normal.set_w(0);

        world_normal.normalize()
    }
    pub fn material(&self) -> Material {
        match self {
            Shape::Sphere(item) => item.material(),
            Shape::Test(item) => item.material(),
            Shape::Plane(item) => item.material(),
        }
    }
    pub fn set_material(&mut self, material: Material) {
        match self {
            Shape::Sphere(item) => item.set_material(material),
            Shape::Test(item) => item.set_material(material),
            Shape::Plane(item) => item.set_material(material),
        }
    }

    pub fn sphere() -> Shape {
        Shape::Sphere(Sphere::new())
    }
    pub fn test_shape() -> Shape {
        Shape::Test(TestShape::new())
    }
    pub fn plane() -> Shape {
        Shape::Plane(Plane::new())
    }
}

impl PartialEq for Shape {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Sphere(l0), Self::Sphere(r0)) => l0 == r0,
            (Self::Test(l0), Self::Test(r0)) => l0 == r0,
            (Self::Plane(l0), Self::Plane(r0)) => l0 == r0,
            _ => false,
        }
    }
}

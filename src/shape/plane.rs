use crate::{Intersection, Material, Matrix4x4, Ray, Shape, Tuple, EPSILON};

#[derive(Clone, Copy)]
pub struct Plane {
    matrix: Matrix4x4,
    material: Material,
}

impl Plane {
    pub fn new() -> Plane {
        Self {
            matrix: Matrix4x4::identity(),
            material: Material::default(),
        }
    }

    pub(super) fn intersect(&self, ray: Ray, shape: Shape) -> Vec<Intersection> {
        if ray.direction().get_y().abs() < EPSILON {
            return vec![];
        }
        let t = -ray.origin().get_y() / ray.direction().get_y();
        vec![Intersection::new(t, shape)]
    }

    pub(super) fn set_transform(&mut self, m: Matrix4x4) {
        self.matrix = m;
    }
    pub(super) fn transform(&self) -> Matrix4x4 {
        self.matrix
    }
    pub(super) fn normal_at(&self, _object_point: Tuple) -> Tuple {
        Tuple::vector(0, 1, 0)
    }
    pub(super) fn material(&self) -> Material {
        self.material
    }
    pub(super) fn set_material(&mut self, material: Material) {
        self.material = material;
    }
}

impl PartialEq for Plane {
    fn eq(&self, other: &Self) -> bool {
        self.transform() == other.transform() && self.material() == other.material()
    }
}

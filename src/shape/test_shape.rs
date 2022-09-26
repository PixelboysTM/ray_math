use crate::{Intersection, Material, Matrix4x4, Ray, Shape, Tuple};

#[derive(Clone, Copy)]
//Spehere
pub struct TestShape {
    matrix: Matrix4x4,
    material: Material,
}

pub(crate) static mut SAVED_RAY: Option<Ray> = None;

impl TestShape {
    pub fn new() -> TestShape {
        Self {
            matrix: Matrix4x4::identity(),
            material: Material::default(),
        }
    }

    pub(super) fn intersect(&self, ray: Ray, _shape: Shape) -> Vec<Intersection> {
        unsafe {
            SAVED_RAY = Some(ray);
        }

        vec![]
    }

    pub(super) fn set_transform(&mut self, m: Matrix4x4) {
        self.matrix = m;
    }
    pub(super) fn transform(&self) -> Matrix4x4 {
        self.matrix
    }
    pub(super) fn normal_at(&self, object_point: Tuple) -> Tuple {
        Tuple::vector(
            object_point.get_x(),
            object_point.get_y(),
            object_point.get_z(),
        )
    }
    pub(super) fn material(&self) -> Material {
        self.material
    }
    pub(super) fn set_material(&mut self, material: Material) {
        self.material = material;
    }
}

impl PartialEq for TestShape {
    fn eq(&self, other: &Self) -> bool {
        self.transform() == other.transform() && self.material() == other.material()
    }
}

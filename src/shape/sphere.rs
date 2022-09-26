use crate::{Intersection, Material, Matrix4x4, Ray, Shape, Tuple};

#[derive(Clone, Copy)]
//Spehere
pub struct Sphere {
    matrix: Matrix4x4,
    material: Material,
}

impl Sphere {
    pub fn new() -> Sphere {
        Self {
            matrix: Matrix4x4::identity(),
            material: Material::default(),
        }
    }

    pub(super) fn intersect(&self, ray: Ray, shape: Shape) -> Vec<Intersection> {
        let sphere_to_ray = ray.origin() - Tuple::point(0, 0, 0);
        let dir = ray.direction();

        let a = dir.dot(&dir);
        let b = 2.0 * dir.dot(&sphere_to_ray);
        let c = sphere_to_ray.dot(&sphere_to_ray) - 1.0;

        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            return Vec::new();
        }

        let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
        let t2 = (-b + discriminant.sqrt()) / (2.0 * a);

        let mut v = Intersection::make(Intersection::new(t1, shape), Intersection::new(t2, shape));
        v.sort_by(|a, b| a.t().partial_cmp(&b.t()).unwrap());
        v
    }

    pub(super) fn set_transform(&mut self, m: Matrix4x4) {
        self.matrix = m;
    }
    pub(super) fn transform(&self) -> Matrix4x4 {
        self.matrix
    }
    pub(super) fn normal_at(&self, object_point: Tuple) -> Tuple {
        object_point - Tuple::point(0, 0, 0)
    }
    pub(super) fn material(&self) -> Material {
        self.material
    }
    pub(super) fn set_material(&mut self, material: Material) {
        self.material = material;
    }
}

impl PartialEq for Sphere {
    fn eq(&self, other: &Self) -> bool {
        self.transform() == other.transform() && self.material() == other.material()
    }
}

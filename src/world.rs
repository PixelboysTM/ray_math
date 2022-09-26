use crate::{
    hit,
    img::Pixel,
    intersection::{self, Computations, Intersection},
    light::Light,
    material::Material,
    matrix4x4::Matrix4x4,
    ray::Ray,
    shape::Shape,
    tuple::Tuple, equal,
};

pub struct World {
    objects: Vec<Shape>,
    light: Option<Light>,
}

impl World {
    pub fn new() -> World {
        World {
            objects: vec![],
            light: None,
        }
    }
    pub fn default() -> World {
        let light = Light::point(Tuple::point(-10, 10, -10), Pixel::white());
        let mut s1 = Shape::sphere();
        let mut s1_m = Material::default();
        s1_m.set_color(Pixel::rgb(0.8, 1.0, 0.6));
        s1_m.set_diffuse(0.7);
        s1_m.set_specular(0.2);
        s1.set_material(s1_m);
        let mut s2 = Shape::sphere();
        s2.set_transform(Matrix4x4::scaling(0.5, 0.5, 0.5));

        World {
            objects: vec![s1, s2],
            light: Some(light),
        }
    }
}

impl World {
    pub fn objects(&self) -> Vec<Shape> {
        self.objects.clone()
    }
    pub fn light(&self) -> Option<Light> {
        self.light
    }
    pub fn set_light(&mut self, light: Light) {
        self.light = Some(light);
    }
    pub fn set_objects(&mut self, objs: Vec<Shape>) {
        self.objects = objs;
    }

    pub fn intersect_world(&self, ray: Ray) -> Vec<Intersection> {
        let mut hits = Vec::<Intersection>::new();
        for obj in self.objects().clone().iter_mut() {
            hits.append(&mut obj.intersect(ray).clone());
        }

        hits.sort_by(|a, b| a.t().partial_cmp(&b.t()).unwrap());
        hits
    }

    pub fn shade_hit(&self, comps: Computations, remaining: i32) -> Pixel {
        let shadowed = self.is_shadowed(comps.over_point());

        let surface = comps.object().material().lighting(
            &comps.object(),
            &self.light().unwrap(),
            comps.point(),
            comps.eyev(),
            comps.normalv(),
            shadowed,
        );

        let reflected = self.reflected_color(comps, remaining);
        let refracted = self.refracted_color(comps, remaining);

        let material = comps.object().material();
        if material.reflective() > 0.0 && material.transparency() > 0.0 {
            let reflectance = comps.schlick();
            return surface + reflected * reflectance + refracted * (1.0 - reflectance);
        }

        surface + reflected + refracted
    }

    pub fn color_at(&self, ray: Ray, remaining: i32) -> Pixel {
        let xs = self.intersect_world(ray);
        if let Some(hit) = intersection::hit(xs.clone()) {
            let comps = hit.prepare_computations(ray, xs);
            self.shade_hit(comps, remaining)
        } else {
            Pixel::black()
        }
    }

    pub fn is_shadowed(&self, point: Tuple) -> bool {
        let v = self.light.unwrap().position() - point;
        let distance = v.magnitude();
        let direction = v.normalize();

        let r = Ray::new(point, direction);
        let intersections = self.intersect_world(r);

        let h = hit(intersections);
        if let Some(hs) = h {
            hs.t() < distance
        } else {
            false
        }
    }

    pub fn reflected_color(&self, comps: Computations, remaining: i32) -> Pixel{
        if remaining < 1 || equal(comps.object().material().reflective(), 0.0) {
            return Pixel::black();
        }

        let reflected_ray = Ray::new(comps.over_point(), comps.reflectv());
        let color = self.color_at(reflected_ray, remaining - 1);

        color * comps.object().material().reflective()
    }

    pub fn refracted_color(&self, comps: Computations, remaining: i32) -> Pixel{
        if remaining < 1 || equal(comps.object().material().transparency(), 0.0) {
            Pixel::black()
        } else {
            
            let n_ratio = comps.n1() / comps.n2();
            let cos_i = comps.eyev().dot(&comps.normalv());
            let sin2_t = n_ratio.powi(2) * (1.0 - cos_i.powi(2));
            
            if sin2_t > 1.0 {
                Pixel::black()
            } else {
                let cos_t = (1.0 - sin2_t).sqrt();
                let direction = comps.normalv() * (n_ratio * cos_i - cos_t) - comps.eyev() * n_ratio;
                
                let refracted_ray = Ray::new(comps.under_point(), direction);
                
                let color = self.color_at(refracted_ray, remaining - 1) * comps.object().material().transparency();
                println!("=== remaining {} ===\nn1: {}\nn2: {}\neyev: {}\nnormalv: {}\nunder point: {}\nn_ratio: {}\ncos_i: {}\nsin2_t: {}\ncost_t: {}\nrefracted direction: {}\nrefracted color: {}\n", remaining, comps.n1(), comps.n2(), comps.eyev(), comps.normalv(), comps.under_point(), n_ratio, cos_i, sin2_t, cos_t, direction, color);
                color
            }
        }
    }
}

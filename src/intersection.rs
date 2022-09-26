use crate::{ray::Ray, shape::Shape, tuple::Tuple, Num, EPSILON};

#[derive(Clone, Copy)]
pub struct Intersection {
    t: Num,
    obj: Shape,
}

impl Intersection {
    pub fn new<T>(t: T, obj: Shape) -> Intersection
    where
        T: Into<Num>,
    {
        Intersection {
            t: t.into(),
            obj: obj,
        }
    }
    pub fn make(i1: Intersection, i2: Intersection) -> Vec<Intersection> {
        let mut v = Vec::new();
        v.push(i1);
        v.push(i2);

        v
    }
}

impl Intersection {
    pub fn t(&self) -> Num {
        self.t
    }
    pub fn object(&self) -> Shape {
        self.obj
    }
}

impl Intersection {
    pub fn prepare_computations(&self, ray: Ray, xs: Vec<Intersection>) -> Computations {
        let mut comps = Computations::new();

        comps.set_t(self.t());
        comps.set_object(self.object());
        comps.set_point(ray.position(comps.t()));
        comps.set_eyev(-ray.direction());
        comps.set_normalv(comps.object().normal_at(comps.point()));

        if comps.normalv().dot(&comps.eyev()) < 0.0 {
            comps.set_inside(true);
            comps.set_normalv(-comps.normalv());
        } else {
            comps.set_inside(false);
        }
        comps.set_reflectv(ray.direction().reflect(comps.normalv()));

        comps.set_over_point(comps.point() + comps.normalv() * EPSILON);
        comps.set_under_point(comps.point() - comps.normalv() * EPSILON);

        let mut containers: Vec<Shape> = vec![];
        for i in xs {
            if i == *self {
                if containers.is_empty() {
                    comps.set_n1(1.0);
                }else {
                    comps.set_n1(containers.last().unwrap().material().refractive_index())
                }
            }

            if containers.contains(&i.object()) {
                containers.remove(containers.iter().position(|x| *x == i.object()).unwrap());
            }else {
                containers.push(i.object());
            }

            if i == *self {
                if containers.is_empty() {
                    comps.set_n2(1.0);
                } else {
                    comps.set_n2(containers.last().unwrap().material().refractive_index());
                }

                break;
            }
        }

        comps
    }
}

pub fn hit(xs: Vec<Intersection>) -> Option<Intersection> {
    let mut x: Vec<Intersection> = xs.clone();
    x.sort_by(|a, b| a.t().partial_cmp(&b.t()).unwrap());

    if let Some(s) = x.iter().find(|f| f.t() >= 0.0) {
        Some(*s)
    } else {
        None
    }
}

impl PartialEq<Intersection> for Intersection {
    fn eq(&self, other: &Intersection) -> bool {
        self.t == other.t && self.obj == other.obj
    }
}

#[derive(Clone, Copy)]
pub struct Computations {
    t: Num,
    object: Shape,
    point: Tuple,
    eyev: Tuple,
    normalv: Tuple,
    inside: bool,
    over_point: Tuple,
    under_point: Tuple,
    reflectv: Tuple,
    n1: Num,
    n2: Num,
}

impl Computations {
    fn new() -> Computations {
        Computations {
            t: 0.0,
            object: Shape::sphere(),
            point: Tuple::point(0, 0, 0),
            eyev: Tuple::vector(0, 0, 0),
            normalv: Tuple::vector(0, 0, 0),
            inside: false,
            over_point: Tuple::point(0, 0, 0),
            under_point: Tuple::point(0, 0, 0),
            reflectv: Tuple::vector(0, 0, 0),
            n1: 0.0,
            n2: 0.0,
        }
    }

    pub fn schlick(&self) -> Num{
        let mut cos = self.eyev().dot(&self.normalv());

        if self.n1() > self.n2() {
            let n = self.n1() / self.n2();
            let sin2_t = n*n * (1.0 - cos*cos);
            if sin2_t > 1.0 {
                return 1.0;
            }

            let cos_t = (1.0 - sin2_t).sqrt();

            cos = cos_t;
        }

        let r0 = ((self.n1() - self.n2()) / (self.n1() + self.n2())).powi(2);


        return r0 + (1.0 - r0) * (1.0 - cos).powi(5);
    }

    pub fn t(&self) -> Num {
        self.t
    }
    pub fn object(&self) -> Shape {
        self.object
    }
    pub fn point(&self) -> Tuple {
        self.point
    }
    pub fn eyev(&self) -> Tuple {
        self.eyev
    }
    pub fn normalv(&self) -> Tuple {
        self.normalv
    }
    pub fn inside(&self) -> bool {
        self.inside
    }
    pub fn over_point(&self) -> Tuple {
        self.over_point
    }
    pub fn under_point(&self) -> Tuple {
        self.under_point
    }
    pub fn reflectv(&self) -> Tuple{
        self.reflectv
    }
    pub fn n1(&self) -> Num{
        self.n1
    }
    pub fn n2(&self) -> Num{
        self.n2
    }

    fn set_t(&mut self, val: Num) {
        self.t = val;
    }
    fn set_object(&mut self, val: Shape) {
        self.object = val;
    }
    fn set_point(&mut self, val: Tuple) {
        self.point = val;
    }
    fn set_eyev(&mut self, val: Tuple) {
        self.eyev = val;
    }
    fn set_normalv(&mut self, val: Tuple) {
        self.normalv = val;
    }
    fn set_inside(&mut self, val: bool) {
        self.inside = val;
    }
    fn set_over_point(&mut self, val: Tuple) {
        self.over_point = val;
    }
    fn set_under_point(&mut self, val: Tuple) {
        self.under_point = val;
    }
    fn set_reflectv(&mut self, val: Tuple){
        self.reflectv = val;
    }
    fn set_n1(&mut self, val: Num){
        self.n1 = val;
    }
    fn set_n2(&mut self, val: Num){
        self.n2 = val;
    }
}

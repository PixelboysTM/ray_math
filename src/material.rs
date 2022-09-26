use crate::{img::Pixel, light::Light, tuple::Tuple, Num, Pattern, Shape};

#[derive(Clone, Copy)]
pub struct Material {
    color: Pixel,
    ambient: Num,
    diffuse: Num,
    specular: Num,
    shininess: Num,
    pattern: Option<Pattern>,
    reflective: Num,
    transparency: Num,
    refractive_index: Num,
}

impl Material {
    pub fn new<T1, T2, T3, T4>(
        color: Pixel,
        ambient: T1,
        diffuse: T2,
        specular: T3,
        shininess: T4,
    ) -> Material
    where
        T1: Into<Num>,
        T2: Into<Num>,
        T3: Into<Num>,
        T4: Into<Num>,
    {
        Material {
            color,
            ambient: ambient.into(),
            diffuse: diffuse.into(),
            specular: specular.into(),
            shininess: shininess.into(),
            pattern: None,
            reflective: 0.0,
            transparency: 0.0,
            refractive_index: 1.0,
        }
    }
    pub fn default() -> Material {
        Material::new(Pixel::white(), 0.1, 0.9, 0.9, 200)
    }
}

impl Material {
    pub fn color(&self) -> Pixel {
        self.color
    }
    pub fn ambient(&self) -> Num {
        self.ambient
    }
    pub fn diffuse(&self) -> Num {
        self.diffuse
    }
    pub fn specular(&self) -> Num {
        self.specular
    }
    pub fn shininess(&self) -> Num {
        self.shininess
    }
    pub fn pattern(&self) -> Option<Pattern>{
        self.pattern
    }
    pub fn reflective(&self) -> Num{
        self.reflective
    }
    pub fn transparency(&self) -> Num{
        self.transparency
    }
    pub fn refractive_index(&self) -> Num{
        self.refractive_index
    }

    pub fn set_color(&mut self, color: Pixel) {
        self.color = color;
    }
    pub fn set_ambient<T>(&mut self, val: T)
    where
        T: Into<Num>,
    {
        self.ambient = val.into();
    }
    pub fn set_diffuse<T>(&mut self, val: T)
    where
        T: Into<Num>,
    {
        self.diffuse = val.into();
    }
    pub fn set_specular<T>(&mut self, val: T)
    where
        T: Into<Num>,
    {
        self.specular = val.into();
    }
    pub fn set_shininess<T>(&mut self, val: T)
    where
        T: Into<Num>,
    {
        self.shininess = val.into();
    }
    pub fn set_pattern(&mut self, pattern: Pattern){
        self.pattern = Some(pattern);
    }
    pub fn set_reflective<T>(&mut self, val: T) where T: Into<Num>, {
        self.reflective = val.into();
    }
    pub fn set_transparency<T>(&mut self, val: T) where T: Into<Num>, {
        self.transparency = val.into();
    }
    pub fn set_refractive_index<T>(&mut self, val: T) where T: Into<Num>, {
        self.refractive_index = val.into();
    }
}

impl Material {
    pub fn lighting(
        &self,
        object: &Shape,
        light: &Light,
        point: Tuple,
        eyev: Tuple,
        normalv: Tuple,
        in_shadow: bool,
    ) -> Pixel {
        let color = if let Some(pattern) = self.pattern() {
            pattern.at_object(object, point)
        }else{
            self.color()
        };

        let effective_color = color * light.intensity();
        let lightv = (light.position() - point).normalize();
        let ambient = effective_color * self.ambient;

        let light_dot_normal = lightv.dot(&normalv);
        let (diffuse, specular) = if light_dot_normal < 0.0 {
            (Pixel::black(), Pixel::black())
        } else {
            let diffuse = effective_color * self.diffuse * light_dot_normal;

            let reflectv = (-lightv).reflect(normalv);
            let relfect_dot_eye = reflectv.dot(&eyev);

            let specular = if relfect_dot_eye <= 0.0 {
                Pixel::black()
            } else {
                let factor = relfect_dot_eye.powf(self.shininess);
                light.intensity() * self.specular * factor
            };

            (diffuse, specular)
        };

        ambient
            + if in_shadow {
                Pixel::black()
            } else {
                diffuse + specular
            }
    }
}

impl PartialEq<Material> for Material {
    fn eq(&self, other: &Material) -> bool {
        self.color == other.color
            && self.ambient == other.ambient
            && self.diffuse == other.diffuse
            && self.specular == other.specular
            && self.shininess == other.shininess
    }
}

use crate::{Canvas, Matrix4x4, Num, Ray, Tuple, World};

pub struct Camera {
    hsize: i32,
    vsize: i32,
    field_of_view: Num,
    transform: Matrix4x4,
    pixel_size: Num,
    half_width: Num,
    half_height: Num,
}

impl Camera {
    pub fn new<T>(hsize: i32, vsize: i32, field_of_view: T) -> Camera
    where
        T: Into<Num>,
    {
        let fov = field_of_view.into();
        let (pixel_size, half_width, half_height) = get_pixel_size(hsize, vsize, fov);
        Camera {
            hsize,
            vsize,
            field_of_view: fov,
            transform: Matrix4x4::identity(),
            pixel_size,
            half_height,
            half_width,
        }
    }
    pub fn with_transform<T>(
        hsize: i32,
        vsize: i32,
        field_of_view: T,
        transform: Matrix4x4,
    ) -> Camera
    where
        T: Into<Num>,
    {
        let mut c = Camera::new(hsize, vsize, field_of_view);
        c.transform = transform;
        c
    }
}

impl Camera {
    pub fn hsize(&self) -> i32 {
        self.hsize
    }
    pub fn vsize(&self) -> i32 {
        self.vsize
    }
    pub fn field_of_view(&self) -> Num {
        self.field_of_view
    }
    pub fn transform(&self) -> Matrix4x4 {
        self.transform
    }
    pub fn pixel_size(&self) -> Num {
        self.pixel_size
    }

    pub fn ray_for_pixel(&self, px: usize, py: usize) -> Ray {
        let xoffset = (px as Num + 0.5) * self.pixel_size();
        let yoffset = (py as Num + 0.5) * self.pixel_size();

        let world_x = self.half_width - xoffset;
        let world_y = self.half_height - yoffset;

        let transform_inv = self.transform().inverse().unwrap();

        let pixel = transform_inv * Tuple::point(world_x, world_y, -1);
        let origin = transform_inv * Tuple::point(0, 0, 0);
        let direction = (pixel - origin).normalize();

        Ray::new(origin, direction)
    }

    pub fn render(&self, world: World) -> Canvas {
        const MAX_RECURSION_DEPTH: i32 = 5;
        let mut image = Canvas::with_dimesnions(self.hsize() as u32, self.vsize() as u32);
        for x in 0..self.hsize() {
            for y in 0..self.vsize() {
                println!("|||||||||||({}/{})|||||||||||", x, y);
                // if x == 125 && y == 125 {
                //     println!("");
                // }

                let ray = self.ray_for_pixel(x as usize, y as usize);
                let color = world.color_at(ray, MAX_RECURSION_DEPTH);
                image.set(x as u32, y as u32, color);
            }
        }

        image
    }
}

fn get_pixel_size(hsize: i32, vsize: i32, field_of_view: Num) -> (Num, Num, Num) {
    let half_view = (field_of_view / 2.0).tan();
    let aspect = hsize as Num / vsize as Num;

    let (half_width, half_height) = if aspect >= 1.0 {
        (half_view, half_view / aspect)
    } else {
        (half_view * aspect, half_view)
    };

    ((half_width * 2.0) / hsize as Num, half_width, half_height)
}

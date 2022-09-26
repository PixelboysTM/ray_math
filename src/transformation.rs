use crate::{matrix4x4::Matrix4x4, tuple::Tuple, Num};

impl Matrix4x4 {
    pub fn view(from: Tuple, to: Tuple, up: Tuple) -> Matrix4x4 {
        let forward = (to - from).normalize();
        let left = forward.cross(&up.normalize());
        let true_up = left.cross(&forward);

        let orientation = Matrix4x4::new([
            left.get_x(),
            left.get_y(),
            left.get_z(),
            0.0,
            true_up.get_x(),
            true_up.get_y(),
            true_up.get_z(),
            0.0,
            -forward.get_x(),
            -forward.get_y(),
            -forward.get_z(),
            0.0,
            0.0,
            0.0,
            0.0,
            1.0,
        ]);

        orientation * Matrix4x4::translation(-from.get_x(), -from.get_y(), -from.get_z())
    }

    pub fn translation<T1, T2, T3>(x: T1, y: T2, z: T3) -> Matrix4x4
    where
        T1: Into<Num>,
        T2: Into<Num>,
        T3: Into<Num>,
    {
        let mut m = Matrix4x4::identity();
        m.set(0, 3, x);
        m.set(1, 3, y);
        m.set(2, 3, z);
        m
    }

    pub fn scaling<T1, T2, T3>(x: T1, y: T2, z: T3) -> Matrix4x4
    where
        T1: Into<Num>,
        T2: Into<Num>,
        T3: Into<Num>,
    {
        let mut m = Matrix4x4::identity();
        m.set(0, 0, x);
        m.set(1, 1, y);
        m.set(2, 2, z);
        m
    }

    pub fn rotation_x<T>(rad: T) -> Matrix4x4
    where
        T: Into<Num>,
    {
        let mut m = Matrix4x4::identity();
        let num: Num = rad.into();
        let cos = num.cos();
        let sin = num.sin();

        m.set(1, 1, cos);
        m.set(1, 2, -sin);
        m.set(2, 1, sin);
        m.set(2, 2, cos);
        m
    }

    pub fn rotation_y<T>(rad: T) -> Matrix4x4
    where
        T: Into<Num>,
    {
        let mut m = Matrix4x4::identity();
        let num: Num = rad.into();
        let cos = num.cos();
        let sin = num.sin();

        m.set(0, 0, cos);
        m.set(0, 2, sin);
        m.set(2, 0, -sin);
        m.set(2, 2, cos);
        m
    }

    pub fn rotation_z<T>(rad: T) -> Matrix4x4
    where
        T: Into<Num>,
    {
        let mut m = Matrix4x4::identity();
        let num: Num = rad.into();
        let cos = num.cos();
        let sin = num.sin();

        m.set(0, 0, cos);
        m.set(0, 1, -sin);
        m.set(1, 0, sin);
        m.set(1, 1, cos);
        m
    }

    pub fn shearing<T1, T2, T3, T4, T5, T6>(
        xy: T1,
        xz: T2,
        yx: T3,
        yz: T4,
        zx: T5,
        zy: T6,
    ) -> Matrix4x4
    where
        T1: Into<Num>,
        T2: Into<Num>,
        T3: Into<Num>,
        T4: Into<Num>,
        T5: Into<Num>,
        T6: Into<Num>,
    {
        let mut m = Matrix4x4::identity();
        m.set(0, 1, xy);
        m.set(0, 2, xz);
        m.set(1, 0, yx);
        m.set(1, 2, yz);
        m.set(2, 0, zx);
        m.set(2, 1, zy);
        m
    }
}

pub struct TransformationBuilder {
    mats: Vec<Matrix4x4>,
}

impl TransformationBuilder {
    pub fn new() -> TransformationBuilder {
        Self { mats: Vec::new() }
    }

    pub fn translation<T1, T2, T3>(x: T1, y: T2, z: T3) -> TransformationBuilder
    where
        T1: Into<Num>,
        T2: Into<Num>,
        T3: Into<Num>,
    {
        let mut m = TransformationBuilder::new();
        m.mats.push(Matrix4x4::translation(x, y, z));
        m
    }

    pub fn scaling<T1, T2, T3>(x: T1, y: T2, z: T3) -> TransformationBuilder
    where
        T1: Into<Num>,
        T2: Into<Num>,
        T3: Into<Num>,
    {
        let mut m = TransformationBuilder::new();
        m.mats.push(Matrix4x4::scaling(x, y, z));
        m
    }

    pub fn rotation_x<T>(rad: T) -> TransformationBuilder
    where
        T: Into<Num>,
    {
        let mut m = TransformationBuilder::new();
        m.mats.push(Matrix4x4::rotation_x(rad));
        m
    }

    pub fn rotation_y<T>(rad: T) -> TransformationBuilder
    where
        T: Into<Num>,
    {
        let mut m = TransformationBuilder::new();
        m.mats.push(Matrix4x4::rotation_y(rad));
        m
    }

    pub fn rotation_z<T>(rad: T) -> TransformationBuilder
    where
        T: Into<Num>,
    {
        let mut m = TransformationBuilder::new();
        m.mats.push(Matrix4x4::rotation_z(rad));
        m
    }

    pub fn shearing<T1, T2, T3, T4, T5, T6>(
        xy: T1,
        xz: T2,
        yx: T3,
        yz: T4,
        zx: T5,
        zy: T6,
    ) -> TransformationBuilder
    where
        T1: Into<Num>,
        T2: Into<Num>,
        T3: Into<Num>,
        T4: Into<Num>,
        T5: Into<Num>,
        T6: Into<Num>,
    {
        let mut m = TransformationBuilder::new();
        m.mats.push(Matrix4x4::shearing(xy, xz, yx, yz, zx, zy));
        m
    }
}

impl TransformationBuilder {
    pub fn translate<T1, T2, T3>(mut self, x: T1, y: T2, z: T3) -> TransformationBuilder
    where
        T1: Into<Num>,
        T2: Into<Num>,
        T3: Into<Num>,
    {
        self.mats.push(Matrix4x4::translation(x, y, z));
        self
    }

    pub fn scale<T1, T2, T3>(mut self, x: T1, y: T2, z: T3) -> TransformationBuilder
    where
        T1: Into<Num>,
        T2: Into<Num>,
        T3: Into<Num>,
    {
        self.mats.push(Matrix4x4::scaling(x, y, z));
        self
    }

    pub fn rotate_x<T>(mut self, rad: T) -> TransformationBuilder
    where
        T: Into<Num>,
    {
        self.mats.push(Matrix4x4::rotation_x(rad));
        self
    }

    pub fn rotate_y<T>(mut self, rad: T) -> TransformationBuilder
    where
        T: Into<Num>,
    {
        self.mats.push(Matrix4x4::rotation_y(rad));
        self
    }

    pub fn rotate_z<T>(mut self, rad: T) -> TransformationBuilder
    where
        T: Into<Num>,
    {
        self.mats.push(Matrix4x4::rotation_z(rad));
        self
    }

    pub fn shear<T1, T2, T3, T4, T5, T6>(
        mut self,
        xy: T1,
        xz: T2,
        yx: T3,
        yz: T4,
        zx: T5,
        zy: T6,
    ) -> TransformationBuilder
    where
        T1: Into<Num>,
        T2: Into<Num>,
        T3: Into<Num>,
        T4: Into<Num>,
        T5: Into<Num>,
        T6: Into<Num>,
    {
        self.mats.push(Matrix4x4::shearing(xy, xz, yx, yz, zx, zy));
        self
    }

    pub fn build(mut self) -> Matrix4x4 {
        let mut m = Matrix4x4::identity();
        if self.mats.is_empty() {
            return m;
        }

        while !self.mats.is_empty() {
            let new = self.mats.pop().unwrap();
            m = m * new;
        }

        m
    }
}

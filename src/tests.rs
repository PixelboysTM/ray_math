mod chapter1 {
    use crate::{equal, tuple::Tuple};

    #[test]
    fn adding() {
        let a1 = Tuple::new(3.0, -2.0, 5.0, 1.0);
        let a2 = Tuple::new(-2.0, 3.0, 1.0, 0.0);
        let res = a1 + a2;
        assert!(res == Tuple::new(1.0, 1.0, 6.0, 1.0));
    }
    #[test]
    fn subtracting() {
        let a1 = Tuple::point(3.0, 2.0, 1.0);
        let a2 = Tuple::point(5.0, 6.0, 7.0);
        let res = a1 - a2;
        assert!(res == Tuple::vector(-2.0, -4.0, -6.0));

        {
            let p = Tuple::point(3.0, 2.0, 1.0);
            let v = Tuple::vector(5.0, 6.0, 7.0);
            assert!(p - v == Tuple::point(-2.0, -4.0, -6.0));
        }

        {
            let v1 = Tuple::vector(3.0, 2.0, 1.0);
            let v2 = Tuple::vector(5.0, 6.0, 7.0);
            assert!(v1 - v2 == Tuple::vector(-2.0, -4.0, -6.0));
        }

        {
            let zero = Tuple::vector(0.0, 0.0, 0.0);
            let v = Tuple::vector(1.0, -2.0, 3.0);
            assert!(zero - v == Tuple::vector(-1.0, 2.0, -3.0));
        }

        {
            let a = Tuple::new(1.0, -2.0, 3.0, -4.0);
            assert!(-a == Tuple::new(-1.0, 2.0, -3.0, 4.0));
        }
    }

    #[test]
    fn mul() {
        {
            let a = Tuple::new(1.0, -2.0, 3.0, -4.0);
            assert!(a * 3.5 == Tuple::new(3.5, -7.0, 10.5, -14.0));
        }

        {
            let a = Tuple::new(1.0, -2.0, 3.0, -4.0);
            assert!(a * 0.5 == Tuple::new(0.5, -1.0, 1.5, -2.0));
        }

        {
            let a = Tuple::new(1.0, -2.0, 3.0, -4.0);
            assert!(a / 2.0 == Tuple::new(0.5, -1.0, 1.5, -2.0));
        }
    }

    #[test]
    fn magnitude() {
        {
            let v = Tuple::vector(1.0, 0.0, 0.0);
            assert!(equal(v.magnitude(), 1.0));
        }

        {
            let v = Tuple::vector(0.0, 1.0, 0.0);
            assert!(equal(v.magnitude(), 1.0));
        }

        {
            let v = Tuple::vector(0.0, 0.0, 1.0);
            assert!(equal(v.magnitude(), 1.0));
        }

        {
            let v = Tuple::vector(1.0, 2.0, 3.0);
            assert!(equal(v.magnitude(), 14.0_f64.sqrt()));
        }

        {
            let v = Tuple::vector(-1.0, -2.0, -3.0);
            assert!(equal(v.magnitude(), 14.0_f64.sqrt()));
        }
    }

    #[test]
    fn normalize() {
        {
            let v = Tuple::vector(4.0, 0.0, 0.0);
            assert!(v.normalize() == Tuple::vector(1.0, 0.0, 0.0));
        }

        {
            let v = Tuple::vector(1.0, 2.0, 3.0);
            assert!(v.normalize() == Tuple::vector(0.26726, 0.53452, 0.80178));
        }

        {
            let v = Tuple::vector(1.0, 2.0, 3.0);
            let norm = v.normalize();
            assert!(equal(norm.magnitude(), 1.0));
        }
    }

    #[test]
    fn dot() {
        let a = Tuple::vector(1.0, 2.0, 3.0);
        let b = Tuple::vector(2.0, 3.0, 4.0);
        assert!(equal(a.dot(&b), 20.0))
    }

    #[test]
    fn cross() {
        let a = Tuple::vector(1.0, 2.0, 3.0);
        let b = Tuple::vector(2.0, 3.0, 4.0);

        assert!(a.cross(&b) == Tuple::vector(-1.0, 2.0, -1.0));
        assert!(b.cross(&a) == Tuple::vector(1.0, -2.0, 1.0));
    }
}

mod chapter3 {
    use crate::{
        equal, matrix2x2::Matrix2x2, matrix3x3::Matrix3x3, matrix4x4::Matrix4x4, tuple::Tuple,
    };

    #[test]
    fn creating4x4() {
        let m = Matrix4x4::new([
            1.0, 2.0, 3.0, 4.0, 5.5, 6.5, 7.5, 8.5, 9.0, 10.0, 11.0, 12.0, 13.5, 14.5, 15.5, 16.5,
        ]);
        assert!(equal(m.at(0, 0), 1.0));
        assert!(equal(m.at(0, 3), 4.0));
        assert!(equal(m.at(1, 0), 5.5));
        assert!(equal(m.at(1, 2), 7.5));
        assert!(equal(m.at(2, 2), 11.0));
        assert!(equal(m.at(3, 0), 13.5));
        assert!(equal(m.at(3, 2), 15.5));
    }

    #[test]
    fn creating2x2() {
        let m = Matrix2x2::new([-3.0, 5.0, 1.0, -2.0]);
        assert!(equal(m.at(0, 0), -3.0));
        assert!(equal(m.at(0, 1), 5.0));
        assert!(equal(m.at(1, 0), 1.0));
        assert!(equal(m.at(1, 1), -2.0));
    }

    #[test]
    fn creating3x3() {
        let m = Matrix3x3::new([-3.0, 5.0, 0.0, 1.0, -2.0, -7.0, 0.0, 1.0, 1.0]);
        assert!(equal(m.at(0, 0), -3.0));
        assert!(equal(m.at(1, 1), -2.0));
        assert!(equal(m.at(2, 2), 1.0));
    }

    #[test]
    fn matrices() {
        {
            let a = Matrix4x4::new([
                1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0,
            ]);
            let b = Matrix4x4::new([
                1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0,
            ]);
            assert!(a == b);
        }

        {
            let a = Matrix4x4::new([
                1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0,
            ]);
            let b = Matrix4x4::new([
                2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0, 1.0,
            ]);
            assert!(a != b);
        }

        {
            let a = Matrix4x4::new([
                1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0,
            ]);
            let b = Matrix4x4::new([
                -2.0, 1.0, 2.0, 3.0, 3.0, 2.0, 1.0, -1.0, 4.0, 3.0, 6.0, 5.0, 1.0, 2.0, 7.0, 8.0,
            ]);
            assert!(
                a * b
                    == Matrix4x4::new([
                        20.0, 22.0, 50.0, 48.0, 44.0, 54.0, 114.0, 108.0, 40.0, 58.0, 110.0, 102.0,
                        16.0, 26.0, 46.0, 42.0
                    ])
            );
        }

        {
            let a = Matrix4x4::new([
                1.0, 2.0, 3.0, 4.0, 2.0, 4.0, 4.0, 2.0, 8.0, 6.0, 4.0, 1.0, 0.0, 0.0, 0.0, 1.0,
            ]);
            let b = Tuple::new(1.0, 2.0, 3.0, 1.0);
            assert!(a * b == Tuple::new(18.0, 24.0, 33.0, 1.0));
        }

        {
            let a = Matrix4x4::new([
                0.0, 1.0, 2.0, 4.0, 1.0, 2.0, 4.0, 8.0, 2.0, 4.0, 8.0, 16.0, 4.0, 8.0, 16.0, 32.0,
            ]);
            assert!(a * Matrix4x4::identity() == a);
        }

        {
            let a = Tuple::new(1.0, 2.0, 3.0, 4.0);
            assert!(Matrix4x4::identity() * a == a);
        }

        {
            let a = Matrix4x4::new([
                0.0, 9.0, 3.0, 0.0, 9.0, 8.0, 0.0, 8.0, 1.0, 8.0, 5.0, 3.0, 0.0, 0.0, 5.0, 8.0,
            ]);
            assert!(
                a.transpose()
                    == Matrix4x4::new([
                        0.0, 9.0, 1.0, 0.0, 9.0, 8.0, 8.0, 0.0, 3.0, 0.0, 5.0, 5.0, 0.0, 8.0, 3.0,
                        8.0,
                    ])
            )
        }

        {
            let a = Matrix4x4::identity();
            assert!(a.transpose() == a);
        }
    }

    #[test]
    fn inverse() {
        {
            let a = Matrix2x2::new([1.0, 5.0, -3.0, 2.0]);
            assert!(equal(a.determinant(), 17.0));
        }

        {
            let a = Matrix3x3::new([1.0, 5.0, 0.0, -3.0, 2.0, 7.0, 0.0, 6.0, -3.0]);
            assert!(a.submatrix(0, 2) == Matrix2x2::new([-3.0, 2.0, 0.0, 6.0,]));
        }

        {
            let a = Matrix4x4::new([
                -6.0, 1.0, 1.0, 6.0, -8.0, 5.0, 8.0, 6.0, -1.0, 0.0, 8.0, 2.0, -7.0, 1.0, -1.0, 1.0,
            ]);

            assert!(
                a.submatrix(2, 1)
                    == Matrix3x3::new([-6.0, 1.0, 6.0, -8.0, 8.0, 6.0, -7.0, -1.0, 1.0,])
            )
        }

        {
            let a = Matrix3x3::new([3.0, 5.0, 0.0, 2.0, -1.0, -7.0, 6.0, -1.0, 5.0]);
            let b = a.submatrix(1, 0);
            assert!(equal(b.determinant(), 25.0));
            assert!(equal(a.minor(1, 0), 25.0));
        }

        {
            let a = Matrix3x3::new([3.0, 5.0, 0.0, 2.0, -1.0, -7.0, 6.0, -1.0, 5.0]);
            assert!(equal(a.minor(0, 0), -12.0));
            assert!(equal(a.cofactor(0, 0), -12.0));
            assert!(equal(a.minor(1, 0), 25.0));
            assert!(equal(a.cofactor(1, 0), -25.0));
        }

        {
            let a = Matrix3x3::new([1.0, 2.0, 6.0, -5.0, 8.0, -4.0, 2.0, 6.0, 4.0]);
            assert!(equal(a.cofactor(0, 0), 56.0));
            assert!(equal(a.cofactor(0, 1), 12.0));
            assert!(equal(a.cofactor(0, 2), -46.0));
            assert!(equal(a.determinant(), -196.0));
        }

        {
            let a = Matrix4x4::new([
                -2.0, -8.0, 3.0, 5.0, -3.0, 1.0, 7.0, 3.0, 1.0, 2.0, -9.0, 6.0, -6.0, 7.0, 7.0,
                -9.0,
            ]);
            assert!(equal(a.cofactor(0, 0), 690.0));
            assert!(equal(a.cofactor(0, 1), 447.0));
            assert!(equal(a.cofactor(0, 2), 210.0));
            assert!(equal(a.cofactor(0, 3), 51.0));
            assert!(equal(a.determinant(), -4071.0))
        }

        {
            let a = Matrix4x4::new([
                6.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 6.0, 4.0, -9.0, 3.0, -7.0, 9.0, 1.0, 7.0, -6.0,
            ]);
            assert!(equal(a.determinant(), -2120.0));
            assert!(a.invertible())
        }

        {
            let a = Matrix4x4::new([
                -4.0, 2.0, -2.0, -3.0, 9.0, 6.0, 2.0, 6.0, 0.0, -5.0, 1.0, -5.0, 0.0, 0.0, 0.0, 0.0,
            ]);
            assert!(equal(a.determinant(), 0.0));
            assert!(!a.invertible());
        }

        {
            let a = Matrix4x4::new([
                -5.0, 2.0, 6.0, -8.0, 1.0, -5.0, 1.0, 8.0, 7.0, 7.0, -6.0, -7.0, 1.0, -3.0, 7.0,
                4.0,
            ]);
            let b = a.inverse().unwrap();
            assert!(equal(a.determinant(), 532.0));
            assert!(equal(a.cofactor(2, 3), -160.0));
            assert!(equal(b.at(3, 2), -160.0 / 532.0));
            assert!(equal(a.cofactor(3, 2), 105.0));
            assert!(equal(b.at(2, 3), 105.0 / 532.0));
            assert!(
                b == Matrix4x4::new([
                    0.21805, 0.45113, 0.24060, -0.04511, -0.80827, -1.45677, -0.44361, 0.52068,
                    -0.07895, -0.22368, -0.05263, 0.19737, -0.52256, -0.81391, -0.30075, 0.30639
                ])
            );
        }

        {
            let a = Matrix4x4::new([8, -5, 9, 2, 7, 5, 6, 1, -6, 0, 9, 6, -3, 0, -9, -4]);
            assert!(
                a.inverse().unwrap()
                    == Matrix4x4::new([
                        -0.15385, -0.15385, -0.28205, -0.53846, -0.07692, 0.12308, 0.02564,
                        0.03077, 0.35897, 0.35897, 0.43590, 0.92308, -0.69231, -0.69231, -0.76923,
                        -1.92308,
                    ])
            );
        }

        {
            let a = Matrix4x4::new([9, 3, 0, 9, -5, -2, -6, -3, -4, 9, 6, 4, -7, 6, 6, 2]);
            assert!(
                a.inverse().unwrap()
                    == Matrix4x4::new([
                        -0.04074, -0.07778, 0.14444, -0.22222, -0.07778, 0.03333, 0.36667,
                        -0.33333, -0.02901, -0.14630, -0.10926, 0.12963, 0.17778, 0.06667,
                        -0.26667, 0.33333,
                    ])
            );
        }

        {
            let a = Matrix4x4::new([3, -9, 7, 3, 3, -8, 2, -9, -4, 4, 4, 1, -6, 5, -1, 1]);
            let b = Matrix4x4::new([8, 2, 2, 2, 3, -1, 7, 0, 7, 0, 5, 4, 6, -2, 0, 5]);
            let c = a * b;
            assert!(c * b.inverse().unwrap() == a);
        }
    }
}

mod chapter4 {
    use crate::{matrix4x4::Matrix4x4, transformation::TransformationBuilder, tuple::Tuple, PI};

    #[test]
    fn translation() {
        {
            let transform = Matrix4x4::translation(5, -3, 2);
            let p = Tuple::point(-3, 4, 5);
            assert!(transform * p == Tuple::point(2, 1, 7.0))
        }

        {
            let transform = Matrix4x4::translation(5, -3, 2);
            let inv = transform.inverse().unwrap();
            let p = Tuple::point(-3, 4, 5);
            assert!(inv * p == Tuple::point(-8, 7, 3));
        }

        {
            let transform = Matrix4x4::translation(5, -3, 2);
            let v = Tuple::vector(-3, 4, 5);
            assert!(transform * v == v);
        }
    }

    #[test]
    fn scaling() {
        {
            let transform = Matrix4x4::scaling(2, 3, 4);
            let p = Tuple::point(-4, 6, 8);
            assert!(transform * p == Tuple::point(-8, 18, 32));
        }

        {
            let transform = Matrix4x4::scaling(2, 3, 4);
            let v = Tuple::vector(-4, 6, 8);
            assert!(transform * v == Tuple::vector(-8, 18, 32));
        }

        {
            let transform = Matrix4x4::scaling(2, 3, 4);
            let inv = transform.inverse().unwrap();
            let v = Tuple::vector(-4, 6, 8);
            assert!(inv * v == Tuple::vector(-2, 2, 2));
        }

        {
            let transform = Matrix4x4::scaling(-1, 1, 1);
            let p = Tuple::point(2, 3, 4);
            assert!(transform * p == Tuple::point(-2, 3, 4));
        }
    }

    #[test]
    fn rotation() {
        {
            let p = Tuple::point(0, 1, 0);
            let half_quarter = Matrix4x4::rotation_x(PI / 4.0);
            let full_quarter = Matrix4x4::rotation_x(PI / 2.0);
            assert!(
                half_quarter * p == Tuple::point(0, 2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0)
            );
            assert!(full_quarter * p == Tuple::point(0, 0, 1));
        }

        {
            let p = Tuple::point(0, 1, 0);
            let half_quarter = Matrix4x4::rotation_x(PI / 4.0);
            let inv = half_quarter.inverse().unwrap();
            assert!(inv * p == Tuple::point(0, 2.0_f64.sqrt() / 2.0, -2.0_f64.sqrt() / 2.0))
        }

        {
            let p = Tuple::point(0, 0, 1);
            let half_quarter = Matrix4x4::rotation_y(PI / 4.0);
            let full_quarter = Matrix4x4::rotation_y(PI / 2.0);
            assert!(
                half_quarter * p == Tuple::point(2.0_f64.sqrt() / 2.0, 0.0, 2.0_f64.sqrt() / 2.0)
            );
            assert!(full_quarter * p == Tuple::point(1, 0, 0));
        }

        {
            let p = Tuple::point(0, 1, 0);
            let half_quarter = Matrix4x4::rotation_z(PI / 4.0);
            let full_quarter = Matrix4x4::rotation_z(PI / 2.0);
            assert!(
                half_quarter * p == Tuple::point(-2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0, 0.0)
            );
            assert!(full_quarter * p == Tuple::point(-1, 0, 0));
        }
    }

    #[test]
    fn shearing() {
        {
            let transform = Matrix4x4::shearing(1, 0, 0, 0, 0, 0);
            let p = Tuple::point(2, 3, 4);
            assert!(transform * p == Tuple::point(5, 3, 4));
        }

        {
            let transform = Matrix4x4::shearing(0, 1, 0, 0, 0, 0);
            let p = Tuple::point(2, 3, 4);
            assert!(transform * p == Tuple::point(6, 3, 4));
        }

        {
            let transform = Matrix4x4::shearing(0, 0, 1, 0, 0, 0);
            let p = Tuple::point(2, 3, 4);
            assert!(transform * p == Tuple::point(2, 5, 4));
        }

        {
            let transform = Matrix4x4::shearing(0, 0, 0, 1, 0, 0);
            let p = Tuple::point(2, 3, 4);
            assert!(transform * p == Tuple::point(2, 7, 4));
        }

        {
            let transform = Matrix4x4::shearing(0, 0, 0, 0, 1, 0);
            let p = Tuple::point(2, 3, 4);
            assert!(transform * p == Tuple::point(2, 3, 6));
        }

        {
            let transform = Matrix4x4::shearing(0, 0, 0, 0, 0, 1);
            let p = Tuple::point(2, 3, 4);
            assert!(transform * p == Tuple::point(2, 3, 7));
        }
    }

    #[test]
    fn shaining() {
        {
            let p = Tuple::point(1, 0, 1);
            let a = Matrix4x4::rotation_x(PI / 2.0);
            let b = Matrix4x4::scaling(5, 5, 5);
            let c = Matrix4x4::translation(10, 5, 7);

            let p2 = a * p;
            assert!(p2 == Tuple::point(1, -1, 0));

            let p3 = b * p2;
            assert!(p3 == Tuple::point(5, -5, 0));

            let p4 = c * p3;
            assert!(p4 == Tuple::point(15, 0, 7));

            let t = c * b * a;
            assert!(t * p == Tuple::point(15, 0, 7));
        }

        {
            let t = TransformationBuilder::new()
                .rotate_x(PI / 2.0)
                .scale(5, 5, 5)
                .translate(10, 5, 7)
                .build();
            let p = Tuple::point(1, 0, 1);
            assert!(t * p == Tuple::point(15, 0, 7));
        }
    }
}

mod chapter5 {
    use crate::{
        equal,
        img::{Canvas, Pixel},
        intersection::{self, Intersection},
        matrix4x4::Matrix4x4,
        ray::Ray,
        shape::{Shape, Sphere},
        tuple::Tuple,
        Num,
    };

    #[test]
    fn creating_rays() {
        {
            let origin = Tuple::point(1, 2, 3);
            let direction = Tuple::vector(4, 5, 6);
            let r = Ray::new(origin, direction);
            assert!(r.origin() == origin);
            assert!(r.direction() == direction);
        }

        {
            let r = Ray::new(Tuple::point(2, 3, 4), Tuple::vector(1, 0, 0));
            assert!(r.position(0) == Tuple::point(2, 3, 4));
            assert!(r.position(1) == Tuple::point(3, 3, 4));
            assert!(r.position(-1) == Tuple::point(1, 3, 4));
            assert!(r.position(2.5) == Tuple::point(4.5, 3, 4));
        }
    }

    #[test]
    fn intersecting_ras_with_speheres() {
        {
            let r = Ray::new(Tuple::point(0, 0, -5), Tuple::vector(0, 0, 1));
            let s = Shape::Sphere(Sphere::new());
            let xs = s.intersect(r);
            assert!(xs.len() == 2);
            assert!(equal(xs[0].t(), 4.0));
            assert!(equal(xs[1].t(), 6.0));
        }

        {
            let r = Ray::new(Tuple::point(0, 1, -5), Tuple::vector(0, 0, 1));
            let s = Shape::Sphere(Sphere::new());
            let xs = s.intersect(r);
            assert!(xs.len() == 2);
            assert!(equal(xs[0].t(), 5.0));
            assert!(equal(xs[1].t(), 5.0));
        }

        {
            let r = Ray::new(Tuple::point(0, 2, -5), Tuple::vector(0, 0, 1));
            let s = Shape::Sphere(Sphere::new());
            let xs = s.intersect(r);
            assert!(xs.len() == 0);
        }

        {
            let r = Ray::new(Tuple::point(0, 0, 0), Tuple::vector(0, 0, 1));
            let s = Shape::Sphere(Sphere::new());
            let xs = s.intersect(r);
            assert!(xs.len() == 2);
            assert!(equal(xs[0].t(), -1.0));
            assert!(equal(xs[1].t(), 1.0));
        }

        {
            let r = Ray::new(Tuple::point(0, 0, 5), Tuple::vector(0, 0, 1));
            let s = Shape::Sphere(Sphere::new());
            let xs = s.intersect(r);
            assert!(xs.len() == 2);
            assert!(equal(xs[0].t(), -6.0));
            assert!(equal(xs[1].t(), -4.0));
        }
    }

    #[test]
    fn tracking_intersections() {
        {
            let s = Shape::Sphere(Sphere::new());
            let i = Intersection::new(3.5, s);
            assert!(equal(i.t(), 3.5));
            assert!(i.object() == s);
        }

        {
            let s = Shape::Sphere(Sphere::new());
            let i1 = Intersection::new(1, s);
            let i2 = Intersection::new(2, s);
            let xs = Intersection::make(i1, i2);
            assert!(xs.len() == 2);
            assert!(equal(xs[0].t(), 1.0));
            assert!(equal(xs[1].t(), 2.0));
        }

        {
            let r = Ray::new(Tuple::point(0, 0, -5), Tuple::vector(0, 0, 1));
            let s = Shape::sphere();
            let xs = s.intersect(r);
            assert!(xs.len() == 2);
            assert!(xs[0].object() == s);
            assert!(xs[1].object() == s);
        }
    }

    #[test]
    fn identifying_hits() {
        {
            let s = Shape::sphere();
            let i1 = Intersection::new(1, s);
            let i2 = Intersection::new(2, s);
            let xs = Intersection::make(i2, i1);
            let i = intersection::hit(xs).unwrap();
            assert!(i == i1);
        }

        {
            let s = Shape::sphere();
            let i1 = Intersection::new(-1, s);
            let i2 = Intersection::new(1, s);
            let xs = Intersection::make(i2, i1);
            let i = intersection::hit(xs).unwrap();
            assert!(i == i2);
        }

        {
            let s = Shape::sphere();
            let i1 = Intersection::new(5, s);
            let i2 = Intersection::new(7, s);
            let i3 = Intersection::new(-3, s);
            let i4 = Intersection::new(2, s);
            let xs = vec![i1, i2, i3, i4];
            let i = intersection::hit(xs).unwrap();
            assert!(i == i4);
        }
    }

    #[test]
    fn transforming_rays_and_spheres() {
        {
            let r = Ray::new(Tuple::point(1, 2, 3), Tuple::vector(0, 1, 0));
            let m = Matrix4x4::translation(3, 4, 5);
            let r2 = r.transform(m);
            assert!(r2.origin() == Tuple::point(4, 6, 8));
            assert!(r2.direction() == Tuple::vector(0, 1, 0));
        }

        {
            let r = Ray::new(Tuple::point(1, 2, 3), Tuple::vector(0, 1, 0));
            let m = Matrix4x4::scaling(2, 3, 4);
            let r2 = r.transform(m);
            assert!(r2.origin() == Tuple::point(2, 6, 12));
            assert!(r2.direction() == Tuple::vector(0, 3, 0));
        }

        {
            let s = Shape::sphere();
            assert!(s.transform() == Matrix4x4::identity());
        }

        {
            let mut s = Shape::sphere();
            let t = Matrix4x4::translation(2, 3, 4);
            s.set_transform(t);
            assert!(s.transform() == t);
        }

        {
            let r = Ray::new(Tuple::point(0, 0, -5), Tuple::vector(0, 0, 1));
            let mut s = Shape::sphere();
            s.set_transform(Matrix4x4::scaling(2, 2, 2));
            let xs = s.intersect(r);
            assert!(xs.len() == 2);
            assert!(equal(xs[0].t(), 3.0));
            assert!(equal(xs[1].t(), 7.0));
        }

        {
            let r = Ray::new(Tuple::point(0, 0, -5), Tuple::vector(0, 0, 1));
            let mut s = Shape::sphere();
            s.set_transform(Matrix4x4::translation(5, 0, 0));
            let xs = s.intersect(r);
            assert!(xs.len() == 0);
        }
    }

    #[test]
    fn putting_it_together() {
        let ray_origin = Tuple::point(0, 0, -5);
        let wall_z = 10.0;
        let wall_size = 7.0;
        let canvas_pixels = 100;
        let pixel_size = wall_size / canvas_pixels as Num;
        let half = wall_size / 2.0;

        let mut canvas = Canvas::with_dimesnions(canvas_pixels, canvas_pixels);
        let color = Pixel::red();
        let shape = Shape::sphere();

        for y in 0..canvas_pixels {
            let world_y = half - pixel_size * y as Num;

            for x in 0..canvas_pixels {
                let world_x = -half + pixel_size * x as Num;

                let position = Tuple::point(world_x, world_y, wall_z);
                let r = Ray::new(ray_origin, (position - ray_origin).normalize());
                let xs = shape.intersect(r);
                if intersection::hit(xs).is_some() {
                    canvas.set(x, y, color);
                }
            }
        }
    }
}

mod chapter6 {
    use crate::{
        equal,
        img::{Canvas, Pixel},
        intersection,
        light::Light,
        material::Material,
        matrix4x4::Matrix4x4,
        ray::Ray,
        shape::Shape,
        transformation::TransformationBuilder,
        tuple::Tuple,
        Num, PI,
    };

    #[test]
    fn surface_normals() {
        {
            let s = Shape::sphere();
            let n = s.normal_at(Tuple::point(1, 0, 0));
            assert!(n == Tuple::vector(1, 0, 0));
        }

        {
            let s = Shape::sphere();
            let n = s.normal_at(Tuple::point(0, 1, 0));
            assert!(n == Tuple::vector(0, 1, 0));
        }

        {
            let s = Shape::sphere();
            let n = s.normal_at(Tuple::point(0, 0, 1));
            assert!(n == Tuple::vector(0, 0, 1));
        }

        {
            let s = Shape::sphere();
            let n = s.normal_at(Tuple::point(
                3.0_f64.sqrt() / 3.0,
                3.0_f64.sqrt() / 3.0,
                3.0_f64.sqrt() / 3.0,
            ));
            assert!(
                n == Tuple::vector(
                    3.0_f64.sqrt() / 3.0,
                    3.0_f64.sqrt() / 3.0,
                    3.0_f64.sqrt() / 3.0
                )
            );
        }

        {
            let s = Shape::sphere();
            let n = s.normal_at(Tuple::point(
                3.0_f64.sqrt() / 3.0,
                3.0_f64.sqrt() / 3.0,
                3.0_f64.sqrt() / 3.0,
            ));
            assert!(n == n.normalize());
        }

        {
            let mut s = Shape::sphere();
            s.set_transform(Matrix4x4::translation(0, 1, 0));
            let n = s.normal_at(Tuple::point(0, 1.70711, -0.70711));
            assert!(n == Tuple::vector(0.0, 0.70711, -0.70711));
        }

        {
            let mut s = Shape::sphere();
            let m = TransformationBuilder::rotation_z(PI / 5.0)
                .scale(1, 0.5, 1)
                .build();
            s.set_transform(m);
            let n = s.normal_at(Tuple::point(0, 2.0_f64.sqrt() / 2.0, -2.0_f64.sqrt() / 2.0));
            assert!(n == Tuple::vector(0.0, 0.97014, -0.24254));
        }
    }

    #[test]
    fn reflecting_vectors() {
        {
            let v = Tuple::vector(1, -1, 0);
            let n = Tuple::vector(0, 1, 0);
            let r = v.reflect(n);
            assert!(r == Tuple::vector(1, 1, 0));
        }

        {
            let v = Tuple::vector(0, -1, 0);
            let n = Tuple::vector(2_f64.sqrt() / 2.0, 2_f64.sqrt() / 2.0, 0);
            let r = v.reflect(n);
            assert!(r == Tuple::vector(1, 0, 0));
        }
    }

    #[test]
    fn phong_relfection() {
        {
            let intensity = Pixel::white();
            let position = Tuple::point(0, 0, 0);
            let light = Light::point(position, intensity);
            assert!(light.position() == position);
            assert!(light.intensity() == intensity);
        }

        {
            let m = Material::default();
            assert!(m.color() == Pixel::white());
            assert!(equal(m.ambient(), 0.1));
            assert!(equal(m.diffuse(), 0.9));
            assert!(equal(m.specular(), 0.9));
            assert!(equal(m.shininess(), 200.0));
        }

        {
            let s = Shape::sphere();
            let m = s.material();
            assert!(m == Material::default());
        }

        {
            let mut s = Shape::sphere();
            let mut m = s.material();
            m.set_ambient(1);
            s.set_material(m);
            assert!(s.material() == m);
        }

        let m = Material::default();
        let position = Tuple::point(0, 0, 0);

        {
            let eyev = Tuple::vector(0, 0, -1);
            let normalv = Tuple::vector(0, 0, -1);
            let light = Light::point(Tuple::point(0, 0, -10), Pixel::white());
            let result = m.lighting(&Shape::sphere(), &light, position, eyev, normalv, false);
            assert!(result == Pixel::rgb(1.9, 1.9, 1.9));
        }

        {
            let eyev = Tuple::vector(0, 2f64.sqrt() / 2.0, -2f64.sqrt() / 2.0);
            let normalv = Tuple::vector(0, 0, -1);
            let light = Light::point(Tuple::point(0, 0, -10), Pixel::white());
            let result = m.lighting(&Shape::sphere(), &light, position, eyev, normalv, false);
            assert!(result == Pixel::rgb(1.0, 1.0, 1.0));
        }

        {
            let eyev = Tuple::vector(0, 0, -1);
            let normalv = Tuple::vector(0, 0, -1);
            let light = Light::point(Tuple::point(0, 10, -10), Pixel::white());
            let result = m.lighting(&Shape::sphere(), &light, position, eyev, normalv, false);
            assert!(result == Pixel::rgb(0.7364, 0.7364, 0.7364));
        }

        {
            let eyev = Tuple::vector(0, -2.0f64.sqrt() / 2.0, -2.0f64.sqrt() / 2.0);
            let normalv = Tuple::vector(0, 0, -1);
            let light = Light::point(Tuple::point(0, 10, -10), Pixel::white());
            let result = m.lighting(&Shape::sphere(), &light, position, eyev, normalv, false);
            assert!(result == Pixel::rgb(1.6364, 1.6364, 1.6364));
        }

        {
            let eyev = Tuple::vector(0, 0, -1);
            let normalv = Tuple::vector(0, 0, -1);
            let light = Light::point(Tuple::point(0, 0, 10), Pixel::white());
            let result = m.lighting(&Shape::sphere(), &light, position, eyev, normalv, false);
            assert!(result == Pixel::rgb(0.1, 0.1, 0.1));
        }
    }

    #[test]
    fn putting_it_together() {
        let ray_origin = Tuple::point(0, 0, -5);
        let wall_z = 10.0;
        let wall_size = 7.0;
        let canvas_pixels = 100;
        let pixel_size = wall_size / canvas_pixels as Num;
        let half = wall_size / 2.0;

        let mut canvas = Canvas::with_dimesnions(canvas_pixels, canvas_pixels);
        let mut shape = Shape::sphere();
        let mut m = Material::default();
        m.set_color(Pixel::rgb(1.0, 0.2, 1.0));
        shape.set_material(m);

        let light = Light::point(Tuple::point(-10, 10, -10), Pixel::white());

        for y in 0..canvas_pixels {
            let world_y = half - pixel_size * y as Num;

            for x in 0..canvas_pixels {
                let world_x = -half + pixel_size * x as Num;

                let position = Tuple::point(world_x, world_y, wall_z);
                let r = Ray::new(ray_origin, (position - ray_origin).normalize());
                let xs = shape.intersect(r);
                if let Some(hit) = intersection::hit(xs) {
                    let point = r.position(hit.t());
                    let normal = hit.object().normal_at(point);
                    let eye = -r.direction();

                    let color = hit
                        .object()
                        .material()
                        .lighting(&hit.object(), &light, point, eye, normal, false);

                    canvas.set(x, y, color);
                }
            }
        }
    }
}

mod chapter7 {
    use crate::{
        equal, img::Pixel, intersection::Intersection, light::Light, material::Material,
        matrix4x4::Matrix4x4, ray::Ray, shape::Shape, tuple::Tuple, world::World, Camera, Canvas,
        TransformationBuilder, PI,
    };

    #[test]
    fn building_a_world() {
        {
            let w = World::new();
            assert!(w.objects().len() == 0);
            assert!(w.light().is_none());
        }

        {
            let light = Light::point(Tuple::point(-10, 10, -10), Pixel::white());
            let mut s1 = Shape::sphere();
            let mut s1_m = Material::default();
            s1_m.set_color(Pixel::rgb(0.8, 1.0, 0.6));
            s1_m.set_diffuse(0.7);
            s1_m.set_specular(0.2);
            s1.set_material(s1_m);
            let mut s2 = Shape::sphere();
            s2.set_transform(Matrix4x4::scaling(0.5, 0.5, 0.5));

            let w = World::default();
            assert!(w.light().unwrap() == light);
            assert!(w.objects().contains(&s1));
            assert!(w.objects().contains(&s2));
        }

        {
            let w = World::default();
            let r = Ray::new(Tuple::point(0, 0, -5), Tuple::vector(0, 0, 1));
            let xs = w.intersect_world(r);
            assert!(xs.len() == 4);
            assert!(equal(xs[0].t(), 4.0));
            assert!(equal(xs[1].t(), 4.5));
            assert!(equal(xs[2].t(), 5.5));
            assert!(equal(xs[3].t(), 6.0));
        }

        {
            let r = Ray::new(Tuple::point(0, 0, -5), Tuple::vector(0, 0, 1));
            let shape = Shape::sphere();
            let i = Intersection::new(4, shape);
            let comps = i.prepare_computations(r, vec![i]);
            assert!(equal(comps.t(), i.t()));
            assert!(comps.object() == i.object());
            assert!(comps.point() == Tuple::point(0, 0, -1));
            assert!(comps.eyev() == Tuple::vector(0, 0, -1));
            assert!(comps.normalv() == Tuple::vector(0, 0, -1));
        }

        {
            let r = Ray::new(Tuple::point(0, 0, -5), Tuple::vector(0, 0, 1));
            let shape = Shape::sphere();
            let i = Intersection::new(4, shape);
            let comps = i.prepare_computations(r, vec![i]);
            assert!(comps.inside() == false);
        }

        {
            let r = Ray::new(Tuple::point(0, 0, 0), Tuple::vector(0, 0, 1));
            let shape = Shape::sphere();
            let i = Intersection::new(1, shape);
            let comps = i.prepare_computations(r, vec![i]);

            assert!(comps.point() == Tuple::point(0, 0, 1));
            assert!(comps.eyev() == Tuple::vector(0, 0, -1));
            assert!(comps.inside() == true);
        }

        {
            let w = World::default();
            let r = Ray::new(Tuple::point(0, 0, -5), Tuple::vector(0, 0, 1));
            let shape = w.objects()[0];
            let i = Intersection::new(4, shape);
            let comps = i.prepare_computations(r, vec![i]);
            let c = w.shade_hit(comps, 0);
            assert!(c == Pixel::rgb(0.38066, 0.47583, 0.2855));
        }

        {
            let mut w = World::default();
            w.set_light(Light::point(Tuple::point(0, 0.25, 0), Pixel::white()));
            let r = Ray::new(Tuple::point(0, 0, 0), Tuple::vector(0, 0, 1));
            let shape = w.objects()[1];
            let i = Intersection::new(0.5, shape);
            let comps = i.prepare_computations(r, vec![i]);
            let c = w.shade_hit(comps, 0);
            assert!(c == Pixel::rgb(0.90498, 0.90498, 0.90498));
        }

        {
            let w = World::default();
            let r = Ray::new(Tuple::point(0, 0, -5), Tuple::vector(0, 1, 0));
            let c = w.color_at(r, 0);
            assert!(c == Pixel::black());
        }

        {
            let w = World::default();
            let r = Ray::new(Tuple::point(0, 0, -5), Tuple::vector(0, 0, 1));
            let c = w.color_at(r, 0);
            assert!(c == Pixel::rgb(0.38066, 0.47583, 0.2855));
        }

        {
            let mut w = World::default();
            let mut objs = w.objects();
            let mut s1_m = objs[0].material();
            s1_m.set_ambient(1);
            objs[0].set_material(s1_m);
            let mut s2_m = objs[1].material();
            s2_m.set_ambient(1);
            objs[1].set_material(s2_m);
            w.set_objects(objs.clone());

            let r = Ray::new(Tuple::point(0, 0, 0.75), Tuple::vector(0, 0, -1));
            let c = w.color_at(r, 0);
            assert!(c == objs[1].material().color());
        }
    }

    #[test]
    fn defining_a_view_transform() {
        {
            let from = Tuple::point(0, 0, 0);
            let to = Tuple::point(0, 0, -1);
            let up = Tuple::vector(0, 1, 0);
            let t = Matrix4x4::view(from, to, up);
            assert!(t == Matrix4x4::identity());
        }

        {
            let from = Tuple::point(0, 0, 0);
            let to = Tuple::point(0, 0, 1);
            let up = Tuple::vector(0, 1, 0);
            let t = Matrix4x4::view(from, to, up);
            assert!(t == Matrix4x4::scaling(-1, 1, -1));
        }

        {
            let from = Tuple::point(0, 0, 8);
            let to = Tuple::point(0, 0, 0);
            let up = Tuple::vector(0, 1, 0);
            let t = Matrix4x4::view(from, to, up);
            assert!(t == Matrix4x4::translation(0, 0, -8));
        }

        {
            let from = Tuple::point(1, 3, 2);
            let to = Tuple::point(4, -2, 8);
            let up = Tuple::vector(1, 1, 0);
            let t = Matrix4x4::view(from, to, up);
            assert!(
                t == Matrix4x4::new([
                    -0.50709, 0.50709, 0.67612, -2.36643, 0.76772, 0.60609, 0.12122, -2.82843,
                    -0.35857, 0.59761, -0.71714, 0.00000, 0.00000, 0.00000, 0.00000, 1.00000,
                ])
            );
        }
    }

    #[test]
    fn implementing_a_camera() {
        {
            let hsize = 160;
            let vsize = 120;
            let field_of_view = PI / 2.0;
            let c = Camera::new(hsize, vsize, field_of_view);
            assert!(c.hsize() == 160);
            assert!(c.vsize() == 120);
            assert!(equal(c.field_of_view(), PI / 2.0));
            assert!(c.transform() == Matrix4x4::identity());
        }

        {
            let c = Camera::new(200, 125, PI / 2.0);
            assert!(equal(c.pixel_size(), 0.01));
        }

        {
            let c = Camera::new(125, 200, PI / 2.0);
            assert!(equal(c.pixel_size(), 0.01));
        }

        {
            let c = Camera::new(201, 101, PI / 2.0);
            let r = c.ray_for_pixel(100, 50);
            assert!(r.origin() == Tuple::point(0, 0, 0));
            assert!(r.direction() == Tuple::vector(0, 0, -1));
        }

        {
            let c = Camera::new(201, 101, PI / 2.0);
            let r = c.ray_for_pixel(0, 0);
            assert!(r.origin() == Tuple::point(0, 0, 0));
            assert!(r.direction() == Tuple::vector(0.66519, 0.33259, -0.66851));
        }

        {
            let c = Camera::with_transform(
                201,
                101,
                PI / 2.0,
                TransformationBuilder::translation(0, -2, 5)
                    .rotate_y(PI / 4.0)
                    .build(),
            );
            let r = c.ray_for_pixel(100, 50);
            assert!(r.origin() == Tuple::point(0, 2, -5));
            assert!(r.direction() == Tuple::vector(2f64.sqrt() / 2.0, 0.0, -2f64.sqrt() / 2.0));
        }

        {
            let w = World::default();
            let c = Camera::with_transform(
                11,
                11,
                PI / 2.0,
                Matrix4x4::view(
                    Tuple::point(0, 0, -5),
                    Tuple::point(0, 0, 0),
                    Tuple::vector(0, 1, 0),
                ),
            );
            let image: Canvas = c.render(w);
            assert!(image.get(5, 5) == Pixel::rgb(0.38066, 0.47583, 0.2855));
        }
    }

    #[test]
    fn putting_it_together() {
        let mut floor = Shape::sphere();
        floor.set_transform(Matrix4x4::scaling(10, 0.01, 10));
        let mut floor_mat = Material::default();
        floor_mat.set_color(Pixel::rgb(1.0, 0.9, 0.9));
        floor_mat.set_specular(0);
        floor.set_material(floor_mat);

        let mut left_wall = Shape::sphere();
        left_wall.set_transform(
            Matrix4x4::translation(0, 0, 5)
                * Matrix4x4::rotation_y(-PI / 4.0)
                * Matrix4x4::rotation_x(PI / 2.0)
                * Matrix4x4::scaling(10, 0.01, 10),
        );
        left_wall.set_material(floor_mat);

        let mut right_wall = Shape::sphere();
        right_wall.set_transform(
            Matrix4x4::translation(0, 0, 5)
                * Matrix4x4::rotation_y(PI / 4.0)
                * Matrix4x4::rotation_x(PI / 2.0)
                * Matrix4x4::scaling(10, 0.01, 10),
        );
        right_wall.set_material(floor_mat);

        let mut middle = Shape::sphere();
        middle.set_transform(Matrix4x4::translation(-0.5, 1, 0.5));
        let mut middle_mat = Material::default();
        middle_mat.set_color(Pixel::rgb(0.1, 1.0, 0.5));
        middle_mat.set_diffuse(0.7);
        middle_mat.set_specular(0.3);
        middle.set_material(middle_mat);

        let mut right = Shape::sphere();
        right.set_transform(
            Matrix4x4::translation(1.5, 0.5, -0.5) * Matrix4x4::scaling(0.5, 0.5, 0.5),
        );
        let mut right_mat = Material::default();
        right_mat.set_color(Pixel::rgb(0.5, 1.0, 0.1));
        right_mat.set_diffuse(0.7);
        right_mat.set_specular(0.3);
        right.set_material(right_mat);

        let mut left = Shape::sphere();
        left.set_transform(
            Matrix4x4::translation(-1.5, 0.33, -0.75) * Matrix4x4::scaling(0.33, 0.33, 0.33),
        );
        let mut left_mat = Material::default();
        left_mat.set_color(Pixel::rgb(1.0, 0.8, 0.1));
        left_mat.set_diffuse(0.7);
        left_mat.set_specular(0.3);
        left.set_material(left_mat);

        let mut world = World::default();
        world.set_light(Light::point(Tuple::point(-10, 10, -10), Pixel::white()));
        world.set_objects(vec![floor, left_wall, right_wall, left, right, middle]);

        let camera = Camera::with_transform(
            100,
            50,
            PI / 2.0,
            Matrix4x4::view(
                Tuple::point(0, 1.5, -5),
                Tuple::point(0, 1, 0),
                Tuple::vector(0, 1, 0),
            ),
        );
        let _canvas = camera.render(world);
    }
}

mod chapter8 {
    use crate::{
        Intersection, Light, Material, Matrix4x4, Pixel, Ray, Shape, Tuple, World, EPSILON,
    };

    #[test]
    fn lighting_in_shadows() {
        let m = Material::default();
        let position = Tuple::point(0, 0, 0);

        {
            let eyev = Tuple::vector(0, 0, -1);
            let normalv = Tuple::vector(0, 0, -1);

            let light = Light::point(Tuple::point(0, 0, -10), Pixel::white());
            let in_shadow = true;
            let result = m.lighting(&Shape::sphere(), &light, position, eyev, normalv, in_shadow);
            assert!(result == Pixel::rgb(0.1, 0.1, 0.1));
        }
    }

    #[test]
    fn testing_for_shadows() {
        {
            let w = World::default();
            let p = Tuple::point(0, 10, 0);
            assert!(!w.is_shadowed(p))
        }

        {
            let w = World::default();
            let p = Tuple::point(10, -10, 10);
            assert!(w.is_shadowed(p));
        }

        {
            let w = World::default();
            let p = Tuple::point(-20, 20, -20);
            assert!(!w.is_shadowed(p));
        }

        {
            let w = World::default();
            let p = Tuple::point(-2, 2, -2);
            assert!(!w.is_shadowed(p));
        }
    }

    #[test]
    fn rendering_shadows() {
        {
            let mut w = World::new();
            w.set_light(Light::point(Tuple::point(0, 0, -10), Pixel::white()));
            let s1 = Shape::sphere();
            let mut s2 = Shape::sphere();
            s2.set_transform(Matrix4x4::translation(0, 0, 10));
            w.set_objects(vec![s1, s2]);
            let r = Ray::new(Tuple::point(0, 0, 5), Tuple::vector(0, 0, 1));
            let i = Intersection::new(4, s2);
            let comps = i.prepare_computations(r, vec![i]);
            let c = w.shade_hit(comps, 0);
            assert!(c == Pixel::rgb(0.1, 0.1, 0.1));
        }

        {
            let r = Ray::new(Tuple::point(0, 0, -5), Tuple::vector(0, 0, 1));
            let mut shape = Shape::sphere();
            shape.set_transform(Matrix4x4::translation(0, 0, 1));

            let i = Intersection::new(5, shape);
            let comps = i.prepare_computations(r, vec![i]);
            assert!(comps.over_point().get_z() < -EPSILON / 2.0);
            assert!(comps.point().get_z() > comps.over_point().get_z());
        }
    }
}

mod chapter9 {
    use crate::{equal, shape, Material, Matrix4x4, Ray, Shape, Tuple, PI};

    #[test]
    fn refactoring_shapes() {
        {
            let s = Shape::test_shape();
            assert!(s.transform() == Matrix4x4::identity());
        }

        {
            let mut s = Shape::test_shape();
            s.set_transform(Matrix4x4::translation(2, 3, 4));
            assert!(s.transform() == Matrix4x4::translation(2, 3, 4));
        }

        {
            let s = Shape::test_shape();
            let m = s.material();
            assert!(m == Material::default());
        }

        {
            let mut s = Shape::test_shape();
            let mut m = Material::default();
            m.set_ambient(1);
            s.set_material(m);
            assert!(s.material() == m);
        }

        {
            let r = Ray::new(Tuple::point(0, 0, -5), Tuple::vector(0, 0, 1));
            let mut s = Shape::test_shape();
            s.set_transform(Matrix4x4::scaling(2, 2, 2));
            let _xs = s.intersect(r);
            unsafe {
                assert!(shape::SAVED_RAY.unwrap().origin() == Tuple::point(0, 0, -2.5));
                assert!(shape::SAVED_RAY.unwrap().direction() == Tuple::vector(0, 0, 0.5));
            }
        }

        {
            let r = Ray::new(Tuple::point(0, 0, -5), Tuple::vector(0, 0, 1));
            let mut s = Shape::test_shape();
            s.set_transform(Matrix4x4::translation(5, 0, 0));
            let _xs = s.intersect(r);
            unsafe {
                assert!(shape::SAVED_RAY.unwrap().origin() == Tuple::point(-5, 0, -5));
                assert!(shape::SAVED_RAY.unwrap().direction() == Tuple::vector(0, 0, 1));
            }
        }

        {
            let mut s = Shape::test_shape();
            s.set_transform(Matrix4x4::translation(0, 1, 0));
            let n = s.normal_at(Tuple::point(0, 1.70711, -0.70711));
            assert!(n == Tuple::vector(0, 0.70711, -0.70711));
        }

        {
            let mut s = Shape::test_shape();
            let m = Matrix4x4::scaling(1, 0.5, 1) * Matrix4x4::rotation_z(PI / 5.0);
            s.set_transform(m);
            let n = s.normal_at(Tuple::point(0, 2f64.sqrt() / 2.0, -2f64.sqrt() / 2.0));
            assert!(n == Tuple::vector(0, 0.97014, -0.24254));
        }
    }

    #[test]
    fn implementing_a_plane() {
        {
            let p = Shape::plane();
            let n1 = p.normal_at(Tuple::point(0, 0, 0));
            let n2 = p.normal_at(Tuple::point(10, 0, -10));
            let n3 = p.normal_at(Tuple::point(-5, 0, 150));
            assert!(n1 == Tuple::vector(0, 1, 0));
            assert!(n2 == Tuple::vector(0, 1, 0));
            assert!(n3 == Tuple::vector(0, 1, 0));
        }

        {
            let p = Shape::plane();
            let r = Ray::new(Tuple::point(0, 10, 0), Tuple::vector(0, 0, 1));
            let xs = p.intersect(r);
            assert!(xs.len() == 0);
        }

        {
            let p = Shape::plane();
            let r = Ray::new(Tuple::point(0, 0, 0), Tuple::vector(0, 0, 1));
            let xs = p.intersect(r);
            assert!(xs.len() == 0);
        }

        {
            let p = Shape::plane();
            let r = Ray::new(Tuple::point(0, 1, 0), Tuple::vector(0, -1, 0));
            let xs = p.intersect(r);
            assert!(xs.len() == 1);
            assert!(equal(xs[0].t(), 1.0));
            assert!(xs[0].object() == p);
        }

        {
            let p = Shape::plane();
            let r = Ray::new(Tuple::point(0, -1, 0), Tuple::vector(0, 1, 0));
            let xs = p.intersect(r);
            assert!(xs.len() == 1);
            assert!(equal(xs[0].t(), 1.0));
            assert!(xs[0].object() == p);
        }
    }
}

mod chapter10{
    use crate::{Pixel, Tuple, Pattern, Material, Light, Shape, Matrix4x4};


    #[test]
    fn making_a_stripped_pattern(){
        let black = Pixel::black();
        let white = Pixel::white();

        {
            let pattern = Pattern::stripe(white, black);
            assert!(pattern.at(Tuple::point(0, 0, 0)) == white);
            assert!(pattern.at(Tuple::point(0, 1, 0)) == white);
            assert!(pattern.at(Tuple::point(0, 2, 0)) == white);
        }

        {
            let pattern = Pattern::stripe(white, black);
            assert!(pattern.at(Tuple::point(0, 0, 0)) == white);
            assert!(pattern.at(Tuple::point(0, 0, 1)) == white);
            assert!(pattern.at(Tuple::point(0, 0, 2)) == white);
        }

        {
            let pattern = Pattern::stripe(white, black);
            assert!(pattern.at(Tuple::point(0, 0, 0)) == white);
            assert!(pattern.at(Tuple::point(0.9, 0, 0)) == white);
            assert!(pattern.at(Tuple::point(1, 0, 0)) == black);
            assert!(pattern.at(Tuple::point(-0.1, 0, 0)) == black);
            assert!(pattern.at(Tuple::point(-1, 0, 0)) == black);
            assert!(pattern.at(Tuple::point(-1.1, 0, 0)) == white);
        }

        {
            let mut m = Material::default();
            m.set_pattern(Pattern::stripe(white, black));
            m.set_ambient(1);
            m.set_diffuse(0);
            m.set_specular(0);
            let eyev = Tuple::vector(0, 0, -1);
            let normalv = Tuple::vector(0, 0, -1);
            let light = Light::point(Tuple::point(0, 0, -10), white);
            let c1 = m.lighting(&Shape::sphere(), &light, Tuple::point(0.9, 0, -10), eyev, normalv, false);
            let c2 = m.lighting(&Shape::sphere(), &light, Tuple::point(1.1, 0, 0), eyev, normalv, false);
            assert!(c1 == white);
            assert!(c2 == black);
        }
    } 

    #[test]
    fn transforming_patterns(){
        let black = Pixel::black();
        let white = Pixel::white();

        {
            let mut object = Shape::sphere();
            object.set_transform(Matrix4x4::scaling(2, 2, 2));
            let pattern = Pattern::stripe(white, black);
            let c = pattern.at_object(&object, Tuple::point(1.5, 0, 0));
            assert!(c == white);
        }

        {
            let object = Shape::sphere();
            let mut pattern = Pattern::stripe(white, black);
            pattern.set_transform(Matrix4x4::scaling(2, 2, 2));
            let c = pattern.at_object(&object, Tuple::point(1.5, 0, 0));
            assert!(c == white);
        }

        {
            let mut object = Shape::sphere();
            object.set_transform(Matrix4x4::scaling(2, 2, 2));
            let mut pattern = Pattern::stripe(white, black);
            pattern.set_transform(Matrix4x4::translation(0.5, 0, 0));
            let c = pattern.at_object(&object, Tuple::point(2.5, 0, 0));
            assert!(c == white);
        }
    }

    #[test]
    fn making_a_gradient_pattern(){
        let black = Pixel::black();
        let white = Pixel::white();

        let pattern: Pattern = Pattern::gradient(white, black);
        assert!(pattern.at(Tuple::point(0, 0, 0)) == white);
        assert!(pattern.at(Tuple::point(0.25, 0, 0)) == Pixel::rgb(0.75, 0.75, 0.75));
        assert!(pattern.at(Tuple::point(0.5, 0, 0)) == Pixel::rgb(0.5, 0.5, 0.5));
        assert!(pattern.at(Tuple::point(0.75, 0, 0)) == Pixel::rgb(0.25, 0.25, 0.25));
    }

    #[test]
    fn making_a_ring_pattern(){
        let black = Pixel::black();
        let white = Pixel::white();

        let pattern: Pattern = Pattern::ring(white, black);
        assert!(pattern.at(Tuple::point(0, 0, 0)) == white);
        assert!(pattern.at(Tuple::point(1, 0, 0)) == black);
        assert!(pattern.at(Tuple::point(0, 0, 1)) == black);
        assert!(pattern.at(Tuple::point(0.708, 0, 0.708)) == black);
    }

    #[test]
    fn making_a_3d_checker_pattern(){
        let black = Pixel::black();
        let white = Pixel::white();

        {
            let pattern: Pattern = Pattern::checkers(white, black);
            assert!(pattern.at(Tuple::point(0, 0, 0)) == white);
            assert!(pattern.at(Tuple::point(0.99, 0, 0)) == white);
            assert!(pattern.at(Tuple::point(1.01, 0, 0)) == black);
        }

        {
            let pattern: Pattern = Pattern::checkers(white, black);
            assert!(pattern.at(Tuple::point(0, 0, 0)) == white);
            assert!(pattern.at(Tuple::point(0, 0.99, 0)) == white);
            assert!(pattern.at(Tuple::point(0, 1.01, 0)) == black);
        }

        {
            let pattern: Pattern = Pattern::checkers(white, black);
            assert!(pattern.at(Tuple::point(0, 0, 0)) == white);
            assert!(pattern.at(Tuple::point(0, 0, 0.99)) == white);
            assert!(pattern.at(Tuple::point(0, 0, 1.01)) == black);
        }
    }
}

mod chapter11{
    use std::vec;

    use crate::{Material, equal, Shape, Ray, Tuple, Intersection, World, Pixel, Matrix4x4, Light, EPSILON, shape, Pattern, Camera};


    #[test]
    fn reflection(){
        {
            let m = Material::default();
            assert!(equal(m.reflective(), 0.0));
        }

        {
            let shape = Shape::plane();
            let r = Ray::new(Tuple::point(0, 1, -1), Tuple::vector(0, -2f64.sqrt()/2.0, 2f64.sqrt()/2.0));
            let i = Intersection::new(2f64.sqrt(), shape);
            let comps = i.prepare_computations(r, vec![i]);
            assert!(comps.reflectv() == Tuple::vector(0, 2f64.sqrt()/2.0, 2f64.sqrt()/2.0));
        }

        {
            let w = World::default();
            let r = Ray::new(Tuple::point(0, 0, 0), Tuple::vector(0, 0, 1));
            let mut shape = w.objects()[1];
            let mut shape_mat = shape.material();
            shape_mat.set_ambient(1);
            shape.set_material(shape_mat);
            let i = Intersection::new(1, shape);
            let comps = i.prepare_computations(r, vec![i]);
            let color = w.reflected_color(comps, 5);
            assert!(color == Pixel::black());
        }

        {
            let mut w = World::default();
            let mut objs = w.objects();
            let mut shape = Shape::plane();
            let mut shape_mat = shape.material();
            shape_mat.set_reflective(0.5);
            shape.set_material(shape_mat);
            shape.set_transform(Matrix4x4::translation(0, -1, 0));
            objs.push(shape);
            w.set_objects(objs);

            let r = Ray::new(Tuple::point(0, 0, -3), Tuple::vector(0, -2f64.sqrt()/2.0, 2f64.sqrt()/2.0));
            let i = Intersection::new(2f64.sqrt(), shape);
            let comps = i.prepare_computations(r, vec![i]);
            let color = w.reflected_color(comps, 5);
            assert!(color == Pixel::rgb(0.19032, 0.2379, 0.14274));
        }

        {
            let mut w = World::default();
            let mut objs = w.objects();
            let mut shape = Shape::plane();
            let mut shape_mat = shape.material();
            shape_mat.set_reflective(0.5);
            shape.set_material(shape_mat);
            shape.set_transform(Matrix4x4::translation(0, -1, 0));
            objs.push(shape);
            w.set_objects(objs);

            let r = Ray::new(Tuple::point(0, 0, -3), Tuple::vector(0, -2f64.sqrt()/2.0, 2f64.sqrt()/2.0));
            let i = Intersection::new(2f64.sqrt(), shape);
            let comps = i.prepare_computations(r, vec![i]);
            let color = w.shade_hit(comps, 5);
            assert!(color == Pixel::rgb(0.87677, 0.92436, 0.82918));
        }

        {
            let mut w = World::new();
            w.set_light(Light::point(Tuple::point(0, 0, 0), Pixel::white()));
            let mut lower = Shape::plane();
            let mut shape_mat = lower.material();
            shape_mat.set_reflective(1);
            lower.set_material(shape_mat);
            lower.set_transform(Matrix4x4::translation(0, -1, 0));
            let mut upper = Shape::plane();
            upper.set_material(shape_mat);
            upper.set_transform(Matrix4x4::translation(0, 1, 0));
            w.set_objects(vec![upper, lower]);

            let r = Ray::new(Tuple::point(0, 0, 0), Tuple::vector(0, 1, 0));
            w.color_at(r, 5);
        }

        {
            let mut w = World::default();
            let mut objs = w.objects();
            let mut shape = Shape::plane();
            let mut shape_mat = shape.material();
            shape_mat.set_reflective(0.5);
            shape.set_material(shape_mat);
            shape.set_transform(Matrix4x4::translation(0, -1, 0));
            objs.push(shape);
            w.set_objects(objs);

            let r = Ray::new(Tuple::point(0, 0, -3), Tuple::vector(0, -2f64.sqrt()/2.0, 2f64.sqrt()/2.0));
            let i = Intersection::new(2f64.sqrt(), shape);
            let comps = i.prepare_computations(r, vec![i]);
            let color = w.reflected_color(comps, 0);
            assert!(color == Pixel::black());
        }
    }

    #[test]
    fn transparency_and_refraction(){
        {
            let m = Material::default();
            assert!(equal(m.transparency(), 0.0));
            assert!(equal(m.refractive_index(), 1.0));
        }

        {
            let s = glass_sphere();
            assert!(s.transform() == Matrix4x4::identity());
            assert!(equal(s.material().transparency(), 1.0));
            assert!(equal(s.material().refractive_index(), 1.5));
        }

        {
            let mut a = glass_sphere();
            a.set_transform(Matrix4x4::scaling(2, 2, 2));
            let mut a_mat = a.material();
            a_mat.set_refractive_index(1.5);
            a.set_material(a_mat);

            let mut b = glass_sphere();
            b.set_transform(Matrix4x4::translation(0, 0, -0.25));
            let mut b_mat = b.material();
            b_mat.set_refractive_index(2);
            b.set_material(b_mat);

            let mut c = glass_sphere();
            c.set_transform(Matrix4x4::translation(0, 0, 0.25));
            let mut c_mat = c.material();
            c_mat.set_refractive_index(2.5);
            c.set_material(c_mat);

            let r = Ray::new(Tuple::point(0, 0, -4), Tuple::vector(0, 0, 1));
            let xs = vec![Intersection::new(2, a), Intersection::new(2.75, b), Intersection::new(3.25, c), Intersection::new(4.75, b), Intersection::new(5.25, c), Intersection::new(6, a)];

            let result = vec![(1.0, 1.5),(1.5, 2.0), (2.0, 2.5), (2.5, 2.5), (2.5, 1.5), (1.5, 1.0)];

            let mut i = 0;
            for x in xs.clone() {
                let comps = x.prepare_computations(r, xs.clone());
                assert!(equal(comps.n1(), result[i].0));
                assert!(equal(comps.n2(), result[i].1));

                i += 1;
            }
        }

        {
            let r = Ray::new(Tuple::point(0, 0, -5), Tuple::vector(0, 0, 1));
            let mut shape = glass_sphere();
            shape.set_transform(Matrix4x4::translation(0, 0, 1));
            let i = Intersection::new(5, shape);
            let xs = vec![i];
            let comps = i.prepare_computations(r, xs);
            assert!(comps.under_point().get_z() > EPSILON / 2.0);
            assert!(comps.point().get_z() < comps.under_point().get_z());
        }

        {
            let mut w = World::default();
            let shape = w.objects()[0];

            let r = Ray::new(Tuple::point(0, 0, -5), Tuple::vector(0, 0, 1));
            let xs = vec![Intersection::new(4, shape), Intersection::new(6, shape)];
            let comps = xs[0].clone().prepare_computations(r, xs);
            let c = w.refracted_color(comps, 5);
            assert!(c == Pixel::black());
        }

        {
            let mut w = World::default();
            let mut objs = w.objects();
            let mut mat = objs[0].material();
            mat.set_transparency(1.0);
            mat.set_refractive_index(1.5);
            objs[0].set_material(mat);
            w.set_objects(objs.clone());

            let r = Ray::new(Tuple::point(0, 0, -5), Tuple::vector(0, 0, 1));
            let xs = vec![Intersection::new(4, objs[0]), Intersection::new(6, objs[0])];
            let comps = xs[0].clone().prepare_computations(r, xs);
            let c = w.refracted_color(comps, 0);
            assert!(c == Pixel::black());
        }

        {
            let mut w = World::default();
            let mut objs = w.objects();
            let mut mat = objs[0].material();
            mat.set_transparency(1.0);
            mat.set_refractive_index(1.5);
            objs[0].set_material(mat);
            w.set_objects(objs.clone());

            let r = Ray::new(Tuple::point(0, 0, 2f64.sqrt()/2.0), Tuple::vector(0, 1, 0));
            let xs = vec![Intersection::new(-2f64.sqrt()/2.0, objs[0]), Intersection::new(2f64.sqrt()/2.0, objs[0])];

            let comps = xs[1].clone().prepare_computations(r, xs);
            let c = w.refracted_color(comps, 5);
            assert!(c == Pixel::black());
        }

        {
            let mut w = World::default();
            let mut objs = w.objects();
            let mut mat_a = objs[0].material();
            mat_a.set_ambient(1.0);
            mat_a.set_pattern(Pattern::test());
            objs[0].set_material(mat_a);

            let mut mat_b = objs[1].material();
            mat_b.set_transparency(1.0);
            mat_b.set_refractive_index(1.5);
            objs[1].set_material(mat_b);

            w.set_objects(objs.clone());

            let r = Ray::new(Tuple::point(0, 0, 0.1), Tuple::vector(0, 1, 0));
            let xs = vec![Intersection::new(-0.9899, objs[0].clone()), Intersection::new(-0.4899, objs[1].clone()), Intersection::new(0.4899, objs[1].clone()), Intersection::new(0.9899, objs[0].clone())];
            let comps = xs[2].clone().prepare_computations(r, xs);
            let c = w.refracted_color(comps, 5);
            assert!(c == Pixel::rgb(0.0, 0.99888, 0.04725));
        }

        {
            let mut w = World::default();
            let mut objs = w.objects();

            let mut floor = Shape::plane();
            floor.set_transform(Matrix4x4::translation(0, -1, 0));
            let mut floor_mat = floor.material();
            floor_mat.set_transparency(0.5);
            floor_mat.set_refractive_index(1.5);
            floor.set_material(floor_mat);
            objs.push(floor);
            
            let mut ball = Shape::sphere();
            let mut ball_mat = ball.material();
            ball_mat.set_color(Pixel::red());
            ball_mat.set_ambient(0.5);
            ball.set_material(ball_mat);
            ball.set_transform(Matrix4x4::translation(0, -3.5, -0.5));
            objs.push(ball);

            w.set_objects(objs);

            let r = Ray::new(Tuple::point(0, 0, -3), Tuple::vector(0, -2f64.sqrt()/2.0, 2f64.sqrt()/2.0));
            let xs = vec![Intersection::new(2f64.sqrt(), floor)];
            let comps = xs[0].clone().prepare_computations(r, xs);
            let color = w.shade_hit(comps, 5);
            assert!(color == Pixel::rgb(0.93642, 0.68642, 0.68642));
        }
    }

    #[test]
    fn frenel_effect(){
        {
            let shape = glass_sphere();
            let r = Ray::new(Tuple::point(0, 0, 2f64.sqrt()/2.0), Tuple::vector(0, 1, 0));
            let xs = vec![Intersection::new(-2f64.sqrt()/2.0, shape), Intersection::new(2f64.sqrt()/2.0, shape)];
            let comps = xs[1].clone().prepare_computations(r, xs);
            let reflectance = comps.schlick();
            assert!(equal(reflectance, 1.0));
        }

        {
            let shape = glass_sphere();
            let r = Ray::new(Tuple::point(0, 0, 0), Tuple::vector(0, 1, 0));
            let xs = vec![Intersection::new(-1, shape), Intersection::new(1, shape)];
            let comps = xs[1].clone().prepare_computations(r, xs);
            let reflectance = comps.schlick();
            assert!(equal(reflectance, 0.04));
        }

        {
            let shape = glass_sphere();
            let r = Ray::new(Tuple::point(0, 0.99, -2), Tuple::vector(0, 0, 1));
            let xs = vec![Intersection::new(1.8589, shape)];
            let comps = xs[0].clone().prepare_computations(r, xs);
            let reflectance = comps.schlick();
            assert!(equal(reflectance, 0.48873));
        }

        {
            let mut w = World::default();
            let r = Ray::new(Tuple::point(0, 0, -3), Tuple::vector(0, -2f64.sqrt()/2.0, 2f64.sqrt()/2.0));

            let mut floor = Shape::plane();
            floor.set_transform(Matrix4x4::translation(0, -1, 0));
            let mut floor_mat = floor.material();
            floor_mat.set_reflective(0.5);
            floor_mat.set_transparency(0.5);
            floor_mat.set_refractive_index(1.5);
            floor.set_material(floor_mat);

            let mut ball = Shape::sphere();
            ball.set_transform(Matrix4x4::translation(0, -3.5, -0.5));
            let mut ball_mat = ball.material();
            ball_mat.set_color(Pixel::red());
            ball_mat.set_ambient(0.5);
            ball.set_material(ball_mat);

            let mut objs = w.objects();
            objs.append(&mut vec![floor, ball]);
            w.set_objects(objs);

            let xs = vec![Intersection::new(2f64.sqrt(), floor)];
            let comps = xs[0].clone().prepare_computations(r, xs);
            let color = w.shade_hit(comps, 5);
            assert!(color == Pixel::rgb(0.93391, 0.69643, 0.69243));
        }
    }

    #[test]
    fn testting_it(){
        let mut w = World::new();
    w.set_light(Light::point(Tuple::point(2, 10, -5), Pixel::rgb(0.9, 0.9, 0.9)));

    let mut wall = Shape::plane();
    wall.set_transform(Matrix4x4::translation(0, 0, 10) * Matrix4x4::rotation_x(1.5708));
    let mut floor_mat = wall.material();
    floor_mat.set_pattern(Pattern::checkers(Pixel::white(), Pixel::black()));
    floor_mat.set_ambient(0.8);
    floor_mat.set_diffuse(0.2);
    floor_mat.set_specular(0);
    wall.set_material(floor_mat);

    let mut glass_sphere = Shape::sphere();
    let mut glass_sphere_mat = glass_sphere.material();
    glass_sphere_mat.set_diffuse(0);
    glass_sphere_mat.set_shininess(300);
    glass_sphere_mat.set_transparency(0.9);
    glass_sphere_mat.set_reflective(0.9);
    glass_sphere_mat.set_refractive_index(1.5);
    glass_sphere_mat.set_ambient(0);
    glass_sphere_mat.set_color(Pixel::white());
    glass_sphere_mat.set_specular(0.9);
    glass_sphere.set_material(glass_sphere_mat);
    // glass_sphere.set_transform(Matrix4x4::scaling(0.5, 0.5, 0.5));

    let mut air_sphere = Shape::sphere();
    air_sphere.set_transform(Matrix4x4::scaling(0.5, 0.5, 0.5));
    let mut air_sphere_mat = air_sphere.material();
    air_sphere_mat.set_color(Pixel::white());
    air_sphere_mat.set_diffuse(0);
    air_sphere_mat.set_shininess(300);
    air_sphere_mat.set_reflective(0.9);
    air_sphere_mat.set_transparency(0.9);
    air_sphere_mat.set_ambient(0);
    air_sphere_mat.set_specular(0.9);
    air_sphere_mat.set_refractive_index(1.0000034);
    air_sphere.set_material(air_sphere_mat);

    w.set_objects(vec![wall,glass_sphere, air_sphere]);

    let camera = Camera::with_transform(300, 300, 0.45, Matrix4x4::view(Tuple::point(0, 0, -5), Tuple::point(0, 0, 0), Tuple::vector(0, 1, 0)));
    camera.render(w);
    }

    fn glass_sphere() -> Shape{
        let mut shape = Shape::sphere();
        let mut mat = shape.material();
        mat.set_transparency(1.0);
        mat.set_refractive_index(1.5);
        shape.set_material(mat);
        shape
    }
}
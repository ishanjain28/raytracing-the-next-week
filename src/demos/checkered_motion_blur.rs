use crate::{
    demos::{Demo, ParallelHit},
    hitable::{
        shapes::{MovingSphere, Sphere},
        BvhNode,
    },
    materials::{Dielectric, Lambertian, Metal},
    texture::{Checker, Solid},
    types::Vec3,
    Camera,
};
use rand::{rngs::SmallRng, Rng, SeedableRng};
use std::sync::Arc;

pub struct CheckeredMotionBlur {}

impl Demo for CheckeredMotionBlur {
    type DemoT = BvhNode<Arc<dyn ParallelHit>>;

    fn name(&self) -> &'static str {
        "checkered_motion_blur"
    }

    fn get_background(&self) -> Vec3 {
        Vec3::new(0.7, 0.8, 1.0)
    }

    fn world(&self) -> Self::DemoT {
        let mut world: Vec<Arc<dyn ParallelHit>> = Vec::with_capacity(500);

        let mut rng = rand::thread_rng();
        let mut rng = SmallRng::from_rng(&mut rng).unwrap();

        world.push(Arc::new(Sphere::new(
            Vec3::new(0.0, -1000.0, 0.0),
            1000.0,
            Lambertian::new(Checker::new(
                Solid::new(Vec3::new(0.2, 0.3, 0.1)),
                Solid::new(Vec3::new(0.9, 0.9, 0.9)),
            )),
        )));

        let radius = 0.2;
        let l = Vec3::new(4.0, 0.2, 0.0);

        for a in -10..10 {
            let a = a as f64;
            for b in -10..10 {
                let b = b as f64;
                let choose_material_probability = rng.gen::<f64>();
                let center = Vec3::new(a + 0.9 * rng.gen::<f64>(), 0.2, b + 0.9 * rng.gen::<f64>());

                if (center - l).length() > 0.9 {
                    if choose_material_probability < 0.8 {
                        // diffuse material
                        world.push(Arc::new(MovingSphere::new(
                            center,
                            center + Vec3::new(0.0, 0.5 * rng.gen::<f64>(), 0.0),
                            0.0,
                            1.0,
                            radius,
                            Lambertian::new(Solid::new(Vec3::new(
                                rng.gen::<f64>() * rng.gen::<f64>(),
                                rng.gen::<f64>() * rng.gen::<f64>(),
                                rng.gen::<f64>() * rng.gen::<f64>(),
                            ))),
                        )));
                    } else if choose_material_probability < 0.95 {
                        // metal material
                        world.push(Arc::new(Sphere::new(
                            center,
                            radius,
                            Metal::with_fuzz(
                                Vec3::new(
                                    (1.0 + rng.gen::<f64>()) * 0.5,
                                    (1.0 + rng.gen::<f64>()) * 0.5,
                                    (1.0 + rng.gen::<f64>()) * 0.5,
                                ),
                                0.5 * rng.gen::<f64>(),
                            ),
                        )));
                    } else {
                        // glass material
                        world.push(Arc::new(Sphere::new(center, radius, Dielectric::new(1.5))));
                    }
                }
            }
        }

        world.push(Arc::new(Sphere::new(
            Vec3::new(0.0, 1.0, 0.0),
            1.0,
            Dielectric::new(1.5),
        )));
        world.push(Arc::new(Sphere::new(
            Vec3::new(-4.0, 1.0, 0.0),
            1.0,
            Lambertian::new(Solid::new(Vec3::new(0.4, 0.2, 0.1))),
        )));
        world.push(Arc::new(Sphere::new(
            Vec3::new(4.0, 1.0, 0.0),
            1.0,
            Metal::with_fuzz(Vec3::new(0.7, 0.6, 0.5), 0.0),
        )));

        BvhNode::new(&mut rng, &mut world, 0.0, 1.0)
    }

    fn camera(&self, aspect_ratio: f64) -> Camera {
        let lookfrom = Vec3::new(13.0, 2.0, 3.0);
        let lookat = Vec3::new(0.0, 0.0, 0.0);
        let aperture = 0.1;
        let focus_distance = 10.0;
        Camera::new(
            lookfrom,
            lookat,
            Vec3::new(0.0, 1.0, 0.0),
            20.0,
            aspect_ratio,
            aperture,
            focus_distance,
            0.0,
            1.0,
        )
    }
}

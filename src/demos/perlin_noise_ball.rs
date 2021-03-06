use std::sync::Arc;

use rand::{prelude::SmallRng, SeedableRng};

use crate::{
    demos::{Demo, ParallelHit},
    hitable::{shapes::Sphere, BvhNode},
    materials::Lambertian,
    texture::PerlinNoise,
    types::Vec3,
    Camera,
};

pub struct PerlinNoiseBall {}

impl Demo for PerlinNoiseBall {
    type DemoT = BvhNode<Arc<dyn ParallelHit>>;

    fn name(&self) -> &'static str {
        "perlin_noise"
    }

    fn get_background(&self) -> Vec3 {
        Vec3::new(0.7, 0.8, 1.0)
    }

    fn world(&self) -> Self::DemoT {
        let mut world: Vec<Arc<dyn ParallelHit>> = Vec::with_capacity(2);

        let mut rng = rand::thread_rng();
        let mut rng = SmallRng::from_rng(&mut rng).unwrap();

        world.push(Arc::new(Sphere::new(
            Vec3::new(0.0, -1000.0, 0.0),
            1000.0,
            Lambertian::new(PerlinNoise::with_scale(&mut rng, 4.0)),
        )));

        world.push(Arc::new(Sphere::new(
            Vec3::new(0.0, 2.0, 0.0),
            2.0,
            Lambertian::new(PerlinNoise::with_scale(&mut rng, 4.0)),
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

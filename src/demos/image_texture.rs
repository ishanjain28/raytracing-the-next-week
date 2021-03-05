use std::sync::Arc;

use rand::{prelude::SmallRng, SeedableRng};

use crate::{
    demos::{Demo, ParallelHit},
    materials::Lambertian,
    shapes::Sphere,
    texture::ImageTexture,
    types::Vec3,
    BvhNode, Camera,
};

pub struct ImageTextureDemo {}

impl Demo for ImageTextureDemo {
    type DemoT = BvhNode<Arc<dyn ParallelHit>>;

    fn name(&self) -> &'static str {
        "image_texture"
    }

    fn world(&self) -> Self::DemoT {
        let mut world: Vec<Arc<dyn ParallelHit>> = Vec::with_capacity(1);

        let mut rng = rand::thread_rng();
        let mut rng = SmallRng::from_rng(&mut rng).unwrap();

        let earth_texture = match ImageTexture::from_filename("assets/earthmap.jpg") {
            Ok(v) => v,
            Err(e) => panic!("error in creating image texture: {}", e),
        };

        world.push(Arc::new(Sphere::new(
            Vec3::new(0.0, 0.0, 0.0),
            2.0,
            Lambertian::new(earth_texture),
        )));

        BvhNode::new(&mut rng, &mut world, 0.0, 1.0)
    }

    fn camera(&self, aspect_ratio: f64) -> Camera {
        let lookfrom = Vec3::new(13.0, 2.0, 3.0);
        let lookat = Vec3::new(0.0, 0.0, 0.0);
        let aperture = 0.1;
        let focus_distance = 12.0;
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

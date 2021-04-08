use std::sync::Arc;

use rand::{prelude::SmallRng, SeedableRng};

use crate::{
    demos::{Demo, ParallelHit},
    hitable::{
        shapes::{Cuboid, RectBuilder},
        Hitable,
    },
    materials::{DiffuseLight, Lambertian, MaterialBuilder},
    texture::Solid,
    types::Vec3,
    BvhNode, Camera,
};

pub struct Instances {}

impl Demo for Instances {
    type DemoT = BvhNode<Arc<dyn ParallelHit>>;

    fn name(&self) -> &'static str {
        "instances"
    }

    fn world(&self) -> Self::DemoT {
        let mut world: Vec<Arc<dyn ParallelHit>> = Vec::with_capacity(8);

        let mut rng = rand::thread_rng();
        let mut rng = SmallRng::from_rng(&mut rng).unwrap();

        let red = Lambertian::new(Solid::new(Vec3::new(0.65, 0.05, 0.05)));
        let white = Lambertian::new(Solid::new(Vec3::splat(0.73)));
        let green = Lambertian::new(Solid::new(Vec3::new(0.12, 0.45, 0.15)));
        let light = DiffuseLight::new(Solid::new(Vec3::splat(15.0)));

        world.push(Arc::new(
            RectBuilder
                .y(0.0..=555.0)
                .z(0.0..=555.0)
                .x(555.0)
                .material(green),
        ));
        world.push(Arc::new(
            RectBuilder
                .y(0.0..=555.0)
                .z(0.0..=555.0)
                .x(0.0)
                .material(red),
        ));
        world.push(Arc::new(
            RectBuilder
                .x(213.0..=343.0)
                .z(227.0..=332.0)
                .y(554.0)
                .material(light),
        ));

        world.push(Arc::new(
            RectBuilder
                .x(0.0..=555.0)
                .z(0.0..=555.0)
                .y(0.0)
                .material(white),
        ));
        world.push(Arc::new(
            RectBuilder
                .x(0.0..=555.0)
                .z(0.0..=555.0)
                .y(555.0)
                .material(white),
        ));
        world.push(Arc::new(
            RectBuilder
                .x(0.0..=555.0)
                .y(0.0..=555.0)
                .z(555.0)
                .material(white),
        ));

        // Add the two boxes
        world.push(Arc::new(
            Cuboid::new(
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(165.0, 330.0, 165.0),
                white,
            )
            .rotate_y(15.0)
            .translate(Vec3::new(265.0, 0.0, 295.0)),
        ));
        world.push(Arc::new(
            Cuboid::new(Vec3::new(0.0, 0.0, 0.0), Vec3::splat(165.0), white)
                .rotate_y(-18.0)
                .translate(Vec3::new(130.0, 0.0, 65.0)),
        ));

        BvhNode::new(&mut rng, &mut world, 0.0, 1.0)
    }

    fn camera(&self, aspect_ratio: f64) -> Camera {
        let lookfrom = Vec3::new(278.0, 278.0, -800.0);
        let lookat = Vec3::new(278.0, 278.0, 0.0);
        let aperture = 0.1;
        let focus_distance = 40.0;
        Camera::new(
            lookfrom,
            lookat,
            Vec3::new(0.0, 1.0, 0.0),
            40.0,
            aspect_ratio,
            aperture,
            focus_distance,
            0.0,
            1.0,
        )
    }
}

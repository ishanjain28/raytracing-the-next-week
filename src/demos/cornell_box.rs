use std::sync::Arc;

use rand::{prelude::SmallRng, Rng, SeedableRng};

use crate::{
    demos::Demo,
    hitable::{
        hitable_list::HitableList,
        shapes::{Cuboid, MovingSphere, RectBuilder, Sphere},
        volume::ConstantMedium,
        Hitable,
    },
    materials::{Dielectric, DiffuseLight, Isotropic, Lambertian, MaterialBuilder, Metal},
    texture::{ImageTexture, PerlinNoise, Solid},
    types::Vec3,
    BvhNode, Camera,
};

pub struct CornellBox {}

impl Demo for CornellBox {
    type DemoT = HitableList;

    fn name(&self) -> &'static str {
        "cornell_box"
    }

    fn world(&self) -> Self::DemoT {
        let mut rng = rand::thread_rng();
        let mut rng = SmallRng::from_rng(&mut rng).unwrap();

        let mut ground_boxes = HitableList { list: Vec::new() };
        let ground = Lambertian::new(Solid::new(Vec3::new(0.48, 0.83, 0.53)));

        for i in 0..20 {
            let i = i as f64;
            for j in 0..20 {
                let j = j as f64;

                let w = 100.0;
                let x0 = -1000.0 + i * w;
                let z0 = -1000.0 + j * w;
                let y0 = 0.0;

                let x1 = x0 + w;
                let y1 = rng.gen_range(1.0..=101.0);
                let z1 = z0 + w;

                ground_boxes.push(Arc::new(Cuboid::new(
                    Vec3::new(x0, y0, z0),
                    Vec3::new(x1, y1, z1),
                    ground.clone(),
                )));
            }
        }

        let mut objects = HitableList { list: Vec::new() };
        objects.push(Arc::new(BvhNode::new(
            &mut rng,
            &mut ground_boxes.list,
            0.0,
            1.0,
        )));

        let light = DiffuseLight::new(Solid::new(Vec3::splat(7.0)));
        objects.push(Arc::new(
            RectBuilder
                .x(123.0..=423.0)
                .z(147.0..=412.0)
                .y(554.0)
                .material(light),
        ));

        let center1 = Vec3::new(400.0, 400.0, 200.0);
        let center2 = center1 + Vec3::new(30.0, 0.0, 0.0);
        objects.push(Arc::new(MovingSphere::new(
            center1,
            center2,
            0.0,
            1.0,
            50.0,
            Lambertian::new(Solid::new(Vec3::new(0.7, 0.3, 0.1))),
        )));

        objects.push(Arc::new(Sphere::new(
            Vec3::new(260.0, 150.0, 45.0),
            50.0,
            Dielectric::new(1.5),
        )));

        objects.push(Arc::new(Sphere::new(
            Vec3::new(0.0, 150.0, 145.0),
            50.0,
            Metal::with_fuzz(Vec3::new(0.8, 0.8, 0.9), 1.0),
        )));

        let boundary = Sphere::new(Vec3::new(360.0, 150.0, 145.0), 70.0, Dielectric::new(1.5));
        objects.push(Arc::new(boundary.clone()));
        objects.push(Arc::new(ConstantMedium::new(
            boundary,
            Isotropic::new(Solid::new(Vec3::new(0.2, 0.4, 0.9))),
            0.2,
        )));

        objects.push(Arc::new(ConstantMedium::new(
            Sphere::new(Vec3::splat(0.0), 5000.0, Dielectric::new(1.5)),
            Isotropic::new(Solid::new(Vec3::splat(1.0))),
            0.0001,
        )));

        let earthmap = ImageTexture::from_filename("assets/earthmap.jpg")
            .expect("error in reading assets/earthmap.jpg");
        objects.push(Arc::new(Sphere::new(
            Vec3::new(400.0, 200.0, 400.0),
            100.0,
            Lambertian::new(earthmap),
        )));

        objects.push(Arc::new(Sphere::new(
            Vec3::new(220.0, 280.0, 300.0),
            80.0,
            Lambertian::new(PerlinNoise::with_scale(&mut rng, 0.1)),
        )));

        let mut boxes2 = HitableList { list: Vec::new() };
        let white = Lambertian::new(Solid::new(Vec3::splat(0.73)));
        for _ in 0..1000 {
            boxes2.push(Arc::new(Sphere::new(
                Vec3::random_in_range(&mut rng, 0.0..=165.0),
                10.0,
                white.clone(),
            )));
        }

        objects.push(Arc::new(
            BvhNode::new(&mut rng, &mut boxes2.list, 0.0, 1.0)
                .rotate_y(15.0)
                .translate(Vec3::new(-100.0, 270.0, 395.0)),
        ));

        objects
    }

    fn camera(&self, aspect_ratio: f64) -> Camera {
        let lookfrom = Vec3::new(478.0, 278.0, -600.0);
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

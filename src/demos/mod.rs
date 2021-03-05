use crate::{
    types::{Ray, Vec3},
    Camera, Hitable, HORIZONTAL_PARTITION, VERTICAL_PARTITION,
};
use rand::{rngs::SmallRng, Rng, SeedableRng};
use rayon::prelude::*;
use std::{
    fmt::{Display, Formatter, Result as FmtResult},
    fs::File,
    io::Write,
    sync::{Arc, Mutex},
};

mod checkered_motion_blur;
mod image_texture;
mod perlin_noise_ball;
mod two_spheres;

pub use checkered_motion_blur::CheckeredMotionBlur;
pub use image_texture::ImageTextureDemo;
pub use perlin_noise_ball::PerlinNoiseBall;
pub use two_spheres::TwoSpheres;

#[derive(Debug)]
pub struct Chunk {
    num: usize,
    x: usize,
    y: usize,
    nx: usize,
    ny: usize,
    start_x: usize,
    start_y: usize,
    buffer: Vec<u8>,
}

impl Display for Chunk {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(
            f,
            "Chunk #{}: Start X = {} Start Y = {} Size X = {} Size = {}",
            self.num, self.start_x, self.start_y, self.nx, self.ny
        )
    }
}

pub trait ParallelHit: Hitable + Send + Sync {}
impl<T: Hitable + Send + Sync> ParallelHit for T {}

pub trait Demo: Send + Sync {
    type DemoT: Hitable + Send + Sync;

    fn name(&self) -> &'static str;

    fn world(&self) -> Self::DemoT;

    fn camera(&self, aspect_ratio: f64) -> Camera;

    fn render_chunk(&self, chunk: &mut Chunk, camera: &Camera, world: &Self::DemoT, samples: u8) {
        let &mut Chunk {
            num: _,
            x,
            y,
            nx,
            ny,
            start_x,
            start_y,
            ref mut buffer,
        } = chunk;
        let mut offset = 0;
        let mut rng = rand::thread_rng();
        let mut rng = SmallRng::from_rng(&mut rng).unwrap();

        assert!(buffer.len() >= nx * ny * 4);

        (start_y..start_y + ny).for_each(|j| {
            (start_x..start_x + nx).for_each(|i| {
                let mut color = Vec3::new(0.0, 0.0, 0.0);
                for _s in 0..samples {
                    let u = (i as f64 + rng.gen::<f64>()) / x as f64;
                    let v = (j as f64 + rng.gen::<f64>()) / y as f64;

                    let ray = camera.get_ray(u, v, &mut rng);
                    color += calc_color(ray, world, 0, &mut rng);
                }

                color /= samples as f64;
                self.update_rgb(buffer, color, offset);
                offset += 4;
            });
        });
    }

    fn render(&self, buf: &mut Vec<u8>, x: usize, y: usize, samples: u8) {
        let world = self.world();
        let delta_x = x / VERTICAL_PARTITION;
        let delta_y = y / HORIZONTAL_PARTITION;
        let remx = x % VERTICAL_PARTITION;
        let remy = y % HORIZONTAL_PARTITION;

        // There can be tiny error here if the canvas height/width is not perfectly divisible
        // by vertical/horizontal partitions in the chunks around the edges
        // but umm, i'll just ignore those for now.
        let camera = self.camera(delta_x as f64 / delta_y as f64);
        let buf = Arc::new(Mutex::new(buf));

        (0..VERTICAL_PARTITION).into_par_iter().for_each(|j| {
            let buf = buf.clone();
            (0..HORIZONTAL_PARTITION).into_par_iter().for_each(|i| {
                let mut nx = delta_x;
                let mut ny = delta_y;
                let start_y = j * ny;
                let start_x = i * nx;

                match (i + 1, j + 1) {
                    (HORIZONTAL_PARTITION, VERTICAL_PARTITION) => {
                        nx += remx;
                        ny += remy;
                    }
                    (HORIZONTAL_PARTITION, _) => nx += remx,
                    (_, VERTICAL_PARTITION) => ny += remy,
                    _ => (),
                };

                let mut chunk = Chunk {
                    num: j * HORIZONTAL_PARTITION + i,
                    x,
                    y,
                    nx,
                    ny,
                    start_x,
                    start_y,
                    buffer: vec![0; nx * ny * 4],
                };

                println!("{}", chunk);
                self.render_chunk(&mut chunk, &camera, &world, samples);

                let mut buf = buf.lock().unwrap();
                let mut temp_offset = 0;
                for j in start_y..start_y + ny {
                    let real_offset = ((y - j - 1) * x + start_x) * 4;

                    buf[real_offset..real_offset + nx * 4]
                        .copy_from_slice(&chunk.buffer[temp_offset..temp_offset + nx * 4]);

                    temp_offset += nx * 4;
                }
                println!("Rendered {}", chunk);
            });
        });
    }

    #[inline]
    fn update_rgb(&self, buffer: &mut [u8], color: Vec3, offset: usize) {
        if let Some(pos) = buffer.get_mut(offset) {
            *pos = (255.99 * color.r().sqrt()) as u8;
        }
        if let Some(pos) = buffer.get_mut(offset + 1) {
            *pos = (255.99 * color.g().sqrt()) as u8;
        }
        if let Some(pos) = buffer.get_mut(offset + 2) {
            *pos = (255.99 * color.b().sqrt()) as u8;
        }
    }

    fn save_as_ppm(&self, buf: &[u8], width: usize, height: usize, samples: u8) {
        let header = format!("P3\n{} {}\n255\n", width, height);

        let mut file = match File::create(&format!(
            "{}-{}x{}_{}.ppm",
            self.name(),
            width,
            height,
            samples,
        )) {
            Ok(file) => file,
            Err(e) => panic!("couldn't create {}: {}", self.name(), e),
        };
        file.write_all(header.as_bytes())
            .expect("error in writing file header");

        for i in buf.chunks(4) {
            match file.write_all(format!("{} {} {}\n", i[0], i[1], i[2]).as_bytes()) {
                Ok(_) => (),
                Err(e) => panic!("couldn't write to {}: {}", self.name(), e),
            }
        }
    }
}

fn calc_color<T: Hitable>(ray: Ray, world: &T, depth: u32, rng: &mut SmallRng) -> Vec3 {
    if let Some(hit_rec) = world.hit(&ray, 0.001, std::f64::MAX) {
        if depth >= 50 {
            Vec3::new(0.0, 0.0, 0.0)
        } else {
            let material = hit_rec.material;
            if let (attenuation, Some(scattered_ray)) = material.scatter(&ray, &hit_rec, rng) {
                calc_color(scattered_ray, world, depth + 1, rng) * attenuation
            } else {
                Vec3::new(0.0, 0.0, 0.0)
            }
        }
    } else {
        let unit_direction = ray.direction().unit_vector();
        let t = 0.5 * (unit_direction.y() + 1.0);
        Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
    }
}

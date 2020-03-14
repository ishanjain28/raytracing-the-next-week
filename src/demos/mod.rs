use {
    crate::{
        types::{HitableList, Vec3},
        Camera, HORIZONTAL_PARTITION, VERTICAL_PARTITION,
    },
    rayon::prelude::*,
    std::{
        fs::File,
        io::Write,
        sync::{Arc, Mutex},
    },
};

mod motion_blur;

pub use motion_blur::MotionBlur;

pub struct Chunk {
    x: usize,
    y: usize,
    nx: usize,
    ny: usize,
    start_x: usize,
    start_y: usize,
    buffer: Vec<u8>,
}

pub trait Demo: std::marker::Sync {
    fn render(&self, buf: &mut Vec<u8>, width: usize, height: usize, samples: u8) {
        let nx = width / VERTICAL_PARTITION;
        let ny = height / HORIZONTAL_PARTITION;
        let world = self.world();
        let camera = self.camera(nx as f32 / ny as f32);

        let buf = Arc::new(Mutex::new(buf));

        (0..VERTICAL_PARTITION).into_par_iter().for_each(|j| {
            let buf = buf.clone();
            (0..HORIZONTAL_PARTITION).into_par_iter().for_each(|i| {
                let start_y = j * ny;
                let start_x = i * nx;
                let x = width;
                let y = height;
                let mut chunk = Chunk {
                    x,
                    y,
                    nx,
                    ny,
                    start_x,
                    start_y,
                    buffer: vec![0; nx * ny * 4],
                };
                self.render_chunk(&mut chunk, camera.as_ref(), world.as_ref(), samples);

                let mut buf = buf.lock().unwrap();

                let mut temp_offset = 0;
                for j in start_y..start_y + ny {
                    let real_offset = ((y - j - 1) * x + start_x) * 4;

                    buf[real_offset..real_offset + nx * 4]
                        .copy_from_slice(&chunk.buffer[temp_offset..temp_offset + nx * 4]);

                    temp_offset += nx * 4;
                }
            })
        });
    }

    fn world(&self) -> Option<HitableList> {
        None
    }

    fn camera(&self, aspect_ratio: f32) -> Option<Camera> {
        let lookfrom = Vec3::new(0.0, 0.0, 0.0);
        let lookat = Vec3::new(0.0, 0.0, -1.0);
        Some(Camera::new(
            lookfrom,
            lookat,
            Vec3::new(0.0, 1.0, 0.0),
            90.0,
            aspect_ratio,
            0.0, //aperture
            1.0, // focus_distance
            0.0, // shutter open time
            1.0, // shutter close time
        ))
    }

    fn render_chunk(
        &self,
        chunk: &mut Chunk,
        camera: Option<&Camera>,
        world: Option<&HitableList>,
        samples: u8,
    );

    fn name(&self) -> &'static str;

    fn save_as_ppm(&self, buf: &[u8], width: usize, height: usize) {
        let header = format!("P3\n{} {}\n255\n", width, height);

        let mut file = match File::create(&format!("{}-{}x{}.ppm", self.name(), width, height)) {
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

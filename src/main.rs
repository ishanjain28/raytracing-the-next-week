#![allow(clippy::suspicious_arithmetic_impl)]

mod aabb;
mod camera;
mod demos;
mod hitable;
mod materials;
mod texture;
mod types;

pub use aabb::Aabb;
pub use camera::Camera;
pub use materials::Material;
pub use texture::Texture;
pub use types::{Dimension, X, Y, Z};

use crate::hitable::BvhNode;
use demos::DemoWrapper;

use std::time::Instant;

pub trait Asf64: num_traits::AsPrimitive<f64> {}
impl<T: num_traits::AsPrimitive<f64>> Asf64 for T {}

const NUM_SAMPLES: u16 = 500;
const VERTICAL_PARTITION: usize = 12;
const HORIZONTAL_PARTITION: usize = 12;
const WIDTH: usize = 500;
const HEIGHT: usize = 500;

fn main() -> Result<(), String> {
    run(WIDTH, HEIGHT)
}

#[cfg(feature = "gui")]
fn run(mut width: usize, mut height: usize) -> Result<(), String> {
    use sdl2::{
        event::{Event, WindowEvent},
        keyboard::Keycode,
        pixels::PixelFormatEnum,
    };

    let sdl_ctx = sdl2::init()?;
    let video_subsys = sdl_ctx.video()?;
    let window = video_subsys
        .window("Ray tracing the Next Week", width as u32, height as u32)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut event_pump = sdl_ctx.event_pump()?;

    let mut canvas = window
        .into_canvas()
        .target_texture()
        .build()
        .map_err(|e| e.to_string())?;

    // RGBA framebuffer
    let mut buffer = vec![0; height * width * 4];

    let texture_creator = canvas.texture_creator();
    let mut texture = texture_creator
        .create_texture_static(PixelFormatEnum::BGR888, width as u32, height as u32)
        .map_err(|e| e.to_string())?;

    let mut active_demo = DemoWrapper::HitableList(Box::new(demos::CornellBox {}));
    let mut should_update = true;

    loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => return Ok(()),
                Event::KeyUp { keycode, .. } => {
                    match keycode {
                        Some(Keycode::S) => {
                            active_demo.save_as_ppm(&buffer, width, height, NUM_SAMPLES);
                            should_update = false;
                        }
                        Some(Keycode::Num1) => {
                            active_demo =
                                DemoWrapper::BVHNode(Box::new(demos::CheckeredMotionBlur {}));
                            should_update = true;
                        }
                        Some(Keycode::Num2) => {
                            active_demo = DemoWrapper::BVHNode(Box::new(demos::TwoSpheres {}));
                            should_update = true;
                        }
                        Some(Keycode::Num3) => {
                            active_demo = DemoWrapper::BVHNode(Box::new(demos::PerlinNoiseBall {}));
                            should_update = true;
                        }
                        Some(Keycode::Num4) => {
                            active_demo =
                                DemoWrapper::BVHNode(Box::new(demos::ImageTextureDemo {}));
                            should_update = true;
                        }
                        Some(Keycode::Num5) => {
                            active_demo = DemoWrapper::BVHNode(Box::new(demos::SimpleLight {}));
                            should_update = true;
                        }
                        Some(Keycode::Num6) => {
                            active_demo = DemoWrapper::BVHNode(Box::new(demos::Instances {}));
                            should_update = true;
                        }
                        Some(Keycode::Num7) => {
                            active_demo =
                                DemoWrapper::BVHNode(Box::new(demos::CornellSmokeAndFog {}));
                            should_update = true;
                        }
                        Some(Keycode::Num8) => {
                            active_demo = DemoWrapper::HitableList(Box::new(demos::CornellBox {}));
                            should_update = true;
                        }
                        None => unreachable!(),
                        _ => (),
                    };
                }
                Event::Window {
                    win_event: WindowEvent::Resized(w, h),
                    ..
                } => {
                    width = w as usize;
                    height = h as usize;
                    buffer.resize(width * height * 4, 0);
                    texture = texture_creator
                        .create_texture_static(PixelFormatEnum::BGR888, width as u32, height as u32)
                        .expect("error in resizing texture");
                    should_update = true;
                }
                _ => {}
            };
        }
        if should_update {
            let now = Instant::now();
            active_demo.render(&mut buffer, width, height, NUM_SAMPLES);
            println!(
                "Demo {} Time Taken(s) = {}",
                active_demo.name(),
                now.elapsed().as_secs_f64()
            );
            texture.update(None, &buffer, width * 4).unwrap();
            canvas.copy(&texture, None, None).unwrap();
            canvas.present();
            should_update = false;
        }
    }
}

#[cfg(not(feature = "gui"))]
fn run(width: usize, height: usize) -> Result<(), String> {
    let demos: [DemoWrapper; 8] = [
        DemoWrapper::BVHNode(Box::new(demos::CheckeredMotionBlur {})),
        DemoWrapper::BVHNode(Box::new(demos::TwoSpheres {})),
        DemoWrapper::BVHNode(Box::new(demos::PerlinNoiseBall {})),
        DemoWrapper::BVHNode(Box::new(demos::ImageTextureDemo {})),
        DemoWrapper::BVHNode(Box::new(demos::SimpleLight {})),
        DemoWrapper::BVHNode(Box::new(demos::Instances {})),
        DemoWrapper::BVHNode(Box::new(demos::CornellSmokeAndFog {})),
        DemoWrapper::HitableList(Box::new(demos::CornellBox {})),
    ];

    for demo in demos.iter() {
        run_and_save_demo(demo, width, height)
    }

    Ok(())
}

#[cfg(not(feature = "gui"))]
fn run_and_save_demo(demo: &DemoWrapper, width: usize, height: usize) {
    let mut buffer = vec![0; width * height * 4];

    println!(
        "Starting {} at {}x{} with {} samples",
        demo.name(),
        width,
        height,
        NUM_SAMPLES
    );

    let now = Instant::now();
    demo.render(&mut buffer, width, height, NUM_SAMPLES);
    println!(
        "Rendered Demo {}. Time Taken(s) = {}",
        demo.name(),
        now.elapsed().as_secs_f64()
    );

    demo.save_as_ppm(&buffer, width, height, NUM_SAMPLES);
}

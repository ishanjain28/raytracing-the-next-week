#![allow(clippy::suspicious_arithmetic_impl)]

mod aabb;
mod bvh;
mod camera;
mod demos;
mod hitable;
mod hitable_list;
mod materials;
mod shapes;
mod texture;
mod types;

pub use aabb::Aabb;
pub use bvh::BvhNode;
pub use camera::Camera;
pub use hitable::{HitRecord, Hitable};
pub use hitable_list::HitableList;
pub use materials::Material;
pub use texture::Texture;

use demos::Demo;

use std::time::Instant;

const NUM_SAMPLES: u8 = 100;
const VERTICAL_PARTITION: usize = 12;
const HORIZONTAL_PARTITION: usize = 12;
const WIDTH: usize = 1920;
const HEIGHT: usize = 1080;

fn main() -> Result<(), String> {
    run(WIDTH, HEIGHT)
}

#[cfg(feature = "gui")]
fn run(mut width: usize, mut height: usize) -> Result<(), String> {
    use demos::ParallelHit;
    use sdl2::{
        event::{Event, WindowEvent},
        keyboard::Keycode,
        pixels::PixelFormatEnum,
    };
    use std::sync::Arc;

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

    let mut active_demo: &dyn Demo<DemoT = BvhNode<Arc<dyn ParallelHit>>> =
        &demos::ImageTextureDemo {};
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
                            active_demo = &demos::CheckeredMotionBlur {};
                            should_update = true;
                        }
                        Some(Keycode::Num2) => {
                            active_demo = &demos::TwoSpheres {};
                            should_update = true;
                        }
                        Some(Keycode::Num3) => {
                            active_demo = &demos::PerlinNoiseBall {};
                            should_update = true;
                        }
                        Some(Keycode::Num4) => {
                            active_demo = &demos::ImageTextureDemo {};
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
    run_and_save_demo(demos::CheckeredMotionBlur {}, width, height);

    run_and_save_demo(demos::TwoSpheres {}, width, height);

    run_and_save_demo(demos::PerlinNoiseBall {}, width, height);

    run_and_save_demo(demos::ImageTexture {}, width, height);

    Ok(())
}

#[cfg(not(feature = "gui"))]
fn run_and_save_demo(demo: impl Demo, width: usize, height: usize) {
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

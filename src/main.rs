#![allow(clippy::suspicious_arithmetic_impl)]
#![feature(test)]
extern crate test;

mod camera;
mod demo;
mod types;

pub use camera::Camera;

use std::time::Instant;

const NUM_SAMPLES: u8 = 100;
const VERTICAL_PARTITION: usize = 8;
const HORIZONTAL_PARTITION: usize = 8;
const WIDTH: usize = 800;
const HEIGHT: usize = 800;

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
        .window("Ray tracing in a weekend", width as u32, height as u32)
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

    let active_demo = demo::Demo;

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
                            active_demo.save_as_ppm(&buffer, width, height);
                            should_update = false;
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
    let mut buffer = vec![0; width * height * 4];

    let demo = demo::Demo;

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

    demo.save_as_ppm(&buffer, width, height);

    Ok(())
}

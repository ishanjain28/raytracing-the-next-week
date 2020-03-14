#![allow(clippy::suspicious_arithmetic_impl)]
#![feature(test)]
extern crate test;

mod camera;
mod demos;
mod types;

pub use camera::Camera;

use {
    demos::Demo,
    sdl2::{
        event::{Event, WindowEvent},
        keyboard::Keycode,
        pixels::PixelFormatEnum,
    },
    std::time::Instant,
};

const NUM_SAMPLES: u8 = 100;
const VERTICAL_PARTITION: usize = 8;
const HORIZONTAL_PARTITION: usize = 8;

fn main() -> Result<(), String> {
    let sdl_ctx = sdl2::init()?;
    let video_subsys = sdl_ctx.video()?;
    let (mut width, mut height): (usize, usize) = (2000, 1000);

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

    //println!("{:?} {:?} {:?}", texture.query(), texture.color_mod(), texture.alpha_mod());

    let mut active_demo: &dyn Demo = &demos::MotionBlur;
    // TODO: Should update when window is unfocus since the project window retains
    // data from overlapped window
    // TODO: Maybe consider using condition variable to make loop {} not run at full
    // speed at all times pinning a core at 100%
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
                now.elapsed().as_secs_f32()
            );

            texture.update(None, &buffer, width * 4).unwrap();
            canvas.copy(&texture, None, None).unwrap();
            canvas.present();
            should_update = false;
        }
    }
}

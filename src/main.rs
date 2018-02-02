extern crate enigo;
extern crate failure;
#[macro_use]
extern crate failure_derive;
extern crate sdl2;
#[macro_use]
extern crate serde_derive;

pub mod easel;
pub mod image_trans;

use enigo::Enigo;
use std::env;
use std::time::Duration;
use std::thread;
use std::process;
use sdl2::keyboard::Keycode;
use sdl2::event::Event;
use std::u64;

fn main() {
    thread::spawn(move || {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let _window = video_subsystem
            .window("Passpartout Printer", 1920, 1200)
            .build()
            .unwrap();
        let mut event_pump = sdl_context.event_pump().unwrap();
        loop {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => process::exit(1),
                    _ => (),
                }
            }
        }
    });

    let draw_thread = thread::spawn(move || {
        let picture: String = env::args().nth(1).unwrap();
        let duration: u64 = match env::args().nth(2) {
            Some(v) => v.parse().unwrap(),
            None => 10_u64,
        };
        let wait_time = Duration::from_millis(duration);
        let mut enigo = Enigo::new();
        image_trans::draw_image(
            &picture,
            String::from("coords.json"),
            &mut enigo,
            &wait_time,
        ).unwrap();
    });

    draw_thread.join().unwrap();
}

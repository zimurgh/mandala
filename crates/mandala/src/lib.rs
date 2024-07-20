// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::{
    io::{self, Write},
    net::UdpSocket,
    time::Duration,
};

mod config;
mod error;

pub use config::{ClientConfig, ClientConfigBuilder, ServerConfig, ServerConfigBuilder};
pub use error::MandalaResult;
use sdl2::{event::Event, keyboard::Keycode, pixels::Color};

// Rust follow along of [_Ray Tracing in One Weekend_](https://raytracing.github.io/books/RayTracingInOneWeekend.html)
pub fn run_ray_tracer() {
    let image_width = 256;
    let image_height = 256;

    println!("P3");
    println!("{} {}", image_width, image_height);
    println!("255");

    for j in 0..image_height {
        eprint!("\rScanlines remaining: {} ", image_height - j);
        io::stderr().flush().unwrap();
        for i in 0..image_width {
            let r = i as f64 / (image_width - 1) as f64;
            let g = j as f64 / (image_height - 1) as f64;
            let b = 0.0;

            let ir = (255.999 * r) as i32;
            let ig = (255.999 * g) as i32;
            let ib = (255.999 * b) as i32;

            println!("{} {} {}", ir, ig, ib);
        }
    }

    eprintln!("\rDone.               ");
}

pub fn run_client(mut client: MandalaClient) -> MandalaResult<()> {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("Mandala", 2560, 1440)
        .fullscreen()
        .position_centered()
        .hidden()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    canvas.window_mut().show();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;
    'running: loop {
        i = (i + 1) % 255;
        canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}

pub struct MandalaClient {
    _config: ClientConfig,
    _socket: Option<UdpSocket>,
}

impl MandalaClient {
    pub fn new(_config: ClientConfig) -> MandalaClient {
        MandalaClient {
            _config,
            _socket: None,
        }
    }
}

pub fn run_server(mut server: MandalaServer) -> MandalaResult<()> {
    let socket = UdpSocket::bind(server.config.addr)?;
    socket.set_nonblocking(true)?;
    server.socket = Some(socket);

    Ok(())
}

#[derive(Debug)]
pub struct MandalaServer {
    config: ServerConfig,
    socket: Option<UdpSocket>,
}

impl MandalaServer {
    pub fn new(config: ServerConfig) -> MandalaServer {
        MandalaServer {
            config,
            socket: None,
        }
    }
}

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::net::UdpSocket;

use gpu::Gpu;
use log::debug;
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::WindowId,
};

mod config;
mod error;
mod gpu;

pub use config::{ClientConfig, ClientConfigBuilder, ServerConfig, ServerConfigBuilder};
pub use error::MandalaResult;

pub fn run_client(mut client: MandalaClient) -> MandalaResult<()> {
    let event_loop = EventLoop::new().unwrap();

    event_loop.set_control_flow(ControlFlow::Poll);

    let _ = event_loop.run_app(&mut client);

    Ok(())
}

pub struct MandalaClient {
    _config: ClientConfig,
    gpu: Option<Gpu>,
    _socket: Option<UdpSocket>,
}

impl MandalaClient {
    pub fn new(_config: ClientConfig) -> MandalaClient {
        MandalaClient {
            _config,
            gpu: None,
            _socket: None,
        }
    }
}

impl ApplicationHandler for MandalaClient {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.gpu = Some(Gpu::init(&event_loop).unwrap());
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                debug!("The close button was pressed; stopping");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                /*
                if let Some(gpu) = &self.gpu {
                    gpu.window().request_redraw();
                }
                */
            }
            _ => (),
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

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::{net::UdpSocket, sync::Arc};

use error::GpuResult;
use log::debug;
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::{Window, WindowId},
};

mod error;

pub use error::MandalaResult;

#[derive(Debug, Clone)]
pub struct MandalaClientConfig {}

#[derive(Debug, Default, Clone)]
pub struct MandalaClientConfigBuilder {}

impl MandalaClientConfigBuilder {
    pub fn new() -> MandalaClientConfigBuilder {
        MandalaClientConfigBuilder {}
    }

    pub fn build(&self) -> MandalaResult<MandalaClientConfig> {
        Ok(MandalaClientConfig {})
    }
}

#[derive(Debug, Clone)]
pub struct MandalaServerConfig {}

#[derive(Debug, Default, Clone)]
pub struct MandalaServerConfigBuilder {}

impl MandalaServerConfigBuilder {
    pub fn new() -> MandalaServerConfigBuilder {
        MandalaServerConfigBuilder {}
    }

    pub fn build(&mut self) -> MandalaResult<MandalaServerConfig> {
        Ok(MandalaServerConfig {})
    }
}

pub fn run_client(mut client: MandalaClient) -> MandalaResult<()> {
    let event_loop = EventLoop::new().unwrap();

    event_loop.set_control_flow(ControlFlow::Poll);

    let _ = event_loop.run_app(&mut client);

    Ok(())
}

pub struct MandalaClient {
    config: MandalaClientConfig,
    gpu: Option<Gpu>,
    socket: Option<UdpSocket>,
}

impl MandalaClient {
    pub fn new(config: MandalaClientConfig) -> MandalaClient {
        MandalaClient {
            config,
            gpu: None,
            socket: None,
        }
    }
}

impl ApplicationHandler for MandalaClient {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.gpu = Some(Gpu::init(&event_loop).unwrap());
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                debug!("The close button was pressed; stopping");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                if let Some(gpu) = &self.gpu {
                    gpu.window.request_redraw();
                }
            }
            _ => (),
        }
    }
}

pub fn run_server(mut server: MandalaServer) -> MandalaResult<()> {
    let socket = UdpSocket::bind("0.0.0.0:49474")?;
    socket.set_nonblocking(true)?;
    server.socket = Some(socket);
    Ok(())
}

#[derive(Debug)]
pub struct MandalaServer {
    config: MandalaServerConfig,
    socket: Option<UdpSocket>,
}

impl MandalaServer {
    pub fn new(config: MandalaServerConfig) -> MandalaServer {
        MandalaServer {
            config,
            socket: None,
        }
    }
}

pub struct Gpu {
    window: Arc<Window>,
    size: winit::dpi::PhysicalSize<u32>,
}

impl Gpu {
    pub fn init(event_loop: &ActiveEventLoop) -> GpuResult<Gpu> {
        let window = Arc::new(event_loop.create_window(Window::default_attributes())?);
        let size = window.inner_size();

        Ok(Gpu { window, size })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mandala() {
        assert_eq!(2 + 2, 4);
    }
}

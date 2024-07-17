// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use log::info;
use mandala::{run_server, MandalaResult, MandalaServer, ServerConfigBuilder};
use simplelog::{Config, SimpleLogger};

fn main() -> MandalaResult<()> {
    SimpleLogger::init(log::LevelFilter::Debug, Config::default())?;
    info!("Starting Mandala server...");
    let config = ServerConfigBuilder::new()
        .server_addr(SocketAddr::new(
            IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)),
            49474,
        ))
        .build()?;
    run_server(MandalaServer::new(config))?;
    Ok(())
}

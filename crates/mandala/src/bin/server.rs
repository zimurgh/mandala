// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use log::info;
use mandala::{run_server, MandalaResult, MandalaServer, MandalaServerConfigBuilder};
use simplelog::{Config, SimpleLogger};

fn main() -> MandalaResult<()> {
    SimpleLogger::init(log::LevelFilter::Debug, Config::default())?;
    info!("Starting Mandala server...");
    let config = MandalaServerConfigBuilder::new().build()?;
    run_server(MandalaServer::new(config))?;
    Ok(())
}

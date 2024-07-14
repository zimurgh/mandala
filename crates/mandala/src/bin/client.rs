// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use log::info;
use mandala::{run_client, MandalaClient, MandalaClientConfigBuilder, MandalaResult};
use simplelog::{Config, SimpleLogger};

fn main() -> MandalaResult<()> {
    SimpleLogger::init(log::LevelFilter::Debug, Config::default())?;
    info!("Starting Mandala client...");
    let config = MandalaClientConfigBuilder::new().build()?;
    run_client(MandalaClient::new(config))?;
    Ok(())
}

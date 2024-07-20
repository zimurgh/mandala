// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::io;

pub type MandalaResult<T> = std::result::Result<T, MandalaError>;

#[derive(Debug)]
pub enum MandalaError {
    Io(io::Error),
    SetLoggger(log::SetLoggerError),
    ConfigError(ConfigError),
    Other,
}

impl From<io::Error> for MandalaError {
    fn from(value: io::Error) -> MandalaError {
        MandalaError::Io(value)
    }
}

impl From<log::SetLoggerError> for MandalaError {
    fn from(value: log::SetLoggerError) -> MandalaError {
        MandalaError::SetLoggger(value)
    }
}

impl From<ConfigError> for MandalaError {
    fn from(value: ConfigError) -> MandalaError {
        MandalaError::ConfigError(value)
    }
}

#[derive(Debug)]
pub enum ConfigError {
    MissingServerAddr,
}

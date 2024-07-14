// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::io;

use winit::error::OsError;

pub type MandalaResult<T> = std::result::Result<T, MandalaError>;

pub type GpuResult<T> = std::result::Result<T, GpuError>;

#[derive(Debug)]
pub enum MandalaError {
    Io(io::Error),
    SetLoggger(log::SetLoggerError),
    GpuError(GpuError),
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

#[derive(Debug)]
pub enum GpuError {
    WinitOsError(OsError),
    Other,
}

impl From<OsError> for GpuError {
    fn from(value: OsError) -> GpuError {
        GpuError::WinitOsError(value)
    }
}

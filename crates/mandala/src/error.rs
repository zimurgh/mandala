// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::io;

use ash::{vk::Result, LoadingError};
use winit::{error::OsError, raw_window_handle::HandleError};

pub type MandalaResult<T> = std::result::Result<T, MandalaError>;

pub type GpuResult<T> = std::result::Result<T, GpuError>;

#[derive(Debug)]
pub enum MandalaError {
    Io(io::Error),
    SetLoggger(log::SetLoggerError),
    GpuError(GpuError),
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

#[derive(Debug)]
pub enum GpuError {
    VulkanError(ash::vk::Result),
    AshLoadingError(LoadingError),
    WinitHandleError(HandleError),
    WinitOsError(OsError),
    Other,
}

impl From<LoadingError> for GpuError {
    fn from(value: LoadingError) -> GpuError {
        GpuError::AshLoadingError(value)
    }
}

impl From<OsError> for GpuError {
    fn from(value: OsError) -> GpuError {
        GpuError::WinitOsError(value)
    }
}

impl From<ash::vk::Result> for GpuError {
    fn from(value: ash::vk::Result) -> GpuError {
        GpuError::VulkanError(value)
    }
}

impl From<HandleError> for GpuError {
    fn from(value: HandleError) -> GpuError {
        GpuError::WinitHandleError(value)
    }
}

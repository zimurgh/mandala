use std::{
    ffi::{self, c_char},
    sync::Arc,
};

use ash::{ext::debug_utils, vk, Entry};
use winit::{event_loop::ActiveEventLoop, window::Window};

use crate::error::GpuResult;

pub struct Gpu {
    window: Option<Arc<Window>>,
}

impl Gpu {
    pub fn new() -> Gpu {
        Gpu { window: None }
    }

    pub fn init(event_loop: &ActiveEventLoop) -> GpuResult<Gpu> {
        let window =
            Arc::new(event_loop.create_window(Window::default_attributes().with_visible(false))?);

        let entry = unsafe { Entry::load()? };

        let layer_names = unsafe {
            [ffi::CStr::from_bytes_with_nul_unchecked(
                b"VK_LAYER_KHRONOS_validation\0",
            )]
        };

        let layers_names_raw: Vec<*const c_char> = layer_names
            .iter()
            .map(|raw_name| raw_name.as_ptr())
            .collect();
        let extension_names = vec![debug_utils::NAME.as_ptr()];

        let app_name = unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(b"Mandala\0") };
        let application_info = vk::ApplicationInfo::default()
            .application_name(app_name)
            .application_version(0)
            .engine_name(app_name)
            .engine_version(0)
            .api_version(vk::make_api_version(0, 1, 3, 0));
        let create_info = vk::InstanceCreateInfo::default()
            .enabled_layer_names(&layers_names_raw)
            .enabled_extension_names(&extension_names)
            .application_info(&application_info);
        let instance = unsafe { entry.create_instance(&create_info, None).unwrap() };
        let physical_devices = unsafe { instance.enumerate_physical_devices().unwrap() };
        assert!(!physical_devices.is_empty());

        Ok(Gpu {
            window: Some(window),
        })
    }
}

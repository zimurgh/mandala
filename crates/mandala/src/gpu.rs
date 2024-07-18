use std::{
    borrow::Cow,
    ffi::{self, c_char},
    ops::Deref,
    sync::Arc,
};

use ash::{
    ext::debug_utils,
    vk::{self},
    Entry, Instance,
};
use log::{debug, error, trace, warn};
use winit::{
    event_loop::ActiveEventLoop,
    raw_window_handle::{HasDisplayHandle, HasWindowHandle},
    window::Window,
};

use crate::error::GpuResult;

pub struct Gpu {
    window: Arc<Window>,
    entry: Entry,
    instance: Instance,
    surface: vk::SurfaceKHR,
    debug_call_back: vk::DebugUtilsMessengerEXT,
    debug_utils_loader: debug_utils::Instance,
}

impl Gpu {
    pub fn init(event_loop: &ActiveEventLoop) -> GpuResult<Gpu> {
        let window = Arc::new(event_loop.create_window(Window::default_attributes())?);

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

        let mut extension_names =
            ash_window::enumerate_required_extensions(window.display_handle()?.as_raw())
                .unwrap()
                .to_vec();
        extension_names.push(debug_utils::NAME.as_ptr());

        #[cfg(any(target_os = "macos", target_os = "ios"))]
        {
            extension_names.push(ash::khr::portability_enumeration::NAME.as_ptr());
            // Enabling this extension is a requirement when using `VK_KHR_portability_subset`
            extension_names.push(ash::khr::get_physical_device_properties2::NAME.as_ptr());
        }

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

        let debug_info = vk::DebugUtilsMessengerCreateInfoEXT::default()
            .message_severity(
                vk::DebugUtilsMessageSeverityFlagsEXT::ERROR
                    | vk::DebugUtilsMessageSeverityFlagsEXT::WARNING
                    | vk::DebugUtilsMessageSeverityFlagsEXT::INFO,
            )
            .message_type(
                vk::DebugUtilsMessageTypeFlagsEXT::GENERAL
                    | vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION
                    | vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE,
            )
            .pfn_user_callback(Some(vulkan_debug_callback));

        let debug_utils_loader = debug_utils::Instance::new(&entry, &instance);
        let debug_call_back =
            unsafe { debug_utils_loader.create_debug_utils_messenger(&debug_info, None)? };
        let surface = unsafe {
            ash_window::create_surface(
                &entry,
                &instance,
                window.display_handle()?.as_raw(),
                window.window_handle()?.as_raw(),
                None,
            )?
        };

        Ok(Gpu {
            window,
            entry,
            instance,
            surface,
            debug_call_back,
            debug_utils_loader,
        })
    }

    pub fn window(&mut self) -> &Window {
        self.window.deref()
    }
}

unsafe extern "system" fn vulkan_debug_callback(
    message_severity: vk::DebugUtilsMessageSeverityFlagsEXT,
    message_type: vk::DebugUtilsMessageTypeFlagsEXT,
    p_callback_data: *const vk::DebugUtilsMessengerCallbackDataEXT<'_>,
    _user_data: *mut std::os::raw::c_void,
) -> vk::Bool32 {
    let callback_data = *p_callback_data;
    let message_id_number = callback_data.message_id_number;

    let message_id_name = if callback_data.p_message_id_name.is_null() {
        Cow::from("")
    } else {
        ffi::CStr::from_ptr(callback_data.p_message_id_name).to_string_lossy()
    };

    let message = if callback_data.p_message.is_null() {
        Cow::from("")
    } else {
        ffi::CStr::from_ptr(callback_data.p_message).to_string_lossy()
    };

    match message_severity {
        vk::DebugUtilsMessageSeverityFlagsEXT::INFO => {
            debug!("{message_type:?} [{message_id_name} ({message_id_number})] : {message}");
        }
        vk::DebugUtilsMessageSeverityFlagsEXT::WARNING => {
            warn!("{message_type:?} [{message_id_name} ({message_id_number})] : {message}");
        }
        vk::DebugUtilsMessageSeverityFlagsEXT::ERROR => {
            error!("{message_type:?} [{message_id_name} ({message_id_number})] : {message}");
        }
        vk::DebugUtilsMessageSeverityFlagsEXT::VERBOSE => {
            trace!("{message_type:?} [{message_id_name} ({message_id_number})] : {message}");
        }
        _ => {}
    }

    vk::FALSE
}

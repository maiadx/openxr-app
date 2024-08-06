use ash::{vk, Device, Instance};
use std::ffi::CStr;
use std::collections::HashSet;

pub struct GraphicsDevice {
    pub physical_device: vk::PhysicalDevice,
    pub device: Device,
    pub graphics_queue: vk::Queue,
    pub present_queue: vk::Queue,
}

impl GraphicsDevice {
    pub fn new(instance: &Instance, surface: vk::SurfaceKHR, surface_loader: &ash::extensions::khr::Surface) -> Self {
        let physical_device = Self::select_physical_device(instance, surface, surface_loader)
            .expect("Failed to find a suitable GPU!");

        let indices = Self::find_queue_families(instance, physical_device, surface, surface_loader);
        let (device, graphics_queue, present_queue) = Self::create_logical_device(instance, physical_device, &indices);

        GraphicsDevice {
            physical_device,
            device,
            graphics_queue,
            present_queue,
        }
    }

    fn select_physical_device(
        instance: &Instance,
        surface: vk::SurfaceKHR,
        surface_loader: &ash::extensions::khr::Surface,
    ) -> Option<vk::PhysicalDevice> {
        let devices = unsafe {
            instance
                .enumerate_physical_devices()
                .expect("Failed to enumerate physical devices.")
        };

        devices
            .into_iter()
            .filter(|&device| Self::is_device_suitable(instance, device, surface, surface_loader))
            .max_by_key(|&device| Self::score_physical_device(instance, device))
    }

    fn is_device_suitable(
        instance: &Instance,
        device: vk::PhysicalDevice,
        surface: vk::SurfaceKHR,
        surface_loader: &ash::extensions::khr::Surface,
    ) -> bool {
        let indices = Self::find_queue_families(instance, device, surface, surface_loader);
        let extensions_supported = Self::check_device_extension_support(device);
        let swap_chain_adequate = if extensions_supported {
            let swap_chain_support = Self::query_swap_chain_support(device, surface, surface_loader);
            !swap_chain_support.formats.is_empty() && !swap_chain_support.present_modes.is_empty()
        } else {
            false
        };

        indices.is_complete() && extensions_supported && swap_chain_adequate
    }

    fn score_physical_device(instance: &Instance, device: vk::PhysicalDevice) -> u32 {
        let properties = unsafe { instance.get_physical_device_properties(device) };
        let features = unsafe { instance.get_physical_device_features(device) };

        if !features.geometry_shader {
            return 0;
        }

        let score = if properties.device_type == vk::PhysicalDeviceType::DISCRETE_GPU {
            1000
        } else {
            0
        } + properties.limits.max_image_dimension2d;

        score
    }

    fn find_queue_families(
        instance: &Instance,
        device: vk::PhysicalDevice,
        surface: vk::SurfaceKHR,
        surface_loader: &ash::extensions::khr::Surface,
    ) -> QueueFamilyIndices {
        let queue_families = unsafe { instance.get_physical_device_queue_family_properties(device) };

        let mut indices = QueueFamilyIndices::default();

        for (i, queue_family) in queue_families.iter().enumerate() {
            if queue_family.queue_flags.contains(vk::QueueFlags::GRAPHICS) {
                indices.graphics_family = Some(i as u32);
            }

            let present_support = unsafe {
                surface_loader
                    .get_physical_device_surface_support(device, i as u32, surface)
                    .unwrap_or(false)
            };

            if present_support {
                indices.present_family = Some(i as u32);
            }

            if indices.is_complete() {
                break;
            }
        }

        indices
    }

    fn check_device_extension_support(device: vk::PhysicalDevice) -> bool {
        let required_extensions: HashSet<&'static CStr> = HashSet::from([
            ash::extensions::khr::Swapchain::name(),
            // Add more required extensions here
        ]);

        let available_extensions = unsafe {
            ash::version::InstanceV1_0::enumerate_device_extension_properties(
                &ash::version::InstanceV1_0::null(), // null instance because we only need device info
                device,
                None,
            )
            .expect("Failed to get device extension properties.")
        };

        let available_extension_names = available_extensions
            .iter()
            .map(|ext| unsafe { CStr::from_ptr(ext.extension_name.as_ptr()) })
            .collect::<HashSet<_>>();

        required_extensions.is_subset(&available_extension_names)
    }

    fn query_swap_chain_support(
        device: vk::PhysicalDevice,
        surface: vk::SurfaceKHR,
        surface_loader: &ash::extensions::khr::Surface,
    ) -> SwapChainSupportDetails {
        let capabilities = unsafe {
            surface_loader
                .get_physical_device_surface_capabilities(device, surface)
                .expect("Failed to get surface capabilities.")
        };

        let formats = unsafe {
            surface_loader
                .get_physical_device_surface_formats(device, surface)
                .expect("Failed to get surface formats.")
        };

        let present_modes = unsafe {
            surface_loader
                .get_physical_device_surface_present_modes(device, surface)
                .expect("Failed to get present modes.")
        };

        SwapChainSupportDetails {
            capabilities,
            formats,
            present_modes,
        }
    }

    fn create_logical_device(
        instance: &Instance,
        physical_device: vk::PhysicalDevice,
        indices: &QueueFamilyIndices,
    ) -> (Device, vk::Queue, vk::Queue) {
        let unique_queue_families: HashSet<u32> = [indices.graphics_family.unwrap(), indices.present_family.unwrap()].into_iter().collect();
        let queue_priorities = [1.0_f32];

        let queue_create_infos: Vec<vk::DeviceQueueCreateInfo> = unique_queue_families
            .iter()
            .map(|&queue_family| {
                vk::DeviceQueueCreateInfo::builder()
                    .queue_family_index(queue_family)
                    .queue_priorities(&queue_priorities)
                    .build()
            })
            .collect();

        let device_features = vk::PhysicalDeviceFeatures::builder();

        let device_create_info = vk::DeviceCreateInfo::builder()
            .queue_create_infos(&queue_create_infos)
            .enabled_features(&device_features);

        let device = unsafe {
            instance
                .create_device(physical_device, &device_create_info, None)
                .expect("Failed to create logical device.")
        };

        let graphics_queue = unsafe { device.get_device_queue(indices.graphics_family.unwrap(), 0) };
        let present_queue = unsafe { device.get_device_queue(indices.present_family.unwrap(), 0) };

        (device, graphics_queue, present_queue)
    }
}

impl Drop for GraphicsDevice {
    fn drop(&mut self) {
        unsafe {
            self.device.destroy_device(None);
        }
    }
}

#[derive(Default)]
struct QueueFamilyIndices {
    graphics_family: Option<u32>,
    present_family: Option<u32>,
}

impl QueueFamilyIndices {
    fn is_complete(&self) -> bool {
        self.graphics_family.is_some() && self.present_family.is_some()
    }
}

struct SwapChainSupportDetails {
    capabilities: vk::SurfaceCapabilitiesKHR,
    formats: Vec<vk::SurfaceFormatKHR>,
    present_modes: Vec<vk::PresentModeKHR>,
}

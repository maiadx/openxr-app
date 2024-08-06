use ash::version::{EntryV1_0, InstanceV1_0, DeviceV1_0};
use ash::vk;
use ash::Entry;
use std::ffi::CString;
use std::ptr;

#[cfg(feature = "build_debug")]
const USE_VK_VALIDATION_LAYERS: bool = true;

#[cfg(not(feature = "build_debug"))]
const USE_VK_VALIDATION_LAYERS: bool = false;

struct VulkanSyncObjects {
    current_frame: usize,
    image_available_semaphores: Vec<vk::Semaphore>,
    render_finished_semaphores: Vec<vk::Semaphore>,
    in_flight_fences: Vec<vk::Fence>,
    images_in_flight: Vec<vk::Fence>,
}

impl Default for VulkanSyncObjects {
    fn default() -> Self {
        Self {
            current_frame: 0,
            image_available_semaphores: Vec::new(),
            render_finished_semaphores: Vec::new(),
            in_flight_fences: Vec::new(),
            images_in_flight: Vec::new(),
        }
    }
}

impl VulkanSyncObjects {
    const MAX_FRAMES_IN_FLIGHT: u32 = 2;

    fn initialize_sync_objects(device: &ash::Device, swapchain_images_size: usize) -> Self {
        let mut image_available_semaphores = Vec::with_capacity(Self::MAX_FRAMES_IN_FLIGHT as usize);
        let mut render_finished_semaphores = Vec::with_capacity(Self::MAX_FRAMES_IN_FLIGHT as usize);
        let mut in_flight_fences = Vec::with_capacity(Self::MAX_FRAMES_IN_FLIGHT as usize);
        let mut images_in_flight = vec![vk::Fence::null(); swapchain_images_size];

        let semaphore_info = vk::SemaphoreCreateInfo::builder();
        let fence_info = vk::FenceCreateInfo::builder().flags(vk::FenceCreateFlags::SIGNALED);

        for _ in 0..Self::MAX_FRAMES_IN_FLIGHT {
            unsafe {
                let image_available_semaphore = device.create_semaphore(&semaphore_info, None)
                    .expect("Failed to create semaphore.");
                let render_finished_semaphore = device.create_semaphore(&semaphore_info, None)
                    .expect("Failed to create semaphore.");
                let in_flight_fence = device.create_fence(&fence_info, None)
                    .expect("Failed to create fence.");

                image_available_semaphores.push(image_available_semaphore);
                render_finished_semaphores.push(render_finished_semaphore);
                in_flight_fences.push(in_flight_fence);
            }
        }

        Self {
            current_frame: 0,
            image_available_semaphores,
            render_finished_semaphores,
            in_flight_fences,
            images_in_flight,
        }
    }
}

struct VulkanContext {
    entry: Entry,
    instance: ash::Instance,
    // Other Vulkan objects can be added here, like PhysicalDevice, Device, Queue, Swapchain, etc.
}

impl VulkanContext {
    fn new() -> Result<Self, Box<dyn std::error::Error>> {
        // Load Vulkan entry points
        let entry = Entry::new()?;
        let app_name = CString::new("Vulkan Application")?;
        let engine_name = CString::new("No Engine")?;

        // Application and instance info
        let app_info = vk::ApplicationInfo::builder()
            .application_name(&app_name)
            .application_version(0)
            .engine_name(&engine_name)
            .engine_version(0)
            .api_version(vk::make_version(1, 0, 0));

        // Layers and extensions
        let layer_names = [CString::new("VK_LAYER_KHRONOS_validation")?];
        let layer_names_raw: Vec<*const i8> = layer_names.iter()
            .map(|layer_name| layer_name.as_ptr())
            .collect();

        let extension_names_raw = ash::extensions::ext::DebugUtils::name().as_ptr();

        let create_info = vk::InstanceCreateInfo::builder()
            .application_info(&app_info)
            .enabled_layer_names(if USE_VK_VALIDATION_LAYERS { &layer_names_raw } else { &[] })
            .enabled_extension_names(&[extension_names_raw]);

        // Create Vulkan instance
        let instance: ash::Instance = unsafe {
            entry.create_instance(&create_info, None)?
        };

        Ok(Self {
            entry,
            instance,
        })
    }
}

impl Drop for VulkanContext {
    fn drop(&mut self) {
        unsafe {
            self.instance.destroy_instance(None);
        }
    }
}

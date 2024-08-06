use ash::{vk, Entry, Instance, Device};
use ash::extensions::khr::{Surface, Swapchain};
use std::ffi::CString;
use std::marker::{PhantomData, PhantomPinned};
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

    fn initialize_sync_objects(device: &Device, swapchain_images_size: usize) -> Self {
        let mut image_available_semaphores = Vec::with_capacity(Self::MAX_FRAMES_IN_FLIGHT as usize);
        let mut render_finished_semaphores = Vec::with_capacity(Self::MAX_FRAMES_IN_FLIGHT as usize);
        let mut in_flight_fences = Vec::with_capacity(Self::MAX_FRAMES_IN_FLIGHT as usize);
        let mut images_in_flight = vec![vk::Fence::null(); swapchain_images_size];

        let semaphore_info = vk::SemaphoreCreateInfo {
            s_type: vk::StructureType::SEMAPHORE_CREATE_INFO,
            p_next: ptr::null(),
            flags: vk::SemaphoreCreateFlags::empty(),
            _marker: PhantomData
        };

        let fence_info = vk::FenceCreateInfo {
            s_type: vk::StructureType::FENCE_CREATE_INFO,
            p_next: ptr::null(),
            flags: vk::FenceCreateFlags::SIGNALED,
            _marker: PhantomData
        };

        for _ in 0..Self::MAX_FRAMES_IN_FLIGHT {
            unsafe {
                let image_available_semaphore = device
                    .create_semaphore(&semaphore_info, None)
                    .expect("Failed to create semaphore.");
                let render_finished_semaphore = device
                    .create_semaphore(&semaphore_info, None)
                    .expect("Failed to create semaphore.");
                let in_flight_fence = device
                    .create_fence(&fence_info, None)
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
    instance: Instance,
}

impl VulkanContext {
    fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let entry = unsafe { Entry::new()? };
        let app_name = CString::new("Vulkan Application")?;
        let engine_name = CString::new("No Engine")?;
    
        let app_info = vk::ApplicationInfo {
            s_type: vk::StructureType::APPLICATION_INFO,
            p_next: std::ptr::null(),
            p_application_name: app_name.as_ptr(),
            application_version: 0,
            p_engine_name: engine_name.as_ptr(),
            engine_version: 0,
            api_version: vk::make_api_version(0, 1, 0, 0), // Updated to use `make_api_version`
            _marker:PhantomData
        };
    
        let layer_names = [CString::new("VK_LAYER_KHRONOS_validation")?];
        let layer_names_raw: Vec<*const i8> = layer_names.iter().map(|layer_name| layer_name.as_ptr()).collect();
    
        let extension_names_raw = vec![ash::extensions::ext::DebugUtils::name().as_ptr()];
    
        let create_info = vk::InstanceCreateInfo {
            s_type: vk::StructureType::INSTANCE_CREATE_INFO,
            p_next: std::ptr::null(),
            flags: vk::InstanceCreateFlags::empty(),
            p_application_info: &app_info,
            enabled_layer_count: if USE_VK_VALIDATION_LAYERS { layer_names_raw.len() as u32 } else { 0 },
            pp_enabled_layer_names: if USE_VK_VALIDATION_LAYERS { layer_names_raw.as_ptr() } else { std::ptr::null() },
            enabled_extension_count: extension_names_raw.len() as u32,
            pp_enabled_extension_names: extension_names_raw.as_ptr(),
            _marker:PhantomData
        };
    
        let instance = unsafe { entry.create_instance(&create_info, None)? };
    
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

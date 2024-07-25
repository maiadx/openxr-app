


#[cfg(feature = "build_debug")]
const USE_VK_VALIDATION_LAYERS: bool = true;

#[cfg(not(feature = "build_debug"))]
const USE_VK_VALIDATION_LAYERS: bool = false;


struct VulkanSyncObjects {
    current_frame : usize,
    image_available_semaphores : vec<VulkanSemaphore>,
    render_finished_semaphores : vec<VulkanSemaphore>,
    in_flight_fences : vec<VulkanFence>,
    images_in_flight : vec<VulkanFence>,



}

impl Default for VulkanSyncObjects {
    fn default() -> Self {
        Self {
            current_frame: 0,
        }
    }
}


impl VulkanSyncObjects {
    const MAX_FRAMES_IN_FLIGHT : u32 = 2;
} 


struct VulkanContext {
    instance : VulkanInstance,

}
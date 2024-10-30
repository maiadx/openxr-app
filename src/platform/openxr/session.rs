

use ash::vk::Handle;
use openxr as xr;

pub struct OpenXRSession {
    pub session: xr::Session<xr::Vulkan>,
    pub frame_wait: xr::FrameWaiter,
    pub frame_stream: xr::FrameStream<xr::Vulkan>,
    pub stage: xr::Space,
}

impl OpenXRSession {
    pub fn new(
        xr_instance: &xr::Instance,
        vk_instance: &ash::Instance,
        vk_physical_device: &ash::vk::PhysicalDevice,
        vk_device: &ash::Device,
        queue_family_index: u32,
    ) -> xr::Result<Self> {
        // Create session
        let (session, frame_wait, frame_stream) = unsafe {
            xr_instance.create_session::<xr::Vulkan>(
                xr_instance.system(xr::FormFactor::HEAD_MOUNTED_DISPLAY)?,
                &xr::vulkan::SessionCreateInfo {
                    instance: vk_instance.handle().as_raw() as _,
                    physical_device: vk_physical_device.as_raw() as _,
                    device: vk_device.handle().as_raw() as _,
                    queue_family_index,
                    queue_index: 0,
                },
            )?
        };

        // Create reference space (STAGE or LOCAL)
        let stage = session.create_reference_space(xr::ReferenceSpaceType::STAGE, xr::Posef::IDENTITY)?;

        Ok(Self {
            session,
            frame_wait,
            frame_stream,
            stage,
        })
    }
}
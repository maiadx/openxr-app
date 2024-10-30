use ash::{vk, Entry, Instance, Device};
use ash::khr::surface;
use ash::khr::swapchain;
use ash::ext::debug_utils;
use ash::util::read_spv;
use ash::vk::Handle;
use std::ffi::CString;
use std::marker::{PhantomData, PhantomPinned};
use std::ptr;
use openxr::{self as xr, Vulkan};

use mlog::*;


use crate::io;
#[cfg(debug_assertions)]
const USE_VK_VALIDATION_LAYERS: bool = true;

#[cfg(not(debug_assertions))]
const USE_VK_VALIDATION_LAYERS: bool = false;

use super::shader;


pub const VIEW_COUNT: u32 = 2;
pub const COLOR_FORMAT: vk::Format = vk::Format::R8G8B8A8_SRGB;

pub struct VulkanSyncObjects {
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


pub struct VulkanContext {
    pub entry: ash::Entry,
    pub instance: ash::Instance,
    pub physical_device: vk::PhysicalDevice,
    pub device: ash::Device,
    pub queue_family_index: u32,
    pub queue: vk::Queue,
    pub view_mask: u32,
    pub render_pass: vk::RenderPass,
    pub pipeline_layout: vk::PipelineLayout,
    pub pipeline: vk::Pipeline,
    pub vert_shader_mod: vk::ShaderModule,
    pub frag_shader_mod: vk::ShaderModule,
    pub target_vk_version: u32,
    pub swapchain: vk::SwapchainKHR,  // Swapchain handle
    pub swapchain_images: Vec<vk::Image>,  // Swapchain images
    pub framebuffers: Vec<vk::Framebuffer>,  // Framebuffers associated with the swapchain images
}



impl VulkanContext {
    pub fn new(
        xr_instance: &xr::Instance,
        xr_system: xr::SystemId,
    ) -> Self {
        unsafe {
            let entry = ash::Entry::load().unwrap();
            let target_vk_version = vk::make_api_version(0, 1, 1, 0);

            // Vulkan instance, device, etc. initialization
            let instance = ...;  // Vulkan instance setup
            let physical_device = ...;  // Vulkan physical device setup
            let device = ...;  // Vulkan logical device setup
            let queue_family_index = ...;  // Queue family index
            let queue = ...;  // Graphics queue
            let render_pass = ...;  // Render pass setup
            let swapchain = ...;  // Swapchain setup

            // Get the swapchain images
            let swapchain_images = device
                .get_swapchain_images(swapchain)
                .expect("Failed to get swapchain images");

            // Create framebuffers for each swapchain image
            let framebuffers = swapchain_images
                .iter()
                .map(|&image| {
                    let image_view = device.create_image_view(
                        &vk::ImageViewCreateInfo::default()
                            .image(image)
                            .view_type(vk::ImageViewType::TYPE_2D)
                            .format(vk::Format::B8G8R8A8_SRGB)  // Ensure it matches swapchain format
                            .subresource_range(vk::ImageSubresourceRange {
                                aspect_mask: vk::ImageAspectFlags::COLOR,
                                base_mip_level: 0,
                                level_count: 1,
                                base_array_layer: 0,
                                layer_count: 1,
                            }),
                        None,
                    ).expect("Failed to create image view");

                    device.create_framebuffer(
                        &vk::FramebufferCreateInfo::default()
                            .render_pass(render_pass)
                            .attachments(&[image_view])
                            .width(1280)  // Swapchain image width
                            .height(720)  // Swapchain image height
                            .layers(1),
                        None,
                    ).expect("Failed to create framebuffer")
                })
                .collect::<Vec<_>>();

            VulkanContext {
                entry,
                instance,
                physical_device,
                device,
                queue_family_index,
                queue,
                view_mask: !(!0 << VIEW_COUNT),
                render_pass,
                pipeline_layout: vk::PipelineLayout::null(),
                pipeline: vk::Pipeline::null(),
                vert_shader_mod: vk::ShaderModule::null(),
                frag_shader_mod: vk::ShaderModule::null(),
                target_vk_version,
                swapchain,
                swapchain_images,
                framebuffers,
            }
        }
    }

    // Cleanup resources
    pub fn cleanup(&self) {
        unsafe {
            for &framebuffer in &self.framebuffers {
                self.device.destroy_framebuffer(framebuffer, None);
            }
            self.device.destroy_pipeline(self.pipeline, None);
            self.device.destroy_pipeline_layout(self.pipeline_layout, None);
            self.device.destroy_shader_module(self.vert_shader_mod, None);
            self.device.destroy_shader_module(self.frag_shader_mod, None);
            self.device.destroy_render_pass(self.render_pass, None);
            self.device.destroy_device(None);
            self.instance.destroy_instance(None);
        }
    }
    
    pub fn create_command_buffers(&self) -> Vec<vk::CommandBuffer> {
        let command_pool = vk::CommandPool::default();  // Placeholder: Initialize properly
        let allocate_info = vk::CommandBufferAllocateInfo::default()
            .command_pool(command_pool)
            .level(vk::CommandBufferLevel::PRIMARY)
            .command_buffer_count(self.framebuffers.len() as u32);

        let command_buffers = unsafe {
            self.device.allocate_command_buffers(&allocate_info)
                .expect("Failed to allocate command buffers")
        };

        for (i, &command_buffer) in command_buffers.iter().enumerate() {
            let begin_info = vk::CommandBufferBeginInfo::default();

            unsafe {
                self.device.begin_command_buffer(command_buffer, &begin_info)
                    .expect("Failed to begin command buffer");

                let render_pass_info = vk::RenderPassBeginInfo::default()
                    .render_pass(self.render_pass)
                    .framebuffer(self.framebuffers[i])
                    .render_area(vk::Rect2D {
                        offset: vk::Offset2D { x: 0, y: 0 },
                        extent: vk::Extent2D { width: 1280, height: 720 },
                    })
                    .clear_values(&[vk::ClearValue {
                        color: vk::ClearColorValue {
                            float32: [0.0, 0.0, 0.0, 1.0],  // Clear to black
                        },
                    }]);

                self.device.cmd_begin_render_pass(command_buffer, &render_pass_info, vk::SubpassContents::INLINE);

                // Record commands (bind pipeline, draw commands, etc.)
                self.device.cmd_bind_pipeline(command_buffer, vk::PipelineBindPoint::GRAPHICS, self.pipeline);
                self.device.cmd_draw(command_buffer, 3, 1, 0, 0);  // Example draw call for a triangle

                self.device.cmd_end_render_pass(command_buffer);
                self.device.end_command_buffer(command_buffer)
                    .expect("Failed to record command buffer");
            }
        }

        command_buffers
    }


    pub fn render_frame(&self, command_buffer: vk::CommandBuffer) {
        // Logic to submit the command buffer and present the rendered image using the swapchain
    }


}











// old ver.

// impl VulkanContext { 
//     fn new(xr_instance : xr::Instance, xr_system : xr::SystemId) {

//         unsafe {
//             let vk_entry = ash::Entry::load().unwrap();
//             let target_vk_version = vk::make_api_version(0, 1, 1, 0);



//             let vk_app_info = vk::ApplicationInfo::default()
//                 .application_version(0)
//                 .engine_version(0)
//                 .api_version(target_vk_version);

//                 let vk_instance = {
//                     let vk_instance = xr_instance
//                         .create_vulkan_instance(
//                             xr_system,
//                             std::mem::transmute(vk_entry.static_fn().get_instance_proc_addr),
//                             &vk::InstanceCreateInfo::default().application_info(&vk_app_info) as *const _
//                                 as *const _,
//                         )
//                         .expect("XR error creating Vulkan instance")
//                         .map_err(vk::Result::from_raw)
//                         .expect("Vulkan error creating Vulkan instance");
//                     ash::Instance::load(
//                         vk_entry.static_fn(),
//                         vk::Instance::from_raw(vk_instance as _),
//                     )
//                 };


//             let vk_physical_device = vk::PhysicalDevice::from_raw(
//                 xr_instance
//                     .vulkan_graphics_device(xr_system, vk_instance.handle().as_raw() as _)
//                     .unwrap() as _,
//             );

//             let vk_device_properties = vk_instance.get_physical_device_properties(vk_physical_device);
//             if vk_device_properties.api_version < target_vk_version {
//                 vk_instance.destroy_instance(None);
//                 panic!("Vulkan phyiscal device doesn't support version 1.1");
//             }
    
//             let vk_queue_family_index = vk_instance
//             .get_physical_device_queue_family_properties(vk_physical_device)
//             .into_iter()
//             .enumerate()
//             .find_map(|(queue_family_index, info)| {
//                 if info.queue_flags.contains(vk::QueueFlags::GRAPHICS) {
//                     Some(queue_family_index as u32)
//                 } else {
//                     None
//                 }
//             })
//             .expect("Vulkan device has no graphics queue");

            
//             let vk_device = {
//                 let vk_device = xr_instance
//                     .create_vulkan_device(
//                         xr_system,
//                         std::mem::transmute(vk_entry.static_fn().get_instance_proc_addr),
//                         vk_physical_device.as_raw() as _,
//                         &vk::DeviceCreateInfo::default()
//                             .queue_create_infos(&[vk::DeviceQueueCreateInfo::default()
//                                 .queue_family_index(vk_queue_family_index)
//                                 .queue_priorities(&[1.0])])
//                             .push_next(&mut vk::PhysicalDeviceMultiviewFeatures {
//                                 multiview: vk::TRUE,
//                                 ..Default::default()
//                             }) as *const _ as *const _,
//                     )
//                     .expect("XR error creating Vulkan device")
//                     .map_err(vk::Result::from_raw)
//                     .expect("Vulkan error creating Vulkan device");
    
//                 ash::Device::load(vk_instance.fp_v1_0(), vk::Device::from_raw(vk_device as _))
//             };
            
//             let vk_queue = vk_device.get_device_queue(vk_queue_family_index, 0);
    
//             let vk_view_mask = !(!0 << VIEW_COUNT);

//             let vk_render_pass = vk_device
//                 .create_render_pass(
//                     &vk::RenderPassCreateInfo::default()
//                         .attachments(&[vk::AttachmentDescription {
//                             format: COLOR_FORMAT,
//                             samples: vk::SampleCountFlags::TYPE_1,
//                             load_op: vk::AttachmentLoadOp::CLEAR,
//                             store_op: vk::AttachmentStoreOp::STORE,
//                             initial_layout: vk::ImageLayout::UNDEFINED,
//                             final_layout: vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
//                             ..Default::default()
//                         }])
//                         .subpasses(&[vk::SubpassDescription::default()
//                             .color_attachments(&[vk::AttachmentReference {
//                                 attachment: 0,
//                                 layout: vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
//                             }])
//                             .pipeline_bind_point(vk::PipelineBindPoint::GRAPHICS)])
//                         .dependencies(&[vk::SubpassDependency {
//                             src_subpass: vk::SUBPASS_EXTERNAL,
//                             dst_subpass: 0,
//                             src_stage_mask: vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
//                             dst_stage_mask: vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
//                             dst_access_mask: vk::AccessFlags::COLOR_ATTACHMENT_WRITE,
//                             ..Default::default()
//                         }])
//                         .push_next(
//                             &mut vk::RenderPassMultiviewCreateInfo::default()
//                                 .view_masks(&[vk_view_mask])
//                                 .correlation_masks(&[vk_view_mask]),
//                         ),
//                     None,
//                 )
//                 .unwrap();


//                 // load and compile shaders:
//                 io::shader_compiler::compile_all_shaders().expect("Something went wrong with shader compilation");
//                 let (vert_shader, frag_shader) = shader::create_shader_modules(&vk_device);
                
//                 let pipeline_layout = vk_device
//                 .create_pipeline_layout(
//                     &vk::PipelineLayoutCreateInfo::default().set_layouts(&[]),
//                     None,
//                 )
//                 .unwrap();

//                 let noop_stencil_state = vk::StencilOpState {
//                     fail_op: vk::StencilOp::KEEP,
//                     pass_op: vk::StencilOp::KEEP,
//                     depth_fail_op: vk::StencilOp::KEEP,
//                     compare_op: vk::CompareOp::ALWAYS,
//                     compare_mask: 0,
//                     write_mask: 0,
//                     reference: 0,
//                 };
                
//                 let pipeline = vk_device
//                 .create_graphics_pipelines(
//                     vk::PipelineCache::null(),
//                     &[vk::GraphicsPipelineCreateInfo::default()
//                         .stages(&[
//                             vk::PipelineShaderStageCreateInfo {
//                                 stage: vk::ShaderStageFlags::VERTEX,
//                                 module: vert_shader,
//                                 p_name: b"main\0".as_ptr() as _,
//                                 ..Default::default()
//                             },
//                             vk::PipelineShaderStageCreateInfo {
//                                 stage: vk::ShaderStageFlags::FRAGMENT,
//                                 module: frag_shader,
//                                 p_name: b"main\0".as_ptr() as _,
//                                 ..Default::default()
//                             },
//                         ])
//                         .vertex_input_state(&vk::PipelineVertexInputStateCreateInfo::default())
//                         .input_assembly_state(
//                             &vk::PipelineInputAssemblyStateCreateInfo::default()
//                                 .topology(vk::PrimitiveTopology::TRIANGLE_LIST),
//                         )
//                         .viewport_state(
//                             &vk::PipelineViewportStateCreateInfo::default()
//                                 .scissor_count(1)
//                                 .viewport_count(1),
//                         )
//                         .rasterization_state(
//                             &vk::PipelineRasterizationStateCreateInfo::default()
//                                 .cull_mode(vk::CullModeFlags::NONE)
//                                 .polygon_mode(vk::PolygonMode::FILL)
//                                 .line_width(1.0),
//                         )
//                         .multisample_state(
//                             &vk::PipelineMultisampleStateCreateInfo::default()
//                                 .rasterization_samples(vk::SampleCountFlags::TYPE_1),
//                         )
//                         .depth_stencil_state(
//                             &vk::PipelineDepthStencilStateCreateInfo::default()
//                                 .depth_test_enable(false)
//                                 .depth_write_enable(false)
//                                 .front(noop_stencil_state)
//                                 .back(noop_stencil_state),
//                         )
//                         .color_blend_state(
//                             &vk::PipelineColorBlendStateCreateInfo::default().attachments(&[
//                                 vk::PipelineColorBlendAttachmentState {
//                                     blend_enable: vk::TRUE,
//                                     src_color_blend_factor: vk::BlendFactor::ONE,
//                                     dst_color_blend_factor: vk::BlendFactor::ZERO,
//                                     color_blend_op: vk::BlendOp::ADD,
//                                     color_write_mask: vk::ColorComponentFlags::R
//                                         | vk::ColorComponentFlags::G
//                                         | vk::ColorComponentFlags::B,
//                                     ..Default::default()
//                                 },
//                             ]),
//                         )
//                         .dynamic_state(
//                             &vk::PipelineDynamicStateCreateInfo::default().dynamic_states(&[
//                                 vk::DynamicState::VIEWPORT,
//                                 vk::DynamicState::SCISSOR,
//                             ]),
//                         )
//                         .layout(pipeline_layout)
//                         .render_pass(vk_render_pass)
//                         .subpass(0)],
//                     None,
//                 )
//                 .unwrap()[0];

//                 vk_device.destroy_shader_module(vert_shader, None);
//                 vk_device.destroy_shader_module(frag_shader, None);


//         }
//     }
// }
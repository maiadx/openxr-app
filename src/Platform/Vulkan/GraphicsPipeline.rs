use ash::version::DeviceV1_0;
use ash::vk;
use std::ffi::CString;
use std::path::Path;
use std::ptr;

pub struct VulkanGraphicsPipeline {
    pub pipeline: vk::Pipeline,
    pub layout: vk::PipelineLayout,
}

impl VulkanGraphicsPipeline {
    pub fn create(device: &ash::Device, swapchain_extent: vk::Extent2D, render_pass: vk::RenderPass) -> Self {
        let vert_shader_code = Self::read_shader_code("path/to/vert.spv");
        let frag_shader_code = Self::read_shader_code("path/to/frag.spv");

        let vert_shader_module = Self::create_shader_module(device, &vert_shader_code);
        let frag_shader_module = Self::create_shader_module(device, &frag_shader_code);

        let main_function_name = CString::new("main").unwrap();

        let vert_shader_stage_info = vk::PipelineShaderStageCreateInfo::builder()
            .stage(vk::ShaderStageFlags::VERTEX)
            .module(vert_shader_module)
            .name(&main_function_name)
            .build();

        let frag_shader_stage_info = vk::PipelineShaderStageCreateInfo::builder()
            .stage(vk::ShaderStageFlags::FRAGMENT)
            .module(frag_shader_module)
            .name(&main_function_name)
            .build();

        let shader_stages = [vert_shader_stage_info, frag_shader_stage_info];

        let vertex_input_info = vk::PipelineVertexInputStateCreateInfo::builder()
            .vertex_binding_descriptions(&[])
            .vertex_attribute_descriptions(&[])
            .build();

        let input_assembly = vk::PipelineInputAssemblyStateCreateInfo::builder()
            .topology(vk::PrimitiveTopology::TRIANGLE_LIST)
            .primitive_restart_enable(false)
            .build();

        let viewport = vk::Viewport::builder()
            .x(0.0)
            .y(0.0)
            .width(swapchain_extent.width as f32)
            .height(swapchain_extent.height as f32)
            .min_depth(0.0)
            .max_depth(1.0)
            .build();

        let scissor = vk::Rect2D::builder()
            .offset(vk::Offset2D { x: 0, y: 0 })
            .extent(swapchain_extent)
            .build();

        let viewport_state = vk::PipelineViewportStateCreateInfo::builder()
            .viewports(&[viewport])
            .scissors(&[scissor])
            .build();

        let rasterizer = vk::PipelineRasterizationStateCreateInfo::builder()
            .depth_clamp_enable(false)
            .rasterizer_discard_enable(false)
            .polygon_mode(vk::PolygonMode::FILL)
            .line_width(1.0)
            .cull_mode(vk::CullModeFlags::BACK)
            .front_face(vk::FrontFace::CLOCKWISE)
            .depth_bias_enable(false)
            .build();

        let multisampling = vk::PipelineMultisampleStateCreateInfo::builder()
            .sample_shading_enable(false)
            .rasterization_samples(vk::SampleCountFlags::TYPE_1)
            .build();

        let color_blend_attachment = vk::PipelineColorBlendAttachmentState::builder()
            .color_write_mask(vk::ColorComponentFlags::R | vk::ColorComponentFlags::G | vk::ColorComponentFlags::B | vk::ColorComponentFlags::A)
            .blend_enable(false)
            .build();

        let color_blending = vk::PipelineColorBlendStateCreateInfo::builder()
            .logic_op_enable(false)
            .logic_op(vk::LogicOp::COPY)
            .attachments(&[color_blend_attachment])
            .blend_constants([0.0, 0.0, 0.0, 0.0])
            .build();

        let pipeline_layout_info = vk::PipelineLayoutCreateInfo::builder()
            .set_layouts(&[])
            .build();

        let layout = unsafe {
            device.create_pipeline_layout(&pipeline_layout_info, None)
                .expect("Failed to create pipeline layout")
        };

        let pipeline_info = vk::GraphicsPipelineCreateInfo::builder()
            .stages(&shader_stages)
            .vertex_input_state(&vertex_input_info)
            .input_assembly_state(&input_assembly)
            .viewport_state(&viewport_state)
            .rasterization_state(&rasterizer)
            .multisample_state(&multisampling)
            .color_blend_state(&color_blending)
            .layout(layout)
            .render_pass(render_pass)
            .subpass(0)
            .build();

        let pipeline = unsafe {
            device.create_graphics_pipelines(vk::PipelineCache::null(), &[pipeline_info], None)
                .expect("Failed to create graphics pipeline")[0]
        };

        unsafe {
            device.destroy_shader_module(vert_shader_module, None);
            device.destroy_shader_module(frag_shader_module, None);
        }

        Self { pipeline, layout }
    }

    pub fn destroy(&self, device: &ash::Device) {
        unsafe {
            device.destroy_pipeline(self.pipeline, None);
            device.destroy_pipeline_layout(self.layout, None);
        }
    }

    fn read_shader_code<P: AsRef<Path>>(path: P) -> Vec<u8> {
        use std::fs::File;
        use std::io::Read;
        let mut file = File::open(path).expect("Failed to open shader file");
        let mut code = Vec::new();
        file.read_to_end(&mut code).expect("Failed to read shader file");
        code
    }

    fn create_shader_module(device: &ash::Device, code: &[u8]) -> vk::ShaderModule {
        let create_info = vk::ShaderModuleCreateInfo::builder()
            .code(bytemuck::cast_slice(code))
            .build();

        unsafe {
            device.create_shader_module(&create_info, None)
                .expect("Failed to create shader module")
        }
    }
}

fn example() {
    // Example usage, requires initialization of Vulkan context, device, etc.
    let device: ash::Device = unimplemented!(); // Obtain the Vulkan device
    let swapchain_extent: vk::Extent2D = unimplemented!(); // Obtain swapchain extent
    let render_pass: vk::RenderPass = unimplemented!(); // Obtain render pass

    let graphics_pipeline = VulkanGraphicsPipeline::create(&device, swapchain_extent, render_pass);

    // Do some rendering...

    graphics_pipeline.destroy(&device);
}

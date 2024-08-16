use ash::vk;
use ash::Device;
use std::ffi::CString;
use std::marker::PhantomData;
use std::ptr;
use std::fs::File;
use std::io::Read;
use std::path::Path;
pub struct VulkanGraphicsPipeline<'a> {
    pub pipeline: vk::Pipeline,
    pub layout: vk::PipelineLayout,
    device: &'a Device,
}

impl<'a> VulkanGraphicsPipeline<'a> {

    fn read_shader_code<P: AsRef<Path>>(path: P) -> Vec<u8> {
        let mut file = File::open(path).expect("Failed to open shader file");
        let mut code = Vec::new();
        file.read_to_end(&mut code).expect("Failed to read shader file");
        code
    }

    fn create_shader_module(device: &Device, code: &[u8]) -> vk::ShaderModule {
        let code_u32 = ash::util::read_spv(&mut std::io::Cursor::new(code))
            .expect("Failed to read shader SPIR-V code");
    
        let create_info = vk::ShaderModuleCreateInfo {
            s_type: vk::StructureType::SHADER_MODULE_CREATE_INFO,
            p_next: std::ptr::null(),
            flags: vk::ShaderModuleCreateFlags::empty(),
            code_size: code_u32.len() * std::mem::size_of::<u32>(),
            p_code: code_u32.as_ptr(),
            _marker: PhantomData
        };
    
        unsafe {
            device.create_shader_module(&create_info, None)
                .expect("Failed to create shader module")
        }
    }



    pub fn create(device: &'a Device, swapchain_extent: vk::Extent2D, render_pass: vk::RenderPass) -> Self {
        let vert_shader_code = Self::read_shader_code("path/to/vert.spv");
        let frag_shader_code = Self::read_shader_code("path/to/frag.spv");

        let vert_shader_module = Self::create_shader_module(device, &vert_shader_code);
        let frag_shader_module = Self::create_shader_module(device, &frag_shader_code);

        let main_function_name = CString::new("main").unwrap();

        let vert_shader_stage_info = vk::PipelineShaderStageCreateInfo::default()
            .stage(vk::ShaderStageFlags::VERTEX)
            .module(vert_shader_module)
            .name(&main_function_name);

        let frag_shader_stage_info = vk::PipelineShaderStageCreateInfo::default()
            .stage(vk::ShaderStageFlags::FRAGMENT)
            .module(frag_shader_module)
            .name(&main_function_name);

        let shader_stages = [vert_shader_stage_info, frag_shader_stage_info];

        let vertex_input_info = vk::PipelineVertexInputStateCreateInfo::default();

        let input_assembly = vk::PipelineInputAssemblyStateCreateInfo::default()
            .topology(vk::PrimitiveTopology::TRIANGLE_LIST)
            .primitive_restart_enable(false);

        let viewport = vk::Viewport {
            x: 0.0,
            y: 0.0,
            width: swapchain_extent.width as f32,
            height: swapchain_extent.height as f32,
            min_depth: 0.0,
            max_depth: 1.0,
        };

        let scissor = vk::Rect2D {
            offset: vk::Offset2D { x: 0, y: 0 },
            extent: swapchain_extent,
        };

        let viewports = [viewport];
        let scissors = [scissor];

        let viewport_state = vk::PipelineViewportStateCreateInfo::default()
            .viewports(&viewports)
            .scissors(&scissors);

        let rasterizer = vk::PipelineRasterizationStateCreateInfo::default()
            .depth_clamp_enable(false)
            .rasterizer_discard_enable(false)
            .polygon_mode(vk::PolygonMode::FILL)
            .line_width(1.0)
            .cull_mode(vk::CullModeFlags::BACK)
            .front_face(vk::FrontFace::CLOCKWISE)
            .depth_bias_enable(false);

        let multisampling = vk::PipelineMultisampleStateCreateInfo::default()
            .sample_shading_enable(false)
            .rasterization_samples(vk::SampleCountFlags::TYPE_1);

        let color_blend_attachment = vk::PipelineColorBlendAttachmentState::default()
            .color_write_mask(vk::ColorComponentFlags::R | vk::ColorComponentFlags::G | vk::ColorComponentFlags::B | vk::ColorComponentFlags::A)
            .blend_enable(false);

        let color_blend_attachments = [color_blend_attachment];

        let color_blending = vk::PipelineColorBlendStateCreateInfo::default()
            .logic_op_enable(false)
            .logic_op(vk::logicOp::COPY)
            .attachments(&color_blend_attachments)
            .blend_constants([0.0, 0.0, 0.0, 0.0]);

        let pipeline_layout_info = vk::PipelineLayoutCreateInfo::default();

        let layout = unsafe {
            device.create_pipeline_layout(&pipeline_layout_info, None)
                .expect("Failed to create pipeline layout")
        };

        let pipeline_info = vk::GraphicsPipelineCreateInfo::default()
            .stages(&shader_stages)
            .vertex_input_state(&vertex_input_info)
            .input_assembly_state(&input_assembly)
            .viewport_state(&viewport_state)
            .rasterization_state(&rasterizer)
            .multisample_state(&multisampling)
            .color_blend_state(&color_blending)
            .layout(layout)
            .render_pass(render_pass)
            .subpass(0);

        let pipeline = unsafe {
            device.create_graphics_pipelines(vk::PipelineCache::null(), &[pipeline_info], None)
                .expect("Failed to create graphics pipeline")[0]
        };

        unsafe {
            device.destroy_shader_module(vert_shader_module, None);
            device.destroy_shader_module(frag_shader_module, None);
        }

        Self { pipeline, layout, device }
    }
}
    fn read_shader_code<P: AsRef<std::path::Path>>(path: P) -> Vec<u8> {
        use std::fs::File;
        use std::io::Read;
        let mut file = File::open(path).expect("Failed to open shader file");
        let mut code = Vec::new();
        file.read_to_end(&mut code).expect("Failed to read shader file");
        code
    }

    fn create_shader_module(device: &Device, code: &[u8]) -> vk::ShaderModule {
        let create_info = vk::ShaderModuleCreateInfo {
            s_type: vk::StructureType::SHADER_MODULE_CREATE_INFO,
            p_next: ptr::null(),
            flags: vk::ShaderModuleCreateFlags::empty(),
            code_size: code.len(),
            p_code: code.as_ptr() as *const u32,
            _marker: PhantomData
        };

        unsafe {
            device.create_shader_module(&create_info, None)
                .expect("Failed to create shader module")
        }
    }


impl<'a> Drop for VulkanGraphicsPipeline<'a> {
    fn drop(&mut self) {
        unsafe {
            // Assuming `device` is stored somewhere accessible
            self.device.destroy_pipeline(self.pipeline, None);
            self.device.destroy_pipeline_layout(self.layout, None);
        }
    }
}

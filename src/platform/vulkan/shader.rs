
use ash::vk;
use ash::vk::Device;
use ash::vk::ShaderModuleCreateInfo;

use crate::io::shader_compiler::load_spirv_from_file;


//  load preset shader modules for now.               -> (vertex shader, fragment shader)
pub fn create_shader_modules(vk_device: &ash::Device) -> (vk::ShaderModule, vk::ShaderModule) {
    // Call the new function to load SPIR-V files
    let vert_spv = load_spirv_from_file("resources/shaders/fullscreen.vert.spv");
    let frag_spv = load_spirv_from_file("resources/shaders/debug_pattern.frag.spv");

    // Create shader modules using the SPIR-V bytecode
    // Create shader modules
    let vert_module = unsafe {vk_device
        .create_shader_module(&ShaderModuleCreateInfo::default().code(&vert_spv), None)
        .expect("Failed to create vertex shader module")};
    let frag_module = unsafe { vk_device
        .create_shader_module(&ShaderModuleCreateInfo::default().code(&frag_spv), None)
        .expect("Failed to create fragment shader module") };

    
    (vert_module, frag_module)
}
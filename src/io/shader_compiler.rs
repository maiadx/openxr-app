use std::{
    fs,
    io::Result,
    path::Path,
    process::Command,
    io::Cursor,
    fs::File,
};
use ash::util::read_spv;
use ash::vk::ShaderModule;
use ash::vk::Device;
use ash::vk::ShaderModuleCreateInfo;
use std::io::{Read, Write,Error};

// temp function which just compiles all shaders within directory, will eventually make compilation only occur with shaders which have been changed since compilation using spv comparision
pub fn compile_all_shaders() -> Result<()> {
    info!("Compiling shaders:");

    let shaders_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("resources")
        .join("shaders");
    // info!("Shader source directory: {:?}", shaders_path.to_str().unwrap_or(""));

    // Iterate over files in the directory
    for entry in fs::read_dir(shaders_path.clone())? {
        let entry = entry?;
        let path = entry.path();

        // Only compile files with known shader extensions (e.g., .vert, .frag, .comp)
        if let Some(extension) = path.extension() {
            if extension == "vert" || extension == "frag" || extension == "comp" {
                info!("Compiling shader: {:?}", path.to_str().unwrap_or(""));
                
                let original_extension = path.extension().and_then(|ext| ext.to_str()).unwrap_or("");
                let output_path = path.with_extension(format!("{}.spv", original_extension));
                
                // Invoke glslangValidator on the shader file
                let output = Command::new("glslangValidator")
                    .arg("-V") // Target SPIR-V output
                    .arg(path.as_os_str())
                    .arg("-o")
                    .arg(output_path)
                    .output();

                // Check if the command was successful
                match output {
                    Ok(output) => {
                        if !output.status.success() {
                            crit!("Failed to compile shader: {}", path.to_str().expect("path malformed?"));
                            return Err(Error::new(std::io::ErrorKind::InvalidInput, "Shader Compilation Failed!"))
                        } else {
                            info!("    Successfully compiled: {:?}", path.to_str().unwrap());
                        }
                    }
                    Err(e) => { 
                        crit!("Failed to run glslangValidator: {}", e);
                        return Err(e);
                        }
                }
            }
        }
    }
    info!("Shader compilation successful :)");
    Ok(())
}


pub fn load_spirv(vk_device: &ash::Device) -> (ash::vk::ShaderModule, ash::vk::ShaderModule) {
    let vert_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("resources/shaders/fullscreen.vert.spv");
    let frag_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("resources/shaders/debug_pattern.frag.spv");

    // Read vertex shader SPIR-V
    let mut vert_file = File::open(vert_path).expect("Failed to open vertex shader file");
    let mut vert_bytes = Vec::new();
    vert_file.read_to_end(&mut vert_bytes).expect("Failed to read vertex shader file");
    let vert_spv = read_spv(&mut Cursor::new(&vert_bytes)).expect("Failed to parse vertex shader SPIR-V");

    // Read fragment shader SPIR-V
    let mut frag_file = File::open(frag_path).expect("Failed to open fragment shader file");
    let mut frag_bytes = Vec::new();
    frag_file.read_to_end(&mut frag_bytes).expect("Failed to read fragment shader file");
    let frag_spv = read_spv(&mut Cursor::new(&frag_bytes)).expect("Failed to parse fragment shader SPIR-V");

    // Create shader modules
    let vert_module = unsafe {vk_device
        .create_shader_module(&ShaderModuleCreateInfo::default().code(&vert_spv), None)
        .expect("Failed to create vertex shader module")};
    let frag_module = unsafe { vk_device
        .create_shader_module(&ShaderModuleCreateInfo::default().code(&frag_spv), None)
        .expect("Failed to create fragment shader module") };

    return (vert_module, frag_module)
}


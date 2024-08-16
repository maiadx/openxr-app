use std::{
    fs,
    io::Result,
    path::Path,
    process::Command,
};

use crate::Log;

// temp function which just compiles all shaders within directory, will eventually make compilation only occur with shaders which have been changed since compilation using spv comparision
pub fn compile_all_shaders() -> Result<()> {
    Log::info("Compiling shaders...");

    let shaders_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("resources")
        .join("shaders");
    Log::info(format!("Shader source directory: {:?}", shaders_path.as_os_str()).as_str());

    // Iterate over files in the directory
    for entry in fs::read_dir(shaders_path.clone())? {
        let entry = entry?;
        let path = entry.path();

        // Only compile files with known shader extensions (e.g., .vert, .frag, .comp)
        if let Some(extension) = path.extension() {
            if extension == "vert" || extension == "frag" || extension == "comp" {
                Log::info(format!("Compiling shader: {:?}", path).as_str());
                
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
                            crit!("Failed to compile shader: {}", path.as_os_str().to_string_lossy());
                        } else {
                            Log::info(format!(
                                "Successfully compiled shader: {:?}",
                                path
                            ).as_str());
                        }
                    }
                    Err(e) => crit!("Failed to run glslangValidator: {}", e)
                }
            }
        }
    }
    info!("Shader compilation successful :)");
    Ok(())
}
#[allow(non_snake_case)]

use openxr as xr;

use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
#[macro_use]
mod Log;

mod Platform;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load the OpenXR runtime
    let entry = unsafe { xr::Entry::load()? };
    let application_name = "OpenXR Example";

    // Specify the API version (typically the OpenXR version you're targeting)
    let api_version = xr::Version::new(1, 0, 0); // Targeting OpenXR 1.0.0

    // Define the extension set (currently no extensions enabled)
    let extensions = xr::ExtensionSet::default();

    // Create an OpenXR instance
    let instance = entry.create_instance(
        &xr::ApplicationInfo {
            application_name,
            application_version: 0,
            engine_name: "No Engine",
            engine_version: 0,
            api_version,
        },
        &extensions,
        &[], // No additional layers
    )?;

    // Create an event loop and window
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("OpenXR Example")
        .build(&event_loop)?;

    // Main loop
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                _ => (),
            },
            _ => (),
        }
    });

    Ok(())
}

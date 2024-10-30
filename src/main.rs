#![allow(unused_imports)]
#![allow(dead_code)]
use mlog::*;


// mod io;
mod platform;
// mod renderer;

use openxr as xr;

// use platform::vulkan::context;

// use platform::openxr::{OpenXRSession, ActionSet};
// use platform::openxr::device_emulation::{DeviceManager, VirtualDevice};
// use platform::VulkanContext;
// use xr::{Posef};


fn main() {
    let log_config = mlog::LogConfig {
        log_level: LogLevel::Info,
        application_name: "vr test - neon".to_string(),
        log_filepath: Some("target/debug/neon".to_string()),
        console_flag: true,
        async_flag: true,
        multi_threaded_flag: true,
        time_format: "%H:%M:%S%.3f".to_string(),
    };

    mlog::init(log_config);

      // Initialize the OpenXR instance
      let entry = xr::Entry::linked();
      let instance = entry.create_instance(
          &xr::ApplicationInfo {
              application_name: "Neon",
              application_version: 0,
              engine_name: "Neon Engine",
              engine_version: 0,
          },
          &[],
          &[],
      )?;
  
      // Select the Vulkan graphics API
      let vk_entry = ash::Entry::linked();
      let vk_instance = create_vulkan_instance(&vk_entry)?;
  
      // Create OpenXR system
      let system = instance.system(xr::FormFactor::HEAD_MOUNTED_DISPLAY)?;
  
      // Create session
      let session = unsafe {
          instance.create_session(
              system,
              &xr::vulkan::SessionCreateInfo {
                  instance: vk_instance.handle().as_raw() as _,
                  physical_device: vk::PhysicalDevice::null(), // Replace with actual device
                  device: vk::Device::null(),                 // Replace with actual device
                  queue_family_index: 0,                      // Replace with actual queue family index
                  queue_index: 0,
              },
          )?
      };

    mlog::shutdown();



}
    // 1. Initialize OpenXR and Vulkan Context



//     let xr_entry = xr::Entry::linked();
//     let xr_instance = xr_entry
//         .create_instance(
//             &xr::ApplicationInfo {
//                 application_name: "VR Test",
//                 application_version: 0,
//                 engine_name: "Custom Engine",
//                 engine_version: 0,
//             },
//             &xr::ExtensionSet::default(),
//             &[],
//         )
//         .expect("Failed to create OpenXR instance");






//     let xr_system = xr_instance.system(xr::FormFactor::HEAD_MOUNTED_DISPLAY).expect("Failed to get system");
    
//     // Initialize Vulkan context
//     let vk_context = VulkanContext::new(&xr_instance, xr_system);

//     let xr_session = OpenXRSession::new(
//         &xr_instance,
//         &vk_context.instance,
//         &vk_context.physical_device,
//         &vk_context.device,
//         vk_context.queue_family_index,
//     ).expect("Failed to create OpenXR session");

    

//     // 2. Set up ActionSet
//     let action_set = ActionSet::new(&xr_instance).expect("Failed to create ActionSet");

//     // Attach the ActionSet to the session
//     action_set.attach(&xr_session.session).expect("Failed to attach ActionSet to the session");

//     // 3. Create Action Spaces for virtual controllers
//     let (right_hand_space, left_hand_space) = action_set.create_action_spaces(&xr_session.session)
//         .expect("Failed to create action spaces");

//     // 4. Initialize Device Manager and add virtual devices
//     let mut device_manager = DeviceManager::new();

//     let right_hand_device = VirtualDevice::new("right_hand", right_hand_space);
//     let left_hand_device = VirtualDevice::new("left_hand", left_hand_space);

//     device_manager.add_device(right_hand_device);
//     device_manager.add_device(left_hand_device);

//     // 5. Main Loop: Update poses and simulate device movement
//     let command_buffers = vk_context.create_command_buffers();
//     for frame in 0..10 {  // Simulate 10 frames for this test
//         let new_pose = Posef {
//             orientation: xr::Quaternionf { x: 0.0, y: 0.0, z: 0.0, w: 1.0 },
//             position: xr::Vector3f { x: 1.0, y: 1.0, z: 1.0 },  // Just an example position
//         };

//         // Update right hand device with the new pose
//         device_manager.update_pose("right_hand", &xr_session, new_pose)
//             .expect("Failed to update pose for right hand");

//         // Render frame using Vulkan
//         vk_context.render_frame(command_buffers[frame % command_buffers.len()]);

//         // Simulate a small delay
//         std::thread::sleep(std::time::Duration::from_millis(16));  // Simulate ~60FPS
//     }

//     success!("Test complete");
//     vk_context.cleanup();
//     mlog::shutdown();
// }

//     let xr_entry = xr::Entry::linked();
//     let xr_instance = xr_entry
//         .create_instance(
//             &xr::ApplicationInfo {
//                 application_name: "VR Test",
//                 application_version: 0,
//                 engine_name: "Custom Engine",
//                 engine_version: 0,
//             },
//             &xr::ExtensionSet::default(),
//             &[],
//         )
//         .expect("Failed to create OpenXR instance");






//     let xr_system = xr_instance.system(xr::FormFactor::HEAD_MOUNTED_DISPLAY).expect("Failed to get system");
    
//     // Initialize Vulkan context
//     let vk_context = VulkanContext::new(&xr_instance, xr_system);

//     let xr_session = OpenXRSession::new(
//         &xr_instance,
//         &vk_context.instance,
//         &vk_context.physical_device,
//         &vk_context.device,
//         vk_context.queue_family_index,
//     ).expect("Failed to create OpenXR session");

    

//     // 2. Set up ActionSet
//     let action_set = ActionSet::new(&xr_instance).expect("Failed to create ActionSet");

//     // Attach the ActionSet to the session
//     action_set.attach(&xr_session.session).expect("Failed to attach ActionSet to the session");

//     // 3. Create Action Spaces for virtual controllers
//     let (right_hand_space, left_hand_space) = action_set.create_action_spaces(&xr_session.session)
//         .expect("Failed to create action spaces");

//     // 4. Initialize Device Manager and add virtual devices
//     let mut device_manager = DeviceManager::new();

//     let right_hand_device = VirtualDevice::new("right_hand", right_hand_space);
//     let left_hand_device = VirtualDevice::new("left_hand", left_hand_space);

//     device_manager.add_device(right_hand_device);
//     device_manager.add_device(left_hand_device);

//     // 5. Main Loop: Update poses and simulate device movement
//     let command_buffers = vk_context.create_command_buffers();
//     for frame in 0..10 {  // Simulate 10 frames for this test
//         let new_pose = Posef {
//             orientation: xr::Quaternionf { x: 0.0, y: 0.0, z: 0.0, w: 1.0 },
//             position: xr::Vector3f { x: 1.0, y: 1.0, z: 1.0 },  // Just an example position
//         };

//         // Update right hand device with the new pose
//         device_manager.update_pose("right_hand", &xr_session, new_pose)
//             .expect("Failed to update pose for right hand");

//         // Render frame using Vulkan
//         vk_context.render_frame(command_buffers[frame % command_buffers.len()]);

//         // Simulate a small delay
//         std::thread::sleep(std::time::Duration::from_millis(16));  // Simulate ~60FPS
//     }

//     success!("Test complete");
//     vk_context.cleanup();
//     mlog::shutdown();
// }
// use openxr as xr;




// pub struct OpenXRContext {
//     entry: xr::Entry,
//     instance: xr::Instance,
//     system: xr::SystemId,
//     session: xr::Session<xr::Vulkan>,
//     frame_wait: xr::FrameWaiter,
//     frame_stream: xr::FrameStream<xr::Vulkan>,
// }

// impl OpenXRContext {
//     pub fn new() -> Self {
//         let entry = xr::Entry::linked();
//         let available_extensions = entry.enumerate_extensions().unwrap();

//         let mut enabled_extensions = xr::ExtensionSet::default();
//         enabled_extensions.khr_vulkan_enable2 = true;

//         let instance = entry
//             .create_instance(
//                 &xr::ApplicationInfo {
//                     application_name: "openxrs example",
//                     application_version: 0,
//                     engine_name: "openxrs example",
//                     engine_version: 0,
//                     api_version: xr::Version::new(1, 0, 0),
//                 },
//                 &enabled_extensions,
//                 &[],
//             )
//             .unwrap();

//         let system = instance.system(xr::FormFactor::HEAD_MOUNTED_DISPLAY).unwrap();

//         // Create session with Vulkan integration
//         let (session, frame_wait, frame_stream) = instance
//             .create_session::<xr::Vulkan>(
//                 system,
//                 &xr::vulkan::SessionCreateInfo {
//                     instance: std::ptr::null(), // Fill in Vulkan instance data later
//                     physical_device: std::ptr::null(), // Fill in Vulkan physical device later
//                     device: std::ptr::null(), // Fill in Vulkan device later
//                     queue_family_index: 0,
//                     queue_index: 0,
//                 },
//             )
//             .unwrap();

//         OpenXRContext {
//             entry,
//             instance,
//             system,
//             session,
//             frame_wait,
//             frame_stream,
//         }
//     }

//     pub fn cleanup(&self) {
//         // Clean up OpenXR resources
//         self.session.end().unwrap();
//     }
// }



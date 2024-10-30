use openxr as xr;
use mlog::*;

use crate::platform::openxr::session::OpenXRSession;

pub struct VirtualDevice {
    pub name: String,
    pub space: xr::Space,
}

impl VirtualDevice {
    pub fn new(name: &str, space: xr::Space) -> Self {
        Self {
            name: name.to_string(),
            space,
        }
    }

    pub fn update_pose(&self, session: &OpenXRSession, pose: xr::Posef) -> xr::Result<()> {
        // Here you would normally update the pose for the device
        mlog::info!("Updating pose for device: {} to {:?}", self.name, pose);

        Ok(())
    }
}

pub struct DeviceManager {
    pub devices: Vec<VirtualDevice>,
}

impl DeviceManager {
    pub fn new() -> Self {
        Self {
            devices: Vec::new(),
        }
    }

    pub fn add_device(&mut self, device: VirtualDevice) {
        self.devices.push(device);
    }

    pub fn update_all_devices(&self, session: &OpenXRSession) {
        for device in &self.devices {
            // Here we would update the poses based on some logic or input data
            // For example, you could call device.update_pose(session, new_pose);
        }
    }

    pub fn update_pose(
        &self,
        device_name: &str,
        session: &OpenXRSession,
        new_pose: xr::Posef,
    ) -> xr::Result<()> {
        for device in &self.devices {
            if device.name == device_name {
                return device.update_pose(session, new_pose);
            }
        }
        Err(xr::sys::Result::ERROR_HANDLE_INVALID.into()) // Return an error if device is not found
    }
}
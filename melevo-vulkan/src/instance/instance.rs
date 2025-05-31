use std::sync::Arc;
use crate::PhysicalDevice;
use super::InstanceBuilder;

#[derive(Clone)]
pub struct Instance {
	pub(super) inner: Arc<InstanceInner>,
}

impl Instance {
	pub fn builder() -> InstanceBuilder {
		InstanceBuilder::default()
	}

	pub fn enumerate_physical_devices(&self) -> Vec<PhysicalDevice> {
		let physical_devices = unsafe { self.handle().enumerate_physical_devices().expect("cannot enumerate physical devices") };

		physical_devices.iter().map(|device| {
			PhysicalDevice::from_handle(*device, self.clone())
		}).collect()
	}
	
	pub(crate) fn entry(&self) -> &ash::Entry {
		&self.inner.entry
	}

	pub(crate) fn handle(&self) -> &ash::Instance {
		&self.inner.handle 
	}
}


pub(super) struct InstanceInner {
	pub(super) entry: ash::Entry,
	pub(super) handle: ash::Instance,
}

impl Drop for InstanceInner {
	fn drop(&mut self) {
		unsafe { self.handle.destroy_instance(None) };
	}
}


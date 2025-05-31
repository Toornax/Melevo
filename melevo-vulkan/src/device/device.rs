use std::sync::Arc;

use crate::{Instance, VulkanError};
use super::{queue::{Queue, QueueFamilyInfo}, DeviceBuilder, PhysicalDevice, QueueFlags};

#[derive(Clone)]
pub struct Device {
	pub(crate) inner: Arc<DeviceInner>
}

impl Device {
	pub fn builder(physical_device: PhysicalDevice) -> DeviceBuilder {
		DeviceBuilder::new(physical_device)
	}

	pub fn get_queue(&self, queue_flags: QueueFlags, present_support: bool, index: u32) -> Result<Queue, VulkanError> {
		let family = self.inner.queue_families
			.iter()
			.find(|f| f.flags.contains(queue_flags) && present_support == f.present_support)
			.ok_or(VulkanError::QueueFamilyNotFound)?;

		if index >= family.count {
			return Err(VulkanError::InvalidQueueIndex { index, max: family.count });
		}

		let vk_queue = unsafe { self.inner.handle.get_device_queue(family.family_index, index) };

		Ok(Queue::from_handle(vk_queue))
	}

	pub(crate) fn handle(&self) -> &ash::Device {
		&self.inner.handle
	}

	pub(crate) fn instance(&self) -> &Instance {
		&self.inner.instance
	}
}

pub(crate) struct DeviceInner {
	pub(crate) handle: ash::Device,
	pub(crate) instance: Instance,
	pub(crate) queue_families: Vec<QueueFamilyInfo>,
}

use std::{ffi::{c_char, CString}, sync::Arc};

use crate::{Instance, Surface, VulkanError};

use super::{device::DeviceInner, queue::QueueFamilyInfo, Device, DeviceExtensions, DeviceFeatures, DeviceFeaturesFfi, PhysicalDevice, QueueFlags};

pub struct DeviceBuilder {
	instance: Instance,
	physical_device: PhysicalDevice,

	enabled_extensions: DeviceExtensions,
	enabled_features: DeviceFeatures,
	requested_queues: Vec<QueueFamilyInfo>,
}

impl DeviceBuilder {
	pub(super) fn new(physical_device: PhysicalDevice) -> Self {
		DeviceBuilder {
			instance: physical_device.instance(),
			physical_device,

			enabled_extensions: DeviceExtensions::default(),
			enabled_features: DeviceFeatures::default(),
			requested_queues: Vec::new(),
		}
	}

	pub fn build(self) -> Device {
		let mut device_queue_create_info_list: Vec<ash::vk::DeviceQueueCreateInfo> = Vec::new();

		for requested_queue in &self.requested_queues {
			let device_queue_create_info = ash::vk::DeviceQueueCreateInfo {
				queue_family_index: requested_queue.family_index,
				queue_count: requested_queue.count,
				p_queue_priorities: requested_queue.priorities.as_ptr(),
				..Default::default()
			};

			device_queue_create_info_list.push(device_queue_create_info);
		}

		let cstring_enabled_extensions_names: Vec<CString> = self.enabled_extensions
			.iter()
			.filter(|(_, enabled)| *enabled)
			.map(|(name, _)| CString::new(name).expect("cannot create CString from extension name"))
			.collect();

		let enabled_extensions_names: Vec<*const c_char> = cstring_enabled_extensions_names
			.iter()
			.map(|name| name.as_ptr())
			.collect();

		let mut features_ffi = DeviceFeaturesFfi::from(&self.enabled_features);
		features_ffi.chain_struct();

		let device_create_info = ash::vk::DeviceCreateInfo {
			queue_create_info_count: device_queue_create_info_list.len() as u32,
			p_queue_create_infos: device_queue_create_info_list.as_ptr(),
			enabled_extension_count: enabled_extensions_names.len() as u32,
			pp_enabled_extension_names: enabled_extensions_names.as_ptr(),
			p_next: <*mut _>::cast(&mut features_ffi.vk_features),
			p_enabled_features: std::ptr::null(),
			..Default::default()
		};

		let device = unsafe { self.instance.handle().create_device(self.physical_device.handle(), &device_create_info, None).expect("cannot create logical device") };

		let device_inner = DeviceInner {
			handle: device,
			instance: self.instance,
			queue_families: self.requested_queues,
		};

		Device {
			inner: Arc::new(device_inner)
		}
	}

	pub fn add_queue(mut self, queue_flags: QueueFlags, surface: Option<&Surface>, count: u32, priorities: impl Into<Vec<f32>>) -> Result<Self, VulkanError> {
		let family_index = self.physical_device.queue_family_id(queue_flags, surface)?;
		
		self.requested_queues.push(QueueFamilyInfo { family_index, count, flags: queue_flags, present_support: surface.is_some() ,priorities: priorities.into() });

		Ok(self)
	}

	pub fn enable_extensions(mut self, enabled_extensions: DeviceExtensions) -> Self {
		self.enabled_extensions |= enabled_extensions;

		self
	}

	pub fn enable_features(mut self, enabled_features: DeviceFeatures) -> Self {
		self.enabled_features |= enabled_features;

		self
	}
}
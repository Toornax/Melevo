use std::{ffi::CStr, sync::Arc};

use utils::Version;

use crate::{utils::FromVulkanVersion, DeviceExtensions, DeviceFeatures, DeviceFeaturesFfi, Instance, Surface, VulkanError};

use super::{QueueFamilyID, QueueFamilyProperties, QueueFlags};

#[derive(Clone)]
pub struct PhysicalDevice {
	inner: Arc<PhysicalDeviceInner>
}

impl PhysicalDevice {
	pub(crate) fn from_handle(handle: ash::vk::PhysicalDevice, instance: Instance) -> Self {
		let inner = PhysicalDeviceInner::new(handle, instance);

		Self { inner: Arc::new(inner) }
	}

	pub(crate) fn handle(&self) -> ash::vk::PhysicalDevice {
		self.inner.handle
	}

	pub(crate) fn instance(&self) -> Instance {
		self.inner.instance.clone()
	}

	
	pub fn name(&self) -> String {
		unsafe {
			CStr::from_ptr(self.inner.properties.device_name.as_ptr())
				.to_string_lossy()
				.into_owned()
		}
	}

	pub fn api_version(&self) -> Version {
		Version::from_vk_version(self.inner.properties.api_version)
	}

	pub fn supported_extensions(&self) -> &DeviceExtensions {
		&self.inner.supported_extensions
	}

	pub fn supported_features(&self) -> &DeviceFeatures {
		&self.inner.supported_features
	}

	pub fn queue_family_properties(&self) -> &Vec<QueueFamilyProperties> {
		&self.inner.queue_family_properties
	}

	pub fn queue_family_id(&self, queue_flags: QueueFlags, opt_surface: Option<&Surface>) -> Result<QueueFamilyID, VulkanError> {
		for (i, queue_properties) in self.inner.queue_family_properties.iter().enumerate() {
			if queue_properties.queue_flags.contains(queue_flags) {
				if let Some(surface) = opt_surface {
					if self.support_surface_presentation(i as u32, surface) {						
						return Ok(i as QueueFamilyID)
					}
				} else {
					return Ok(i as QueueFamilyID)
				}
			}
		}

		Err(VulkanError::QueueFamilyNotFound)
	}

	pub fn support_surface_presentation(&self, queue_family_index: u32, surface: &Surface) -> bool {
		unsafe { surface.instance().get_physical_device_surface_support(self.inner.handle, queue_family_index, surface.handle()).expect("cannot check device surface support") }
	}
}

struct PhysicalDeviceInner {
	handle: ash::vk::PhysicalDevice,
	instance: Instance,

	properties: ash::vk::PhysicalDeviceProperties,
	supported_extensions: DeviceExtensions,
	supported_features: DeviceFeatures,
	queue_family_properties: Vec<QueueFamilyProperties>,
}

impl PhysicalDeviceInner {
	fn new(handle: ash::vk::PhysicalDevice, instance: Instance) -> Self {
		let properties = Self::get_device_properties(handle, &instance);
		let supported_extensions = Self::get_supported_extensions(handle, &instance);
		let supported_features = Self::get_supported_features(handle, &instance);
		let queue_family_properties = Self::get_queue_family_properties(handle, &instance);

		Self {
			handle,
			instance,

			properties,
			supported_extensions,
			supported_features,
			queue_family_properties,
		}
	}

	fn get_device_properties(handle: ash::vk::PhysicalDevice, instance: &Instance) -> ash::vk::PhysicalDeviceProperties {
		unsafe { instance.handle().get_physical_device_properties(handle) }
	}

	fn get_supported_extensions(handle: ash::vk::PhysicalDevice, instance: &Instance) -> DeviceExtensions {
		let extension_list = unsafe { instance.handle().enumerate_device_extension_properties(handle).expect("cannot enumerate device extensions") };
		
		extension_list
			.iter()
			.map(|ext| {
				let raw_ext_name = ext.extension_name_as_c_str().expect("cannot read extension name");
				raw_ext_name.to_str().expect("cannot convert CStr to &str")
			})
			.collect()
	}

	fn get_supported_features(handle: ash::vk::PhysicalDevice, instance: &Instance) -> DeviceFeatures {
		let mut features_ffi = DeviceFeaturesFfi::default();
		features_ffi.chain_struct();

		unsafe { instance.handle().get_physical_device_features2(handle, &mut features_ffi.vk_features); }
		
		features_ffi.into()
	}
	
	fn get_queue_family_properties(handle: ash::vk::PhysicalDevice, instance: &Instance) -> Vec<QueueFamilyProperties> {
		unsafe { instance.handle().get_physical_device_queue_family_properties(handle) }
			.iter().map(|prop| QueueFamilyProperties::from(*prop) )
			.collect()
	}
}
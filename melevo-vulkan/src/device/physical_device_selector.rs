#![allow(dead_code)]

use crate::{Instance, PhysicalDevice, Surface};

use utils::Version;

use super::{DeviceExtensions, DeviceFeatures, QueueFlags};

type RequiredFn = dyn Fn(&PhysicalDevice) -> bool;
type PreferedFn  = dyn Fn(&PhysicalDevice) -> u32;

pub struct PhysicalDeviceSelector {
	instance: Instance,
	required: Vec<Box<RequiredFn>>,
	preferred: Vec<Box<PreferedFn>>,
}

impl PhysicalDeviceSelector {
	pub fn new(instance: Instance) -> Self {
		Self {
			instance,
			required: Vec::new(),
			preferred: Vec::new(),
		}
	}


	pub fn require<F>(mut self, predicate: F) -> Self	
		where F: 'static + Fn(&PhysicalDevice) -> bool
	{
		self.required.push(Box::new(predicate));
		self
	}

	pub fn prefer<F>(mut self, scorer: F) -> Self 
		where F: 'static + Fn(&PhysicalDevice) -> u32
	{
		self.preferred.push(Box::new(scorer));
		self
	}


	pub fn require_api_version(self, version: Version) -> Self {
		self.require(move |device| device.api_version() >= version)
	}

	pub fn require_extensions(self, device_extensions: DeviceExtensions) -> Self {
		self.require(move |device| device.supported_extensions().contains(&device_extensions))
	}

	pub fn require_features(self, device_features: DeviceFeatures) -> Self {
		self.require(move |device| device.supported_features().contains(&device_features))
	}

	pub fn require_queue_family(self, queue_flags: QueueFlags) -> Self {
		self.require(move |device| { 
			device.queue_family_properties()
				.iter().any(|queue| {
					queue.queue_flags.contains(queue_flags)
				})
		})
	}

	pub fn require_surface(self, surface: Surface) -> Self {
		self.require(move |device| {
			device.queue_family_properties()
				.iter()
				.enumerate()
				.any(|(queue_id, _)| {
					device.support_surface_presentation(queue_id as u32, &surface)
				})
		})
	}

	pub fn select(self) -> Option<PhysicalDevice> {
		self.select_all().get(0).cloned()
	}

	pub fn select_all(self) -> Vec<PhysicalDevice> {
		let mut candidates = self.instance.enumerate_physical_devices();
		candidates.retain(|device| self.required.iter().all(|pred| pred(&device)));
		
		candidates
	}
}


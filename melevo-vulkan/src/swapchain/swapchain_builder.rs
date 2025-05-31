use std::sync::Arc;
use ash::vk::SwapchainCreateInfoKHR;

use crate::{Device, Extent2D, Image, ImageUsageFlags, PresentMode, Surface, SurfaceFormat};

use super::{swapchain::SwapchainInner, Swapchain};

pub struct SwapchainBuilder {
	device: Device,
	surface: Surface,
	
	desired_format: SurfaceFormat,
	desired_present_mode: PresentMode,
	desired_extent: Extent2D,
	image_usage_flags: ImageUsageFlags,
	min_image_count: u32,
}

impl SwapchainBuilder {
	pub(super) fn new(device: Device, surface: Surface) -> Self {
		let desired_format = SurfaceFormat::default();
		let desired_present_mode = PresentMode::default();
		let desired_extent = Extent2D::default();
		let image_usage_flags = ImageUsageFlags::default();

		Self {
			device,
			surface,

			desired_format,
			desired_present_mode,
			desired_extent,
			image_usage_flags,
			min_image_count: 1,
		}
	}

	pub fn build(self) -> Swapchain {
		let swapchain_create_info = SwapchainCreateInfoKHR {
			surface: self.surface.handle(),
			min_image_count: self.min_image_count, // Typically at least 2 for double buffering
			image_format: self.desired_format.format,
			image_color_space: self.desired_format.color_space,
			image_extent: self.desired_extent,
			image_array_layers: 1, // Single layer for 2D images
			image_usage: self.image_usage_flags,

			..Default::default()
		};

		let vk_swapchain = unsafe { self.device.as_swapchain_device().create_swapchain(&swapchain_create_info, None).expect("cannot create swapchain") };
		let swapchain_device = self.device.as_swapchain_device();

		let images = unsafe {
			swapchain_device.get_swapchain_images(vk_swapchain).expect("Failed to get swapchain images")
				.into_iter()
				.map(|image| Image::from_existing(image, self.desired_format.format, self.device.clone()))
				.collect::<Vec<_>>()
		};

		let image_views = images.iter()
			.map(|image| image.view().clone())
			.collect::<Vec<_>>();

		let inner_swapchain = SwapchainInner {
			handle: vk_swapchain,
			device: self.device,
			images,
			image_views,
		};

		Swapchain {
			inner: Arc::new(inner_swapchain),
		}
	}

	pub fn set_desired_format(mut self, surface_format: SurfaceFormat) -> Self {
		self.desired_format = surface_format;

		self
	}

	pub fn set_desired_present_mode(mut self, present_mode: PresentMode) -> Self {
		self.desired_present_mode = present_mode;

		self
	}

	pub fn set_desired_extent(mut self, extent: Extent2D) -> Self {
		self.desired_extent = extent;

		self
	}

	pub fn set_image_usage_flags(mut self, image_usage_flags: ImageUsageFlags) -> Self {
		self.image_usage_flags = image_usage_flags;

		self
	}

	pub fn set_min_image_count(mut self, min_image_count: u32) -> Self {
		self.min_image_count = min_image_count;
		
		self
	}
}

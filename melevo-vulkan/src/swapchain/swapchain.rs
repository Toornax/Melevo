use std::sync::Arc;

use crate::{Device, Image, ImageView, Surface};

use super::SwapchainBuilder;

#[derive(Clone)]
pub struct Swapchain {
	pub(super) inner: Arc<SwapchainInner>,
}

impl Swapchain {
	pub fn builder(device: Device, surface: Surface) -> SwapchainBuilder {
		SwapchainBuilder::new(device, surface)
	}

	pub(crate) fn handle(&self) -> ash::vk::SwapchainKHR {
		self.inner.handle
	}

	pub fn get_images(&self) -> &Vec<Image> {
		&self.inner.images
	}

	pub fn get_image_views(&self) -> &Vec<ImageView> {
		&self.inner.image_views
	}
}

pub(super) struct SwapchainInner {
	pub(super) handle: ash::vk::SwapchainKHR,
	pub(super) device: Device,

	pub(super) images: Vec<Image>,
	pub(super) image_views: Vec<ImageView>,
}

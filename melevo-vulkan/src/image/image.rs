use std::{cell::OnceCell, sync::Arc};

use crate::{Device, Format};

use super::ImageView;

#[derive(Clone)]
pub struct Image {
	inner: Arc<ImageInner>,
}

impl Image {
	pub(crate) fn from_existing(handle: ash::vk::Image, format: Format, device: Device) -> Self {
		let inner = Arc::new(ImageInner { 
			handle, 
			memory: None,
			device,
			format,
			view: OnceCell::new(),
			is_owned: false,
		});
		
		Self {
			inner: inner,
		}
	}

	pub(crate) fn handle(&self) -> ash::vk::Image {
		self.inner.handle
	}

	pub fn view(&self) -> &ImageView {
		self.inner.view.get_or_init(|| {
			ImageView::new(self.inner.clone())
		})
	}	
}

pub(super) struct ImageInner {
	pub(super) handle: ash::vk::Image,
	memory: Option<ash::vk::DeviceMemory>,
	pub(super) device: Device,
	view: OnceCell<ImageView>,
	pub(super) format: Format,
	is_owned: bool,
}

impl Drop for ImageInner {
	fn drop(&mut self) {
		if self.is_owned {
			unsafe {
				self.device.handle().destroy_image(self.handle, None);
				if let Some(memory) = self.memory {
					self.device.handle().free_memory(memory, None);
				}
			}
		}
	}
}
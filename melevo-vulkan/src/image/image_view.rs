use std::sync::Arc;

use super::image::ImageInner;

#[derive(Clone)]
pub struct ImageView {
	inner: Arc<ImageViewInner>,
}

impl ImageView {
	pub(crate) fn new(image: Arc<ImageInner>) -> Self {
		let image_view_create_info = ash::vk::ImageViewCreateInfo {
			image: image.handle,
			view_type: ash::vk::ImageViewType::TYPE_2D,
			format: image.format,
			subresource_range: ash::vk::ImageSubresourceRange {
				aspect_mask: ash::vk::ImageAspectFlags::COLOR,
				base_mip_level: 0,
				level_count: 1,
				base_array_layer: 0,
				layer_count: 1,
			},
			..Default::default()
		};

		let handle = unsafe { image.device.handle().create_image_view(&image_view_create_info, None).expect("cannot create image view") };

		Self {
			inner: Arc::new(ImageViewInner { 
				handle,
				_image_inner: image, // Keep a reference to the image to ensure it is not dropped while the view exists
			}),
		}
	}

	pub(crate) fn handle(&self) -> ash::vk::ImageView {
		self.inner.handle
	}	
}

struct ImageViewInner {
	handle: ash::vk::ImageView,
	_image_inner: Arc<ImageInner>, // Keep a reference to the image to ensure it is not dropped while the view exists
}

impl Drop for ImageViewInner {
	fn drop(&mut self) {
		unsafe {
			self._image_inner.device.handle().destroy_image_view(self.handle, None);
		}
	}
}

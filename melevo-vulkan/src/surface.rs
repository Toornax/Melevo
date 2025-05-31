use std::sync::Arc;

use winit::{raw_window_handle::{HasDisplayHandle, HasWindowHandle}, window::Window};

use crate::Instance;

#[derive(Clone)]
pub struct Surface {
	inner: Arc<SurfaceInner>
}

impl Surface {
	pub fn create(instance: &Instance, window: Arc<Window>) -> Self {
		let inner = Arc::new(SurfaceInner::create(instance, window));

		Self {
			inner
		}
	}

	pub(crate) fn handle(&self) -> ash::vk::SurfaceKHR {
		self.inner.handle
	}

	pub(crate) fn instance(&self) -> &ash::khr::surface::Instance {
		&self.inner.instance 
	}
}

struct SurfaceInner {
	instance: ash::khr::surface::Instance,
	handle: ash::vk::SurfaceKHR,
} 

impl SurfaceInner {
	pub fn create(instance: &Instance, window: Arc<Window>) -> Self {
		let surface_instance = ash::khr::surface::Instance::new(instance.entry(), instance.handle());

		let handle = unsafe { 
			ash_window::create_surface(
				instance.entry(), 
				instance.handle(), 
				window.display_handle().expect("no display handle").as_raw(), 
				window.window_handle().expect("no window handle").as_raw(), 
				None).expect("cannot create surface")
		};

		Self {
			instance: surface_instance,
			handle,
		}
	}
}

impl Drop for SurfaceInner {
	fn drop(&mut self) {
		unsafe { self.instance.destroy_surface(self.handle, None) };
	}
}


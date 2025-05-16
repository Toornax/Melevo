use std::sync::Arc;

use crate::Instance;

pub struct Surface {
	inner: Arc<SurfaceInner>
}

impl Surface {
	pub fn create(instance: &Instance) -> Self {
		let inner = Arc::new(SurfaceInner::create(instance));

		Self {
			inner
		}
	}
}

struct SurfaceInner {
	vk_surface: ash::khr::surface::Instance,
} 

impl SurfaceInner {
	pub fn create(instance: &Instance) -> Self {
		let vk_surface = ash::khr::surface::Instance::new(instance.entry(), instance.inner());

		Self {
			vk_surface
		}
	}
}


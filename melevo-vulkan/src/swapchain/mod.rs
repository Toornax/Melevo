mod swapchain;
pub use swapchain::Swapchain;

mod swapchain_builder;
pub use swapchain_builder::SwapchainBuilder;

use crate::Device;

impl Device {
	fn as_swapchain_device(&self) -> ash::khr::swapchain::Device {
		ash::khr::swapchain::Device::new(&self.inner.instance.handle(), &self.inner.handle)
	}
}
mod device;
mod error;
mod image;
mod instance;
mod surface;
mod swapchain;
mod utils;
mod vk_type;


pub use device::{
	Device,
	DeviceBuilder,
	DeviceExtensions,
	DeviceFeatures,
	PhysicalDevice,
	PhysicalDeviceSelector,
	QueueFamilyProperties,
	QueueFlags,
};
pub use error::VulkanError;
pub use image::{Image, ImageView};
pub use instance::Instance;
pub use surface::Surface;
pub use swapchain::{Swapchain, SwapchainBuilder};
pub use vk_type::*;

pub(crate) use device::DeviceFeaturesFfi;


const VALIDATION_LAYERS: &[&str] = &[
	"VK_LAYER_KHRONOS_validation"
];

pub mod version {
	use utils::Version;

	pub const V1_0: Version = Version { major: 1, minor: 0, patch: 0, pre_rel: None, meta: None };
	pub const V1_1: Version = Version { major: 1, minor: 1, patch: 0, pre_rel: None, meta: None };
	pub const V1_2: Version = Version { major: 1, minor: 2, patch: 0, pre_rel: None, meta: None };
	pub const V1_3: Version = Version { major: 1, minor: 3, patch: 0, pre_rel: None, meta: None };
	pub const V1_4: Version = Version { major: 1, minor: 4, patch: 0, pre_rel: None, meta: None };
	pub const V1_5: Version = Version { major: 1, minor: 5, patch: 0, pre_rel: None, meta: None };
	pub const V1_6: Version = Version { major: 1, minor: 6, patch: 0, pre_rel: None, meta: None };
}

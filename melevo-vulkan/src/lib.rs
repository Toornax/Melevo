pub use instance::Instance;

mod instance;
mod utils;

const VALIDATION_LAYERS: &[&str] = &[
	"VK_LAYER_KHRONOS_validation"
];


pub mod version {
	pub const V1_0: u32 = ash::vk::make_api_version(0, 1, 0, 0);
	pub const V1_1: u32 = ash::vk::make_api_version(0, 1, 1, 0);
	pub const V1_2: u32 = ash::vk::make_api_version(0, 1, 2, 0);
	pub const V1_3: u32 = ash::vk::make_api_version(0, 1, 3, 0);
	pub const V1_4: u32 = ash::vk::make_api_version(0, 1, 4, 0);
	pub const V1_5: u32 = ash::vk::make_api_version(0, 1, 5, 0);
	pub const V1_6: u32 = ash::vk::make_api_version(0, 1, 6, 0);
}

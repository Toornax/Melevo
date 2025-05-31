use std::sync::Arc;

use utils::Version;
use winit::{raw_window_handle_05::HasRawDisplayHandle, raw_window_handle::HasDisplayHandle, window::Window};

pub struct Engine {
	instance: vk::Instance,
}

impl Engine {
	pub fn init(event_loop: &(impl HasDisplayHandle + HasRawDisplayHandle), window: Arc<Window>) -> Result<Self, vk::VulkanError> {
		log::info!("Initalize engine...");
		let instance = vk::Instance::builder()
			.set_app_name(env!("CARGO_PKG_NAME").to_string())
			.set_app_version(Version::new(env!("CARGO_PKG_VERSION")).expect("invalid app version"))
			.set_engine_name(env!("CARGO_PKG_NAME").to_string())
			.set_engine_version(Version::new(env!("CARGO_PKG_VERSION")).expect("invalid engine version"))
			.require_api_version(vk::version::V1_6)
			.enable_surface_extensions(event_loop)
			.create();

		let surface = vk::Surface::create(&instance, window);


		// Init device and queue
		let enabled_extensions = vk::DeviceExtensions {
			VK_KHR_swapchain: true,
			VK_KHR_dynamic_rendering: true,
			..Default::default()
		};

		let enabled_features = vk::DeviceFeatures {
			geometry_shader: true,
			dynamic_rendering: true,
			..Default::default()
		};

		let queue_flags = vk::QueueFlags::GRAPHICS;

		let physical_device = vk::PhysicalDeviceSelector::new(instance.clone())
			.require_api_version(vk::version::V1_4)
			.require_extensions(enabled_extensions.clone())
			.require_features(enabled_features.clone())
			.require_queue_family(queue_flags)
			.require_surface(surface.clone())
			.select()
			.expect("cannot found any valid GPU");

		let device = vk::Device::builder(physical_device)
			.enable_features(enabled_features)
			.enable_extensions(enabled_extensions)
			.add_queue(queue_flags, Some(&surface), 1, [1.0])?
			.build();

		let queue = device.get_queue(queue_flags, true, 0)?;

		// Init swapchain

		let surface_format = vk::SurfaceFormat {
			format: vk::Format::B8G8R8A8_SRGB,
			color_space: vk::ColorSpace::SRGB_NONLINEAR,
		};

		let swapchain = vk::Swapchain::builder(device.clone(), surface.clone())
			.set_desired_format(surface_format)
			.set_desired_present_mode(vk::PresentMode::default())
			.set_desired_extent(vk::Extent2D::default())
			.set_image_usage_flags(vk::ImageUsageFlags::default())
			.build();

		let swapchain_images = swapchain.get_images();
		let swapchain_image_views = swapchain.get_image_views();

		log::info!("Engine initialized.");
		Ok(Self {
			instance,
		})
	}

	pub fn render(&mut self) {

	}

	pub fn window_resized(&mut self) {
	}

}

impl Engine {
	fn init_vulkan() {
		
	}

	fn init_devices() {

	}

	fn init_swapchain() {		
		
	}

	fn init_pipeline() {
		
	}

	fn create_image_views() {
		
	}

	fn create_swapchain() {

	}

	fn init_frames() {
		
	}

	fn create_command_buffer() {

	}
}



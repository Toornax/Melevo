use std::sync::Arc;

use utils::Version;
use winit::{raw_window_handle_05::HasRawDisplayHandle, raw_window_handle::HasDisplayHandle, window::Window};

pub struct Engine {
	instance: Arc<vk::Instance>,
}

impl Engine {
	pub fn init(event_loop: &(impl HasDisplayHandle + HasRawDisplayHandle), window: Arc<Window>) -> Self {
		let instance = vk::Instance::builder()
			.set_app_name(env!("CARGO_PKG_NAME").to_string())
			.set_app_version(Version::new(env!("CARGO_PKG_VERSION")).expect("invalid app version"))
			.set_engine_name(env!("CARGO_PKG_NAME").to_string())
			.set_engine_version(Version::new(env!("CARGO_PKG_VERSION")).expect("invalid engine version"))
			.require_api_version(vk::version::V1_4)
			.enable_surface_extensions(event_loop)
			.create();

		Self {
			instance,
		}
	}

	pub fn render(&mut self) {

	}

	pub fn window_resized(&mut self) {
	}

}

impl Engine {
	fn init_vulkan() {
		
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



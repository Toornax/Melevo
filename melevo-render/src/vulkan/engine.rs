use std::sync::Arc;

use winit::{raw_window_handle_05::HasRawDisplayHandle, raw_window_handle::HasDisplayHandle, window::Window};

pub struct Engine {
}

impl Engine {
	pub fn init(event_loop: &(impl HasDisplayHandle + HasRawDisplayHandle), window: Arc<Window>) -> Self {

		Self {

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



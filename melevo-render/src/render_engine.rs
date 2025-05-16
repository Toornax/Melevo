use std::sync::Arc;

use super::vkr;
use winit::{raw_window_handle_05::HasRawDisplayHandle, raw_window_handle::HasDisplayHandle, window::Window};

pub struct RenderEngine {
	engine: vkr::Engine,
}

impl RenderEngine {
	pub fn init(event_loop: &(impl HasDisplayHandle + HasRawDisplayHandle), window: Arc<Window>) -> Self {
		let engine = vkr::Engine::init(event_loop, window);

		Self { 
			engine,
		}
	}
	
	pub fn render(&mut self) {
		self.engine.render();
	}

	pub fn window_resized(&mut self) {
		self.engine.window_resized();
	}
}
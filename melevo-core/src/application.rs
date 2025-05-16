use std::sync::Arc;

use render::RenderEngine;

use winit::{
	application::ApplicationHandler, event::WindowEvent, event_loop::{ActiveEventLoop, ControlFlow, EventLoop}, window::{Window, WindowId}
};

pub struct Melevo {
	app: Application,
	event_loop: EventLoop<()>,
}

impl Melevo {
	pub fn init() -> Self {
		let event_loop = EventLoop::new().unwrap();
		event_loop.set_control_flow(ControlFlow::Poll);

		let app = Application::init();

		Self {
			event_loop,
			app
		 }
	}

	pub fn run(mut self) {
		self.event_loop.run_app(&mut self.app).unwrap();
	}
}


struct Application {
	render_engine: Option<RenderEngine>,
	window: Option<Arc<Window>>
}

impl Application {
	pub fn init() -> Self {
		Self {
			render_engine: None,
			window: None,
		}
	}
}

impl ApplicationHandler for Application {
	fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
		let window =  Arc::new(event_loop.create_window(Window::default_attributes()).expect("Cannot create window."));

		let render_engine = RenderEngine::init(event_loop, window.clone());

		self.window = Some(window);
		self.render_engine = Some(render_engine);
	}

	fn window_event(&mut self, event_loop: &ActiveEventLoop, _window_id: WindowId, event: WindowEvent) {
		match event {
			WindowEvent::CloseRequested => {
				log::info!("The close button was pressed; stopping");
				event_loop.exit();
			},

			WindowEvent::Resized(_) => {				
				if let Some(render_engine) = &mut self.render_engine {
					render_engine.window_resized();
				}
			},
			
			WindowEvent::RedrawRequested => {
				self.window.as_ref().unwrap().request_redraw();

				if let Some(render_engine) = &mut self.render_engine {
					render_engine.render();
				}
			}

			_ => ()
		}
	}
}

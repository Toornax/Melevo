use std::{ffi::{CStr, CString}, ptr, sync::Arc};

use winit::{raw_window_handle_05::HasRawDisplayHandle, raw_window_handle::HasDisplayHandle};
use crate::{utils::ToVulkanVersion, VALIDATION_LAYERS};


pub struct Instance {
	_entry: ash::Entry,
	internal: ash::Instance,
}

impl Instance {
	pub fn builder() -> InstanceBuilder {
		InstanceBuilder::default()
	}
}

impl Drop for Instance {
	fn drop(&mut self) {
		unsafe { self.internal.destroy_instance(None) };
	}
}


#[derive(Debug, Default)]
struct ApplicationInfo {
	pub application_name: String,
	pub application_version: u32,
	pub engine_name: String,
	pub engine_version: u32,
	pub api_version: u32,
}

#[derive(Debug, Default)]
struct InstanceCreateInfo {
	pub enabled_layers: Vec<String>,
	pub enabled_extensions: Vec<String>,
	pub flags: ash::vk::InstanceCreateFlags,
}

#[derive(Debug, Default)]
pub struct InstanceBuilder {
	application_info: ApplicationInfo,
	instance_create_info: InstanceCreateInfo,
}

impl InstanceBuilder {
	pub fn create(self) -> Arc<Instance> {		
		let app_name = CString::new(self.application_info.application_name).expect("invalid application name");
		let engine_name = CString::new(self.application_info.engine_name).expect("invalid engine name");

		let app_info = ash::vk::ApplicationInfo {
			s_type: ash::vk::StructureType::APPLICATION_INFO,
			p_next: ptr::null(),
			p_application_name: app_name.as_ptr(),
			application_version: self.application_info.application_version,
			p_engine_name: engine_name.as_ptr(),
			engine_version: self.application_info.engine_version,
			api_version: self.application_info.api_version,
			_marker: std::marker::PhantomData
		};

		let layer_names: Vec<CString> = self.instance_create_info.enabled_layers
			.iter()
			.map(|layer| CString::new(layer.as_str()).expect("invalid layer name"))
			.collect();

		let layer_names_ptr: Vec<*const i8> = layer_names
			.iter()
			.map(|cstr| cstr.as_ptr())
			.collect();

		let ext_names: Vec<CString> = self.instance_create_info.enabled_extensions
			.iter()
			.map(|ext| CString::new(ext.as_str()).expect("invalid extension name"))
			.collect();

		let ext_names_ptr: Vec<*const i8> = ext_names
			.iter()
			.map(|cstr| cstr.as_ptr())
			.collect();

		let inst_create_info = ash::vk::InstanceCreateInfo {
			s_type: ash::vk::StructureType::INSTANCE_CREATE_INFO,
			p_next: ptr::null(),
			flags: self.instance_create_info.flags,
			p_application_info: &app_info,
			enabled_layer_count: self.instance_create_info.enabled_layers.len() as u32,
			pp_enabled_layer_names: layer_names_ptr.as_ptr(),
			enabled_extension_count: self.instance_create_info.enabled_extensions.len() as u32,
			pp_enabled_extension_names: ext_names_ptr.as_ptr(),
			_marker: std::marker::PhantomData,
		};

		let entry = unsafe { ash::Entry::load().expect("cannot load vulkan library") };
		let instance = unsafe { entry.create_instance(&inst_create_info, None).expect("instance creation error") };

		let instance = Instance {
			_entry: entry,
			internal: instance,
		};

		Arc::new(instance)
	}

	pub fn set_app_name(mut self, app_name: String) -> Self {
		self.application_info.application_name = app_name;

		self
	}

	pub fn set_app_version<T: ToVulkanVersion>(mut self, app_version: T) -> Self {
		self.application_info.application_version = app_version.to_vk_version();

		self
	}

	pub fn set_engine_name(mut self, engine_name: String) -> Self {
		self.application_info.engine_name = engine_name;

		self
	}

	pub fn set_engine_version<T: ToVulkanVersion>(mut self, engine_version: T) -> Self {
		self.application_info.engine_version = engine_version.to_vk_version();

		self
	}

	pub fn require_api_version<T: ToVulkanVersion>(mut self, api_version: T) -> Self {
		self.application_info.api_version = api_version.to_vk_version();

		self
	}

	pub fn enable_layer(mut self, layer: String) -> Self {
		self.instance_create_info.enabled_layers.push(layer);

		self
	}

	pub fn enable_layers(mut self, mut layers: Vec<String>) -> Self {
		self.instance_create_info.enabled_layers.append(&mut layers);

		self
	}

	pub fn enable_extension(mut self, extension: String) -> Self {
		self.instance_create_info.enabled_extensions.push(extension);

		self
	}

	pub fn enable_extensions(mut self, mut extensions: Vec<String>) -> Self {
		self.instance_create_info.enabled_extensions.append(&mut extensions);

		self
	}

	pub fn enable_validation_layers(self) -> Self {
		self.enable_layers(VALIDATION_LAYERS.iter().map(|&s| s.to_string()).collect())
	}

	pub fn enable_surface_extensions(self, event_loop: &(impl HasDisplayHandle + HasRawDisplayHandle)) -> Self {
		let display_handle = event_loop.display_handle().unwrap();
		let extensions_raw = ash_window::enumerate_required_extensions(display_handle.as_raw()).expect("cannot get surface extensions");

		let extensions = extensions_raw.iter()
			.map(|&ptr| {
				let cstr = unsafe { CStr::from_ptr(ptr) };
				cstr.to_string_lossy().into_owned()
			})
			.collect();

		self.enable_extensions(extensions)
	}
}

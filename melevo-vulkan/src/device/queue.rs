pub type QueueFamilyProperties = ash::vk::QueueFamilyProperties;
pub type QueueFlags = ash::vk::QueueFlags;
pub type QueueFamilyID = u32;


#[derive(Debug, Clone)]
pub(super) struct QueueFamilyInfo {
	pub(super) family_index: u32,
	pub(super) flags: QueueFlags,
	pub(super) present_support: bool,
	pub(super) count: u32,
	pub(super) priorities: Vec<f32>,
}

pub struct Queue {
	inner: QueueInner,
}

impl Queue {
	pub(super) fn from_handle(handle: ash::vk::Queue) -> Self {
		Self {
			inner: QueueInner { handle },
		}
	}
}

struct QueueInner {
	handle: ash::vk::Queue,
}



#[derive(Debug)]
pub enum VulkanError {
	QueueFamilyNotFound,
	InvalidQueueIndex { index: u32, max: u32 },
}


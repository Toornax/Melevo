pub trait ToVulkanVersion {
	fn to_vk_version(&self) -> u32;
}

impl ToVulkanVersion for versions::SemVer {
	fn to_vk_version(&self) -> u32 {
		ash::vk::make_api_version(0, self.major, self.minor, self.patch)
	}
}

impl ToVulkanVersion for u32 {
	fn to_vk_version(&self) -> u32 {
		*self
	}
}
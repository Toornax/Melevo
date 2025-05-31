use utils::Version;

pub trait ToVulkanVersion {
	fn to_vk_version(&self) -> u32;
}

impl ToVulkanVersion for Version {
	fn to_vk_version(&self) -> u32 {
		ash::vk::make_api_version(0, self.major, self.minor, self.patch)
	}
}

impl ToVulkanVersion for u32 {
	fn to_vk_version(&self) -> u32 {
		*self
	}
}

pub trait FromVulkanVersion {
	fn from_vk_version(vk_version: u32) -> Self;
}

impl FromVulkanVersion for Version {
	fn from_vk_version(vk_version: u32) -> Self {
		let mut version = Version::default();
		version.major = ash::vk::api_version_major(vk_version);
		version.minor = ash::vk::api_version_minor(vk_version);
		version.patch = ash::vk::api_version_patch(vk_version);

		version
	}
}

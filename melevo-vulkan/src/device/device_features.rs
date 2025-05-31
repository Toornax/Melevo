use macros::FlagSet;

#[derive(Default, Clone, FlagSet)]
pub struct DeviceFeatures {
	// Vulkan 1.0 physical device features
	pub robust_buffer_access: bool,
	pub full_draw_index_uint32: bool,
	pub image_cube_array: bool,
	pub independent_blend: bool,
	pub geometry_shader: bool,
	pub tessellation_shader: bool,
	pub sample_rate_shading: bool,
	pub dual_src_blend: bool,
	pub logic_op: bool,
	pub multi_draw_indirect: bool,
	pub draw_indirect_first_instance: bool,
	pub depth_clamp: bool,
	pub depth_bias_clamp: bool,
	pub fill_mode_non_solid: bool,
	pub depth_bounds: bool,
	pub wide_lines: bool,
	pub large_points: bool,
	pub alpha_to_one: bool,
	pub multi_viewport: bool,
	pub sampler_anisotropy: bool,
	pub texture_compression_etc2: bool,
	pub texture_compression_astc_ldr: bool,
	pub texture_compression_bc: bool,
	pub occlusion_query_precise: bool,
	pub pipeline_statistics_query: bool,
	pub vertex_pipeline_stores_and_atomics: bool,
	pub fragment_stores_and_atomics: bool,
	pub shader_tessellation_and_geometry_point_size: bool,
	pub shader_image_gather_extended: bool,
	pub shader_storage_image_extended_formats: bool,
	pub shader_storage_image_multisample: bool,
	pub shader_storage_image_read_without_format: bool,
	pub shader_storage_image_write_without_format: bool,
	pub shader_uniform_buffer_array_dynamic_indexing: bool,
	pub shader_sampled_image_array_dynamic_indexing: bool,
	pub shader_storage_buffer_array_dynamic_indexing: bool,
	pub shader_storage_image_array_dynamic_indexing: bool,
	pub shader_clip_distance: bool,
	pub shader_cull_distance: bool,
	pub shader_float64: bool,
	pub shader_int64: bool,
	pub shader_int16: bool,
	pub shader_resource_residency: bool,
	pub shader_resource_min_lod: bool,
	pub sparse_binding: bool,
	pub sparse_residency_buffer: bool,
	pub sparse_residency_image2_d: bool,
	pub sparse_residency_image3_d: bool,
	pub sparse_residency2_samples: bool,
	pub sparse_residency4_samples: bool,
	pub sparse_residency8_samples: bool,
	pub sparse_residency16_samples: bool,
	pub sparse_residency_aliased: bool,
	pub variable_multisample_rate: bool,
	pub inherited_queries: bool,

	// Vulkan 1.1 physical device features
	pub storage_buffer16_bit_access: bool,
	pub uniform_and_storage_buffer16_bit_access: bool,
	pub storage_push_constant16: bool,
	pub storage_input_output16: bool,
	pub multiview: bool,
	pub multiview_geometry_shader: bool,
	pub multiview_tessellation_shader: bool,
	pub variable_pointers_storage_buffer: bool,
	pub variable_pointers: bool,
	pub protected_memory: bool,
	pub sampler_ycbcr_conversion: bool,
	pub shader_draw_parameters: bool,

	// Vulkan 1.2 physical device features
	pub sampler_mirror_clamp_to_edge: bool,
	pub draw_indirect_count: bool,
	pub storage_buffer8_bit_access: bool,
	pub uniform_and_storage_buffer8_bit_access: bool,
	pub storage_push_constant8: bool,
	pub shader_buffer_int64_atomics: bool,
	pub shader_shared_int64_atomics: bool,
	pub shader_float16: bool,
	pub shader_int8: bool,
	pub descriptor_indexing: bool,
	pub shader_input_attachment_array_dynamic_indexing: bool,
	pub shader_uniform_texel_buffer_array_dynamic_indexing: bool,
	pub shader_storage_texel_buffer_array_dynamic_indexing: bool,
	pub shader_uniform_buffer_array_non_uniform_indexing: bool,
	pub shader_sampled_image_array_non_uniform_indexing: bool,
	pub shader_storage_buffer_array_non_uniform_indexing: bool,
	pub shader_storage_image_array_non_uniform_indexing: bool,
	pub shader_input_attachment_array_non_uniform_indexing: bool,
	pub shader_uniform_texel_buffer_array_non_uniform_indexing: bool,
	pub shader_storage_texel_buffer_array_non_uniform_indexing: bool,
	pub descriptor_binding_uniform_buffer_update_after_bind: bool,
	pub descriptor_binding_sampled_image_update_after_bind: bool,
	pub descriptor_binding_storage_image_update_after_bind: bool,
	pub descriptor_binding_storage_buffer_update_after_bind: bool,
	pub descriptor_binding_uniform_texel_buffer_update_after_bind: bool,
	pub descriptor_binding_storage_texel_buffer_update_after_bind: bool,
	pub descriptor_binding_update_unused_while_pending: bool,
	pub descriptor_binding_partially_bound: bool,
	pub descriptor_binding_variable_descriptor_count: bool,
	pub runtime_descriptor_array: bool,
	pub sampler_filter_minmax: bool,
	pub scalar_block_layout: bool,
	pub imageless_framebuffer: bool,
	pub uniform_buffer_standard_layout: bool,
	pub shader_subgroup_extended_types: bool,
	pub separate_depth_stencil_layouts: bool,
	pub host_query_reset: bool,
	pub timeline_semaphore: bool,
	pub buffer_device_address: bool,
	pub buffer_device_address_capture_replay: bool,
	pub buffer_device_address_multi_device: bool,
	pub vulkan_memory_model: bool,
	pub vulkan_memory_model_device_scope: bool,
	pub vulkan_memory_model_availability_visibility_chains: bool,
	pub shader_output_viewport_index: bool,
	pub shader_output_layer: bool,
	pub subgroup_broadcast_dynamic_id: bool,

	// Vulkan 1.3 physical device features
	pub robust_image_access: bool,
	pub inline_uniform_block: bool,
	pub descriptor_binding_inline_uniform_block_update_after_bind: bool,
	pub pipeline_creation_cache_control: bool,
	pub private_data: bool,
	pub shader_demote_to_helper_invocation: bool,
	pub shader_terminate_invocation: bool,
	pub subgroup_size_control: bool,
	pub compute_full_subgroups: bool,
	pub synchronization2: bool,
	pub texture_compression_astc_hdr: bool,
	pub shader_zero_initialize_workgroup_memory: bool,
	pub dynamic_rendering: bool,
	pub shader_integer_dot_product: bool,
	pub maintenance4: bool,
}


impl From<ash::vk::PhysicalDeviceFeatures> for DeviceFeatures {
	fn from(vk_features: ash::vk::PhysicalDeviceFeatures) -> Self {
		Self {
			robust_buffer_access: vk_features.robust_buffer_access == ash::vk::TRUE,
			full_draw_index_uint32: vk_features.full_draw_index_uint32 == ash::vk::TRUE,
			image_cube_array: vk_features.image_cube_array == ash::vk::TRUE,
			independent_blend: vk_features.independent_blend == ash::vk::TRUE,
			geometry_shader: vk_features.geometry_shader == ash::vk::TRUE,
			tessellation_shader: vk_features.tessellation_shader == ash::vk::TRUE,
			sample_rate_shading: vk_features.sample_rate_shading == ash::vk::TRUE,
			dual_src_blend: vk_features.dual_src_blend == ash::vk::TRUE,
			logic_op: vk_features.logic_op == ash::vk::TRUE,
			multi_draw_indirect: vk_features.multi_draw_indirect == ash::vk::TRUE,
			draw_indirect_first_instance: vk_features.draw_indirect_first_instance == ash::vk::TRUE,
			depth_clamp: vk_features.depth_clamp == ash::vk::TRUE,
			depth_bias_clamp: vk_features.depth_bias_clamp == ash::vk::TRUE,
			fill_mode_non_solid: vk_features.fill_mode_non_solid == ash::vk::TRUE,
			depth_bounds: vk_features.depth_bounds == ash::vk::TRUE,
			wide_lines: vk_features.wide_lines == ash::vk::TRUE,
			large_points: vk_features.large_points == ash::vk::TRUE,
			alpha_to_one: vk_features.alpha_to_one == ash::vk::TRUE,
			multi_viewport: vk_features.multi_viewport == ash::vk::TRUE,
			sampler_anisotropy: vk_features.sampler_anisotropy == ash::vk::TRUE,
			texture_compression_etc2: vk_features.texture_compression_etc2 == ash::vk::TRUE,
			texture_compression_astc_ldr: vk_features.texture_compression_astc_ldr == ash::vk::TRUE,
			texture_compression_bc: vk_features.texture_compression_bc == ash::vk::TRUE,
			occlusion_query_precise: vk_features.occlusion_query_precise == ash::vk::TRUE,
			pipeline_statistics_query: vk_features.pipeline_statistics_query == ash::vk::TRUE,
			vertex_pipeline_stores_and_atomics: vk_features.vertex_pipeline_stores_and_atomics == ash::vk::TRUE,
			fragment_stores_and_atomics: vk_features.fragment_stores_and_atomics == ash::vk::TRUE,
			shader_tessellation_and_geometry_point_size: vk_features.shader_tessellation_and_geometry_point_size == ash::vk::TRUE,
			shader_image_gather_extended: vk_features.shader_image_gather_extended == ash::vk::TRUE,
			shader_storage_image_extended_formats: vk_features.shader_storage_image_extended_formats == ash::vk::TRUE,
			shader_storage_image_multisample: vk_features.shader_storage_image_multisample == ash::vk::TRUE,
			shader_storage_image_read_without_format: vk_features.shader_storage_image_read_without_format == ash::vk::TRUE,
			shader_storage_image_write_without_format: vk_features.shader_storage_image_write_without_format == ash::vk::TRUE,
			shader_uniform_buffer_array_dynamic_indexing: vk_features.shader_uniform_buffer_array_dynamic_indexing == ash::vk::TRUE,
			shader_sampled_image_array_dynamic_indexing: vk_features.shader_sampled_image_array_dynamic_indexing == ash::vk::TRUE,
			shader_storage_buffer_array_dynamic_indexing: vk_features.shader_storage_buffer_array_dynamic_indexing == ash::vk::TRUE,
			shader_storage_image_array_dynamic_indexing: vk_features.shader_storage_image_array_dynamic_indexing == ash::vk::TRUE,
			shader_clip_distance: vk_features.shader_clip_distance == ash::vk::TRUE,
			shader_cull_distance: vk_features.shader_cull_distance == ash::vk::TRUE,
			shader_float64: vk_features.shader_float64 == ash::vk::TRUE,
			shader_int64: vk_features.shader_int64 == ash::vk::TRUE,
			shader_int16: vk_features.shader_int16 == ash::vk::TRUE,
			shader_resource_residency: vk_features.shader_resource_residency == ash::vk::TRUE,
			shader_resource_min_lod: vk_features.shader_resource_min_lod == ash::vk::TRUE,
			sparse_binding: vk_features.sparse_binding == ash::vk::TRUE,
			sparse_residency_buffer: vk_features.sparse_residency_buffer == ash::vk::TRUE,
			sparse_residency_image2_d: vk_features.sparse_residency_image2_d == ash::vk::TRUE,
			sparse_residency_image3_d: vk_features.sparse_residency_image3_d == ash::vk::TRUE,
			sparse_residency2_samples: vk_features.sparse_residency2_samples == ash::vk::TRUE,
			sparse_residency4_samples: vk_features.sparse_residency4_samples == ash::vk::TRUE,
			sparse_residency8_samples: vk_features.sparse_residency8_samples == ash::vk::TRUE,
			sparse_residency16_samples: vk_features.sparse_residency16_samples == ash::vk::TRUE,
			sparse_residency_aliased: vk_features.sparse_residency_aliased == ash::vk::TRUE,
			variable_multisample_rate: vk_features.variable_multisample_rate == ash::vk::TRUE,
			inherited_queries: vk_features.inherited_queries == ash::vk::TRUE,
			..Default::default()
		}		
	}
}

impl From<ash::vk::PhysicalDeviceVulkan11Features<'_>> for DeviceFeatures {
	fn from(vk_features: ash::vk::PhysicalDeviceVulkan11Features) -> Self {
		Self {
			storage_buffer16_bit_access: vk_features.storage_buffer16_bit_access == ash::vk::TRUE,
			uniform_and_storage_buffer16_bit_access: vk_features.uniform_and_storage_buffer16_bit_access == ash::vk::TRUE,
			storage_push_constant16: vk_features.storage_push_constant16 == ash::vk::TRUE,
			storage_input_output16: vk_features.storage_input_output16 == ash::vk::TRUE,
			multiview: vk_features.multiview == ash::vk::TRUE,
			multiview_geometry_shader: vk_features.multiview_geometry_shader == ash::vk::TRUE,
			multiview_tessellation_shader: vk_features.multiview_tessellation_shader == ash::vk::TRUE,
			variable_pointers_storage_buffer: vk_features.variable_pointers_storage_buffer == ash::vk::TRUE,
			variable_pointers: vk_features.variable_pointers == ash::vk::TRUE,
			protected_memory: vk_features.protected_memory == ash::vk::TRUE,
			sampler_ycbcr_conversion: vk_features.sampler_ycbcr_conversion == ash::vk::TRUE,
			shader_draw_parameters: vk_features.shader_draw_parameters == ash::vk::TRUE,
			..Default::default()
		}		
	}
}

impl From<ash::vk::PhysicalDeviceVulkan12Features<'_>> for DeviceFeatures {
	fn from(vk_features: ash::vk::PhysicalDeviceVulkan12Features) -> Self {
		Self {
			sampler_mirror_clamp_to_edge: vk_features.sampler_mirror_clamp_to_edge == ash::vk::TRUE,
			draw_indirect_count: vk_features.draw_indirect_count == ash::vk::TRUE,
			storage_buffer8_bit_access: vk_features.storage_buffer8_bit_access == ash::vk::TRUE,
			uniform_and_storage_buffer8_bit_access: vk_features.uniform_and_storage_buffer8_bit_access == ash::vk::TRUE,
			storage_push_constant8: vk_features.storage_push_constant8 == ash::vk::TRUE,
			shader_buffer_int64_atomics: vk_features.shader_buffer_int64_atomics == ash::vk::TRUE,
			shader_shared_int64_atomics: vk_features.shader_shared_int64_atomics == ash::vk::TRUE,
			shader_float16: vk_features.shader_float16 == ash::vk::TRUE,
			shader_int8: vk_features.shader_int8 == ash::vk::TRUE,
			descriptor_indexing: vk_features.descriptor_indexing == ash::vk::TRUE,
			shader_input_attachment_array_dynamic_indexing: vk_features.shader_input_attachment_array_dynamic_indexing == ash::vk::TRUE,
			shader_uniform_texel_buffer_array_dynamic_indexing: vk_features.shader_uniform_texel_buffer_array_dynamic_indexing == ash::vk::TRUE,
			shader_storage_texel_buffer_array_dynamic_indexing: vk_features.shader_storage_texel_buffer_array_dynamic_indexing == ash::vk::TRUE,
			shader_uniform_buffer_array_non_uniform_indexing: vk_features.shader_uniform_buffer_array_non_uniform_indexing == ash::vk::TRUE,
			shader_sampled_image_array_non_uniform_indexing: vk_features.shader_sampled_image_array_non_uniform_indexing == ash::vk::TRUE,
			shader_storage_buffer_array_non_uniform_indexing: vk_features.shader_storage_buffer_array_non_uniform_indexing == ash::vk::TRUE,
			shader_storage_image_array_non_uniform_indexing: vk_features.shader_storage_image_array_non_uniform_indexing == ash::vk::TRUE,
			shader_input_attachment_array_non_uniform_indexing: vk_features.shader_input_attachment_array_non_uniform_indexing == ash::vk::TRUE,
			shader_uniform_texel_buffer_array_non_uniform_indexing: vk_features.shader_uniform_texel_buffer_array_non_uniform_indexing == ash::vk::TRUE,
			shader_storage_texel_buffer_array_non_uniform_indexing: vk_features.shader_storage_texel_buffer_array_non_uniform_indexing == ash::vk::TRUE,
			descriptor_binding_uniform_buffer_update_after_bind: vk_features.descriptor_binding_uniform_buffer_update_after_bind == ash::vk::TRUE,
			descriptor_binding_sampled_image_update_after_bind: vk_features.descriptor_binding_sampled_image_update_after_bind == ash::vk::TRUE,
			descriptor_binding_storage_image_update_after_bind: vk_features.descriptor_binding_storage_image_update_after_bind == ash::vk::TRUE,
			descriptor_binding_storage_buffer_update_after_bind: vk_features.descriptor_binding_storage_buffer_update_after_bind == ash::vk::TRUE,
			descriptor_binding_uniform_texel_buffer_update_after_bind: vk_features.descriptor_binding_uniform_texel_buffer_update_after_bind == ash::vk::TRUE,
			descriptor_binding_storage_texel_buffer_update_after_bind: vk_features.descriptor_binding_storage_texel_buffer_update_after_bind == ash::vk::TRUE,
			descriptor_binding_update_unused_while_pending: vk_features.descriptor_binding_update_unused_while_pending == ash::vk::TRUE,
			descriptor_binding_partially_bound: vk_features.descriptor_binding_partially_bound == ash::vk::TRUE,
			descriptor_binding_variable_descriptor_count: vk_features.descriptor_binding_variable_descriptor_count == ash::vk::TRUE,
			runtime_descriptor_array: vk_features.runtime_descriptor_array == ash::vk::TRUE,
			sampler_filter_minmax: vk_features.sampler_filter_minmax == ash::vk::TRUE,
			scalar_block_layout: vk_features.scalar_block_layout == ash::vk::TRUE,
			imageless_framebuffer: vk_features.imageless_framebuffer == ash::vk::TRUE,
			uniform_buffer_standard_layout: vk_features.uniform_buffer_standard_layout == ash::vk::TRUE,
			shader_subgroup_extended_types: vk_features.shader_subgroup_extended_types == ash::vk::TRUE,
			separate_depth_stencil_layouts: vk_features.separate_depth_stencil_layouts == ash::vk::TRUE,
			host_query_reset: vk_features.host_query_reset == ash::vk::TRUE,
			timeline_semaphore: vk_features.timeline_semaphore == ash::vk::TRUE,
			buffer_device_address: vk_features.buffer_device_address == ash::vk::TRUE,
			buffer_device_address_capture_replay: vk_features.buffer_device_address_capture_replay == ash::vk::TRUE,
			buffer_device_address_multi_device: vk_features.buffer_device_address_multi_device == ash::vk::TRUE,
			vulkan_memory_model: vk_features.vulkan_memory_model == ash::vk::TRUE,
			vulkan_memory_model_device_scope: vk_features.vulkan_memory_model_device_scope == ash::vk::TRUE,
			vulkan_memory_model_availability_visibility_chains: vk_features.vulkan_memory_model_availability_visibility_chains == ash::vk::TRUE,
			shader_output_viewport_index: vk_features.shader_output_viewport_index == ash::vk::TRUE,
			shader_output_layer: vk_features.shader_output_layer == ash::vk::TRUE,
			subgroup_broadcast_dynamic_id: vk_features.subgroup_broadcast_dynamic_id == ash::vk::TRUE,
			..Default::default()
		}		
	}
}

impl From<ash::vk::PhysicalDeviceVulkan13Features<'_>> for DeviceFeatures {
	fn from(vk_features: ash::vk::PhysicalDeviceVulkan13Features) -> Self {
		Self {
			robust_image_access: vk_features.robust_image_access == ash::vk::TRUE,
			inline_uniform_block: vk_features.inline_uniform_block == ash::vk::TRUE,
			descriptor_binding_inline_uniform_block_update_after_bind: vk_features.descriptor_binding_inline_uniform_block_update_after_bind == ash::vk::TRUE,
			pipeline_creation_cache_control: vk_features.pipeline_creation_cache_control == ash::vk::TRUE,
			private_data: vk_features.private_data == ash::vk::TRUE,
			shader_demote_to_helper_invocation: vk_features.shader_demote_to_helper_invocation == ash::vk::TRUE,
			shader_terminate_invocation: vk_features.shader_terminate_invocation == ash::vk::TRUE,
			subgroup_size_control: vk_features.subgroup_size_control == ash::vk::TRUE,
			compute_full_subgroups: vk_features.compute_full_subgroups == ash::vk::TRUE,
			synchronization2: vk_features.synchronization2 == ash::vk::TRUE,
			texture_compression_astc_hdr: vk_features.texture_compression_astc_hdr == ash::vk::TRUE,
			shader_zero_initialize_workgroup_memory: vk_features.shader_zero_initialize_workgroup_memory == ash::vk::TRUE,
			dynamic_rendering: vk_features.dynamic_rendering == ash::vk::TRUE,
			shader_integer_dot_product: vk_features.shader_integer_dot_product == ash::vk::TRUE,
			maintenance4: vk_features.maintenance4 == ash::vk::TRUE,
			..Default::default()
		}		
	}
}

impl From<DeviceFeaturesFfi<'_>> for DeviceFeatures {
	fn from(vk_features: DeviceFeaturesFfi<'_>) -> Self {
		let features_10 = Self::from(vk_features.vk_features.features);
		let features_11 = Self::from(vk_features.vk_features_11);
		let features_12 = Self::from(vk_features.vk_features_12);
		let features_13 = Self::from(vk_features.vk_features_13);

		features_10 | features_11 | features_12 | features_13
	}
}


impl From<&DeviceFeatures> for ash::vk::PhysicalDeviceFeatures {
	fn from(features: &DeviceFeatures) -> Self {
		Self {
			robust_buffer_access: features.robust_buffer_access.into(),
			full_draw_index_uint32: features.full_draw_index_uint32.into(),
			image_cube_array: features.image_cube_array.into(),
			independent_blend: features.independent_blend.into(),
			geometry_shader: features.geometry_shader.into(),
			tessellation_shader: features.tessellation_shader.into(),
			sample_rate_shading: features.sample_rate_shading.into(),
			dual_src_blend: features.dual_src_blend.into(),
			logic_op: features.logic_op.into(),
			multi_draw_indirect: features.multi_draw_indirect.into(),
			draw_indirect_first_instance: features.draw_indirect_first_instance.into(),
			depth_clamp: features.depth_clamp.into(),
			depth_bias_clamp: features.depth_bias_clamp.into(),
			fill_mode_non_solid: features.fill_mode_non_solid.into(),
			depth_bounds: features.depth_bounds.into(),
			wide_lines: features.wide_lines.into(),
			large_points: features.large_points.into(),
			alpha_to_one: features.alpha_to_one.into(),
			multi_viewport: features.multi_viewport.into(),
			sampler_anisotropy: features.sampler_anisotropy.into(),
			texture_compression_etc2: features.texture_compression_etc2.into(),
			texture_compression_astc_ldr: features.texture_compression_astc_ldr.into(),
			texture_compression_bc: features.texture_compression_bc.into(),
			occlusion_query_precise: features.occlusion_query_precise.into(),
			pipeline_statistics_query: features.pipeline_statistics_query.into(),
			vertex_pipeline_stores_and_atomics: features.vertex_pipeline_stores_and_atomics.into(),
			fragment_stores_and_atomics: features.fragment_stores_and_atomics.into(),
			shader_tessellation_and_geometry_point_size: features.shader_tessellation_and_geometry_point_size.into(),
			shader_image_gather_extended: features.shader_image_gather_extended.into(),
			shader_storage_image_extended_formats: features.shader_storage_image_extended_formats.into(),
			shader_storage_image_multisample: features.shader_storage_image_multisample.into(),
			shader_storage_image_read_without_format: features.shader_storage_image_read_without_format.into(),
			shader_storage_image_write_without_format: features.shader_storage_image_write_without_format.into(),
			shader_uniform_buffer_array_dynamic_indexing: features.shader_uniform_buffer_array_dynamic_indexing.into(),
			shader_sampled_image_array_dynamic_indexing: features.shader_sampled_image_array_dynamic_indexing.into(),
			shader_storage_buffer_array_dynamic_indexing: features.shader_storage_buffer_array_dynamic_indexing.into(),
			shader_storage_image_array_dynamic_indexing: features.shader_storage_image_array_dynamic_indexing.into(),
			shader_clip_distance: features.shader_clip_distance.into(),
			shader_cull_distance: features.shader_cull_distance.into(),
			shader_float64: features.shader_float64.into(),
			shader_int64: features.shader_int64.into(),
			shader_int16: features.shader_int16.into(),
			shader_resource_residency: features.shader_resource_residency.into(),
			shader_resource_min_lod: features.shader_resource_min_lod.into(),
			sparse_binding: features.sparse_binding.into(),
			sparse_residency_buffer: features.sparse_residency_buffer.into(),
			sparse_residency_image2_d: features.sparse_residency_image2_d.into(),
			sparse_residency_image3_d: features.sparse_residency_image3_d.into(),
			sparse_residency2_samples: features.sparse_residency2_samples.into(),
			sparse_residency4_samples: features.sparse_residency4_samples.into(),
			sparse_residency8_samples: features.sparse_residency8_samples.into(),
			sparse_residency16_samples: features.sparse_residency16_samples.into(),
			sparse_residency_aliased: features.sparse_residency_aliased.into(),
			variable_multisample_rate: features.variable_multisample_rate.into(),
			inherited_queries: features.inherited_queries.into(),
			..Default::default()
		}	
	}
}

impl From<&DeviceFeatures> for ash::vk::PhysicalDeviceVulkan11Features<'_> {
	fn from(features: &DeviceFeatures) -> Self {
		Self {
			storage_buffer16_bit_access: features.storage_buffer16_bit_access.into(),
			uniform_and_storage_buffer16_bit_access: features.uniform_and_storage_buffer16_bit_access.into(),
			storage_push_constant16: features.storage_push_constant16.into(),
			storage_input_output16: features.storage_input_output16.into(),
			multiview: features.multiview.into(),
			multiview_geometry_shader: features.multiview_geometry_shader.into(),
			multiview_tessellation_shader: features.multiview_tessellation_shader.into(),
			variable_pointers_storage_buffer: features.variable_pointers_storage_buffer.into(),
			variable_pointers: features.variable_pointers.into(),
			protected_memory: features.protected_memory.into(),
			sampler_ycbcr_conversion: features.sampler_ycbcr_conversion.into(),
			shader_draw_parameters: features.shader_draw_parameters.into(),
			..Default::default()
		}	
	}
}

impl From<&DeviceFeatures> for ash::vk::PhysicalDeviceVulkan12Features<'_> {
	fn from(features: &DeviceFeatures) -> Self {
		Self {
			sampler_mirror_clamp_to_edge: features.sampler_mirror_clamp_to_edge.into(),
			draw_indirect_count: features.draw_indirect_count.into(),
			storage_buffer8_bit_access: features.storage_buffer8_bit_access.into(),
			uniform_and_storage_buffer8_bit_access: features.uniform_and_storage_buffer8_bit_access.into(),
			storage_push_constant8: features.storage_push_constant8.into(),
			shader_buffer_int64_atomics: features.shader_buffer_int64_atomics.into(),
			shader_shared_int64_atomics: features.shader_shared_int64_atomics.into(),
			shader_float16: features.shader_float16.into(),
			shader_int8: features.shader_int8.into(),
			descriptor_indexing: features.descriptor_indexing.into(),
			shader_input_attachment_array_dynamic_indexing: features.shader_input_attachment_array_dynamic_indexing.into(),
			shader_uniform_texel_buffer_array_dynamic_indexing: features.shader_uniform_texel_buffer_array_dynamic_indexing.into(),
			shader_storage_texel_buffer_array_dynamic_indexing: features.shader_storage_texel_buffer_array_dynamic_indexing.into(),
			shader_uniform_buffer_array_non_uniform_indexing: features.shader_uniform_buffer_array_non_uniform_indexing.into(),
			shader_sampled_image_array_non_uniform_indexing: features.shader_sampled_image_array_non_uniform_indexing.into(),
			shader_storage_buffer_array_non_uniform_indexing: features.shader_storage_buffer_array_non_uniform_indexing.into(),
			shader_storage_image_array_non_uniform_indexing: features.shader_storage_image_array_non_uniform_indexing.into(),
			shader_input_attachment_array_non_uniform_indexing: features.shader_input_attachment_array_non_uniform_indexing.into(),
			shader_uniform_texel_buffer_array_non_uniform_indexing: features.shader_uniform_texel_buffer_array_non_uniform_indexing.into(),
			shader_storage_texel_buffer_array_non_uniform_indexing: features.shader_storage_texel_buffer_array_non_uniform_indexing.into(),
			descriptor_binding_uniform_buffer_update_after_bind: features.descriptor_binding_uniform_buffer_update_after_bind.into(),
			descriptor_binding_sampled_image_update_after_bind: features.descriptor_binding_sampled_image_update_after_bind.into(),
			descriptor_binding_storage_image_update_after_bind: features.descriptor_binding_storage_image_update_after_bind.into(),
			descriptor_binding_storage_buffer_update_after_bind: features.descriptor_binding_storage_buffer_update_after_bind.into(),
			descriptor_binding_uniform_texel_buffer_update_after_bind: features.descriptor_binding_uniform_texel_buffer_update_after_bind.into(),
			descriptor_binding_storage_texel_buffer_update_after_bind: features.descriptor_binding_storage_texel_buffer_update_after_bind.into(),
			descriptor_binding_update_unused_while_pending: features.descriptor_binding_update_unused_while_pending.into(),
			descriptor_binding_partially_bound: features.descriptor_binding_partially_bound.into(),
			descriptor_binding_variable_descriptor_count: features.descriptor_binding_variable_descriptor_count.into(),
			runtime_descriptor_array: features.runtime_descriptor_array.into(),
			sampler_filter_minmax: features.sampler_filter_minmax.into(),
			scalar_block_layout: features.scalar_block_layout.into(),
			imageless_framebuffer: features.imageless_framebuffer.into(),
			uniform_buffer_standard_layout: features.uniform_buffer_standard_layout.into(),
			shader_subgroup_extended_types: features.shader_subgroup_extended_types.into(),
			separate_depth_stencil_layouts: features.separate_depth_stencil_layouts.into(),
			host_query_reset: features.host_query_reset.into(),
			timeline_semaphore: features.timeline_semaphore.into(),
			buffer_device_address: features.buffer_device_address.into(),
			buffer_device_address_capture_replay: features.buffer_device_address_capture_replay.into(),
			buffer_device_address_multi_device: features.buffer_device_address_multi_device.into(),
			vulkan_memory_model: features.vulkan_memory_model.into(),
			vulkan_memory_model_device_scope: features.vulkan_memory_model_device_scope.into(),
			vulkan_memory_model_availability_visibility_chains: features.vulkan_memory_model_availability_visibility_chains.into(),
			shader_output_viewport_index: features.shader_output_viewport_index.into(),
			shader_output_layer: features.shader_output_layer.into(),
			subgroup_broadcast_dynamic_id: features.subgroup_broadcast_dynamic_id.into(),
			..Default::default()
		}	
	}
}

impl From<&DeviceFeatures> for ash::vk::PhysicalDeviceVulkan13Features<'_> {
	fn from(features: &DeviceFeatures) -> Self {
		Self {
			robust_image_access: features.robust_image_access.into(),
			inline_uniform_block: features.inline_uniform_block.into(),
			descriptor_binding_inline_uniform_block_update_after_bind: features.descriptor_binding_inline_uniform_block_update_after_bind.into(),
			pipeline_creation_cache_control: features.pipeline_creation_cache_control.into(),
			private_data: features.private_data.into(),
			shader_demote_to_helper_invocation: features.shader_demote_to_helper_invocation.into(),
			shader_terminate_invocation: features.shader_terminate_invocation.into(),
			subgroup_size_control: features.subgroup_size_control.into(),
			compute_full_subgroups: features.compute_full_subgroups.into(),
			synchronization2: features.synchronization2.into(),
			texture_compression_astc_hdr: features.texture_compression_astc_hdr.into(),
			shader_zero_initialize_workgroup_memory: features.shader_zero_initialize_workgroup_memory.into(),
			dynamic_rendering: features.dynamic_rendering.into(),
			shader_integer_dot_product: features.shader_integer_dot_product.into(),
			maintenance4: features.maintenance4.into(),
			..Default::default()
		}	
	}
}

impl From<&DeviceFeatures> for ash::vk::PhysicalDeviceFeatures2<'_> {
	fn from(features: &DeviceFeatures) -> Self {
		ash::vk::PhysicalDeviceFeatures2 {
			features: ash::vk::PhysicalDeviceFeatures::from(features),
			..Default::default()
		}
	}
}

impl From<&DeviceFeatures> for DeviceFeaturesFfi<'_> {
	fn from(features: &DeviceFeatures) -> Self {
		let vk_features = ash::vk::PhysicalDeviceFeatures2::from(features);
		let vk_features_11 = ash::vk::PhysicalDeviceVulkan11Features::from(features);
		let vk_features_12 = ash::vk::PhysicalDeviceVulkan12Features::from(features);
		let vk_features_13 = ash::vk::PhysicalDeviceVulkan13Features::from(features);

		DeviceFeaturesFfi {
			vk_features,
			vk_features_11,
			vk_features_12,
			vk_features_13,
		}
	}
}


#[derive(Debug, Default)]
pub(crate) struct DeviceFeaturesFfi<'a> {
	pub(crate) vk_features: ash::vk::PhysicalDeviceFeatures2<'a>,
	pub(crate) vk_features_11: ash::vk::PhysicalDeviceVulkan11Features<'a>,
	pub(crate) vk_features_12: ash::vk::PhysicalDeviceVulkan12Features<'a>,
	pub(crate) vk_features_13: ash::vk::PhysicalDeviceVulkan13Features<'a>,
}

impl<'a> DeviceFeaturesFfi<'a> {
	pub(crate) fn chain_struct(&mut self) {
		self.vk_features_12.p_next = <*mut _>::cast(&mut self.vk_features_13);
		self.vk_features_11.p_next = <*mut _>::cast(&mut self.vk_features_12);
		self.vk_features.p_next = <*mut _>::cast(&mut self.vk_features_11);
	}
}

















/*
struct DeviceFeaturesInner {
	vk_features10: ash::vk::PhysicalDeviceFeatures,
	vk_features11: ash::vk::PhysicalDeviceVulkan11Features<'static>,
	vk_features12: ash::vk::PhysicalDeviceVulkan12Features<'static>,
	vk_features13: ash::vk::PhysicalDeviceVulkan13Features<'static>,
	vk_raw_features: ash::vk::PhysicalDeviceFeatures2<'static>,
}

impl Default for DeviceFeaturesInner {
	fn default() -> Self {
		let vk_features13 = ash::vk::PhysicalDeviceVulkan13Features::default();

		let vk_features12 = ash::vk::PhysicalDeviceVulkan12Features {
			p_next: &vk_features13 as *const _ as *mut _,
			..Default::default()
		};

		let vk_features11 = ash::vk::PhysicalDeviceVulkan11Features {
			p_next: &vk_features12 as *const _ as *mut _,
			..Default::default()
		};

		let vk_features10 = ash::vk::PhysicalDeviceFeatures::default();

		let vk_raw_features = ash::vk::PhysicalDeviceFeatures2 {
			p_next: &vk_features11 as *const _ as *mut _,
			features: vk_features10,
			..Default::default()
		};

		Self { 
			vk_features10,
			vk_features11,
			vk_features12,
			vk_features13,
			vk_raw_features
		}
	}
}
*/

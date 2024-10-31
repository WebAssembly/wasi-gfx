<h1><a id="imports"></a>World imports</h1>
<ul>
<li>Imports:
<ul>
<li>interface <a href="#wasi_graphics_context_graphics_context"><code>wasi:graphics-context/graphics-context</code></a></li>
<li>interface <a href="#wasi_webgpu_webgpu"><code>wasi:webgpu/webgpu</code></a></li>
</ul>
</li>
</ul>
<h2><a id="wasi_graphics_context_graphics_context"></a>Import interface wasi:graphics-context/graphics-context</h2>
<hr />
<h3>Types</h3>
<h4><a id="context"></a><code>resource context</code></h4>
<h4><a id="abstract_buffer"></a><code>resource abstract-buffer</code></h4>
<hr />
<h3>Functions</h3>
<h4><a id="constructor_context"></a><code>[constructor]context: func</code></h4>
<h5>Return values</h5>
<ul>
<li><a id="constructor_context.0"></a> own&lt;<a href="#context"><a href="#context"><code>context</code></a></a>&gt;</li>
</ul>
<h4><a id="method_context_get_current_buffer"></a><code>[method]context.get-current-buffer: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_context_get_current_buffer.self"></a><code>self</code>: borrow&lt;<a href="#context"><a href="#context"><code>context</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_context_get_current_buffer.0"></a> own&lt;<a href="#abstract_buffer"><a href="#abstract_buffer"><code>abstract-buffer</code></a></a>&gt;</li>
</ul>
<h4><a id="method_context_present"></a><code>[method]context.present: func</code></h4>
<p>TODO: might want to remove this.</p>
<h5>Params</h5>
<ul>
<li><a id="method_context_present.self"></a><code>self</code>: borrow&lt;<a href="#context"><a href="#context"><code>context</code></a></a>&gt;</li>
</ul>
<h2><a id="wasi_webgpu_webgpu"></a>Import interface wasi:webgpu/webgpu</h2>
<hr />
<h3>Types</h3>
<h4><a id="context"></a><code>type context</code></h4>
<p><a href="#context"><a href="#context"><code>context</code></a></a></p>
<p>
#### <a id="abstract_buffer"></a>`type abstract-buffer`
[`abstract-buffer`](#abstract_buffer)
<p>
#### <a id="gpu_supported_limits"></a>`resource gpu-supported-limits`
<h4><a id="gpu_supported_features"></a><code>resource gpu-supported-features</code></h4>
<h4><a id="wgsl_language_features"></a><code>resource wgsl-language-features</code></h4>
<h4><a id="gpu_adapter_info"></a><code>resource gpu-adapter-info</code></h4>
<h4><a id="gpu"></a><code>resource gpu</code></h4>
<h4><a id="gpu_power_preference"></a><code>enum gpu-power-preference</code></h4>
<h5>Enum Cases</h5>
<ul>
<li><a id="gpu_power_preference.low_power"></a><code>low-power</code></li>
<li><a id="gpu_power_preference.high_performance"></a><code>high-performance</code></li>
</ul>
<h4><a id="gpu_request_adapter_options"></a><code>record gpu-request-adapter-options</code></h4>
<h5>Record Fields</h5>
<ul>
<li><a id="gpu_request_adapter_options.power_preference"></a><code>power-preference</code>: option&lt;<a href="#gpu_power_preference"><a href="#gpu_power_preference"><code>gpu-power-preference</code></a></a>&gt;</li>
<li><a id="gpu_request_adapter_options.force_fallback_adapter"></a><code>force-fallback-adapter</code>: option&lt;<code>bool</code>&gt;</li>
</ul>
<h4><a id="gpu_adapter"></a><code>resource gpu-adapter</code></h4>
<h4><a id="record_gpu_size64"></a><code>resource record-gpu-size64</code></h4>
<h4><a id="gpu_feature_name"></a><code>enum gpu-feature-name</code></h4>
<h5>Enum Cases</h5>
<ul>
<li><a id="gpu_feature_name.depth_clip_control"></a><code>depth-clip-control</code></li>
<li><a id="gpu_feature_name.depth32float_stencil8"></a><code>depth32float-stencil8</code></li>
<li><a id="gpu_feature_name.texture_compression_bc"></a><code>texture-compression-bc</code></li>
<li><a id="gpu_feature_name.texture_compression_bc_sliced3d"></a><code>texture-compression-bc-sliced3d</code></li>
<li><a id="gpu_feature_name.texture_compression_etc2"></a><code>texture-compression-etc2</code></li>
<li><a id="gpu_feature_name.texture_compression_astc"></a><code>texture-compression-astc</code></li>
<li><a id="gpu_feature_name.timestamp_query"></a><code>timestamp-query</code></li>
<li><a id="gpu_feature_name.indirect_first_instance"></a><code>indirect-first-instance</code></li>
<li><a id="gpu_feature_name.shader_f16"></a><code>shader-f16</code></li>
<li><a id="gpu_feature_name.rg11b10ufloat_renderable"></a><code>rg11b10ufloat-renderable</code></li>
<li><a id="gpu_feature_name.bgra8unorm_storage"></a><code>bgra8unorm-storage</code></li>
<li><a id="gpu_feature_name.float32_filterable"></a><code>float32-filterable</code></li>
<li><a id="gpu_feature_name.clip_distances"></a><code>clip-distances</code></li>
<li><a id="gpu_feature_name.dual_source_blending"></a><code>dual-source-blending</code></li>
</ul>
<h4><a id="gpu_device"></a><code>resource gpu-device</code></h4>
<h4><a id="gpu_buffer"></a><code>resource gpu-buffer</code></h4>
<h4><a id="gpu_buffer_map_state"></a><code>enum gpu-buffer-map-state</code></h4>
<h5>Enum Cases</h5>
<ul>
<li><a id="gpu_buffer_map_state.unmapped"></a><code>unmapped</code></li>
<li><a id="gpu_buffer_map_state.pending"></a><code>pending</code></li>
<li><a id="gpu_buffer_map_state.mapped"></a><code>mapped</code></li>
</ul>
<h4><a id="gpu_buffer_usage_flags"></a><code>type gpu-buffer-usage-flags</code></h4>
<p><code>u32</code></p>
<p>
#### <a id="gpu_buffer_usage"></a>`resource gpu-buffer-usage`
<h4><a id="gpu_map_mode_flags"></a><code>type gpu-map-mode-flags</code></h4>
<p><code>u32</code></p>
<p>
#### <a id="gpu_map_mode"></a>`resource gpu-map-mode`
<h4><a id="gpu_texture"></a><code>resource gpu-texture</code></h4>
<h4><a id="gpu_texture_dimension"></a><code>enum gpu-texture-dimension</code></h4>
<h5>Enum Cases</h5>
<ul>
<li><a id="gpu_texture_dimension.d1"></a><code>d1</code></li>
<li><a id="gpu_texture_dimension.d2"></a><code>d2</code></li>
<li><a id="gpu_texture_dimension.d3"></a><code>d3</code></li>
</ul>
<h4><a id="gpu_texture_usage_flags"></a><code>type gpu-texture-usage-flags</code></h4>
<p><code>u32</code></p>
<p>
#### <a id="gpu_texture_usage"></a>`resource gpu-texture-usage`
<h4><a id="gpu_texture_view"></a><code>resource gpu-texture-view</code></h4>
<h4><a id="gpu_texture_view_dimension"></a><code>enum gpu-texture-view-dimension</code></h4>
<h5>Enum Cases</h5>
<ul>
<li><a id="gpu_texture_view_dimension.d1"></a><code>d1</code></li>
<li><a id="gpu_texture_view_dimension.d2"></a><code>d2</code></li>
<li><a id="gpu_texture_view_dimension.d2_array"></a><code>d2-array</code></li>
<li><a id="gpu_texture_view_dimension.cube"></a><code>cube</code></li>
<li><a id="gpu_texture_view_dimension.cube_array"></a><code>cube-array</code></li>
<li><a id="gpu_texture_view_dimension.d3"></a><code>d3</code></li>
</ul>
<h4><a id="gpu_texture_aspect"></a><code>enum gpu-texture-aspect</code></h4>
<h5>Enum Cases</h5>
<ul>
<li><a id="gpu_texture_aspect.all"></a><code>all</code></li>
<li><a id="gpu_texture_aspect.stencil_only"></a><code>stencil-only</code></li>
<li><a id="gpu_texture_aspect.depth_only"></a><code>depth-only</code></li>
</ul>
<h4><a id="gpu_texture_format"></a><code>enum gpu-texture-format</code></h4>
<h5>Enum Cases</h5>
<ul>
<li><a id="gpu_texture_format.r8unorm"></a><code>r8unorm</code></li>
<li><a id="gpu_texture_format.r8snorm"></a><code>r8snorm</code></li>
<li><a id="gpu_texture_format.r8uint"></a><code>r8uint</code></li>
<li><a id="gpu_texture_format.r8sint"></a><code>r8sint</code></li>
<li><a id="gpu_texture_format.r16uint"></a><code>r16uint</code></li>
<li><a id="gpu_texture_format.r16sint"></a><code>r16sint</code></li>
<li><a id="gpu_texture_format.r16float"></a><code>r16float</code></li>
<li><a id="gpu_texture_format.rg8unorm"></a><code>rg8unorm</code></li>
<li><a id="gpu_texture_format.rg8snorm"></a><code>rg8snorm</code></li>
<li><a id="gpu_texture_format.rg8uint"></a><code>rg8uint</code></li>
<li><a id="gpu_texture_format.rg8sint"></a><code>rg8sint</code></li>
<li><a id="gpu_texture_format.r32uint"></a><code>r32uint</code></li>
<li><a id="gpu_texture_format.r32sint"></a><code>r32sint</code></li>
<li><a id="gpu_texture_format.r32float"></a><code>r32float</code></li>
<li><a id="gpu_texture_format.rg16uint"></a><code>rg16uint</code></li>
<li><a id="gpu_texture_format.rg16sint"></a><code>rg16sint</code></li>
<li><a id="gpu_texture_format.rg16float"></a><code>rg16float</code></li>
<li><a id="gpu_texture_format.rgba8unorm"></a><code>rgba8unorm</code></li>
<li><a id="gpu_texture_format.rgba8unorm_srgb"></a><code>rgba8unorm-srgb</code></li>
<li><a id="gpu_texture_format.rgba8snorm"></a><code>rgba8snorm</code></li>
<li><a id="gpu_texture_format.rgba8uint"></a><code>rgba8uint</code></li>
<li><a id="gpu_texture_format.rgba8sint"></a><code>rgba8sint</code></li>
<li><a id="gpu_texture_format.bgra8unorm"></a><code>bgra8unorm</code></li>
<li><a id="gpu_texture_format.bgra8unorm_srgb"></a><code>bgra8unorm-srgb</code></li>
<li><a id="gpu_texture_format.rgb9e5ufloat"></a><code>rgb9e5ufloat</code></li>
<li><a id="gpu_texture_format.rgb10a2uint"></a><code>rgb10a2uint</code></li>
<li><a id="gpu_texture_format.rgb10a2unorm"></a><code>rgb10a2unorm</code></li>
<li><a id="gpu_texture_format.rg11b10ufloat"></a><code>rg11b10ufloat</code></li>
<li><a id="gpu_texture_format.rg32uint"></a><code>rg32uint</code></li>
<li><a id="gpu_texture_format.rg32sint"></a><code>rg32sint</code></li>
<li><a id="gpu_texture_format.rg32float"></a><code>rg32float</code></li>
<li><a id="gpu_texture_format.rgba16uint"></a><code>rgba16uint</code></li>
<li><a id="gpu_texture_format.rgba16sint"></a><code>rgba16sint</code></li>
<li><a id="gpu_texture_format.rgba16float"></a><code>rgba16float</code></li>
<li><a id="gpu_texture_format.rgba32uint"></a><code>rgba32uint</code></li>
<li><a id="gpu_texture_format.rgba32sint"></a><code>rgba32sint</code></li>
<li><a id="gpu_texture_format.rgba32float"></a><code>rgba32float</code></li>
<li><a id="gpu_texture_format.stencil8"></a><code>stencil8</code></li>
<li><a id="gpu_texture_format.depth16unorm"></a><code>depth16unorm</code></li>
<li><a id="gpu_texture_format.depth24plus"></a><code>depth24plus</code></li>
<li><a id="gpu_texture_format.depth24plus_stencil8"></a><code>depth24plus-stencil8</code></li>
<li><a id="gpu_texture_format.depth32float"></a><code>depth32float</code></li>
<li><a id="gpu_texture_format.depth32float_stencil8"></a><code>depth32float-stencil8</code></li>
<li><a id="gpu_texture_format.bc1_rgba_unorm"></a><code>bc1-rgba-unorm</code></li>
<li><a id="gpu_texture_format.bc1_rgba_unorm_srgb"></a><code>bc1-rgba-unorm-srgb</code></li>
<li><a id="gpu_texture_format.bc2_rgba_unorm"></a><code>bc2-rgba-unorm</code></li>
<li><a id="gpu_texture_format.bc2_rgba_unorm_srgb"></a><code>bc2-rgba-unorm-srgb</code></li>
<li><a id="gpu_texture_format.bc3_rgba_unorm"></a><code>bc3-rgba-unorm</code></li>
<li><a id="gpu_texture_format.bc3_rgba_unorm_srgb"></a><code>bc3-rgba-unorm-srgb</code></li>
<li><a id="gpu_texture_format.bc4_r_unorm"></a><code>bc4-r-unorm</code></li>
<li><a id="gpu_texture_format.bc4_r_snorm"></a><code>bc4-r-snorm</code></li>
<li><a id="gpu_texture_format.bc5_rg_unorm"></a><code>bc5-rg-unorm</code></li>
<li><a id="gpu_texture_format.bc5_rg_snorm"></a><code>bc5-rg-snorm</code></li>
<li><a id="gpu_texture_format.bc6h_rgb_ufloat"></a><code>bc6h-rgb-ufloat</code></li>
<li><a id="gpu_texture_format.bc6h_rgb_float"></a><code>bc6h-rgb-float</code></li>
<li><a id="gpu_texture_format.bc7_rgba_unorm"></a><code>bc7-rgba-unorm</code></li>
<li><a id="gpu_texture_format.bc7_rgba_unorm_srgb"></a><code>bc7-rgba-unorm-srgb</code></li>
<li><a id="gpu_texture_format.etc2_rgb8unorm"></a><code>etc2-rgb8unorm</code></li>
<li><a id="gpu_texture_format.etc2_rgb8unorm_srgb"></a><code>etc2-rgb8unorm-srgb</code></li>
<li><a id="gpu_texture_format.etc2_rgb8a1unorm"></a><code>etc2-rgb8a1unorm</code></li>
<li><a id="gpu_texture_format.etc2_rgb8a1unorm_srgb"></a><code>etc2-rgb8a1unorm-srgb</code></li>
<li><a id="gpu_texture_format.etc2_rgba8unorm"></a><code>etc2-rgba8unorm</code></li>
<li><a id="gpu_texture_format.etc2_rgba8unorm_srgb"></a><code>etc2-rgba8unorm-srgb</code></li>
<li><a id="gpu_texture_format.eac_r11unorm"></a><code>eac-r11unorm</code></li>
<li><a id="gpu_texture_format.eac_r11snorm"></a><code>eac-r11snorm</code></li>
<li><a id="gpu_texture_format.eac_rg11unorm"></a><code>eac-rg11unorm</code></li>
<li><a id="gpu_texture_format.eac_rg11snorm"></a><code>eac-rg11snorm</code></li>
<li><a id="gpu_texture_format.astc4x4_unorm"></a><code>astc4x4-unorm</code></li>
<li><a id="gpu_texture_format.astc4x4_unorm_srgb"></a><code>astc4x4-unorm-srgb</code></li>
<li><a id="gpu_texture_format.astc5x4_unorm"></a><code>astc5x4-unorm</code></li>
<li><a id="gpu_texture_format.astc5x4_unorm_srgb"></a><code>astc5x4-unorm-srgb</code></li>
<li><a id="gpu_texture_format.astc5x5_unorm"></a><code>astc5x5-unorm</code></li>
<li><a id="gpu_texture_format.astc5x5_unorm_srgb"></a><code>astc5x5-unorm-srgb</code></li>
<li><a id="gpu_texture_format.astc6x5_unorm"></a><code>astc6x5-unorm</code></li>
<li><a id="gpu_texture_format.astc6x5_unorm_srgb"></a><code>astc6x5-unorm-srgb</code></li>
<li><a id="gpu_texture_format.astc6x6_unorm"></a><code>astc6x6-unorm</code></li>
<li><a id="gpu_texture_format.astc6x6_unorm_srgb"></a><code>astc6x6-unorm-srgb</code></li>
<li><a id="gpu_texture_format.astc8x5_unorm"></a><code>astc8x5-unorm</code></li>
<li><a id="gpu_texture_format.astc8x5_unorm_srgb"></a><code>astc8x5-unorm-srgb</code></li>
<li><a id="gpu_texture_format.astc8x6_unorm"></a><code>astc8x6-unorm</code></li>
<li><a id="gpu_texture_format.astc8x6_unorm_srgb"></a><code>astc8x6-unorm-srgb</code></li>
<li><a id="gpu_texture_format.astc8x8_unorm"></a><code>astc8x8-unorm</code></li>
<li><a id="gpu_texture_format.astc8x8_unorm_srgb"></a><code>astc8x8-unorm-srgb</code></li>
<li><a id="gpu_texture_format.astc10x5_unorm"></a><code>astc10x5-unorm</code></li>
<li><a id="gpu_texture_format.astc10x5_unorm_srgb"></a><code>astc10x5-unorm-srgb</code></li>
<li><a id="gpu_texture_format.astc10x6_unorm"></a><code>astc10x6-unorm</code></li>
<li><a id="gpu_texture_format.astc10x6_unorm_srgb"></a><code>astc10x6-unorm-srgb</code></li>
<li><a id="gpu_texture_format.astc10x8_unorm"></a><code>astc10x8-unorm</code></li>
<li><a id="gpu_texture_format.astc10x8_unorm_srgb"></a><code>astc10x8-unorm-srgb</code></li>
<li><a id="gpu_texture_format.astc10x10_unorm"></a><code>astc10x10-unorm</code></li>
<li><a id="gpu_texture_format.astc10x10_unorm_srgb"></a><code>astc10x10-unorm-srgb</code></li>
<li><a id="gpu_texture_format.astc12x10_unorm"></a><code>astc12x10-unorm</code></li>
<li><a id="gpu_texture_format.astc12x10_unorm_srgb"></a><code>astc12x10-unorm-srgb</code></li>
<li><a id="gpu_texture_format.astc12x12_unorm"></a><code>astc12x12-unorm</code></li>
<li><a id="gpu_texture_format.astc12x12_unorm_srgb"></a><code>astc12x12-unorm-srgb</code></li>
</ul>
<h4><a id="gpu_sampler"></a><code>resource gpu-sampler</code></h4>
<h4><a id="gpu_address_mode"></a><code>enum gpu-address-mode</code></h4>
<h5>Enum Cases</h5>
<ul>
<li><a id="gpu_address_mode.clamp_to_edge"></a><code>clamp-to-edge</code></li>
<li><a id="gpu_address_mode.repeat"></a><code>repeat</code></li>
<li><a id="gpu_address_mode.mirror_repeat"></a><code>mirror-repeat</code></li>
</ul>
<h4><a id="gpu_filter_mode"></a><code>enum gpu-filter-mode</code></h4>
<h5>Enum Cases</h5>
<ul>
<li><a id="gpu_filter_mode.nearest"></a><code>nearest</code></li>
<li><a id="gpu_filter_mode.linear"></a><code>linear</code></li>
</ul>
<h4><a id="gpu_mipmap_filter_mode"></a><code>enum gpu-mipmap-filter-mode</code></h4>
<h5>Enum Cases</h5>
<ul>
<li><a id="gpu_mipmap_filter_mode.nearest"></a><code>nearest</code></li>
<li><a id="gpu_mipmap_filter_mode.linear"></a><code>linear</code></li>
</ul>
<h4><a id="gpu_compare_function"></a><code>enum gpu-compare-function</code></h4>
<h5>Enum Cases</h5>
<ul>
<li><a id="gpu_compare_function.never"></a><code>never</code></li>
<li><a id="gpu_compare_function.less"></a><code>less</code></li>
<li><a id="gpu_compare_function.equal"></a><code>equal</code></li>
<li><a id="gpu_compare_function.less_equal"></a><code>less-equal</code></li>
<li><a id="gpu_compare_function.greater"></a><code>greater</code></li>
<li><a id="gpu_compare_function.not_equal"></a><code>not-equal</code></li>
<li><a id="gpu_compare_function.greater_equal"></a><code>greater-equal</code></li>
<li><a id="gpu_compare_function.always"></a><code>always</code></li>
</ul>
<h4><a id="gpu_sampler_descriptor"></a><code>record gpu-sampler-descriptor</code></h4>
<h5>Record Fields</h5>
<ul>
<li><a id="gpu_sampler_descriptor.address_mode_u"></a><code>address-mode-u</code>: option&lt;<a href="#gpu_address_mode"><a href="#gpu_address_mode"><code>gpu-address-mode</code></a></a>&gt;</li>
<li><a id="gpu_sampler_descriptor.address_mode_v"></a><code>address-mode-v</code>: option&lt;<a href="#gpu_address_mode"><a href="#gpu_address_mode"><code>gpu-address-mode</code></a></a>&gt;</li>
<li><a id="gpu_sampler_descriptor.address_mode_w"></a><code>address-mode-w</code>: option&lt;<a href="#gpu_address_mode"><a href="#gpu_address_mode"><code>gpu-address-mode</code></a></a>&gt;</li>
<li><a id="gpu_sampler_descriptor.mag_filter"></a><code>mag-filter</code>: option&lt;<a href="#gpu_filter_mode"><a href="#gpu_filter_mode"><code>gpu-filter-mode</code></a></a>&gt;</li>
<li><a id="gpu_sampler_descriptor.min_filter"></a><code>min-filter</code>: option&lt;<a href="#gpu_filter_mode"><a href="#gpu_filter_mode"><code>gpu-filter-mode</code></a></a>&gt;</li>
<li><a id="gpu_sampler_descriptor.mipmap_filter"></a><code>mipmap-filter</code>: option&lt;<a href="#gpu_mipmap_filter_mode"><a href="#gpu_mipmap_filter_mode"><code>gpu-mipmap-filter-mode</code></a></a>&gt;</li>
<li><a id="gpu_sampler_descriptor.lod_min_clamp"></a><code>lod-min-clamp</code>: option&lt;<code>f32</code>&gt;</li>
<li><a id="gpu_sampler_descriptor.lod_max_clamp"></a><code>lod-max-clamp</code>: option&lt;<code>f32</code>&gt;</li>
<li><a id="gpu_sampler_descriptor.compare"></a><code>compare</code>: option&lt;<a href="#gpu_compare_function"><a href="#gpu_compare_function"><code>gpu-compare-function</code></a></a>&gt;</li>
<li><a id="gpu_sampler_descriptor.max_anisotropy"></a><code>max-anisotropy</code>: option&lt;<code>u16</code>&gt;</li>
<li><a id="gpu_sampler_descriptor.label"></a><code>label</code>: option&lt;<code>string</code>&gt;</li>
</ul>
<h4><a id="gpu_bind_group_layout"></a><code>resource gpu-bind-group-layout</code></h4>
<h4><a id="gpu_shader_stage_flags"></a><code>type gpu-shader-stage-flags</code></h4>
<p><code>u32</code></p>
<p>
#### <a id="gpu_shader_stage"></a>`resource gpu-shader-stage`
<h4><a id="gpu_buffer_binding_type"></a><code>enum gpu-buffer-binding-type</code></h4>
<h5>Enum Cases</h5>
<ul>
<li><a id="gpu_buffer_binding_type.uniform"></a><code>uniform</code></li>
<li><a id="gpu_buffer_binding_type.storage"></a><code>storage</code></li>
<li><a id="gpu_buffer_binding_type.read_only_storage"></a><code>read-only-storage</code></li>
</ul>
<h4><a id="gpu_sampler_binding_type"></a><code>enum gpu-sampler-binding-type</code></h4>
<h5>Enum Cases</h5>
<ul>
<li><a id="gpu_sampler_binding_type.filtering"></a><code>filtering</code></li>
<li><a id="gpu_sampler_binding_type.non_filtering"></a><code>non-filtering</code></li>
<li><a id="gpu_sampler_binding_type.comparison"></a><code>comparison</code></li>
</ul>
<h4><a id="gpu_sampler_binding_layout"></a><code>record gpu-sampler-binding-layout</code></h4>
<h5>Record Fields</h5>
<ul>
<li><a id="gpu_sampler_binding_layout.type"></a><code>type</code>: option&lt;<a href="#gpu_sampler_binding_type"><a href="#gpu_sampler_binding_type"><code>gpu-sampler-binding-type</code></a></a>&gt;</li>
</ul>
<h4><a id="gpu_texture_sample_type"></a><code>enum gpu-texture-sample-type</code></h4>
<h5>Enum Cases</h5>
<ul>
<li><a id="gpu_texture_sample_type.float"></a><code>float</code></li>
<li><a id="gpu_texture_sample_type.unfilterable_float"></a><code>unfilterable-float</code></li>
<li><a id="gpu_texture_sample_type.depth"></a><code>depth</code></li>
<li><a id="gpu_texture_sample_type.sint"></a><code>sint</code></li>
<li><a id="gpu_texture_sample_type.uint"></a><code>uint</code></li>
</ul>
<h4><a id="gpu_texture_binding_layout"></a><code>record gpu-texture-binding-layout</code></h4>
<h5>Record Fields</h5>
<ul>
<li><a id="gpu_texture_binding_layout.sample_type"></a><code>sample-type</code>: option&lt;<a href="#gpu_texture_sample_type"><a href="#gpu_texture_sample_type"><code>gpu-texture-sample-type</code></a></a>&gt;</li>
<li><a id="gpu_texture_binding_layout.view_dimension"></a><code>view-dimension</code>: option&lt;<a href="#gpu_texture_view_dimension"><a href="#gpu_texture_view_dimension"><code>gpu-texture-view-dimension</code></a></a>&gt;</li>
<li><a id="gpu_texture_binding_layout.multisampled"></a><code>multisampled</code>: option&lt;<code>bool</code>&gt;</li>
</ul>
<h4><a id="gpu_storage_texture_access"></a><code>enum gpu-storage-texture-access</code></h4>
<h5>Enum Cases</h5>
<ul>
<li><a id="gpu_storage_texture_access.write_only"></a><code>write-only</code></li>
<li><a id="gpu_storage_texture_access.read_only"></a><code>read-only</code></li>
<li><a id="gpu_storage_texture_access.read_write"></a><code>read-write</code></li>
</ul>
<h4><a id="gpu_storage_texture_binding_layout"></a><code>record gpu-storage-texture-binding-layout</code></h4>
<h5>Record Fields</h5>
<ul>
<li><a id="gpu_storage_texture_binding_layout.access"></a><code>access</code>: option&lt;<a href="#gpu_storage_texture_access"><a href="#gpu_storage_texture_access"><code>gpu-storage-texture-access</code></a></a>&gt;</li>
<li><a id="gpu_storage_texture_binding_layout.format"></a><code>format</code>: <a href="#gpu_texture_format"><a href="#gpu_texture_format"><code>gpu-texture-format</code></a></a></li>
<li><a id="gpu_storage_texture_binding_layout.view_dimension"></a><code>view-dimension</code>: option&lt;<a href="#gpu_texture_view_dimension"><a href="#gpu_texture_view_dimension"><code>gpu-texture-view-dimension</code></a></a>&gt;</li>
</ul>
<h4><a id="gpu_bind_group"></a><code>resource gpu-bind-group</code></h4>
<h4><a id="gpu_pipeline_layout"></a><code>resource gpu-pipeline-layout</code></h4>
<h4><a id="gpu_pipeline_layout_descriptor"></a><code>record gpu-pipeline-layout-descriptor</code></h4>
<h5>Record Fields</h5>
<ul>
<li><a id="gpu_pipeline_layout_descriptor.bind_group_layouts"></a><code>bind-group-layouts</code>: list&lt;borrow&lt;<a href="#gpu_bind_group_layout"><a href="#gpu_bind_group_layout"><code>gpu-bind-group-layout</code></a></a>&gt;&gt;</li>
<li><a id="gpu_pipeline_layout_descriptor.label"></a><code>label</code>: option&lt;<code>string</code>&gt;</li>
</ul>
<h4><a id="gpu_shader_module"></a><code>resource gpu-shader-module</code></h4>
<h4><a id="gpu_compilation_message_type"></a><code>enum gpu-compilation-message-type</code></h4>
<h5>Enum Cases</h5>
<ul>
<li><a id="gpu_compilation_message_type.error"></a><code>error</code></li>
<li><a id="gpu_compilation_message_type.warning"></a><code>warning</code></li>
<li><a id="gpu_compilation_message_type.info"></a><code>info</code></li>
</ul>
<h4><a id="gpu_compilation_message"></a><code>resource gpu-compilation-message</code></h4>
<h4><a id="gpu_compilation_info"></a><code>resource gpu-compilation-info</code></h4>
<h4><a id="gpu_pipeline_error"></a><code>resource gpu-pipeline-error</code></h4>
<h4><a id="gpu_pipeline_error_reason"></a><code>enum gpu-pipeline-error-reason</code></h4>
<h5>Enum Cases</h5>
<ul>
<li><a id="gpu_pipeline_error_reason.validation"></a><code>validation</code></li>
<li><a id="gpu_pipeline_error_reason.internal"></a><code>internal</code></li>
</ul>
<h4><a id="gpu_pipeline_error_init"></a><code>record gpu-pipeline-error-init</code></h4>
<h5>Record Fields</h5>
<ul>
<li><a id="gpu_pipeline_error_init.reason"></a><code>reason</code>: <a href="#gpu_pipeline_error_reason"><a href="#gpu_pipeline_error_reason"><code>gpu-pipeline-error-reason</code></a></a></li>
</ul>
<h4><a id="gpu_auto_layout_mode"></a><code>enum gpu-auto-layout-mode</code></h4>
<h5>Enum Cases</h5>
<ul>
<li><a id="gpu_auto_layout_mode.auto"></a><code>auto</code></li>
</ul>
<h4><a id="gpu_layout"></a><code>variant gpu-layout</code></h4>
<h5>Variant Cases</h5>
<ul>
<li><a id="gpu_layout.gpu_auto_layout_mode"></a><a href="#gpu_auto_layout_mode"><code>gpu-auto-layout-mode</code></a>: <a href="#gpu_auto_layout_mode"><a href="#gpu_auto_layout_mode"><code>gpu-auto-layout-mode</code></a></a></li>
<li><a id="gpu_layout.gpu_pipeline_layout"></a><a href="#gpu_pipeline_layout"><code>gpu-pipeline-layout</code></a>: borrow&lt;<a href="#gpu_pipeline_layout"><a href="#gpu_pipeline_layout"><code>gpu-pipeline-layout</code></a></a>&gt;</li>
</ul>
<h4><a id="gpu_shader_module_compilation_hint"></a><code>record gpu-shader-module-compilation-hint</code></h4>
<h5>Record Fields</h5>
<ul>
<li><a id="gpu_shader_module_compilation_hint.entry_point"></a><code>entry-point</code>: <code>string</code></li>
<li><a id="gpu_shader_module_compilation_hint.layout"></a><code>layout</code>: option&lt;<a href="#gpu_layout"><a href="#gpu_layout"><code>gpu-layout</code></a></a>&gt;</li>
</ul>
<h4><a id="gpu_shader_module_descriptor"></a><code>record gpu-shader-module-descriptor</code></h4>
<h5>Record Fields</h5>
<ul>
<li><a id="gpu_shader_module_descriptor.code"></a><code>code</code>: <code>string</code></li>
<li><a id="gpu_shader_module_descriptor.compilation_hints"></a><code>compilation-hints</code>: option&lt;list&lt;<a href="#gpu_shader_module_compilation_hint"><a href="#gpu_shader_module_compilation_hint"><code>gpu-shader-module-compilation-hint</code></a></a>&gt;&gt;</li>
<li><a id="gpu_shader_module_descriptor.label"></a><code>label</code>: option&lt;<code>string</code>&gt;</li>
</ul>
<h4><a id="record_gpu_pipeline_constant_value"></a><code>resource record-gpu-pipeline-constant-value</code></h4>
<h4><a id="gpu_programmable_stage"></a><code>record gpu-programmable-stage</code></h4>
<h5>Record Fields</h5>
<ul>
<li><a id="gpu_programmable_stage.module"></a><code>module</code>: borrow&lt;<a href="#gpu_shader_module"><a href="#gpu_shader_module"><code>gpu-shader-module</code></a></a>&gt;</li>
<li><a id="gpu_programmable_stage.entry_point"></a><code>entry-point</code>: option&lt;<code>string</code>&gt;</li>
<li><a id="gpu_programmable_stage.constants"></a><code>constants</code>: option&lt;own&lt;<a href="#record_gpu_pipeline_constant_value"><a href="#record_gpu_pipeline_constant_value"><code>record-gpu-pipeline-constant-value</code></a></a>&gt;&gt;</li>
</ul>
<h4><a id="gpu_pipeline_constant_value"></a><code>type gpu-pipeline-constant-value</code></h4>
<p><code>f64</code></p>
<p>
#### <a id="gpu_compute_pipeline"></a>`resource gpu-compute-pipeline`
<h4><a id="gpu_compute_pipeline_descriptor"></a><code>record gpu-compute-pipeline-descriptor</code></h4>
<h5>Record Fields</h5>
<ul>
<li><a id="gpu_compute_pipeline_descriptor.compute"></a><code>compute</code>: <a href="#gpu_programmable_stage"><a href="#gpu_programmable_stage"><code>gpu-programmable-stage</code></a></a></li>
<li><a id="gpu_compute_pipeline_descriptor.layout"></a><code>layout</code>: <a href="#gpu_layout"><a href="#gpu_layout"><code>gpu-layout</code></a></a></li>
<li><a id="gpu_compute_pipeline_descriptor.label"></a><code>label</code>: option&lt;<code>string</code>&gt;</li>
</ul>
<h4><a id="gpu_render_pipeline"></a><code>resource gpu-render-pipeline</code></h4>
<h4><a id="gpu_primitive_topology"></a><code>enum gpu-primitive-topology</code></h4>
<h5>Enum Cases</h5>
<ul>
<li><a id="gpu_primitive_topology.point_list"></a><code>point-list</code></li>
<li><a id="gpu_primitive_topology.line_list"></a><code>line-list</code></li>
<li><a id="gpu_primitive_topology.line_strip"></a><code>line-strip</code></li>
<li><a id="gpu_primitive_topology.triangle_list"></a><code>triangle-list</code></li>
<li><a id="gpu_primitive_topology.triangle_strip"></a><code>triangle-strip</code></li>
</ul>
<h4><a id="gpu_front_face"></a><code>enum gpu-front-face</code></h4>
<h5>Enum Cases</h5>
<ul>
<li><a id="gpu_front_face.ccw"></a><code>ccw</code></li>
<li><a id="gpu_front_face.cw"></a><code>cw</code></li>
</ul>
<h4><a id="gpu_cull_mode"></a><code>enum gpu-cull-mode</code></h4>
<h5>Enum Cases</h5>
<ul>
<li><a id="gpu_cull_mode.none"></a><code>none</code></li>
<li><a id="gpu_cull_mode.front"></a><code>front</code></li>
<li><a id="gpu_cull_mode.back"></a><code>back</code></li>
</ul>
<h4><a id="gpu_color_write_flags"></a><code>type gpu-color-write-flags</code></h4>
<p><code>u32</code></p>
<p>
#### <a id="gpu_color_write"></a>`resource gpu-color-write`
<h4><a id="gpu_blend_factor"></a><code>enum gpu-blend-factor</code></h4>
<h5>Enum Cases</h5>
<ul>
<li><a id="gpu_blend_factor.zero"></a><code>zero</code></li>
<li><a id="gpu_blend_factor.one"></a><code>one</code></li>
<li><a id="gpu_blend_factor.src"></a><code>src</code></li>
<li><a id="gpu_blend_factor.one_minus_src"></a><code>one-minus-src</code></li>
<li><a id="gpu_blend_factor.src_alpha"></a><code>src-alpha</code></li>
<li><a id="gpu_blend_factor.one_minus_src_alpha"></a><code>one-minus-src-alpha</code></li>
<li><a id="gpu_blend_factor.dst"></a><code>dst</code></li>
<li><a id="gpu_blend_factor.one_minus_dst"></a><code>one-minus-dst</code></li>
<li><a id="gpu_blend_factor.dst_alpha"></a><code>dst-alpha</code></li>
<li><a id="gpu_blend_factor.one_minus_dst_alpha"></a><code>one-minus-dst-alpha</code></li>
<li><a id="gpu_blend_factor.src_alpha_saturated"></a><code>src-alpha-saturated</code></li>
<li><a id="gpu_blend_factor.constant"></a><code>constant</code></li>
<li><a id="gpu_blend_factor.one_minus_constant"></a><code>one-minus-constant</code></li>
<li><a id="gpu_blend_factor.src1"></a><code>src1</code></li>
<li><a id="gpu_blend_factor.one_minus_src1"></a><code>one-minus-src1</code></li>
<li><a id="gpu_blend_factor.src1_alpha"></a><code>src1-alpha</code></li>
<li><a id="gpu_blend_factor.one_minus_src1_alpha"></a><code>one-minus-src1-alpha</code></li>
</ul>
<h4><a id="gpu_blend_operation"></a><code>enum gpu-blend-operation</code></h4>
<h5>Enum Cases</h5>
<ul>
<li><a id="gpu_blend_operation.add"></a><code>add</code></li>
<li><a id="gpu_blend_operation.subtract"></a><code>subtract</code></li>
<li><a id="gpu_blend_operation.reverse_subtract"></a><code>reverse-subtract</code></li>
<li><a id="gpu_blend_operation.min"></a><code>min</code></li>
<li><a id="gpu_blend_operation.max"></a><code>max</code></li>
</ul>
<h4><a id="gpu_blend_component"></a><code>record gpu-blend-component</code></h4>
<h5>Record Fields</h5>
<ul>
<li><a id="gpu_blend_component.operation"></a><code>operation</code>: option&lt;<a href="#gpu_blend_operation"><a href="#gpu_blend_operation"><code>gpu-blend-operation</code></a></a>&gt;</li>
<li><a id="gpu_blend_component.src_factor"></a><code>src-factor</code>: option&lt;<a href="#gpu_blend_factor"><a href="#gpu_blend_factor"><code>gpu-blend-factor</code></a></a>&gt;</li>
<li><a id="gpu_blend_component.dst_factor"></a><code>dst-factor</code>: option&lt;<a href="#gpu_blend_factor"><a href="#gpu_blend_factor"><code>gpu-blend-factor</code></a></a>&gt;</li>
</ul>
<h4><a id="gpu_blend_state"></a><code>record gpu-blend-state</code></h4>
<h5>Record Fields</h5>
<ul>
<li><a id="gpu_blend_state.color"></a><code>color</code>: <a href="#gpu_blend_component"><a href="#gpu_blend_component"><code>gpu-blend-component</code></a></a></li>
<li><a id="gpu_blend_state.alpha"></a><code>alpha</code>: <a href="#gpu_blend_component"><a href="#gpu_blend_component"><code>gpu-blend-component</code></a></a></li>
</ul>
<h4><a id="gpu_color_target_state"></a><code>record gpu-color-target-state</code></h4>
<h5>Record Fields</h5>
<ul>
<li><a id="gpu_color_target_state.format"></a><code>format</code>: <a href="#gpu_texture_format"><a href="#gpu_texture_format"><code>gpu-texture-format</code></a></a></li>
<li><a id="gpu_color_target_state.blend"></a><code>blend</code>: option&lt;<a href="#gpu_blend_state"><a href="#gpu_blend_state"><code>gpu-blend-state</code></a></a>&gt;</li>
<li><a id="gpu_color_target_state.write_mask"></a><code>write-mask</code>: option&lt;<a href="#gpu_color_write_flags"><a href="#gpu_color_write_flags"><code>gpu-color-write-flags</code></a></a>&gt;</li>
</ul>
<h4><a id="gpu_fragment_state"></a><code>record gpu-fragment-state</code></h4>
<h5>Record Fields</h5>
<ul>
<li><a id="gpu_fragment_state.targets"></a><code>targets</code>: list&lt;option&lt;<a href="#gpu_color_target_state"><a href="#gpu_color_target_state"><code>gpu-color-target-state</code></a></a>&gt;&gt;</li>
<li><a id="gpu_fragment_state.module"></a><code>module</code>: borrow&lt;<a href="#gpu_shader_module"><a href="#gpu_shader_module"><code>gpu-shader-module</code></a></a>&gt;</li>
<li><a id="gpu_fragment_state.entry_point"></a><code>entry-point</code>: option&lt;<code>string</code>&gt;</li>
<li><a id="gpu_fragment_state.constants"></a><code>constants</code>: option&lt;own&lt;<a href="#record_gpu_pipeline_constant_value"><a href="#record_gpu_pipeline_constant_value"><code>record-gpu-pipeline-constant-value</code></a></a>&gt;&gt;</li>
</ul>
<h4><a id="gpu_stencil_operation"></a><code>enum gpu-stencil-operation</code></h4>
<h5>Enum Cases</h5>
<ul>
<li><a id="gpu_stencil_operation.keep"></a><code>keep</code></li>
<li><a id="gpu_stencil_operation.zero"></a><code>zero</code></li>
<li><a id="gpu_stencil_operation.replace"></a><code>replace</code></li>
<li><a id="gpu_stencil_operation.invert"></a><code>invert</code></li>
<li><a id="gpu_stencil_operation.increment_clamp"></a><code>increment-clamp</code></li>
<li><a id="gpu_stencil_operation.decrement_clamp"></a><code>decrement-clamp</code></li>
<li><a id="gpu_stencil_operation.increment_wrap"></a><code>increment-wrap</code></li>
<li><a id="gpu_stencil_operation.decrement_wrap"></a><code>decrement-wrap</code></li>
</ul>
<h4><a id="gpu_stencil_face_state"></a><code>record gpu-stencil-face-state</code></h4>
<h5>Record Fields</h5>
<ul>
<li><a id="gpu_stencil_face_state.compare"></a><code>compare</code>: option&lt;<a href="#gpu_compare_function"><a href="#gpu_compare_function"><code>gpu-compare-function</code></a></a>&gt;</li>
<li><a id="gpu_stencil_face_state.fail_op"></a><code>fail-op</code>: option&lt;<a href="#gpu_stencil_operation"><a href="#gpu_stencil_operation"><code>gpu-stencil-operation</code></a></a>&gt;</li>
<li><a id="gpu_stencil_face_state.depth_fail_op"></a><code>depth-fail-op</code>: option&lt;<a href="#gpu_stencil_operation"><a href="#gpu_stencil_operation"><code>gpu-stencil-operation</code></a></a>&gt;</li>
<li><a id="gpu_stencil_face_state.pass_op"></a><code>pass-op</code>: option&lt;<a href="#gpu_stencil_operation"><a href="#gpu_stencil_operation"><code>gpu-stencil-operation</code></a></a>&gt;</li>
</ul>
<h4><a id="gpu_index_format"></a><code>enum gpu-index-format</code></h4>
<h5>Enum Cases</h5>
<ul>
<li><a id="gpu_index_format.uint16"></a><code>uint16</code></li>
<li><a id="gpu_index_format.uint32"></a><code>uint32</code></li>
</ul>
<h4><a id="gpu_primitive_state"></a><code>record gpu-primitive-state</code></h4>
<h5>Record Fields</h5>
<ul>
<li><a id="gpu_primitive_state.topology"></a><code>topology</code>: option&lt;<a href="#gpu_primitive_topology"><a href="#gpu_primitive_topology"><code>gpu-primitive-topology</code></a></a>&gt;</li>
<li><a id="gpu_primitive_state.strip_index_format"></a><code>strip-index-format</code>: option&lt;<a href="#gpu_index_format"><a href="#gpu_index_format"><code>gpu-index-format</code></a></a>&gt;</li>
<li><a id="gpu_primitive_state.front_face"></a><code>front-face</code>: option&lt;<a href="#gpu_front_face"><a href="#gpu_front_face"><code>gpu-front-face</code></a></a>&gt;</li>
<li><a id="gpu_primitive_state.cull_mode"></a><code>cull-mode</code>: option&lt;<a href="#gpu_cull_mode"><a href="#gpu_cull_mode"><code>gpu-cull-mode</code></a></a>&gt;</li>
<li><a id="gpu_primitive_state.unclipped_depth"></a><code>unclipped-depth</code>: option&lt;<code>bool</code>&gt;</li>
</ul>
<h4><a id="gpu_vertex_format"></a><code>enum gpu-vertex-format</code></h4>
<h5>Enum Cases</h5>
<ul>
<li><a id="gpu_vertex_format.uint8x2"></a><code>uint8x2</code></li>
<li><a id="gpu_vertex_format.uint8x4"></a><code>uint8x4</code></li>
<li><a id="gpu_vertex_format.sint8x2"></a><code>sint8x2</code></li>
<li><a id="gpu_vertex_format.sint8x4"></a><code>sint8x4</code></li>
<li><a id="gpu_vertex_format.unorm8x2"></a><code>unorm8x2</code></li>
<li><a id="gpu_vertex_format.unorm8x4"></a><code>unorm8x4</code></li>
<li><a id="gpu_vertex_format.snorm8x2"></a><code>snorm8x2</code></li>
<li><a id="gpu_vertex_format.snorm8x4"></a><code>snorm8x4</code></li>
<li><a id="gpu_vertex_format.uint16x2"></a><code>uint16x2</code></li>
<li><a id="gpu_vertex_format.uint16x4"></a><code>uint16x4</code></li>
<li><a id="gpu_vertex_format.sint16x2"></a><code>sint16x2</code></li>
<li><a id="gpu_vertex_format.sint16x4"></a><code>sint16x4</code></li>
<li><a id="gpu_vertex_format.unorm16x2"></a><code>unorm16x2</code></li>
<li><a id="gpu_vertex_format.unorm16x4"></a><code>unorm16x4</code></li>
<li><a id="gpu_vertex_format.snorm16x2"></a><code>snorm16x2</code></li>
<li><a id="gpu_vertex_format.snorm16x4"></a><code>snorm16x4</code></li>
<li><a id="gpu_vertex_format.float16x2"></a><code>float16x2</code></li>
<li><a id="gpu_vertex_format.float16x4"></a><code>float16x4</code></li>
<li><a id="gpu_vertex_format.float32"></a><code>float32</code></li>
<li><a id="gpu_vertex_format.float32x2"></a><code>float32x2</code></li>
<li><a id="gpu_vertex_format.float32x3"></a><code>float32x3</code></li>
<li><a id="gpu_vertex_format.float32x4"></a><code>float32x4</code></li>
<li><a id="gpu_vertex_format.uint32"></a><code>uint32</code></li>
<li><a id="gpu_vertex_format.uint32x2"></a><code>uint32x2</code></li>
<li><a id="gpu_vertex_format.uint32x3"></a><code>uint32x3</code></li>
<li><a id="gpu_vertex_format.uint32x4"></a><code>uint32x4</code></li>
<li><a id="gpu_vertex_format.sint32"></a><code>sint32</code></li>
<li><a id="gpu_vertex_format.sint32x2"></a><code>sint32x2</code></li>
<li><a id="gpu_vertex_format.sint32x3"></a><code>sint32x3</code></li>
<li><a id="gpu_vertex_format.sint32x4"></a><code>sint32x4</code></li>
<li><a id="gpu_vertex_format.unorm1010102"></a><code>unorm1010102</code></li>
</ul>
<h4><a id="gpu_vertex_step_mode"></a><code>enum gpu-vertex-step-mode</code></h4>
<h5>Enum Cases</h5>
<ul>
<li><a id="gpu_vertex_step_mode.vertex"></a><code>vertex</code></li>
<li><a id="gpu_vertex_step_mode.instance"></a><code>instance</code></li>
</ul>
<h4><a id="gpu_command_buffer"></a><code>resource gpu-command-buffer</code></h4>
<h4><a id="gpu_command_buffer_descriptor"></a><code>record gpu-command-buffer-descriptor</code></h4>
<h5>Record Fields</h5>
<ul>
<li><a id="gpu_command_buffer_descriptor.label"></a><code>label</code>: option&lt;<code>string</code>&gt;</li>
</ul>
<h4><a id="gpu_command_encoder"></a><code>resource gpu-command-encoder</code></h4>
<h4><a id="gpu_command_encoder_descriptor"></a><code>record gpu-command-encoder-descriptor</code></h4>
<h5>Record Fields</h5>
<ul>
<li><a id="gpu_command_encoder_descriptor.label"></a><code>label</code>: option&lt;<code>string</code>&gt;</li>
</ul>
<h4><a id="gpu_compute_pass_encoder"></a><code>resource gpu-compute-pass-encoder</code></h4>
<h4><a id="gpu_render_pass_encoder"></a><code>resource gpu-render-pass-encoder</code></h4>
<h4><a id="gpu_load_op"></a><code>enum gpu-load-op</code></h4>
<h5>Enum Cases</h5>
<ul>
<li><a id="gpu_load_op.load"></a><code>load</code></li>
<li><a id="gpu_load_op.clear"></a><code>clear</code></li>
</ul>
<h4><a id="gpu_store_op"></a><code>enum gpu-store-op</code></h4>
<h5>Enum Cases</h5>
<ul>
<li><a id="gpu_store_op.store"></a><code>store</code></li>
<li><a id="gpu_store_op.discard"></a><code>discard</code></li>
</ul>
<h4><a id="gpu_render_bundle"></a><code>resource gpu-render-bundle</code></h4>
<h4><a id="gpu_render_bundle_descriptor"></a><code>record gpu-render-bundle-descriptor</code></h4>
<h5>Record Fields</h5>
<ul>
<li><a id="gpu_render_bundle_descriptor.label"></a><code>label</code>: option&lt;<code>string</code>&gt;</li>
</ul>
<h4><a id="gpu_render_bundle_encoder"></a><code>resource gpu-render-bundle-encoder</code></h4>
<h4><a id="gpu_queue_descriptor"></a><code>record gpu-queue-descriptor</code></h4>
<h5>Record Fields</h5>
<ul>
<li><a id="gpu_queue_descriptor.label"></a><code>label</code>: option&lt;<code>string</code>&gt;</li>
</ul>
<h4><a id="gpu_device_descriptor"></a><code>record gpu-device-descriptor</code></h4>
<h5>Record Fields</h5>
<ul>
<li><a id="gpu_device_descriptor.required_features"></a><code>required-features</code>: option&lt;list&lt;<a href="#gpu_feature_name"><a href="#gpu_feature_name"><code>gpu-feature-name</code></a></a>&gt;&gt;</li>
<li><a id="gpu_device_descriptor.required_limits"></a><code>required-limits</code>: option&lt;own&lt;<a href="#record_gpu_size64"><a href="#record_gpu_size64"><code>record-gpu-size64</code></a></a>&gt;&gt;</li>
<li><a id="gpu_device_descriptor.default_queue"></a><code>default-queue</code>: option&lt;<a href="#gpu_queue_descriptor"><a href="#gpu_queue_descriptor"><code>gpu-queue-descriptor</code></a></a>&gt;</li>
<li><a id="gpu_device_descriptor.label"></a><code>label</code>: option&lt;<code>string</code>&gt;</li>
</ul>
<h4><a id="gpu_queue"></a><code>resource gpu-queue</code></h4>
<h4><a id="gpu_query_set"></a><code>resource gpu-query-set</code></h4>
<h4><a id="gpu_query_type"></a><code>enum gpu-query-type</code></h4>
<h5>Enum Cases</h5>
<ul>
<li><a id="gpu_query_type.occlusion"></a><code>occlusion</code></li>
<li><a id="gpu_query_type.timestamp"></a><code>timestamp</code></li>
</ul>
<h4><a id="gpu_canvas_context"></a><code>resource gpu-canvas-context</code></h4>
<h4><a id="gpu_canvas_alpha_mode"></a><code>enum gpu-canvas-alpha-mode</code></h4>
<h5>Enum Cases</h5>
<ul>
<li><a id="gpu_canvas_alpha_mode.opaque"></a><code>opaque</code></li>
<li><a id="gpu_canvas_alpha_mode.premultiplied"></a><code>premultiplied</code></li>
</ul>
<h4><a id="gpu_canvas_tone_mapping_mode"></a><code>enum gpu-canvas-tone-mapping-mode</code></h4>
<h5>Enum Cases</h5>
<ul>
<li><a id="gpu_canvas_tone_mapping_mode.standard"></a><code>standard</code></li>
<li><a id="gpu_canvas_tone_mapping_mode.extended"></a><code>extended</code></li>
</ul>
<h4><a id="gpu_canvas_tone_mapping"></a><code>record gpu-canvas-tone-mapping</code></h4>
<h5>Record Fields</h5>
<ul>
<li><a id="gpu_canvas_tone_mapping.mode"></a><code>mode</code>: option&lt;<a href="#gpu_canvas_tone_mapping_mode"><a href="#gpu_canvas_tone_mapping_mode"><code>gpu-canvas-tone-mapping-mode</code></a></a>&gt;</li>
</ul>
<h4><a id="gpu_device_lost_reason"></a><code>enum gpu-device-lost-reason</code></h4>
<h5>Enum Cases</h5>
<ul>
<li><a id="gpu_device_lost_reason.unknown"></a><code>unknown</code></li>
<li><a id="gpu_device_lost_reason.destroyed"></a><code>destroyed</code></li>
</ul>
<h4><a id="gpu_device_lost_info"></a><code>resource gpu-device-lost-info</code></h4>
<h4><a id="gpu_error"></a><code>resource gpu-error</code></h4>
<h4><a id="gpu_validation_error"></a><code>resource gpu-validation-error</code></h4>
<h4><a id="gpu_out_of_memory_error"></a><code>resource gpu-out-of-memory-error</code></h4>
<h4><a id="gpu_internal_error"></a><code>resource gpu-internal-error</code></h4>
<h4><a id="gpu_error_filter"></a><code>enum gpu-error-filter</code></h4>
<h5>Enum Cases</h5>
<ul>
<li><a id="gpu_error_filter.validation"></a><code>validation</code></li>
<li><a id="gpu_error_filter.out_of_memory"></a><code>out-of-memory</code></li>
<li><a id="gpu_error_filter.internal"></a><code>internal</code></li>
</ul>
<h4><a id="gpu_uncaptured_error_event"></a><code>resource gpu-uncaptured-error-event</code></h4>
<h4><a id="gpu_uncaptured_error_event_init"></a><code>record gpu-uncaptured-error-event-init</code></h4>
<h5>Record Fields</h5>
<ul>
<li><a id="gpu_uncaptured_error_event_init.error"></a><code>error</code>: borrow&lt;<a href="#gpu_error"><a href="#gpu_error"><code>gpu-error</code></a></a>&gt;</li>
</ul>
<h4><a id="gpu_buffer_dynamic_offset"></a><code>type gpu-buffer-dynamic-offset</code></h4>
<p><code>u32</code></p>
<p>
#### <a id="gpu_stencil_value"></a>`type gpu-stencil-value`
`u32`
<p>
#### <a id="gpu_render_pass_depth_stencil_attachment"></a>`record gpu-render-pass-depth-stencil-attachment`
<h5>Record Fields</h5>
<ul>
<li><a id="gpu_render_pass_depth_stencil_attachment.view"></a><code>view</code>: borrow&lt;<a href="#gpu_texture_view"><a href="#gpu_texture_view"><code>gpu-texture-view</code></a></a>&gt;</li>
<li><a id="gpu_render_pass_depth_stencil_attachment.depth_clear_value"></a><code>depth-clear-value</code>: option&lt;<code>f32</code>&gt;</li>
<li><a id="gpu_render_pass_depth_stencil_attachment.depth_load_op"></a><code>depth-load-op</code>: option&lt;<a href="#gpu_load_op"><a href="#gpu_load_op"><code>gpu-load-op</code></a></a>&gt;</li>
<li><a id="gpu_render_pass_depth_stencil_attachment.depth_store_op"></a><code>depth-store-op</code>: option&lt;<a href="#gpu_store_op"><a href="#gpu_store_op"><code>gpu-store-op</code></a></a>&gt;</li>
<li><a id="gpu_render_pass_depth_stencil_attachment.depth_read_only"></a><code>depth-read-only</code>: option&lt;<code>bool</code>&gt;</li>
<li><a id="gpu_render_pass_depth_stencil_attachment.stencil_clear_value"></a><code>stencil-clear-value</code>: option&lt;<a href="#gpu_stencil_value"><a href="#gpu_stencil_value"><code>gpu-stencil-value</code></a></a>&gt;</li>
<li><a id="gpu_render_pass_depth_stencil_attachment.stencil_load_op"></a><code>stencil-load-op</code>: option&lt;<a href="#gpu_load_op"><a href="#gpu_load_op"><code>gpu-load-op</code></a></a>&gt;</li>
<li><a id="gpu_render_pass_depth_stencil_attachment.stencil_store_op"></a><code>stencil-store-op</code>: option&lt;<a href="#gpu_store_op"><a href="#gpu_store_op"><code>gpu-store-op</code></a></a>&gt;</li>
<li><a id="gpu_render_pass_depth_stencil_attachment.stencil_read_only"></a><code>stencil-read-only</code>: option&lt;<code>bool</code>&gt;</li>
</ul>
<h4><a id="gpu_sample_mask"></a><code>type gpu-sample-mask</code></h4>
<p><code>u32</code></p>
<p>
#### <a id="gpu_depth_bias"></a>`type gpu-depth-bias`
`s32`
<p>
#### <a id="gpu_depth_stencil_state"></a>`record gpu-depth-stencil-state`
<h5>Record Fields</h5>
<ul>
<li><a id="gpu_depth_stencil_state.format"></a><code>format</code>: <a href="#gpu_texture_format"><a href="#gpu_texture_format"><code>gpu-texture-format</code></a></a></li>
<li><a id="gpu_depth_stencil_state.depth_write_enabled"></a><code>depth-write-enabled</code>: option&lt;<code>bool</code>&gt;</li>
<li><a id="gpu_depth_stencil_state.depth_compare"></a><code>depth-compare</code>: option&lt;<a href="#gpu_compare_function"><a href="#gpu_compare_function"><code>gpu-compare-function</code></a></a>&gt;</li>
<li><a id="gpu_depth_stencil_state.stencil_front"></a><code>stencil-front</code>: option&lt;<a href="#gpu_stencil_face_state"><a href="#gpu_stencil_face_state"><code>gpu-stencil-face-state</code></a></a>&gt;</li>
<li><a id="gpu_depth_stencil_state.stencil_back"></a><code>stencil-back</code>: option&lt;<a href="#gpu_stencil_face_state"><a href="#gpu_stencil_face_state"><code>gpu-stencil-face-state</code></a></a>&gt;</li>
<li><a id="gpu_depth_stencil_state.stencil_read_mask"></a><code>stencil-read-mask</code>: option&lt;<a href="#gpu_stencil_value"><a href="#gpu_stencil_value"><code>gpu-stencil-value</code></a></a>&gt;</li>
<li><a id="gpu_depth_stencil_state.stencil_write_mask"></a><code>stencil-write-mask</code>: option&lt;<a href="#gpu_stencil_value"><a href="#gpu_stencil_value"><code>gpu-stencil-value</code></a></a>&gt;</li>
<li><a id="gpu_depth_stencil_state.depth_bias"></a><code>depth-bias</code>: option&lt;<a href="#gpu_depth_bias"><a href="#gpu_depth_bias"><code>gpu-depth-bias</code></a></a>&gt;</li>
<li><a id="gpu_depth_stencil_state.depth_bias_slope_scale"></a><code>depth-bias-slope-scale</code>: option&lt;<code>f32</code>&gt;</li>
<li><a id="gpu_depth_stencil_state.depth_bias_clamp"></a><code>depth-bias-clamp</code>: option&lt;<code>f32</code>&gt;</li>
</ul>
<h4><a id="gpu_size64"></a><code>type gpu-size64</code></h4>
<p><code>u64</code></p>
<p>
#### <a id="gpu_buffer_descriptor"></a>`record gpu-buffer-descriptor`
<h5>Record Fields</h5>
<ul>
<li><a id="gpu_buffer_descriptor.size"></a><code>size</code>: <a href="#gpu_size64"><a href="#gpu_size64"><code>gpu-size64</code></a></a></li>
<li><a id="gpu_buffer_descriptor.usage"></a><code>usage</code>: <a href="#gpu_buffer_usage_flags"><a href="#gpu_buffer_usage_flags"><code>gpu-buffer-usage-flags</code></a></a></li>
<li><a id="gpu_buffer_descriptor.mapped_at_creation"></a><code>mapped-at-creation</code>: option&lt;<code>bool</code>&gt;</li>
<li><a id="gpu_buffer_descriptor.label"></a><code>label</code>: option&lt;<code>string</code>&gt;</li>
</ul>
<h4><a id="gpu_buffer_binding_layout"></a><code>record gpu-buffer-binding-layout</code></h4>
<h5>Record Fields</h5>
<ul>
<li><a id="gpu_buffer_binding_layout.type"></a><code>type</code>: option&lt;<a href="#gpu_buffer_binding_type"><a href="#gpu_buffer_binding_type"><code>gpu-buffer-binding-type</code></a></a>&gt;</li>
<li><a id="gpu_buffer_binding_layout.has_dynamic_offset"></a><code>has-dynamic-offset</code>: option&lt;<code>bool</code>&gt;</li>
<li><a id="gpu_buffer_binding_layout.min_binding_size"></a><code>min-binding-size</code>: option&lt;<a href="#gpu_size64"><a href="#gpu_size64"><code>gpu-size64</code></a></a>&gt;</li>
</ul>
<h4><a id="gpu_buffer_binding"></a><code>record gpu-buffer-binding</code></h4>
<h5>Record Fields</h5>
<ul>
<li><a id="gpu_buffer_binding.buffer"></a><code>buffer</code>: borrow&lt;<a href="#gpu_buffer"><a href="#gpu_buffer"><code>gpu-buffer</code></a></a>&gt;</li>
<li><a id="gpu_buffer_binding.offset"></a><code>offset</code>: option&lt;<a href="#gpu_size64"><a href="#gpu_size64"><code>gpu-size64</code></a></a>&gt;</li>
<li><a id="gpu_buffer_binding.size"></a><code>size</code>: option&lt;<a href="#gpu_size64"><a href="#gpu_size64"><code>gpu-size64</code></a></a>&gt;</li>
</ul>
<h4><a id="gpu_binding_resource"></a><code>variant gpu-binding-resource</code></h4>
<h5>Variant Cases</h5>
<ul>
<li><a id="gpu_binding_resource.gpu_buffer_binding"></a><a href="#gpu_buffer_binding"><code>gpu-buffer-binding</code></a>: <a href="#gpu_buffer_binding"><a href="#gpu_buffer_binding"><code>gpu-buffer-binding</code></a></a></li>
<li><a id="gpu_binding_resource.gpu_sampler"></a><a href="#gpu_sampler"><code>gpu-sampler</code></a>: borrow&lt;<a href="#gpu_sampler"><a href="#gpu_sampler"><code>gpu-sampler</code></a></a>&gt;</li>
<li><a id="gpu_binding_resource.gpu_texture_view"></a><a href="#gpu_texture_view"><code>gpu-texture-view</code></a>: borrow&lt;<a href="#gpu_texture_view"><a href="#gpu_texture_view"><code>gpu-texture-view</code></a></a>&gt;</li>
</ul>
<h4><a id="gpu_integer_coordinate"></a><code>type gpu-integer-coordinate</code></h4>
<p><code>u32</code></p>
<p>
#### <a id="gpu_texture_view_descriptor"></a>`record gpu-texture-view-descriptor`
<h5>Record Fields</h5>
<ul>
<li><a id="gpu_texture_view_descriptor.format"></a><code>format</code>: option&lt;<a href="#gpu_texture_format"><a href="#gpu_texture_format"><code>gpu-texture-format</code></a></a>&gt;</li>
<li><a id="gpu_texture_view_descriptor.dimension"></a><code>dimension</code>: option&lt;<a href="#gpu_texture_view_dimension"><a href="#gpu_texture_view_dimension"><code>gpu-texture-view-dimension</code></a></a>&gt;</li>
<li><a id="gpu_texture_view_descriptor.usage"></a><code>usage</code>: option&lt;<a href="#gpu_texture_usage_flags"><a href="#gpu_texture_usage_flags"><code>gpu-texture-usage-flags</code></a></a>&gt;</li>
<li><a id="gpu_texture_view_descriptor.aspect"></a><code>aspect</code>: option&lt;<a href="#gpu_texture_aspect"><a href="#gpu_texture_aspect"><code>gpu-texture-aspect</code></a></a>&gt;</li>
<li><a id="gpu_texture_view_descriptor.base_mip_level"></a><code>base-mip-level</code>: option&lt;<a href="#gpu_integer_coordinate"><a href="#gpu_integer_coordinate"><code>gpu-integer-coordinate</code></a></a>&gt;</li>
<li><a id="gpu_texture_view_descriptor.mip_level_count"></a><code>mip-level-count</code>: option&lt;<a href="#gpu_integer_coordinate"><a href="#gpu_integer_coordinate"><code>gpu-integer-coordinate</code></a></a>&gt;</li>
<li><a id="gpu_texture_view_descriptor.base_array_layer"></a><code>base-array-layer</code>: option&lt;<a href="#gpu_integer_coordinate"><a href="#gpu_integer_coordinate"><code>gpu-integer-coordinate</code></a></a>&gt;</li>
<li><a id="gpu_texture_view_descriptor.array_layer_count"></a><code>array-layer-count</code>: option&lt;<a href="#gpu_integer_coordinate"><a href="#gpu_integer_coordinate"><code>gpu-integer-coordinate</code></a></a>&gt;</li>
<li><a id="gpu_texture_view_descriptor.label"></a><code>label</code>: option&lt;<code>string</code>&gt;</li>
</ul>
<h4><a id="gpu_index32"></a><code>type gpu-index32</code></h4>
<p><code>u32</code></p>
<p>
#### <a id="gpu_bind_group_layout_entry"></a>`record gpu-bind-group-layout-entry`
<h5>Record Fields</h5>
<ul>
<li><a id="gpu_bind_group_layout_entry.binding"></a><code>binding</code>: <a href="#gpu_index32"><a href="#gpu_index32"><code>gpu-index32</code></a></a></li>
<li><a id="gpu_bind_group_layout_entry.visibility"></a><code>visibility</code>: <a href="#gpu_shader_stage_flags"><a href="#gpu_shader_stage_flags"><code>gpu-shader-stage-flags</code></a></a></li>
<li><a id="gpu_bind_group_layout_entry.buffer"></a><code>buffer</code>: option&lt;<a href="#gpu_buffer_binding_layout"><a href="#gpu_buffer_binding_layout"><code>gpu-buffer-binding-layout</code></a></a>&gt;</li>
<li><a id="gpu_bind_group_layout_entry.sampler"></a><code>sampler</code>: option&lt;<a href="#gpu_sampler_binding_layout"><a href="#gpu_sampler_binding_layout"><code>gpu-sampler-binding-layout</code></a></a>&gt;</li>
<li><a id="gpu_bind_group_layout_entry.texture"></a><code>texture</code>: option&lt;<a href="#gpu_texture_binding_layout"><a href="#gpu_texture_binding_layout"><code>gpu-texture-binding-layout</code></a></a>&gt;</li>
<li><a id="gpu_bind_group_layout_entry.storage_texture"></a><code>storage-texture</code>: option&lt;<a href="#gpu_storage_texture_binding_layout"><a href="#gpu_storage_texture_binding_layout"><code>gpu-storage-texture-binding-layout</code></a></a>&gt;</li>
</ul>
<h4><a id="gpu_bind_group_layout_descriptor"></a><code>record gpu-bind-group-layout-descriptor</code></h4>
<h5>Record Fields</h5>
<ul>
<li><a id="gpu_bind_group_layout_descriptor.entries"></a><code>entries</code>: list&lt;<a href="#gpu_bind_group_layout_entry"><a href="#gpu_bind_group_layout_entry"><code>gpu-bind-group-layout-entry</code></a></a>&gt;</li>
<li><a id="gpu_bind_group_layout_descriptor.label"></a><code>label</code>: option&lt;<code>string</code>&gt;</li>
</ul>
<h4><a id="gpu_bind_group_entry"></a><code>record gpu-bind-group-entry</code></h4>
<h5>Record Fields</h5>
<ul>
<li><a id="gpu_bind_group_entry.binding"></a><code>binding</code>: <a href="#gpu_index32"><a href="#gpu_index32"><code>gpu-index32</code></a></a></li>
<li><a id="gpu_bind_group_entry.resource"></a><code>resource</code>: <a href="#gpu_binding_resource"><a href="#gpu_binding_resource"><code>gpu-binding-resource</code></a></a></li>
</ul>
<h4><a id="gpu_bind_group_descriptor"></a><code>record gpu-bind-group-descriptor</code></h4>
<h5>Record Fields</h5>
<ul>
<li><a id="gpu_bind_group_descriptor.layout"></a><code>layout</code>: borrow&lt;<a href="#gpu_bind_group_layout"><a href="#gpu_bind_group_layout"><code>gpu-bind-group-layout</code></a></a>&gt;</li>
<li><a id="gpu_bind_group_descriptor.entries"></a><code>entries</code>: list&lt;<a href="#gpu_bind_group_entry"><a href="#gpu_bind_group_entry"><code>gpu-bind-group-entry</code></a></a>&gt;</li>
<li><a id="gpu_bind_group_descriptor.label"></a><code>label</code>: option&lt;<code>string</code>&gt;</li>
</ul>
<h4><a id="gpu_vertex_attribute"></a><code>record gpu-vertex-attribute</code></h4>
<h5>Record Fields</h5>
<ul>
<li><a id="gpu_vertex_attribute.format"></a><code>format</code>: <a href="#gpu_vertex_format"><a href="#gpu_vertex_format"><code>gpu-vertex-format</code></a></a></li>
<li><a id="gpu_vertex_attribute.offset"></a><code>offset</code>: <a href="#gpu_size64"><a href="#gpu_size64"><code>gpu-size64</code></a></a></li>
<li><a id="gpu_vertex_attribute.shader_location"></a><code>shader-location</code>: <a href="#gpu_index32"><a href="#gpu_index32"><code>gpu-index32</code></a></a></li>
</ul>
<h4><a id="gpu_vertex_buffer_layout"></a><code>record gpu-vertex-buffer-layout</code></h4>
<h5>Record Fields</h5>
<ul>
<li><a id="gpu_vertex_buffer_layout.array_stride"></a><code>array-stride</code>: <a href="#gpu_size64"><a href="#gpu_size64"><code>gpu-size64</code></a></a></li>
<li><a id="gpu_vertex_buffer_layout.step_mode"></a><code>step-mode</code>: option&lt;<a href="#gpu_vertex_step_mode"><a href="#gpu_vertex_step_mode"><code>gpu-vertex-step-mode</code></a></a>&gt;</li>
<li><a id="gpu_vertex_buffer_layout.attributes"></a><code>attributes</code>: list&lt;<a href="#gpu_vertex_attribute"><a href="#gpu_vertex_attribute"><code>gpu-vertex-attribute</code></a></a>&gt;</li>
</ul>
<h4><a id="gpu_vertex_state"></a><code>record gpu-vertex-state</code></h4>
<h5>Record Fields</h5>
<ul>
<li><a id="gpu_vertex_state.buffers"></a><code>buffers</code>: option&lt;list&lt;option&lt;<a href="#gpu_vertex_buffer_layout"><a href="#gpu_vertex_buffer_layout"><code>gpu-vertex-buffer-layout</code></a></a>&gt;&gt;&gt;</li>
<li><a id="gpu_vertex_state.module"></a><code>module</code>: borrow&lt;<a href="#gpu_shader_module"><a href="#gpu_shader_module"><code>gpu-shader-module</code></a></a>&gt;</li>
<li><a id="gpu_vertex_state.entry_point"></a><code>entry-point</code>: option&lt;<code>string</code>&gt;</li>
<li><a id="gpu_vertex_state.constants"></a><code>constants</code>: option&lt;own&lt;<a href="#record_gpu_pipeline_constant_value"><a href="#record_gpu_pipeline_constant_value"><code>record-gpu-pipeline-constant-value</code></a></a>&gt;&gt;</li>
</ul>
<h4><a id="gpu_size32"></a><code>type gpu-size32</code></h4>
<p><code>u32</code></p>
<p>
#### <a id="gpu_multisample_state"></a>`record gpu-multisample-state`
<h5>Record Fields</h5>
<ul>
<li><a id="gpu_multisample_state.count"></a><code>count</code>: option&lt;<a href="#gpu_size32"><a href="#gpu_size32"><code>gpu-size32</code></a></a>&gt;</li>
<li><a id="gpu_multisample_state.mask"></a><code>mask</code>: option&lt;<a href="#gpu_sample_mask"><a href="#gpu_sample_mask"><code>gpu-sample-mask</code></a></a>&gt;</li>
<li><a id="gpu_multisample_state.alpha_to_coverage_enabled"></a><code>alpha-to-coverage-enabled</code>: option&lt;<code>bool</code>&gt;</li>
</ul>
<h4><a id="gpu_render_pipeline_descriptor"></a><code>record gpu-render-pipeline-descriptor</code></h4>
<h5>Record Fields</h5>
<ul>
<li><a id="gpu_render_pipeline_descriptor.vertex"></a><code>vertex</code>: <a href="#gpu_vertex_state"><a href="#gpu_vertex_state"><code>gpu-vertex-state</code></a></a></li>
<li><a id="gpu_render_pipeline_descriptor.primitive"></a><code>primitive</code>: option&lt;<a href="#gpu_primitive_state"><a href="#gpu_primitive_state"><code>gpu-primitive-state</code></a></a>&gt;</li>
<li><a id="gpu_render_pipeline_descriptor.depth_stencil"></a><code>depth-stencil</code>: option&lt;<a href="#gpu_depth_stencil_state"><a href="#gpu_depth_stencil_state"><code>gpu-depth-stencil-state</code></a></a>&gt;</li>
<li><a id="gpu_render_pipeline_descriptor.multisample"></a><code>multisample</code>: option&lt;<a href="#gpu_multisample_state"><a href="#gpu_multisample_state"><code>gpu-multisample-state</code></a></a>&gt;</li>
<li><a id="gpu_render_pipeline_descriptor.fragment"></a><code>fragment</code>: option&lt;<a href="#gpu_fragment_state"><a href="#gpu_fragment_state"><code>gpu-fragment-state</code></a></a>&gt;</li>
<li><a id="gpu_render_pipeline_descriptor.layout"></a><code>layout</code>: <a href="#gpu_layout"><a href="#gpu_layout"><code>gpu-layout</code></a></a></li>
<li><a id="gpu_render_pipeline_descriptor.label"></a><code>label</code>: option&lt;<code>string</code>&gt;</li>
</ul>
<h4><a id="gpu_image_data_layout"></a><code>record gpu-image-data-layout</code></h4>
<h5>Record Fields</h5>
<ul>
<li><a id="gpu_image_data_layout.offset"></a><code>offset</code>: option&lt;<a href="#gpu_size64"><a href="#gpu_size64"><code>gpu-size64</code></a></a>&gt;</li>
<li><a id="gpu_image_data_layout.bytes_per_row"></a><code>bytes-per-row</code>: option&lt;<a href="#gpu_size32"><a href="#gpu_size32"><code>gpu-size32</code></a></a>&gt;</li>
<li><a id="gpu_image_data_layout.rows_per_image"></a><code>rows-per-image</code>: option&lt;<a href="#gpu_size32"><a href="#gpu_size32"><code>gpu-size32</code></a></a>&gt;</li>
</ul>
<h4><a id="gpu_image_copy_buffer"></a><code>record gpu-image-copy-buffer</code></h4>
<h5>Record Fields</h5>
<ul>
<li><a id="gpu_image_copy_buffer.buffer"></a><code>buffer</code>: borrow&lt;<a href="#gpu_buffer"><a href="#gpu_buffer"><code>gpu-buffer</code></a></a>&gt;</li>
<li><a id="gpu_image_copy_buffer.offset"></a><code>offset</code>: option&lt;<a href="#gpu_size64"><a href="#gpu_size64"><code>gpu-size64</code></a></a>&gt;</li>
<li><a id="gpu_image_copy_buffer.bytes_per_row"></a><code>bytes-per-row</code>: option&lt;<a href="#gpu_size32"><a href="#gpu_size32"><code>gpu-size32</code></a></a>&gt;</li>
<li><a id="gpu_image_copy_buffer.rows_per_image"></a><code>rows-per-image</code>: option&lt;<a href="#gpu_size32"><a href="#gpu_size32"><code>gpu-size32</code></a></a>&gt;</li>
</ul>
<h4><a id="gpu_compute_pass_timestamp_writes"></a><code>record gpu-compute-pass-timestamp-writes</code></h4>
<h5>Record Fields</h5>
<ul>
<li><a id="gpu_compute_pass_timestamp_writes.query_set"></a><code>query-set</code>: borrow&lt;<a href="#gpu_query_set"><a href="#gpu_query_set"><code>gpu-query-set</code></a></a>&gt;</li>
<li><a id="gpu_compute_pass_timestamp_writes.beginning_of_pass_write_index"></a><code>beginning-of-pass-write-index</code>: option&lt;<a href="#gpu_size32"><a href="#gpu_size32"><code>gpu-size32</code></a></a>&gt;</li>
<li><a id="gpu_compute_pass_timestamp_writes.end_of_pass_write_index"></a><code>end-of-pass-write-index</code>: option&lt;<a href="#gpu_size32"><a href="#gpu_size32"><code>gpu-size32</code></a></a>&gt;</li>
</ul>
<h4><a id="gpu_compute_pass_descriptor"></a><code>record gpu-compute-pass-descriptor</code></h4>
<h5>Record Fields</h5>
<ul>
<li><a id="gpu_compute_pass_descriptor.timestamp_writes"></a><code>timestamp-writes</code>: option&lt;<a href="#gpu_compute_pass_timestamp_writes"><a href="#gpu_compute_pass_timestamp_writes"><code>gpu-compute-pass-timestamp-writes</code></a></a>&gt;</li>
<li><a id="gpu_compute_pass_descriptor.label"></a><code>label</code>: option&lt;<code>string</code>&gt;</li>
</ul>
<h4><a id="gpu_render_pass_timestamp_writes"></a><code>record gpu-render-pass-timestamp-writes</code></h4>
<h5>Record Fields</h5>
<ul>
<li><a id="gpu_render_pass_timestamp_writes.query_set"></a><code>query-set</code>: borrow&lt;<a href="#gpu_query_set"><a href="#gpu_query_set"><code>gpu-query-set</code></a></a>&gt;</li>
<li><a id="gpu_render_pass_timestamp_writes.beginning_of_pass_write_index"></a><code>beginning-of-pass-write-index</code>: option&lt;<a href="#gpu_size32"><a href="#gpu_size32"><code>gpu-size32</code></a></a>&gt;</li>
<li><a id="gpu_render_pass_timestamp_writes.end_of_pass_write_index"></a><code>end-of-pass-write-index</code>: option&lt;<a href="#gpu_size32"><a href="#gpu_size32"><code>gpu-size32</code></a></a>&gt;</li>
</ul>
<h4><a id="gpu_render_pass_layout"></a><code>record gpu-render-pass-layout</code></h4>
<h5>Record Fields</h5>
<ul>
<li><a id="gpu_render_pass_layout.color_formats"></a><code>color-formats</code>: list&lt;option&lt;<a href="#gpu_texture_format"><a href="#gpu_texture_format"><code>gpu-texture-format</code></a></a>&gt;&gt;</li>
<li><a id="gpu_render_pass_layout.depth_stencil_format"></a><code>depth-stencil-format</code>: option&lt;<a href="#gpu_texture_format"><a href="#gpu_texture_format"><code>gpu-texture-format</code></a></a>&gt;</li>
<li><a id="gpu_render_pass_layout.sample_count"></a><code>sample-count</code>: option&lt;<a href="#gpu_size32"><a href="#gpu_size32"><code>gpu-size32</code></a></a>&gt;</li>
<li><a id="gpu_render_pass_layout.label"></a><code>label</code>: option&lt;<code>string</code>&gt;</li>
</ul>
<h4><a id="gpu_render_bundle_encoder_descriptor"></a><code>record gpu-render-bundle-encoder-descriptor</code></h4>
<h5>Record Fields</h5>
<ul>
<li><a id="gpu_render_bundle_encoder_descriptor.depth_read_only"></a><code>depth-read-only</code>: option&lt;<code>bool</code>&gt;</li>
<li><a id="gpu_render_bundle_encoder_descriptor.stencil_read_only"></a><code>stencil-read-only</code>: option&lt;<code>bool</code>&gt;</li>
<li><a id="gpu_render_bundle_encoder_descriptor.color_formats"></a><code>color-formats</code>: list&lt;option&lt;<a href="#gpu_texture_format"><a href="#gpu_texture_format"><code>gpu-texture-format</code></a></a>&gt;&gt;</li>
<li><a id="gpu_render_bundle_encoder_descriptor.depth_stencil_format"></a><code>depth-stencil-format</code>: option&lt;<a href="#gpu_texture_format"><a href="#gpu_texture_format"><code>gpu-texture-format</code></a></a>&gt;</li>
<li><a id="gpu_render_bundle_encoder_descriptor.sample_count"></a><code>sample-count</code>: option&lt;<a href="#gpu_size32"><a href="#gpu_size32"><code>gpu-size32</code></a></a>&gt;</li>
<li><a id="gpu_render_bundle_encoder_descriptor.label"></a><code>label</code>: option&lt;<code>string</code>&gt;</li>
</ul>
<h4><a id="gpu_query_set_descriptor"></a><code>record gpu-query-set-descriptor</code></h4>
<h5>Record Fields</h5>
<ul>
<li><a id="gpu_query_set_descriptor.type"></a><code>type</code>: <a href="#gpu_query_type"><a href="#gpu_query_type"><code>gpu-query-type</code></a></a></li>
<li><a id="gpu_query_set_descriptor.count"></a><code>count</code>: <a href="#gpu_size32"><a href="#gpu_size32"><code>gpu-size32</code></a></a></li>
<li><a id="gpu_query_set_descriptor.label"></a><code>label</code>: option&lt;<code>string</code>&gt;</li>
</ul>
<h4><a id="gpu_signed_offset32"></a><code>type gpu-signed-offset32</code></h4>
<p><code>s32</code></p>
<p>
#### <a id="gpu_size64_out"></a>`type gpu-size64-out`
`u64`
<p>
#### <a id="gpu_integer_coordinate_out"></a>`type gpu-integer-coordinate-out`
`u32`
<p>
#### <a id="gpu_size32_out"></a>`type gpu-size32-out`
`u32`
<p>
#### <a id="gpu_flags_constant"></a>`type gpu-flags-constant`
`u32`
<p>
#### <a id="gpu_color"></a>`record gpu-color`
<h5>Record Fields</h5>
<ul>
<li><a id="gpu_color.r"></a><code>r</code>: <code>f64</code></li>
<li><a id="gpu_color.g"></a><code>g</code>: <code>f64</code></li>
<li><a id="gpu_color.b"></a><code>b</code>: <code>f64</code></li>
<li><a id="gpu_color.a"></a><code>a</code>: <code>f64</code></li>
</ul>
<h4><a id="gpu_render_pass_color_attachment"></a><code>record gpu-render-pass-color-attachment</code></h4>
<h5>Record Fields</h5>
<ul>
<li><a id="gpu_render_pass_color_attachment.view"></a><code>view</code>: borrow&lt;<a href="#gpu_texture_view"><a href="#gpu_texture_view"><code>gpu-texture-view</code></a></a>&gt;</li>
<li><a id="gpu_render_pass_color_attachment.depth_slice"></a><code>depth-slice</code>: option&lt;<a href="#gpu_integer_coordinate"><a href="#gpu_integer_coordinate"><code>gpu-integer-coordinate</code></a></a>&gt;</li>
<li><a id="gpu_render_pass_color_attachment.resolve_target"></a><code>resolve-target</code>: option&lt;borrow&lt;<a href="#gpu_texture_view"><a href="#gpu_texture_view"><code>gpu-texture-view</code></a></a>&gt;&gt;</li>
<li><a id="gpu_render_pass_color_attachment.clear_value"></a><code>clear-value</code>: option&lt;<a href="#gpu_color"><a href="#gpu_color"><code>gpu-color</code></a></a>&gt;</li>
<li><a id="gpu_render_pass_color_attachment.load_op"></a><code>load-op</code>: <a href="#gpu_load_op"><a href="#gpu_load_op"><code>gpu-load-op</code></a></a></li>
<li><a id="gpu_render_pass_color_attachment.store_op"></a><code>store-op</code>: <a href="#gpu_store_op"><a href="#gpu_store_op"><code>gpu-store-op</code></a></a></li>
</ul>
<h4><a id="gpu_render_pass_descriptor"></a><code>record gpu-render-pass-descriptor</code></h4>
<h5>Record Fields</h5>
<ul>
<li><a id="gpu_render_pass_descriptor.color_attachments"></a><code>color-attachments</code>: list&lt;option&lt;<a href="#gpu_render_pass_color_attachment"><a href="#gpu_render_pass_color_attachment"><code>gpu-render-pass-color-attachment</code></a></a>&gt;&gt;</li>
<li><a id="gpu_render_pass_descriptor.depth_stencil_attachment"></a><code>depth-stencil-attachment</code>: option&lt;<a href="#gpu_render_pass_depth_stencil_attachment"><a href="#gpu_render_pass_depth_stencil_attachment"><code>gpu-render-pass-depth-stencil-attachment</code></a></a>&gt;</li>
<li><a id="gpu_render_pass_descriptor.occlusion_query_set"></a><code>occlusion-query-set</code>: option&lt;borrow&lt;<a href="#gpu_query_set"><a href="#gpu_query_set"><code>gpu-query-set</code></a></a>&gt;&gt;</li>
<li><a id="gpu_render_pass_descriptor.timestamp_writes"></a><code>timestamp-writes</code>: option&lt;<a href="#gpu_render_pass_timestamp_writes"><a href="#gpu_render_pass_timestamp_writes"><code>gpu-render-pass-timestamp-writes</code></a></a>&gt;</li>
<li><a id="gpu_render_pass_descriptor.max_draw_count"></a><code>max-draw-count</code>: option&lt;<a href="#gpu_size64"><a href="#gpu_size64"><code>gpu-size64</code></a></a>&gt;</li>
<li><a id="gpu_render_pass_descriptor.label"></a><code>label</code>: option&lt;<code>string</code>&gt;</li>
</ul>
<h4><a id="gpu_origin3_d"></a><code>record gpu-origin3-d</code></h4>
<h5>Record Fields</h5>
<ul>
<li><a id="gpu_origin3_d.x"></a><code>x</code>: option&lt;<a href="#gpu_integer_coordinate"><a href="#gpu_integer_coordinate"><code>gpu-integer-coordinate</code></a></a>&gt;</li>
<li><a id="gpu_origin3_d.y"></a><code>y</code>: option&lt;<a href="#gpu_integer_coordinate"><a href="#gpu_integer_coordinate"><code>gpu-integer-coordinate</code></a></a>&gt;</li>
<li><a id="gpu_origin3_d.z"></a><code>z</code>: option&lt;<a href="#gpu_integer_coordinate"><a href="#gpu_integer_coordinate"><code>gpu-integer-coordinate</code></a></a>&gt;</li>
</ul>
<h4><a id="gpu_image_copy_texture"></a><code>record gpu-image-copy-texture</code></h4>
<h5>Record Fields</h5>
<ul>
<li><a id="gpu_image_copy_texture.texture"></a><code>texture</code>: borrow&lt;<a href="#gpu_texture"><a href="#gpu_texture"><code>gpu-texture</code></a></a>&gt;</li>
<li><a id="gpu_image_copy_texture.mip_level"></a><code>mip-level</code>: option&lt;<a href="#gpu_integer_coordinate"><a href="#gpu_integer_coordinate"><code>gpu-integer-coordinate</code></a></a>&gt;</li>
<li><a id="gpu_image_copy_texture.origin"></a><code>origin</code>: option&lt;<a href="#gpu_origin3_d"><a href="#gpu_origin3_d"><code>gpu-origin3-d</code></a></a>&gt;</li>
<li><a id="gpu_image_copy_texture.aspect"></a><code>aspect</code>: option&lt;<a href="#gpu_texture_aspect"><a href="#gpu_texture_aspect"><code>gpu-texture-aspect</code></a></a>&gt;</li>
</ul>
<h4><a id="gpu_extent3_d"></a><code>record gpu-extent3-d</code></h4>
<h5>Record Fields</h5>
<ul>
<li><a id="gpu_extent3_d.width"></a><code>width</code>: <a href="#gpu_integer_coordinate"><a href="#gpu_integer_coordinate"><code>gpu-integer-coordinate</code></a></a></li>
<li><a id="gpu_extent3_d.height"></a><code>height</code>: option&lt;<a href="#gpu_integer_coordinate"><a href="#gpu_integer_coordinate"><code>gpu-integer-coordinate</code></a></a>&gt;</li>
<li><a id="gpu_extent3_d.depth_or_array_layers"></a><code>depth-or-array-layers</code>: option&lt;<a href="#gpu_integer_coordinate"><a href="#gpu_integer_coordinate"><code>gpu-integer-coordinate</code></a></a>&gt;</li>
</ul>
<h4><a id="gpu_texture_descriptor"></a><code>record gpu-texture-descriptor</code></h4>
<h5>Record Fields</h5>
<ul>
<li><a id="gpu_texture_descriptor.size"></a><code>size</code>: <a href="#gpu_extent3_d"><a href="#gpu_extent3_d"><code>gpu-extent3-d</code></a></a></li>
<li><a id="gpu_texture_descriptor.mip_level_count"></a><code>mip-level-count</code>: option&lt;<a href="#gpu_integer_coordinate"><a href="#gpu_integer_coordinate"><code>gpu-integer-coordinate</code></a></a>&gt;</li>
<li><a id="gpu_texture_descriptor.sample_count"></a><code>sample-count</code>: option&lt;<a href="#gpu_size32"><a href="#gpu_size32"><code>gpu-size32</code></a></a>&gt;</li>
<li><a id="gpu_texture_descriptor.dimension"></a><code>dimension</code>: option&lt;<a href="#gpu_texture_dimension"><a href="#gpu_texture_dimension"><code>gpu-texture-dimension</code></a></a>&gt;</li>
<li><a id="gpu_texture_descriptor.format"></a><code>format</code>: <a href="#gpu_texture_format"><a href="#gpu_texture_format"><code>gpu-texture-format</code></a></a></li>
<li><a id="gpu_texture_descriptor.usage"></a><code>usage</code>: <a href="#gpu_texture_usage_flags"><a href="#gpu_texture_usage_flags"><code>gpu-texture-usage-flags</code></a></a></li>
<li><a id="gpu_texture_descriptor.view_formats"></a><code>view-formats</code>: option&lt;list&lt;<a href="#gpu_texture_format"><a href="#gpu_texture_format"><code>gpu-texture-format</code></a></a>&gt;&gt;</li>
<li><a id="gpu_texture_descriptor.label"></a><code>label</code>: option&lt;<code>string</code>&gt;</li>
</ul>
<h4><a id="predefined_color_space"></a><code>enum predefined-color-space</code></h4>
<h5>Enum Cases</h5>
<ul>
<li><a id="predefined_color_space.srgb"></a><code>srgb</code></li>
<li><a id="predefined_color_space.display_p3"></a><code>display-p3</code></li>
</ul>
<h4><a id="gpu_device_configuration"></a><code>record gpu-device-configuration</code></h4>
<h5>Record Fields</h5>
<ul>
<li><a id="gpu_device_configuration.format"></a><code>format</code>: <a href="#gpu_texture_format"><a href="#gpu_texture_format"><code>gpu-texture-format</code></a></a></li>
<li><a id="gpu_device_configuration.usage"></a><code>usage</code>: option&lt;<a href="#gpu_texture_usage_flags"><a href="#gpu_texture_usage_flags"><code>gpu-texture-usage-flags</code></a></a>&gt;</li>
<li><a id="gpu_device_configuration.view_formats"></a><code>view-formats</code>: option&lt;list&lt;<a href="#gpu_texture_format"><a href="#gpu_texture_format"><code>gpu-texture-format</code></a></a>&gt;&gt;</li>
<li><a id="gpu_device_configuration.color_space"></a><code>color-space</code>: option&lt;<a href="#predefined_color_space"><a href="#predefined_color_space"><code>predefined-color-space</code></a></a>&gt;</li>
<li><a id="gpu_device_configuration.alpha_mode"></a><code>alpha-mode</code>: option&lt;<a href="#gpu_canvas_alpha_mode"><a href="#gpu_canvas_alpha_mode"><code>gpu-canvas-alpha-mode</code></a></a>&gt;</li>
</ul>
<h4><a id="gpu_canvas_configuration"></a><code>record gpu-canvas-configuration</code></h4>
<h5>Record Fields</h5>
<ul>
<li><a id="gpu_canvas_configuration.device"></a><code>device</code>: borrow&lt;<a href="#gpu_device"><a href="#gpu_device"><code>gpu-device</code></a></a>&gt;</li>
<li><a id="gpu_canvas_configuration.format"></a><code>format</code>: <a href="#gpu_texture_format"><a href="#gpu_texture_format"><code>gpu-texture-format</code></a></a></li>
<li><a id="gpu_canvas_configuration.usage"></a><code>usage</code>: option&lt;<a href="#gpu_texture_usage_flags"><a href="#gpu_texture_usage_flags"><code>gpu-texture-usage-flags</code></a></a>&gt;</li>
<li><a id="gpu_canvas_configuration.view_formats"></a><code>view-formats</code>: option&lt;list&lt;<a href="#gpu_texture_format"><a href="#gpu_texture_format"><code>gpu-texture-format</code></a></a>&gt;&gt;</li>
<li><a id="gpu_canvas_configuration.color_space"></a><code>color-space</code>: option&lt;<a href="#predefined_color_space"><a href="#predefined_color_space"><code>predefined-color-space</code></a></a>&gt;</li>
<li><a id="gpu_canvas_configuration.tone_mapping"></a><code>tone-mapping</code>: option&lt;<a href="#gpu_canvas_tone_mapping"><a href="#gpu_canvas_tone_mapping"><code>gpu-canvas-tone-mapping</code></a></a>&gt;</li>
<li><a id="gpu_canvas_configuration.alpha_mode"></a><code>alpha-mode</code>: option&lt;<a href="#gpu_canvas_alpha_mode"><a href="#gpu_canvas_alpha_mode"><code>gpu-canvas-alpha-mode</code></a></a>&gt;</li>
</ul>
<h4><a id="gpu_image_copy_texture_tagged"></a><code>record gpu-image-copy-texture-tagged</code></h4>
<h5>Record Fields</h5>
<ul>
<li><a id="gpu_image_copy_texture_tagged.color_space"></a><code>color-space</code>: option&lt;<a href="#predefined_color_space"><a href="#predefined_color_space"><code>predefined-color-space</code></a></a>&gt;</li>
<li><a id="gpu_image_copy_texture_tagged.premultiplied_alpha"></a><code>premultiplied-alpha</code>: option&lt;<code>bool</code>&gt;</li>
<li><a id="gpu_image_copy_texture_tagged.texture"></a><code>texture</code>: borrow&lt;<a href="#gpu_texture"><a href="#gpu_texture"><code>gpu-texture</code></a></a>&gt;</li>
<li><a id="gpu_image_copy_texture_tagged.mip_level"></a><code>mip-level</code>: option&lt;<a href="#gpu_integer_coordinate"><a href="#gpu_integer_coordinate"><code>gpu-integer-coordinate</code></a></a>&gt;</li>
<li><a id="gpu_image_copy_texture_tagged.origin"></a><code>origin</code>: option&lt;<a href="#gpu_origin3_d"><a href="#gpu_origin3_d"><code>gpu-origin3-d</code></a></a>&gt;</li>
<li><a id="gpu_image_copy_texture_tagged.aspect"></a><code>aspect</code>: option&lt;<a href="#gpu_texture_aspect"><a href="#gpu_texture_aspect"><code>gpu-texture-aspect</code></a></a>&gt;</li>
</ul>
<h4><a id="non_standard_buffer"></a><code>resource non-standard-buffer</code></h4>
<h2>TODO: This should be replcated with something that doesn't require a copy.</h2>
<h3>Functions</h3>
<h4><a id="get_gpu"></a><code>get-gpu: func</code></h4>
<h5>Return values</h5>
<ul>
<li><a id="get_gpu.0"></a> own&lt;<a href="#gpu"><a href="#gpu"><code>gpu</code></a></a>&gt;</li>
</ul>
<h4><a id="method_gpu_supported_limits_max_texture_dimension1_d"></a><code>[method]gpu-supported-limits.max-texture-dimension1-d: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_supported_limits_max_texture_dimension1_d.self"></a><code>self</code>: borrow&lt;<a href="#gpu_supported_limits"><a href="#gpu_supported_limits"><code>gpu-supported-limits</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_gpu_supported_limits_max_texture_dimension1_d.0"></a> <code>u32</code></li>
</ul>
<h4><a id="method_gpu_supported_limits_max_texture_dimension2_d"></a><code>[method]gpu-supported-limits.max-texture-dimension2-d: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_supported_limits_max_texture_dimension2_d.self"></a><code>self</code>: borrow&lt;<a href="#gpu_supported_limits"><a href="#gpu_supported_limits"><code>gpu-supported-limits</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_gpu_supported_limits_max_texture_dimension2_d.0"></a> <code>u32</code></li>
</ul>
<h4><a id="method_gpu_supported_limits_max_texture_dimension3_d"></a><code>[method]gpu-supported-limits.max-texture-dimension3-d: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_supported_limits_max_texture_dimension3_d.self"></a><code>self</code>: borrow&lt;<a href="#gpu_supported_limits"><a href="#gpu_supported_limits"><code>gpu-supported-limits</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_gpu_supported_limits_max_texture_dimension3_d.0"></a> <code>u32</code></li>
</ul>
<h4><a id="method_gpu_supported_limits_max_texture_array_layers"></a><code>[method]gpu-supported-limits.max-texture-array-layers: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_supported_limits_max_texture_array_layers.self"></a><code>self</code>: borrow&lt;<a href="#gpu_supported_limits"><a href="#gpu_supported_limits"><code>gpu-supported-limits</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_gpu_supported_limits_max_texture_array_layers.0"></a> <code>u32</code></li>
</ul>
<h4><a id="method_gpu_supported_limits_max_bind_groups"></a><code>[method]gpu-supported-limits.max-bind-groups: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_supported_limits_max_bind_groups.self"></a><code>self</code>: borrow&lt;<a href="#gpu_supported_limits"><a href="#gpu_supported_limits"><code>gpu-supported-limits</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_gpu_supported_limits_max_bind_groups.0"></a> <code>u32</code></li>
</ul>
<h4><a id="method_gpu_supported_limits_max_bind_groups_plus_vertex_buffers"></a><code>[method]gpu-supported-limits.max-bind-groups-plus-vertex-buffers: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_supported_limits_max_bind_groups_plus_vertex_buffers.self"></a><code>self</code>: borrow&lt;<a href="#gpu_supported_limits"><a href="#gpu_supported_limits"><code>gpu-supported-limits</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_gpu_supported_limits_max_bind_groups_plus_vertex_buffers.0"></a> <code>u32</code></li>
</ul>
<h4><a id="method_gpu_supported_limits_max_bindings_per_bind_group"></a><code>[method]gpu-supported-limits.max-bindings-per-bind-group: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_supported_limits_max_bindings_per_bind_group.self"></a><code>self</code>: borrow&lt;<a href="#gpu_supported_limits"><a href="#gpu_supported_limits"><code>gpu-supported-limits</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_gpu_supported_limits_max_bindings_per_bind_group.0"></a> <code>u32</code></li>
</ul>
<h4><a id="method_gpu_supported_limits_max_dynamic_uniform_buffers_per_pipeline_layout"></a><code>[method]gpu-supported-limits.max-dynamic-uniform-buffers-per-pipeline-layout: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_supported_limits_max_dynamic_uniform_buffers_per_pipeline_layout.self"></a><code>self</code>: borrow&lt;<a href="#gpu_supported_limits"><a href="#gpu_supported_limits"><code>gpu-supported-limits</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_gpu_supported_limits_max_dynamic_uniform_buffers_per_pipeline_layout.0"></a> <code>u32</code></li>
</ul>
<h4><a id="method_gpu_supported_limits_max_dynamic_storage_buffers_per_pipeline_layout"></a><code>[method]gpu-supported-limits.max-dynamic-storage-buffers-per-pipeline-layout: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_supported_limits_max_dynamic_storage_buffers_per_pipeline_layout.self"></a><code>self</code>: borrow&lt;<a href="#gpu_supported_limits"><a href="#gpu_supported_limits"><code>gpu-supported-limits</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_gpu_supported_limits_max_dynamic_storage_buffers_per_pipeline_layout.0"></a> <code>u32</code></li>
</ul>
<h4><a id="method_gpu_supported_limits_max_sampled_textures_per_shader_stage"></a><code>[method]gpu-supported-limits.max-sampled-textures-per-shader-stage: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_supported_limits_max_sampled_textures_per_shader_stage.self"></a><code>self</code>: borrow&lt;<a href="#gpu_supported_limits"><a href="#gpu_supported_limits"><code>gpu-supported-limits</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_gpu_supported_limits_max_sampled_textures_per_shader_stage.0"></a> <code>u32</code></li>
</ul>
<h4><a id="method_gpu_supported_limits_max_samplers_per_shader_stage"></a><code>[method]gpu-supported-limits.max-samplers-per-shader-stage: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_supported_limits_max_samplers_per_shader_stage.self"></a><code>self</code>: borrow&lt;<a href="#gpu_supported_limits"><a href="#gpu_supported_limits"><code>gpu-supported-limits</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_gpu_supported_limits_max_samplers_per_shader_stage.0"></a> <code>u32</code></li>
</ul>
<h4><a id="method_gpu_supported_limits_max_storage_buffers_per_shader_stage"></a><code>[method]gpu-supported-limits.max-storage-buffers-per-shader-stage: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_supported_limits_max_storage_buffers_per_shader_stage.self"></a><code>self</code>: borrow&lt;<a href="#gpu_supported_limits"><a href="#gpu_supported_limits"><code>gpu-supported-limits</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_gpu_supported_limits_max_storage_buffers_per_shader_stage.0"></a> <code>u32</code></li>
</ul>
<h4><a id="method_gpu_supported_limits_max_storage_textures_per_shader_stage"></a><code>[method]gpu-supported-limits.max-storage-textures-per-shader-stage: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_supported_limits_max_storage_textures_per_shader_stage.self"></a><code>self</code>: borrow&lt;<a href="#gpu_supported_limits"><a href="#gpu_supported_limits"><code>gpu-supported-limits</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_gpu_supported_limits_max_storage_textures_per_shader_stage.0"></a> <code>u32</code></li>
</ul>
<h4><a id="method_gpu_supported_limits_max_uniform_buffers_per_shader_stage"></a><code>[method]gpu-supported-limits.max-uniform-buffers-per-shader-stage: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_supported_limits_max_uniform_buffers_per_shader_stage.self"></a><code>self</code>: borrow&lt;<a href="#gpu_supported_limits"><a href="#gpu_supported_limits"><code>gpu-supported-limits</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_gpu_supported_limits_max_uniform_buffers_per_shader_stage.0"></a> <code>u32</code></li>
</ul>
<h4><a id="method_gpu_supported_limits_max_uniform_buffer_binding_size"></a><code>[method]gpu-supported-limits.max-uniform-buffer-binding-size: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_supported_limits_max_uniform_buffer_binding_size.self"></a><code>self</code>: borrow&lt;<a href="#gpu_supported_limits"><a href="#gpu_supported_limits"><code>gpu-supported-limits</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_gpu_supported_limits_max_uniform_buffer_binding_size.0"></a> <code>u64</code></li>
</ul>
<h4><a id="method_gpu_supported_limits_max_storage_buffer_binding_size"></a><code>[method]gpu-supported-limits.max-storage-buffer-binding-size: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_supported_limits_max_storage_buffer_binding_size.self"></a><code>self</code>: borrow&lt;<a href="#gpu_supported_limits"><a href="#gpu_supported_limits"><code>gpu-supported-limits</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_gpu_supported_limits_max_storage_buffer_binding_size.0"></a> <code>u64</code></li>
</ul>
<h4><a id="method_gpu_supported_limits_min_uniform_buffer_offset_alignment"></a><code>[method]gpu-supported-limits.min-uniform-buffer-offset-alignment: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_supported_limits_min_uniform_buffer_offset_alignment.self"></a><code>self</code>: borrow&lt;<a href="#gpu_supported_limits"><a href="#gpu_supported_limits"><code>gpu-supported-limits</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_gpu_supported_limits_min_uniform_buffer_offset_alignment.0"></a> <code>u32</code></li>
</ul>
<h4><a id="method_gpu_supported_limits_min_storage_buffer_offset_alignment"></a><code>[method]gpu-supported-limits.min-storage-buffer-offset-alignment: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_supported_limits_min_storage_buffer_offset_alignment.self"></a><code>self</code>: borrow&lt;<a href="#gpu_supported_limits"><a href="#gpu_supported_limits"><code>gpu-supported-limits</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_gpu_supported_limits_min_storage_buffer_offset_alignment.0"></a> <code>u32</code></li>
</ul>
<h4><a id="method_gpu_supported_limits_max_vertex_buffers"></a><code>[method]gpu-supported-limits.max-vertex-buffers: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_supported_limits_max_vertex_buffers.self"></a><code>self</code>: borrow&lt;<a href="#gpu_supported_limits"><a href="#gpu_supported_limits"><code>gpu-supported-limits</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_gpu_supported_limits_max_vertex_buffers.0"></a> <code>u32</code></li>
</ul>
<h4><a id="method_gpu_supported_limits_max_buffer_size"></a><code>[method]gpu-supported-limits.max-buffer-size: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_supported_limits_max_buffer_size.self"></a><code>self</code>: borrow&lt;<a href="#gpu_supported_limits"><a href="#gpu_supported_limits"><code>gpu-supported-limits</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_gpu_supported_limits_max_buffer_size.0"></a> <code>u64</code></li>
</ul>
<h4><a id="method_gpu_supported_limits_max_vertex_attributes"></a><code>[method]gpu-supported-limits.max-vertex-attributes: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_supported_limits_max_vertex_attributes.self"></a><code>self</code>: borrow&lt;<a href="#gpu_supported_limits"><a href="#gpu_supported_limits"><code>gpu-supported-limits</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_gpu_supported_limits_max_vertex_attributes.0"></a> <code>u32</code></li>
</ul>
<h4><a id="method_gpu_supported_limits_max_vertex_buffer_array_stride"></a><code>[method]gpu-supported-limits.max-vertex-buffer-array-stride: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_supported_limits_max_vertex_buffer_array_stride.self"></a><code>self</code>: borrow&lt;<a href="#gpu_supported_limits"><a href="#gpu_supported_limits"><code>gpu-supported-limits</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_gpu_supported_limits_max_vertex_buffer_array_stride.0"></a> <code>u32</code></li>
</ul>
<h4><a id="method_gpu_supported_limits_max_inter_stage_shader_variables"></a><code>[method]gpu-supported-limits.max-inter-stage-shader-variables: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_supported_limits_max_inter_stage_shader_variables.self"></a><code>self</code>: borrow&lt;<a href="#gpu_supported_limits"><a href="#gpu_supported_limits"><code>gpu-supported-limits</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_gpu_supported_limits_max_inter_stage_shader_variables.0"></a> <code>u32</code></li>
</ul>
<h4><a id="method_gpu_supported_limits_max_color_attachments"></a><code>[method]gpu-supported-limits.max-color-attachments: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_supported_limits_max_color_attachments.self"></a><code>self</code>: borrow&lt;<a href="#gpu_supported_limits"><a href="#gpu_supported_limits"><code>gpu-supported-limits</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_gpu_supported_limits_max_color_attachments.0"></a> <code>u32</code></li>
</ul>
<h4><a id="method_gpu_supported_limits_max_color_attachment_bytes_per_sample"></a><code>[method]gpu-supported-limits.max-color-attachment-bytes-per-sample: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_supported_limits_max_color_attachment_bytes_per_sample.self"></a><code>self</code>: borrow&lt;<a href="#gpu_supported_limits"><a href="#gpu_supported_limits"><code>gpu-supported-limits</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_gpu_supported_limits_max_color_attachment_bytes_per_sample.0"></a> <code>u32</code></li>
</ul>
<h4><a id="method_gpu_supported_limits_max_compute_workgroup_storage_size"></a><code>[method]gpu-supported-limits.max-compute-workgroup-storage-size: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_supported_limits_max_compute_workgroup_storage_size.self"></a><code>self</code>: borrow&lt;<a href="#gpu_supported_limits"><a href="#gpu_supported_limits"><code>gpu-supported-limits</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_gpu_supported_limits_max_compute_workgroup_storage_size.0"></a> <code>u32</code></li>
</ul>
<h4><a id="method_gpu_supported_limits_max_compute_invocations_per_workgroup"></a><code>[method]gpu-supported-limits.max-compute-invocations-per-workgroup: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_supported_limits_max_compute_invocations_per_workgroup.self"></a><code>self</code>: borrow&lt;<a href="#gpu_supported_limits"><a href="#gpu_supported_limits"><code>gpu-supported-limits</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_gpu_supported_limits_max_compute_invocations_per_workgroup.0"></a> <code>u32</code></li>
</ul>
<h4><a id="method_gpu_supported_limits_max_compute_workgroup_size_x"></a><code>[method]gpu-supported-limits.max-compute-workgroup-size-x: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_supported_limits_max_compute_workgroup_size_x.self"></a><code>self</code>: borrow&lt;<a href="#gpu_supported_limits"><a href="#gpu_supported_limits"><code>gpu-supported-limits</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_gpu_supported_limits_max_compute_workgroup_size_x.0"></a> <code>u32</code></li>
</ul>
<h4><a id="method_gpu_supported_limits_max_compute_workgroup_size_y"></a><code>[method]gpu-supported-limits.max-compute-workgroup-size-y: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_supported_limits_max_compute_workgroup_size_y.self"></a><code>self</code>: borrow&lt;<a href="#gpu_supported_limits"><a href="#gpu_supported_limits"><code>gpu-supported-limits</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_gpu_supported_limits_max_compute_workgroup_size_y.0"></a> <code>u32</code></li>
</ul>
<h4><a id="method_gpu_supported_limits_max_compute_workgroup_size_z"></a><code>[method]gpu-supported-limits.max-compute-workgroup-size-z: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_supported_limits_max_compute_workgroup_size_z.self"></a><code>self</code>: borrow&lt;<a href="#gpu_supported_limits"><a href="#gpu_supported_limits"><code>gpu-supported-limits</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_gpu_supported_limits_max_compute_workgroup_size_z.0"></a> <code>u32</code></li>
</ul>
<h4><a id="method_gpu_supported_limits_max_compute_workgroups_per_dimension"></a><code>[method]gpu-supported-limits.max-compute-workgroups-per-dimension: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_supported_limits_max_compute_workgroups_per_dimension.self"></a><code>self</code>: borrow&lt;<a href="#gpu_supported_limits"><a href="#gpu_supported_limits"><code>gpu-supported-limits</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_gpu_supported_limits_max_compute_workgroups_per_dimension.0"></a> <code>u32</code></li>
</ul>
<h4><a id="method_gpu_supported_features_has"></a><code>[method]gpu-supported-features.has: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_supported_features_has.self"></a><code>self</code>: borrow&lt;<a href="#gpu_supported_features"><a href="#gpu_supported_features"><code>gpu-supported-features</code></a></a>&gt;</li>
<li><a id="method_gpu_supported_features_has.value"></a><code>value</code>: <code>string</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_gpu_supported_features_has.0"></a> <code>bool</code></li>
</ul>
<h4><a id="method_wgsl_language_features_has"></a><code>[method]wgsl-language-features.has: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_wgsl_language_features_has.self"></a><code>self</code>: borrow&lt;<a href="#wgsl_language_features"><a href="#wgsl_language_features"><code>wgsl-language-features</code></a></a>&gt;</li>
<li><a id="method_wgsl_language_features_has.value"></a><code>value</code>: <code>string</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_wgsl_language_features_has.0"></a> <code>bool</code></li>
</ul>
<h4><a id="method_gpu_adapter_info_vendor"></a><code>[method]gpu-adapter-info.vendor: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_adapter_info_vendor.self"></a><code>self</code>: borrow&lt;<a href="#gpu_adapter_info"><a href="#gpu_adapter_info"><code>gpu-adapter-info</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_gpu_adapter_info_vendor.0"></a> <code>string</code></li>
</ul>
<h4><a id="method_gpu_adapter_info_architecture"></a><code>[method]gpu-adapter-info.architecture: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_adapter_info_architecture.self"></a><code>self</code>: borrow&lt;<a href="#gpu_adapter_info"><a href="#gpu_adapter_info"><code>gpu-adapter-info</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_gpu_adapter_info_architecture.0"></a> <code>string</code></li>
</ul>
<h4><a id="method_gpu_adapter_info_device"></a><code>[method]gpu-adapter-info.device: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_adapter_info_device.self"></a><code>self</code>: borrow&lt;<a href="#gpu_adapter_info"><a href="#gpu_adapter_info"><code>gpu-adapter-info</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_gpu_adapter_info_device.0"></a> <code>string</code></li>
</ul>
<h4><a id="method_gpu_adapter_info_description"></a><code>[method]gpu-adapter-info.description: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_adapter_info_description.self"></a><code>self</code>: borrow&lt;<a href="#gpu_adapter_info"><a href="#gpu_adapter_info"><code>gpu-adapter-info</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_gpu_adapter_info_description.0"></a> <code>string</code></li>
</ul>
<h4><a id="method_gpu_request_adapter"></a><code>[method]gpu.request-adapter: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_request_adapter.self"></a><code>self</code>: borrow&lt;<a href="#gpu"><a href="#gpu"><code>gpu</code></a></a>&gt;</li>
<li><a id="method_gpu_request_adapter.options"></a><code>options</code>: option&lt;<a href="#gpu_request_adapter_options"><a href="#gpu_request_adapter_options"><code>gpu-request-adapter-options</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_gpu_request_adapter.0"></a> option&lt;own&lt;<a href="#gpu_adapter"><a href="#gpu_adapter"><code>gpu-adapter</code></a></a>&gt;&gt;</li>
</ul>
<h4><a id="method_gpu_get_preferred_canvas_format"></a><code>[method]gpu.get-preferred-canvas-format: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_get_preferred_canvas_format.self"></a><code>self</code>: borrow&lt;<a href="#gpu"><a href="#gpu"><code>gpu</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_gpu_get_preferred_canvas_format.0"></a> <a href="#gpu_texture_format"><a href="#gpu_texture_format"><code>gpu-texture-format</code></a></a></li>
</ul>
<h4><a id="method_gpu_wgsl_language_features"></a><code>[method]gpu.wgsl-language-features: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_wgsl_language_features.self"></a><code>self</code>: borrow&lt;<a href="#gpu"><a href="#gpu"><code>gpu</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_gpu_wgsl_language_features.0"></a> own&lt;<a href="#wgsl_language_features"><a href="#wgsl_language_features"><code>wgsl-language-features</code></a></a>&gt;</li>
</ul>
<h4><a id="method_gpu_adapter_features"></a><code>[method]gpu-adapter.features: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_adapter_features.self"></a><code>self</code>: borrow&lt;<a href="#gpu_adapter"><a href="#gpu_adapter"><code>gpu-adapter</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_gpu_adapter_features.0"></a> own&lt;<a href="#gpu_supported_features"><a href="#gpu_supported_features"><code>gpu-supported-features</code></a></a>&gt;</li>
</ul>
<h4><a id="method_gpu_adapter_limits"></a><code>[method]gpu-adapter.limits: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_adapter_limits.self"></a><code>self</code>: borrow&lt;<a href="#gpu_adapter"><a href="#gpu_adapter"><code>gpu-adapter</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_gpu_adapter_limits.0"></a> own&lt;<a href="#gpu_supported_limits"><a href="#gpu_supported_limits"><code>gpu-supported-limits</code></a></a>&gt;</li>
</ul>
<h4><a id="method_gpu_adapter_info"></a><code>[method]gpu-adapter.info: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_adapter_info.self"></a><code>self</code>: borrow&lt;<a href="#gpu_adapter"><a href="#gpu_adapter"><code>gpu-adapter</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_gpu_adapter_info.0"></a> own&lt;<a href="#gpu_adapter_info"><a href="#gpu_adapter_info"><code>gpu-adapter-info</code></a></a>&gt;</li>
</ul>
<h4><a id="method_gpu_adapter_is_fallback_adapter"></a><code>[method]gpu-adapter.is-fallback-adapter: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_adapter_is_fallback_adapter.self"></a><code>self</code>: borrow&lt;<a href="#gpu_adapter"><a href="#gpu_adapter"><code>gpu-adapter</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_gpu_adapter_is_fallback_adapter.0"></a> <code>bool</code></li>
</ul>
<h4><a id="method_gpu_adapter_request_device"></a><code>[method]gpu-adapter.request-device: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_adapter_request_device.self"></a><code>self</code>: borrow&lt;<a href="#gpu_adapter"><a href="#gpu_adapter"><code>gpu-adapter</code></a></a>&gt;</li>
<li><a id="method_gpu_adapter_request_device.descriptor"></a><code>descriptor</code>: option&lt;<a href="#gpu_device_descriptor"><a href="#gpu_device_descriptor"><code>gpu-device-descriptor</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_gpu_adapter_request_device.0"></a> own&lt;<a href="#gpu_device"><a href="#gpu_device"><code>gpu-device</code></a></a>&gt;</li>
</ul>
<h4><a id="constructor_record_gpu_size64"></a><code>[constructor]record-gpu-size64: func</code></h4>
<h5>Return values</h5>
<ul>
<li><a id="constructor_record_gpu_size64.0"></a> own&lt;<a href="#record_gpu_size64"><a href="#record_gpu_size64"><code>record-gpu-size64</code></a></a>&gt;</li>
</ul>
<h4><a id="method_record_gpu_size64_add"></a><code>[method]record-gpu-size64.add: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_record_gpu_size64_add.self"></a><code>self</code>: borrow&lt;<a href="#record_gpu_size64"><a href="#record_gpu_size64"><code>record-gpu-size64</code></a></a>&gt;</li>
<li><a id="method_record_gpu_size64_add.key"></a><code>key</code>: <code>string</code></li>
<li><a id="method_record_gpu_size64_add.value"></a><code>value</code>: <a href="#gpu_size64"><a href="#gpu_size64"><code>gpu-size64</code></a></a></li>
</ul>
<h4><a id="method_record_gpu_size64_get"></a><code>[method]record-gpu-size64.get: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_record_gpu_size64_get.self"></a><code>self</code>: borrow&lt;<a href="#record_gpu_size64"><a href="#record_gpu_size64"><code>record-gpu-size64</code></a></a>&gt;</li>
<li><a id="method_record_gpu_size64_get.key"></a><code>key</code>: <code>string</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_record_gpu_size64_get.0"></a> <a href="#gpu_size64"><a href="#gpu_size64"><code>gpu-size64</code></a></a></li>
</ul>
<h4><a id="method_record_gpu_size64_has"></a><code>[method]record-gpu-size64.has: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_record_gpu_size64_has.self"></a><code>self</code>: borrow&lt;<a href="#record_gpu_size64"><a href="#record_gpu_size64"><code>record-gpu-size64</code></a></a>&gt;</li>
<li><a id="method_record_gpu_size64_has.key"></a><code>key</code>: <code>string</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_record_gpu_size64_has.0"></a> <code>bool</code></li>
</ul>
<h4><a id="method_record_gpu_size64_remove"></a><code>[method]record-gpu-size64.remove: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_record_gpu_size64_remove.self"></a><code>self</code>: borrow&lt;<a href="#record_gpu_size64"><a href="#record_gpu_size64"><code>record-gpu-size64</code></a></a>&gt;</li>
<li><a id="method_record_gpu_size64_remove.key"></a><code>key</code>: <code>string</code></li>
</ul>
<h4><a id="method_record_gpu_size64_keys"></a><code>[method]record-gpu-size64.keys: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_record_gpu_size64_keys.self"></a><code>self</code>: borrow&lt;<a href="#record_gpu_size64"><a href="#record_gpu_size64"><code>record-gpu-size64</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_record_gpu_size64_keys.0"></a> list&lt;<code>string</code>&gt;</li>
</ul>
<h4><a id="method_record_gpu_size64_values"></a><code>[method]record-gpu-size64.values: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_record_gpu_size64_values.self"></a><code>self</code>: borrow&lt;<a href="#record_gpu_size64"><a href="#record_gpu_size64"><code>record-gpu-size64</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_record_gpu_size64_values.0"></a> list&lt;<a href="#gpu_size64"><a href="#gpu_size64"><code>gpu-size64</code></a></a>&gt;</li>
</ul>
<h4><a id="method_record_gpu_size64_entries"></a><code>[method]record-gpu-size64.entries: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_record_gpu_size64_entries.self"></a><code>self</code>: borrow&lt;<a href="#record_gpu_size64"><a href="#record_gpu_size64"><code>record-gpu-size64</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_record_gpu_size64_entries.0"></a> (<code>string</code>, <a href="#gpu_size64"><a href="#gpu_size64"><code>gpu-size64</code></a></a>)</li>
</ul>
<h4><a id="method_gpu_device_connect_graphics_context"></a><code>[method]gpu-device.connect-graphics-context: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_device_connect_graphics_context.self"></a><code>self</code>: borrow&lt;<a href="#gpu_device"><a href="#gpu_device"><code>gpu-device</code></a></a>&gt;</li>
<li><a id="method_gpu_device_connect_graphics_context.context"></a><a href="#context"><code>context</code></a>: borrow&lt;<a href="#context"><a href="#context"><code>context</code></a></a>&gt;</li>
</ul>
<h4><a id="method_gpu_device_features"></a><code>[method]gpu-device.features: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_device_features.self"></a><code>self</code>: borrow&lt;<a href="#gpu_device"><a href="#gpu_device"><code>gpu-device</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_gpu_device_features.0"></a> own&lt;<a href="#gpu_supported_features"><a href="#gpu_supported_features"><code>gpu-supported-features</code></a></a>&gt;</li>
</ul>
<h4><a id="method_gpu_device_limits"></a><code>[method]gpu-device.limits: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_device_limits.self"></a><code>self</code>: borrow&lt;<a href="#gpu_device"><a href="#gpu_device"><code>gpu-device</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_gpu_device_limits.0"></a> own&lt;<a href="#gpu_supported_limits"><a href="#gpu_supported_limits"><code>gpu-supported-limits</code></a></a>&gt;</li>
</ul>
<h4><a id="method_gpu_device_queue"></a><code>[method]gpu-device.queue: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_device_queue.self"></a><code>self</code>: borrow&lt;<a href="#gpu_device"><a href="#gpu_device"><code>gpu-device</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_gpu_device_queue.0"></a> own&lt;<a href="#gpu_queue"><a href="#gpu_queue"><code>gpu-queue</code></a></a>&gt;</li>
</ul>
<h4><a id="method_gpu_device_destroy"></a><code>[method]gpu-device.destroy: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_device_destroy.self"></a><code>self</code>: borrow&lt;<a href="#gpu_device"><a href="#gpu_device"><code>gpu-device</code></a></a>&gt;</li>
</ul>
<h4><a id="method_gpu_device_create_buffer"></a><code>[method]gpu-device.create-buffer: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_device_create_buffer.self"></a><code>self</code>: borrow&lt;<a href="#gpu_device"><a href="#gpu_device"><code>gpu-device</code></a></a>&gt;</li>
<li><a id="method_gpu_device_create_buffer.descriptor"></a><code>descriptor</code>: <a href="#gpu_buffer_descriptor"><a href="#gpu_buffer_descriptor"><code>gpu-buffer-descriptor</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_gpu_device_create_buffer.0"></a> own&lt;<a href="#gpu_buffer"><a href="#gpu_buffer"><code>gpu-buffer</code></a></a>&gt;</li>
</ul>
<h4><a id="method_gpu_device_create_texture"></a><code>[method]gpu-device.create-texture: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_device_create_texture.self"></a><code>self</code>: borrow&lt;<a href="#gpu_device"><a href="#gpu_device"><code>gpu-device</code></a></a>&gt;</li>
<li><a id="method_gpu_device_create_texture.descriptor"></a><code>descriptor</code>: <a href="#gpu_texture_descriptor"><a href="#gpu_texture_descriptor"><code>gpu-texture-descriptor</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_gpu_device_create_texture.0"></a> own&lt;<a href="#gpu_texture"><a href="#gpu_texture"><code>gpu-texture</code></a></a>&gt;</li>
</ul>
<h4><a id="method_gpu_device_create_sampler"></a><code>[method]gpu-device.create-sampler: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_device_create_sampler.self"></a><code>self</code>: borrow&lt;<a href="#gpu_device"><a href="#gpu_device"><code>gpu-device</code></a></a>&gt;</li>
<li><a id="method_gpu_device_create_sampler.descriptor"></a><code>descriptor</code>: option&lt;<a href="#gpu_sampler_descriptor"><a href="#gpu_sampler_descriptor"><code>gpu-sampler-descriptor</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_gpu_device_create_sampler.0"></a> own&lt;<a href="#gpu_sampler"><a href="#gpu_sampler"><code>gpu-sampler</code></a></a>&gt;</li>
</ul>
<h4><a id="method_gpu_device_create_bind_group_layout"></a><code>[method]gpu-device.create-bind-group-layout: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_device_create_bind_group_layout.self"></a><code>self</code>: borrow&lt;<a href="#gpu_device"><a href="#gpu_device"><code>gpu-device</code></a></a>&gt;</li>
<li><a id="method_gpu_device_create_bind_group_layout.descriptor"></a><code>descriptor</code>: <a href="#gpu_bind_group_layout_descriptor"><a href="#gpu_bind_group_layout_descriptor"><code>gpu-bind-group-layout-descriptor</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_gpu_device_create_bind_group_layout.0"></a> own&lt;<a href="#gpu_bind_group_layout"><a href="#gpu_bind_group_layout"><code>gpu-bind-group-layout</code></a></a>&gt;</li>
</ul>
<h4><a id="method_gpu_device_create_pipeline_layout"></a><code>[method]gpu-device.create-pipeline-layout: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_device_create_pipeline_layout.self"></a><code>self</code>: borrow&lt;<a href="#gpu_device"><a href="#gpu_device"><code>gpu-device</code></a></a>&gt;</li>
<li><a id="method_gpu_device_create_pipeline_layout.descriptor"></a><code>descriptor</code>: <a href="#gpu_pipeline_layout_descriptor"><a href="#gpu_pipeline_layout_descriptor"><code>gpu-pipeline-layout-descriptor</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_gpu_device_create_pipeline_layout.0"></a> own&lt;<a href="#gpu_pipeline_layout"><a href="#gpu_pipeline_layout"><code>gpu-pipeline-layout</code></a></a>&gt;</li>
</ul>
<h4><a id="method_gpu_device_create_bind_group"></a><code>[method]gpu-device.create-bind-group: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_device_create_bind_group.self"></a><code>self</code>: borrow&lt;<a href="#gpu_device"><a href="#gpu_device"><code>gpu-device</code></a></a>&gt;</li>
<li><a id="method_gpu_device_create_bind_group.descriptor"></a><code>descriptor</code>: <a href="#gpu_bind_group_descriptor"><a href="#gpu_bind_group_descriptor"><code>gpu-bind-group-descriptor</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_gpu_device_create_bind_group.0"></a> own&lt;<a href="#gpu_bind_group"><a href="#gpu_bind_group"><code>gpu-bind-group</code></a></a>&gt;</li>
</ul>
<h4><a id="method_gpu_device_create_shader_module"></a><code>[method]gpu-device.create-shader-module: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_device_create_shader_module.self"></a><code>self</code>: borrow&lt;<a href="#gpu_device"><a href="#gpu_device"><code>gpu-device</code></a></a>&gt;</li>
<li><a id="method_gpu_device_create_shader_module.descriptor"></a><code>descriptor</code>: <a href="#gpu_shader_module_descriptor"><a href="#gpu_shader_module_descriptor"><code>gpu-shader-module-descriptor</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_gpu_device_create_shader_module.0"></a> own&lt;<a href="#gpu_shader_module"><a href="#gpu_shader_module"><code>gpu-shader-module</code></a></a>&gt;</li>
</ul>
<h4><a id="method_gpu_device_create_compute_pipeline"></a><code>[method]gpu-device.create-compute-pipeline: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_device_create_compute_pipeline.self"></a><code>self</code>: borrow&lt;<a href="#gpu_device"><a href="#gpu_device"><code>gpu-device</code></a></a>&gt;</li>
<li><a id="method_gpu_device_create_compute_pipeline.descriptor"></a><code>descriptor</code>: <a href="#gpu_compute_pipeline_descriptor"><a href="#gpu_compute_pipeline_descriptor"><code>gpu-compute-pipeline-descriptor</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_gpu_device_create_compute_pipeline.0"></a> own&lt;<a href="#gpu_compute_pipeline"><a href="#gpu_compute_pipeline"><code>gpu-compute-pipeline</code></a></a>&gt;</li>
</ul>
<h4><a id="method_gpu_device_create_render_pipeline"></a><code>[method]gpu-device.create-render-pipeline: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_device_create_render_pipeline.self"></a><code>self</code>: borrow&lt;<a href="#gpu_device"><a href="#gpu_device"><code>gpu-device</code></a></a>&gt;</li>
<li><a id="method_gpu_device_create_render_pipeline.descriptor"></a><code>descriptor</code>: <a href="#gpu_render_pipeline_descriptor"><a href="#gpu_render_pipeline_descriptor"><code>gpu-render-pipeline-descriptor</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_gpu_device_create_render_pipeline.0"></a> own&lt;<a href="#gpu_render_pipeline"><a href="#gpu_render_pipeline"><code>gpu-render-pipeline</code></a></a>&gt;</li>
</ul>
<h4><a id="method_gpu_device_create_compute_pipeline_async"></a><code>[method]gpu-device.create-compute-pipeline-async: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_device_create_compute_pipeline_async.self"></a><code>self</code>: borrow&lt;<a href="#gpu_device"><a href="#gpu_device"><code>gpu-device</code></a></a>&gt;</li>
<li><a id="method_gpu_device_create_compute_pipeline_async.descriptor"></a><code>descriptor</code>: <a href="#gpu_compute_pipeline_descriptor"><a href="#gpu_compute_pipeline_descriptor"><code>gpu-compute-pipeline-descriptor</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_gpu_device_create_compute_pipeline_async.0"></a> own&lt;<a href="#gpu_compute_pipeline"><a href="#gpu_compute_pipeline"><code>gpu-compute-pipeline</code></a></a>&gt;</li>
</ul>
<h4><a id="method_gpu_device_create_render_pipeline_async"></a><code>[method]gpu-device.create-render-pipeline-async: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_device_create_render_pipeline_async.self"></a><code>self</code>: borrow&lt;<a href="#gpu_device"><a href="#gpu_device"><code>gpu-device</code></a></a>&gt;</li>
<li><a id="method_gpu_device_create_render_pipeline_async.descriptor"></a><code>descriptor</code>: <a href="#gpu_render_pipeline_descriptor"><a href="#gpu_render_pipeline_descriptor"><code>gpu-render-pipeline-descriptor</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_gpu_device_create_render_pipeline_async.0"></a> own&lt;<a href="#gpu_render_pipeline"><a href="#gpu_render_pipeline"><code>gpu-render-pipeline</code></a></a>&gt;</li>
</ul>
<h4><a id="method_gpu_device_create_command_encoder"></a><code>[method]gpu-device.create-command-encoder: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_device_create_command_encoder.self"></a><code>self</code>: borrow&lt;<a href="#gpu_device"><a href="#gpu_device"><code>gpu-device</code></a></a>&gt;</li>
<li><a id="method_gpu_device_create_command_encoder.descriptor"></a><code>descriptor</code>: option&lt;<a href="#gpu_command_encoder_descriptor"><a href="#gpu_command_encoder_descriptor"><code>gpu-command-encoder-descriptor</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_gpu_device_create_command_encoder.0"></a> own&lt;<a href="#gpu_command_encoder"><a href="#gpu_command_encoder"><code>gpu-command-encoder</code></a></a>&gt;</li>
</ul>
<h4><a id="method_gpu_device_create_render_bundle_encoder"></a><code>[method]gpu-device.create-render-bundle-encoder: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_device_create_render_bundle_encoder.self"></a><code>self</code>: borrow&lt;<a href="#gpu_device"><a href="#gpu_device"><code>gpu-device</code></a></a>&gt;</li>
<li><a id="method_gpu_device_create_render_bundle_encoder.descriptor"></a><code>descriptor</code>: <a href="#gpu_render_bundle_encoder_descriptor"><a href="#gpu_render_bundle_encoder_descriptor"><code>gpu-render-bundle-encoder-descriptor</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_gpu_device_create_render_bundle_encoder.0"></a> own&lt;<a href="#gpu_render_bundle_encoder"><a href="#gpu_render_bundle_encoder"><code>gpu-render-bundle-encoder</code></a></a>&gt;</li>
</ul>
<h4><a id="method_gpu_device_create_query_set"></a><code>[method]gpu-device.create-query-set: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_device_create_query_set.self"></a><code>self</code>: borrow&lt;<a href="#gpu_device"><a href="#gpu_device"><code>gpu-device</code></a></a>&gt;</li>
<li><a id="method_gpu_device_create_query_set.descriptor"></a><code>descriptor</code>: <a href="#gpu_query_set_descriptor"><a href="#gpu_query_set_descriptor"><code>gpu-query-set-descriptor</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_gpu_device_create_query_set.0"></a> own&lt;<a href="#gpu_query_set"><a href="#gpu_query_set"><code>gpu-query-set</code></a></a>&gt;</li>
</ul>
<h4><a id="method_gpu_device_label"></a><code>[method]gpu-device.label: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_device_label.self"></a><code>self</code>: borrow&lt;<a href="#gpu_device"><a href="#gpu_device"><code>gpu-device</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_gpu_device_label.0"></a> <code>string</code></li>
</ul>
<h4><a id="method_gpu_device_set_label"></a><code>[method]gpu-device.set-label: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_device_set_label.self"></a><code>self</code>: borrow&lt;<a href="#gpu_device"><a href="#gpu_device"><code>gpu-device</code></a></a>&gt;</li>
<li><a id="method_gpu_device_set_label.label"></a><code>label</code>: <code>string</code></li>
</ul>
<h4><a id="method_gpu_device_lost"></a><code>[method]gpu-device.lost: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_device_lost.self"></a><code>self</code>: borrow&lt;<a href="#gpu_device"><a href="#gpu_device"><code>gpu-device</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_gpu_device_lost.0"></a> own&lt;<a href="#gpu_device_lost_info"><a href="#gpu_device_lost_info"><code>gpu-device-lost-info</code></a></a>&gt;</li>
</ul>
<h4><a id="method_gpu_device_push_error_scope"></a><code>[method]gpu-device.push-error-scope: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_device_push_error_scope.self"></a><code>self</code>: borrow&lt;<a href="#gpu_device"><a href="#gpu_device"><code>gpu-device</code></a></a>&gt;</li>
<li><a id="method_gpu_device_push_error_scope.filter"></a><code>filter</code>: <a href="#gpu_error_filter"><a href="#gpu_error_filter"><code>gpu-error-filter</code></a></a></li>
</ul>
<h4><a id="method_gpu_device_pop_error_scope"></a><code>[method]gpu-device.pop-error-scope: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_device_pop_error_scope.self"></a><code>self</code>: borrow&lt;<a href="#gpu_device"><a href="#gpu_device"><code>gpu-device</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_gpu_device_pop_error_scope.0"></a> option&lt;own&lt;<a href="#gpu_error"><a href="#gpu_error"><code>gpu-error</code></a></a>&gt;&gt;</li>
</ul>
<h4><a id="method_gpu_device_uncaptured_errors"></a><code>[method]gpu-device.uncaptured-errors: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_device_uncaptured_errors.self"></a><code>self</code>: borrow&lt;<a href="#gpu_device"><a href="#gpu_device"><code>gpu-device</code></a></a>&gt;</li>
</ul>
<h4><a id="method_gpu_device_configure"></a><code>[method]gpu-device.configure: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_device_configure.self"></a><code>self</code>: borrow&lt;<a href="#gpu_device"><a href="#gpu_device"><code>gpu-device</code></a></a>&gt;</li>
<li><a id="method_gpu_device_configure.configuration"></a><code>configuration</code>: <a href="#gpu_device_configuration"><a href="#gpu_device_configuration"><code>gpu-device-configuration</code></a></a></li>
</ul>
<h4><a id="method_gpu_buffer_size"></a><code>[method]gpu-buffer.size: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_buffer_size.self"></a><code>self</code>: borrow&lt;<a href="#gpu_buffer"><a href="#gpu_buffer"><code>gpu-buffer</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_gpu_buffer_size.0"></a> <a href="#gpu_size64_out"><a href="#gpu_size64_out"><code>gpu-size64-out</code></a></a></li>
</ul>
<h4><a id="method_gpu_buffer_usage"></a><code>[method]gpu-buffer.usage: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_buffer_usage.self"></a><code>self</code>: borrow&lt;<a href="#gpu_buffer"><a href="#gpu_buffer"><code>gpu-buffer</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_gpu_buffer_usage.0"></a> <a href="#gpu_flags_constant"><a href="#gpu_flags_constant"><code>gpu-flags-constant</code></a></a></li>
</ul>
<h4><a id="method_gpu_buffer_map_state"></a><code>[method]gpu-buffer.map-state: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_buffer_map_state.self"></a><code>self</code>: borrow&lt;<a href="#gpu_buffer"><a href="#gpu_buffer"><code>gpu-buffer</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_gpu_buffer_map_state.0"></a> <a href="#gpu_buffer_map_state"><a href="#gpu_buffer_map_state"><code>gpu-buffer-map-state</code></a></a></li>
</ul>
<h4><a id="method_gpu_buffer_map_async"></a><code>[method]gpu-buffer.map-async: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_buffer_map_async.self"></a><code>self</code>: borrow&lt;<a href="#gpu_buffer"><a href="#gpu_buffer"><code>gpu-buffer</code></a></a>&gt;</li>
<li><a id="method_gpu_buffer_map_async.mode"></a><code>mode</code>: <a href="#gpu_map_mode_flags"><a href="#gpu_map_mode_flags"><code>gpu-map-mode-flags</code></a></a></li>
<li><a id="method_gpu_buffer_map_async.offset"></a><code>offset</code>: option&lt;<a href="#gpu_size64"><a href="#gpu_size64"><code>gpu-size64</code></a></a>&gt;</li>
<li><a id="method_gpu_buffer_map_async.size"></a><code>size</code>: option&lt;<a href="#gpu_size64"><a href="#gpu_size64"><code>gpu-size64</code></a></a>&gt;</li>
</ul>
<h4><a id="method_gpu_buffer_get_mapped_range"></a><code>[method]gpu-buffer.get-mapped-range: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_buffer_get_mapped_range.self"></a><code>self</code>: borrow&lt;<a href="#gpu_buffer"><a href="#gpu_buffer"><code>gpu-buffer</code></a></a>&gt;</li>
<li><a id="method_gpu_buffer_get_mapped_range.offset"></a><code>offset</code>: option&lt;<a href="#gpu_size64"><a href="#gpu_size64"><code>gpu-size64</code></a></a>&gt;</li>
<li><a id="method_gpu_buffer_get_mapped_range.size"></a><code>size</code>: option&lt;<a href="#gpu_size64"><a href="#gpu_size64"><code>gpu-size64</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_gpu_buffer_get_mapped_range.0"></a> own&lt;<a href="#non_standard_buffer"><a href="#non_standard_buffer"><code>non-standard-buffer</code></a></a>&gt;</li>
</ul>
<h4><a id="method_gpu_buffer_unmap"></a><code>[method]gpu-buffer.unmap: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_buffer_unmap.self"></a><code>self</code>: borrow&lt;<a href="#gpu_buffer"><a href="#gpu_buffer"><code>gpu-buffer</code></a></a>&gt;</li>
</ul>
<h4><a id="method_gpu_buffer_destroy"></a><code>[method]gpu-buffer.destroy: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_buffer_destroy.self"></a><code>self</code>: borrow&lt;<a href="#gpu_buffer"><a href="#gpu_buffer"><code>gpu-buffer</code></a></a>&gt;</li>
</ul>
<h4><a id="method_gpu_buffer_label"></a><code>[method]gpu-buffer.label: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_buffer_label.self"></a><code>self</code>: borrow&lt;<a href="#gpu_buffer"><a href="#gpu_buffer"><code>gpu-buffer</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_gpu_buffer_label.0"></a> <code>string</code></li>
</ul>
<h4><a id="method_gpu_buffer_set_label"></a><code>[method]gpu-buffer.set-label: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_buffer_set_label.self"></a><code>self</code>: borrow&lt;<a href="#gpu_buffer"><a href="#gpu_buffer"><code>gpu-buffer</code></a></a>&gt;</li>
<li><a id="method_gpu_buffer_set_label.label"></a><code>label</code>: <code>string</code></li>
</ul>
<h4><a id="static_gpu_buffer_usage_map_read"></a><code>[static]gpu-buffer-usage.MAP-READ: func</code></h4>
<h5>Return values</h5>
<ul>
<li><a id="static_gpu_buffer_usage_map_read.0"></a> <a href="#gpu_flags_constant"><a href="#gpu_flags_constant"><code>gpu-flags-constant</code></a></a></li>
</ul>
<h4><a id="static_gpu_buffer_usage_map_write"></a><code>[static]gpu-buffer-usage.MAP-WRITE: func</code></h4>
<h5>Return values</h5>
<ul>
<li><a id="static_gpu_buffer_usage_map_write.0"></a> <a href="#gpu_flags_constant"><a href="#gpu_flags_constant"><code>gpu-flags-constant</code></a></a></li>
</ul>
<h4><a id="static_gpu_buffer_usage_copy_src"></a><code>[static]gpu-buffer-usage.COPY-SRC: func</code></h4>
<h5>Return values</h5>
<ul>
<li><a id="static_gpu_buffer_usage_copy_src.0"></a> <a href="#gpu_flags_constant"><a href="#gpu_flags_constant"><code>gpu-flags-constant</code></a></a></li>
</ul>
<h4><a id="static_gpu_buffer_usage_copy_dst"></a><code>[static]gpu-buffer-usage.COPY-DST: func</code></h4>
<h5>Return values</h5>
<ul>
<li><a id="static_gpu_buffer_usage_copy_dst.0"></a> <a href="#gpu_flags_constant"><a href="#gpu_flags_constant"><code>gpu-flags-constant</code></a></a></li>
</ul>
<h4><a id="static_gpu_buffer_usage_index"></a><code>[static]gpu-buffer-usage.INDEX: func</code></h4>
<h5>Return values</h5>
<ul>
<li><a id="static_gpu_buffer_usage_index.0"></a> <a href="#gpu_flags_constant"><a href="#gpu_flags_constant"><code>gpu-flags-constant</code></a></a></li>
</ul>
<h4><a id="static_gpu_buffer_usage_vertex"></a><code>[static]gpu-buffer-usage.VERTEX: func</code></h4>
<h5>Return values</h5>
<ul>
<li><a id="static_gpu_buffer_usage_vertex.0"></a> <a href="#gpu_flags_constant"><a href="#gpu_flags_constant"><code>gpu-flags-constant</code></a></a></li>
</ul>
<h4><a id="static_gpu_buffer_usage_uniform"></a><code>[static]gpu-buffer-usage.UNIFORM: func</code></h4>
<h5>Return values</h5>
<ul>
<li><a id="static_gpu_buffer_usage_uniform.0"></a> <a href="#gpu_flags_constant"><a href="#gpu_flags_constant"><code>gpu-flags-constant</code></a></a></li>
</ul>
<h4><a id="static_gpu_buffer_usage_storage"></a><code>[static]gpu-buffer-usage.STORAGE: func</code></h4>
<h5>Return values</h5>
<ul>
<li><a id="static_gpu_buffer_usage_storage.0"></a> <a href="#gpu_flags_constant"><a href="#gpu_flags_constant"><code>gpu-flags-constant</code></a></a></li>
</ul>
<h4><a id="static_gpu_buffer_usage_indirect"></a><code>[static]gpu-buffer-usage.INDIRECT: func</code></h4>
<h5>Return values</h5>
<ul>
<li><a id="static_gpu_buffer_usage_indirect.0"></a> <a href="#gpu_flags_constant"><a href="#gpu_flags_constant"><code>gpu-flags-constant</code></a></a></li>
</ul>
<h4><a id="static_gpu_buffer_usage_query_resolve"></a><code>[static]gpu-buffer-usage.QUERY-RESOLVE: func</code></h4>
<h5>Return values</h5>
<ul>
<li><a id="static_gpu_buffer_usage_query_resolve.0"></a> <a href="#gpu_flags_constant"><a href="#gpu_flags_constant"><code>gpu-flags-constant</code></a></a></li>
</ul>
<h4><a id="static_gpu_map_mode_read"></a><code>[static]gpu-map-mode.READ: func</code></h4>
<h5>Return values</h5>
<ul>
<li><a id="static_gpu_map_mode_read.0"></a> <a href="#gpu_flags_constant"><a href="#gpu_flags_constant"><code>gpu-flags-constant</code></a></a></li>
</ul>
<h4><a id="static_gpu_map_mode_write"></a><code>[static]gpu-map-mode.WRITE: func</code></h4>
<h5>Return values</h5>
<ul>
<li><a id="static_gpu_map_mode_write.0"></a> <a href="#gpu_flags_constant"><a href="#gpu_flags_constant"><code>gpu-flags-constant</code></a></a></li>
</ul>
<h4><a id="static_gpu_texture_from_graphics_buffer"></a><code>[static]gpu-texture.from-graphics-buffer: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="static_gpu_texture_from_graphics_buffer.buffer"></a><code>buffer</code>: own&lt;<a href="#abstract_buffer"><a href="#abstract_buffer"><code>abstract-buffer</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="static_gpu_texture_from_graphics_buffer.0"></a> own&lt;<a href="#gpu_texture"><a href="#gpu_texture"><code>gpu-texture</code></a></a>&gt;</li>
</ul>
<h4><a id="method_gpu_texture_create_view"></a><code>[method]gpu-texture.create-view: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_texture_create_view.self"></a><code>self</code>: borrow&lt;<a href="#gpu_texture"><a href="#gpu_texture"><code>gpu-texture</code></a></a>&gt;</li>
<li><a id="method_gpu_texture_create_view.descriptor"></a><code>descriptor</code>: option&lt;<a href="#gpu_texture_view_descriptor"><a href="#gpu_texture_view_descriptor"><code>gpu-texture-view-descriptor</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_gpu_texture_create_view.0"></a> own&lt;<a href="#gpu_texture_view"><a href="#gpu_texture_view"><code>gpu-texture-view</code></a></a>&gt;</li>
</ul>
<h4><a id="method_gpu_texture_destroy"></a><code>[method]gpu-texture.destroy: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_texture_destroy.self"></a><code>self</code>: borrow&lt;<a href="#gpu_texture"><a href="#gpu_texture"><code>gpu-texture</code></a></a>&gt;</li>
</ul>
<h4><a id="method_gpu_texture_width"></a><code>[method]gpu-texture.width: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_texture_width.self"></a><code>self</code>: borrow&lt;<a href="#gpu_texture"><a href="#gpu_texture"><code>gpu-texture</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_gpu_texture_width.0"></a> <a href="#gpu_integer_coordinate_out"><a href="#gpu_integer_coordinate_out"><code>gpu-integer-coordinate-out</code></a></a></li>
</ul>
<h4><a id="method_gpu_texture_height"></a><code>[method]gpu-texture.height: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_texture_height.self"></a><code>self</code>: borrow&lt;<a href="#gpu_texture"><a href="#gpu_texture"><code>gpu-texture</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_gpu_texture_height.0"></a> <a href="#gpu_integer_coordinate_out"><a href="#gpu_integer_coordinate_out"><code>gpu-integer-coordinate-out</code></a></a></li>
</ul>
<h4><a id="method_gpu_texture_depth_or_array_layers"></a><code>[method]gpu-texture.depth-or-array-layers: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_texture_depth_or_array_layers.self"></a><code>self</code>: borrow&lt;<a href="#gpu_texture"><a href="#gpu_texture"><code>gpu-texture</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_gpu_texture_depth_or_array_layers.0"></a> <a href="#gpu_integer_coordinate_out"><a href="#gpu_integer_coordinate_out"><code>gpu-integer-coordinate-out</code></a></a></li>
</ul>
<h4><a id="method_gpu_texture_mip_level_count"></a><code>[method]gpu-texture.mip-level-count: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_texture_mip_level_count.self"></a><code>self</code>: borrow&lt;<a href="#gpu_texture"><a href="#gpu_texture"><code>gpu-texture</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_gpu_texture_mip_level_count.0"></a> <a href="#gpu_integer_coordinate_out"><a href="#gpu_integer_coordinate_out"><code>gpu-integer-coordinate-out</code></a></a></li>
</ul>
<h4><a id="method_gpu_texture_sample_count"></a><code>[method]gpu-texture.sample-count: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_texture_sample_count.self"></a><code>self</code>: borrow&lt;<a href="#gpu_texture"><a href="#gpu_texture"><code>gpu-texture</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_gpu_texture_sample_count.0"></a> <a href="#gpu_size32_out"><a href="#gpu_size32_out"><code>gpu-size32-out</code></a></a></li>
</ul>
<h4><a id="method_gpu_texture_dimension"></a><code>[method]gpu-texture.dimension: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_texture_dimension.self"></a><code>self</code>: borrow&lt;<a href="#gpu_texture"><a href="#gpu_texture"><code>gpu-texture</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_gpu_texture_dimension.0"></a> <a href="#gpu_texture_dimension"><a href="#gpu_texture_dimension"><code>gpu-texture-dimension</code></a></a></li>
</ul>
<h4><a id="method_gpu_texture_format"></a><code>[method]gpu-texture.format: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_texture_format.self"></a><code>self</code>: borrow&lt;<a href="#gpu_texture"><a href="#gpu_texture"><code>gpu-texture</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_gpu_texture_format.0"></a> <a href="#gpu_texture_format"><a href="#gpu_texture_format"><code>gpu-texture-format</code></a></a></li>
</ul>
<h4><a id="method_gpu_texture_usage"></a><code>[method]gpu-texture.usage: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_texture_usage.self"></a><code>self</code>: borrow&lt;<a href="#gpu_texture"><a href="#gpu_texture"><code>gpu-texture</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_gpu_texture_usage.0"></a> <a href="#gpu_flags_constant"><a href="#gpu_flags_constant"><code>gpu-flags-constant</code></a></a></li>
</ul>
<h4><a id="method_gpu_texture_label"></a><code>[method]gpu-texture.label: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_texture_label.self"></a><code>self</code>: borrow&lt;<a href="#gpu_texture"><a href="#gpu_texture"><code>gpu-texture</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_gpu_texture_label.0"></a> <code>string</code></li>
</ul>
<h4><a id="method_gpu_texture_set_label"></a><code>[method]gpu-texture.set-label: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_texture_set_label.self"></a><code>self</code>: borrow&lt;<a href="#gpu_texture"><a href="#gpu_texture"><code>gpu-texture</code></a></a>&gt;</li>
<li><a id="method_gpu_texture_set_label.label"></a><code>label</code>: <code>string</code></li>
</ul>
<h4><a id="static_gpu_texture_usage_copy_src"></a><code>[static]gpu-texture-usage.COPY-SRC: func</code></h4>
<h5>Return values</h5>
<ul>
<li><a id="static_gpu_texture_usage_copy_src.0"></a> <a href="#gpu_flags_constant"><a href="#gpu_flags_constant"><code>gpu-flags-constant</code></a></a></li>
</ul>
<h4><a id="static_gpu_texture_usage_copy_dst"></a><code>[static]gpu-texture-usage.COPY-DST: func</code></h4>
<h5>Return values</h5>
<ul>
<li><a id="static_gpu_texture_usage_copy_dst.0"></a> <a href="#gpu_flags_constant"><a href="#gpu_flags_constant"><code>gpu-flags-constant</code></a></a></li>
</ul>
<h4><a id="static_gpu_texture_usage_texture_binding"></a><code>[static]gpu-texture-usage.TEXTURE-BINDING: func</code></h4>
<h5>Return values</h5>
<ul>
<li><a id="static_gpu_texture_usage_texture_binding.0"></a> <a href="#gpu_flags_constant"><a href="#gpu_flags_constant"><code>gpu-flags-constant</code></a></a></li>
</ul>
<h4><a id="static_gpu_texture_usage_storage_binding"></a><code>[static]gpu-texture-usage.STORAGE-BINDING: func</code></h4>
<h5>Return values</h5>
<ul>
<li><a id="static_gpu_texture_usage_storage_binding.0"></a> <a href="#gpu_flags_constant"><a href="#gpu_flags_constant"><code>gpu-flags-constant</code></a></a></li>
</ul>
<h4><a id="static_gpu_texture_usage_render_attachment"></a><code>[static]gpu-texture-usage.RENDER-ATTACHMENT: func</code></h4>
<h5>Return values</h5>
<ul>
<li><a id="static_gpu_texture_usage_render_attachment.0"></a> <a href="#gpu_flags_constant"><a href="#gpu_flags_constant"><code>gpu-flags-constant</code></a></a></li>
</ul>
<h4><a id="method_gpu_texture_view_label"></a><code>[method]gpu-texture-view.label: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_texture_view_label.self"></a><code>self</code>: borrow&lt;<a href="#gpu_texture_view"><a href="#gpu_texture_view"><code>gpu-texture-view</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_gpu_texture_view_label.0"></a> <code>string</code></li>
</ul>
<h4><a id="method_gpu_texture_view_set_label"></a><code>[method]gpu-texture-view.set-label: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_texture_view_set_label.self"></a><code>self</code>: borrow&lt;<a href="#gpu_texture_view"><a href="#gpu_texture_view"><code>gpu-texture-view</code></a></a>&gt;</li>
<li><a id="method_gpu_texture_view_set_label.label"></a><code>label</code>: <code>string</code></li>
</ul>
<h4><a id="method_gpu_sampler_label"></a><code>[method]gpu-sampler.label: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_sampler_label.self"></a><code>self</code>: borrow&lt;<a href="#gpu_sampler"><a href="#gpu_sampler"><code>gpu-sampler</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_gpu_sampler_label.0"></a> <code>string</code></li>
</ul>
<h4><a id="method_gpu_sampler_set_label"></a><code>[method]gpu-sampler.set-label: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_sampler_set_label.self"></a><code>self</code>: borrow&lt;<a href="#gpu_sampler"><a href="#gpu_sampler"><code>gpu-sampler</code></a></a>&gt;</li>
<li><a id="method_gpu_sampler_set_label.label"></a><code>label</code>: <code>string</code></li>
</ul>
<h4><a id="method_gpu_bind_group_layout_label"></a><code>[method]gpu-bind-group-layout.label: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_bind_group_layout_label.self"></a><code>self</code>: borrow&lt;<a href="#gpu_bind_group_layout"><a href="#gpu_bind_group_layout"><code>gpu-bind-group-layout</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_gpu_bind_group_layout_label.0"></a> <code>string</code></li>
</ul>
<h4><a id="method_gpu_bind_group_layout_set_label"></a><code>[method]gpu-bind-group-layout.set-label: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_bind_group_layout_set_label.self"></a><code>self</code>: borrow&lt;<a href="#gpu_bind_group_layout"><a href="#gpu_bind_group_layout"><code>gpu-bind-group-layout</code></a></a>&gt;</li>
<li><a id="method_gpu_bind_group_layout_set_label.label"></a><code>label</code>: <code>string</code></li>
</ul>
<h4><a id="static_gpu_shader_stage_vertex"></a><code>[static]gpu-shader-stage.VERTEX: func</code></h4>
<h5>Return values</h5>
<ul>
<li><a id="static_gpu_shader_stage_vertex.0"></a> <a href="#gpu_flags_constant"><a href="#gpu_flags_constant"><code>gpu-flags-constant</code></a></a></li>
</ul>
<h4><a id="static_gpu_shader_stage_fragment"></a><code>[static]gpu-shader-stage.FRAGMENT: func</code></h4>
<h5>Return values</h5>
<ul>
<li><a id="static_gpu_shader_stage_fragment.0"></a> <a href="#gpu_flags_constant"><a href="#gpu_flags_constant"><code>gpu-flags-constant</code></a></a></li>
</ul>
<h4><a id="static_gpu_shader_stage_compute"></a><code>[static]gpu-shader-stage.COMPUTE: func</code></h4>
<h5>Return values</h5>
<ul>
<li><a id="static_gpu_shader_stage_compute.0"></a> <a href="#gpu_flags_constant"><a href="#gpu_flags_constant"><code>gpu-flags-constant</code></a></a></li>
</ul>
<h4><a id="method_gpu_bind_group_label"></a><code>[method]gpu-bind-group.label: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_bind_group_label.self"></a><code>self</code>: borrow&lt;<a href="#gpu_bind_group"><a href="#gpu_bind_group"><code>gpu-bind-group</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_gpu_bind_group_label.0"></a> <code>string</code></li>
</ul>
<h4><a id="method_gpu_bind_group_set_label"></a><code>[method]gpu-bind-group.set-label: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_bind_group_set_label.self"></a><code>self</code>: borrow&lt;<a href="#gpu_bind_group"><a href="#gpu_bind_group"><code>gpu-bind-group</code></a></a>&gt;</li>
<li><a id="method_gpu_bind_group_set_label.label"></a><code>label</code>: <code>string</code></li>
</ul>
<h4><a id="method_gpu_pipeline_layout_label"></a><code>[method]gpu-pipeline-layout.label: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_pipeline_layout_label.self"></a><code>self</code>: borrow&lt;<a href="#gpu_pipeline_layout"><a href="#gpu_pipeline_layout"><code>gpu-pipeline-layout</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_gpu_pipeline_layout_label.0"></a> <code>string</code></li>
</ul>
<h4><a id="method_gpu_pipeline_layout_set_label"></a><code>[method]gpu-pipeline-layout.set-label: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_pipeline_layout_set_label.self"></a><code>self</code>: borrow&lt;<a href="#gpu_pipeline_layout"><a href="#gpu_pipeline_layout"><code>gpu-pipeline-layout</code></a></a>&gt;</li>
<li><a id="method_gpu_pipeline_layout_set_label.label"></a><code>label</code>: <code>string</code></li>
</ul>
<h4><a id="method_gpu_shader_module_get_compilation_info"></a><code>[method]gpu-shader-module.get-compilation-info: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_shader_module_get_compilation_info.self"></a><code>self</code>: borrow&lt;<a href="#gpu_shader_module"><a href="#gpu_shader_module"><code>gpu-shader-module</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_gpu_shader_module_get_compilation_info.0"></a> own&lt;<a href="#gpu_compilation_info"><a href="#gpu_compilation_info"><code>gpu-compilation-info</code></a></a>&gt;</li>
</ul>
<h4><a id="method_gpu_shader_module_label"></a><code>[method]gpu-shader-module.label: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_shader_module_label.self"></a><code>self</code>: borrow&lt;<a href="#gpu_shader_module"><a href="#gpu_shader_module"><code>gpu-shader-module</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_gpu_shader_module_label.0"></a> <code>string</code></li>
</ul>
<h4><a id="method_gpu_shader_module_set_label"></a><code>[method]gpu-shader-module.set-label: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_shader_module_set_label.self"></a><code>self</code>: borrow&lt;<a href="#gpu_shader_module"><a href="#gpu_shader_module"><code>gpu-shader-module</code></a></a>&gt;</li>
<li><a id="method_gpu_shader_module_set_label.label"></a><code>label</code>: <code>string</code></li>
</ul>
<h4><a id="method_gpu_compilation_message_message"></a><code>[method]gpu-compilation-message.message: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_compilation_message_message.self"></a><code>self</code>: borrow&lt;<a href="#gpu_compilation_message"><a href="#gpu_compilation_message"><code>gpu-compilation-message</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_gpu_compilation_message_message.0"></a> <code>string</code></li>
</ul>
<h4><a id="method_gpu_compilation_message_type"></a><code>[method]gpu-compilation-message.type: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_compilation_message_type.self"></a><code>self</code>: borrow&lt;<a href="#gpu_compilation_message"><a href="#gpu_compilation_message"><code>gpu-compilation-message</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_gpu_compilation_message_type.0"></a> <a href="#gpu_compilation_message_type"><a href="#gpu_compilation_message_type"><code>gpu-compilation-message-type</code></a></a></li>
</ul>
<h4><a id="method_gpu_compilation_message_line_num"></a><code>[method]gpu-compilation-message.line-num: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_compilation_message_line_num.self"></a><code>self</code>: borrow&lt;<a href="#gpu_compilation_message"><a href="#gpu_compilation_message"><code>gpu-compilation-message</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_gpu_compilation_message_line_num.0"></a> <code>u64</code></li>
</ul>
<h4><a id="method_gpu_compilation_message_line_pos"></a><code>[method]gpu-compilation-message.line-pos: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_compilation_message_line_pos.self"></a><code>self</code>: borrow&lt;<a href="#gpu_compilation_message"><a href="#gpu_compilation_message"><code>gpu-compilation-message</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_gpu_compilation_message_line_pos.0"></a> <code>u64</code></li>
</ul>
<h4><a id="method_gpu_compilation_message_offset"></a><code>[method]gpu-compilation-message.offset: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_compilation_message_offset.self"></a><code>self</code>: borrow&lt;<a href="#gpu_compilation_message"><a href="#gpu_compilation_message"><code>gpu-compilation-message</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_gpu_compilation_message_offset.0"></a> <code>u64</code></li>
</ul>
<h4><a id="method_gpu_compilation_message_length"></a><code>[method]gpu-compilation-message.length: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_compilation_message_length.self"></a><code>self</code>: borrow&lt;<a href="#gpu_compilation_message"><a href="#gpu_compilation_message"><code>gpu-compilation-message</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_gpu_compilation_message_length.0"></a> <code>u64</code></li>
</ul>
<h4><a id="method_gpu_compilation_info_messages"></a><code>[method]gpu-compilation-info.messages: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_compilation_info_messages.self"></a><code>self</code>: borrow&lt;<a href="#gpu_compilation_info"><a href="#gpu_compilation_info"><code>gpu-compilation-info</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_gpu_compilation_info_messages.0"></a> list&lt;own&lt;<a href="#gpu_compilation_message"><a href="#gpu_compilation_message"><code>gpu-compilation-message</code></a></a>&gt;&gt;</li>
</ul>
<h4><a id="constructor_gpu_pipeline_error"></a><code>[constructor]gpu-pipeline-error: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="constructor_gpu_pipeline_error.message"></a><code>message</code>: option&lt;<code>string</code>&gt;</li>
<li><a id="constructor_gpu_pipeline_error.options"></a><code>options</code>: <a href="#gpu_pipeline_error_init"><a href="#gpu_pipeline_error_init"><code>gpu-pipeline-error-init</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="constructor_gpu_pipeline_error.0"></a> own&lt;<a href="#gpu_pipeline_error"><a href="#gpu_pipeline_error"><code>gpu-pipeline-error</code></a></a>&gt;</li>
</ul>
<h4><a id="method_gpu_pipeline_error_reason"></a><code>[method]gpu-pipeline-error.reason: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_pipeline_error_reason.self"></a><code>self</code>: borrow&lt;<a href="#gpu_pipeline_error"><a href="#gpu_pipeline_error"><code>gpu-pipeline-error</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_gpu_pipeline_error_reason.0"></a> <a href="#gpu_pipeline_error_reason"><a href="#gpu_pipeline_error_reason"><code>gpu-pipeline-error-reason</code></a></a></li>
</ul>
<h4><a id="constructor_record_gpu_pipeline_constant_value"></a><code>[constructor]record-gpu-pipeline-constant-value: func</code></h4>
<h5>Return values</h5>
<ul>
<li><a id="constructor_record_gpu_pipeline_constant_value.0"></a> own&lt;<a href="#record_gpu_pipeline_constant_value"><a href="#record_gpu_pipeline_constant_value"><code>record-gpu-pipeline-constant-value</code></a></a>&gt;</li>
</ul>
<h4><a id="method_record_gpu_pipeline_constant_value_add"></a><code>[method]record-gpu-pipeline-constant-value.add: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_record_gpu_pipeline_constant_value_add.self"></a><code>self</code>: borrow&lt;<a href="#record_gpu_pipeline_constant_value"><a href="#record_gpu_pipeline_constant_value"><code>record-gpu-pipeline-constant-value</code></a></a>&gt;</li>
<li><a id="method_record_gpu_pipeline_constant_value_add.key"></a><code>key</code>: <code>string</code></li>
<li><a id="method_record_gpu_pipeline_constant_value_add.value"></a><code>value</code>: <a href="#gpu_pipeline_constant_value"><a href="#gpu_pipeline_constant_value"><code>gpu-pipeline-constant-value</code></a></a></li>
</ul>
<h4><a id="method_record_gpu_pipeline_constant_value_get"></a><code>[method]record-gpu-pipeline-constant-value.get: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_record_gpu_pipeline_constant_value_get.self"></a><code>self</code>: borrow&lt;<a href="#record_gpu_pipeline_constant_value"><a href="#record_gpu_pipeline_constant_value"><code>record-gpu-pipeline-constant-value</code></a></a>&gt;</li>
<li><a id="method_record_gpu_pipeline_constant_value_get.key"></a><code>key</code>: <code>string</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_record_gpu_pipeline_constant_value_get.0"></a> <a href="#gpu_pipeline_constant_value"><a href="#gpu_pipeline_constant_value"><code>gpu-pipeline-constant-value</code></a></a></li>
</ul>
<h4><a id="method_record_gpu_pipeline_constant_value_has"></a><code>[method]record-gpu-pipeline-constant-value.has: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_record_gpu_pipeline_constant_value_has.self"></a><code>self</code>: borrow&lt;<a href="#record_gpu_pipeline_constant_value"><a href="#record_gpu_pipeline_constant_value"><code>record-gpu-pipeline-constant-value</code></a></a>&gt;</li>
<li><a id="method_record_gpu_pipeline_constant_value_has.key"></a><code>key</code>: <code>string</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_record_gpu_pipeline_constant_value_has.0"></a> <code>bool</code></li>
</ul>
<h4><a id="method_record_gpu_pipeline_constant_value_remove"></a><code>[method]record-gpu-pipeline-constant-value.remove: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_record_gpu_pipeline_constant_value_remove.self"></a><code>self</code>: borrow&lt;<a href="#record_gpu_pipeline_constant_value"><a href="#record_gpu_pipeline_constant_value"><code>record-gpu-pipeline-constant-value</code></a></a>&gt;</li>
<li><a id="method_record_gpu_pipeline_constant_value_remove.key"></a><code>key</code>: <code>string</code></li>
</ul>
<h4><a id="method_record_gpu_pipeline_constant_value_keys"></a><code>[method]record-gpu-pipeline-constant-value.keys: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_record_gpu_pipeline_constant_value_keys.self"></a><code>self</code>: borrow&lt;<a href="#record_gpu_pipeline_constant_value"><a href="#record_gpu_pipeline_constant_value"><code>record-gpu-pipeline-constant-value</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_record_gpu_pipeline_constant_value_keys.0"></a> list&lt;<code>string</code>&gt;</li>
</ul>
<h4><a id="method_record_gpu_pipeline_constant_value_values"></a><code>[method]record-gpu-pipeline-constant-value.values: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_record_gpu_pipeline_constant_value_values.self"></a><code>self</code>: borrow&lt;<a href="#record_gpu_pipeline_constant_value"><a href="#record_gpu_pipeline_constant_value"><code>record-gpu-pipeline-constant-value</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_record_gpu_pipeline_constant_value_values.0"></a> list&lt;<a href="#gpu_pipeline_constant_value"><a href="#gpu_pipeline_constant_value"><code>gpu-pipeline-constant-value</code></a></a>&gt;</li>
</ul>
<h4><a id="method_record_gpu_pipeline_constant_value_entries"></a><code>[method]record-gpu-pipeline-constant-value.entries: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_record_gpu_pipeline_constant_value_entries.self"></a><code>self</code>: borrow&lt;<a href="#record_gpu_pipeline_constant_value"><a href="#record_gpu_pipeline_constant_value"><code>record-gpu-pipeline-constant-value</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_record_gpu_pipeline_constant_value_entries.0"></a> (<code>string</code>, <a href="#gpu_pipeline_constant_value"><a href="#gpu_pipeline_constant_value"><code>gpu-pipeline-constant-value</code></a></a>)</li>
</ul>
<h4><a id="method_gpu_compute_pipeline_label"></a><code>[method]gpu-compute-pipeline.label: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_compute_pipeline_label.self"></a><code>self</code>: borrow&lt;<a href="#gpu_compute_pipeline"><a href="#gpu_compute_pipeline"><code>gpu-compute-pipeline</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_gpu_compute_pipeline_label.0"></a> <code>string</code></li>
</ul>
<h4><a id="method_gpu_compute_pipeline_set_label"></a><code>[method]gpu-compute-pipeline.set-label: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_compute_pipeline_set_label.self"></a><code>self</code>: borrow&lt;<a href="#gpu_compute_pipeline"><a href="#gpu_compute_pipeline"><code>gpu-compute-pipeline</code></a></a>&gt;</li>
<li><a id="method_gpu_compute_pipeline_set_label.label"></a><code>label</code>: <code>string</code></li>
</ul>
<h4><a id="method_gpu_compute_pipeline_get_bind_group_layout"></a><code>[method]gpu-compute-pipeline.get-bind-group-layout: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_compute_pipeline_get_bind_group_layout.self"></a><code>self</code>: borrow&lt;<a href="#gpu_compute_pipeline"><a href="#gpu_compute_pipeline"><code>gpu-compute-pipeline</code></a></a>&gt;</li>
<li><a id="method_gpu_compute_pipeline_get_bind_group_layout.index"></a><code>index</code>: <code>u32</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_gpu_compute_pipeline_get_bind_group_layout.0"></a> own&lt;<a href="#gpu_bind_group_layout"><a href="#gpu_bind_group_layout"><code>gpu-bind-group-layout</code></a></a>&gt;</li>
</ul>
<h4><a id="method_gpu_render_pipeline_label"></a><code>[method]gpu-render-pipeline.label: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_render_pipeline_label.self"></a><code>self</code>: borrow&lt;<a href="#gpu_render_pipeline"><a href="#gpu_render_pipeline"><code>gpu-render-pipeline</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_gpu_render_pipeline_label.0"></a> <code>string</code></li>
</ul>
<h4><a id="method_gpu_render_pipeline_set_label"></a><code>[method]gpu-render-pipeline.set-label: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_render_pipeline_set_label.self"></a><code>self</code>: borrow&lt;<a href="#gpu_render_pipeline"><a href="#gpu_render_pipeline"><code>gpu-render-pipeline</code></a></a>&gt;</li>
<li><a id="method_gpu_render_pipeline_set_label.label"></a><code>label</code>: <code>string</code></li>
</ul>
<h4><a id="method_gpu_render_pipeline_get_bind_group_layout"></a><code>[method]gpu-render-pipeline.get-bind-group-layout: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_render_pipeline_get_bind_group_layout.self"></a><code>self</code>: borrow&lt;<a href="#gpu_render_pipeline"><a href="#gpu_render_pipeline"><code>gpu-render-pipeline</code></a></a>&gt;</li>
<li><a id="method_gpu_render_pipeline_get_bind_group_layout.index"></a><code>index</code>: <code>u32</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_gpu_render_pipeline_get_bind_group_layout.0"></a> own&lt;<a href="#gpu_bind_group_layout"><a href="#gpu_bind_group_layout"><code>gpu-bind-group-layout</code></a></a>&gt;</li>
</ul>
<h4><a id="static_gpu_color_write_red"></a><code>[static]gpu-color-write.RED: func</code></h4>
<h5>Return values</h5>
<ul>
<li><a id="static_gpu_color_write_red.0"></a> <a href="#gpu_flags_constant"><a href="#gpu_flags_constant"><code>gpu-flags-constant</code></a></a></li>
</ul>
<h4><a id="static_gpu_color_write_green"></a><code>[static]gpu-color-write.GREEN: func</code></h4>
<h5>Return values</h5>
<ul>
<li><a id="static_gpu_color_write_green.0"></a> <a href="#gpu_flags_constant"><a href="#gpu_flags_constant"><code>gpu-flags-constant</code></a></a></li>
</ul>
<h4><a id="static_gpu_color_write_blue"></a><code>[static]gpu-color-write.BLUE: func</code></h4>
<h5>Return values</h5>
<ul>
<li><a id="static_gpu_color_write_blue.0"></a> <a href="#gpu_flags_constant"><a href="#gpu_flags_constant"><code>gpu-flags-constant</code></a></a></li>
</ul>
<h4><a id="static_gpu_color_write_alpha"></a><code>[static]gpu-color-write.ALPHA: func</code></h4>
<h5>Return values</h5>
<ul>
<li><a id="static_gpu_color_write_alpha.0"></a> <a href="#gpu_flags_constant"><a href="#gpu_flags_constant"><code>gpu-flags-constant</code></a></a></li>
</ul>
<h4><a id="static_gpu_color_write_all"></a><code>[static]gpu-color-write.ALL: func</code></h4>
<h5>Return values</h5>
<ul>
<li><a id="static_gpu_color_write_all.0"></a> <a href="#gpu_flags_constant"><a href="#gpu_flags_constant"><code>gpu-flags-constant</code></a></a></li>
</ul>
<h4><a id="method_gpu_command_buffer_label"></a><code>[method]gpu-command-buffer.label: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_command_buffer_label.self"></a><code>self</code>: borrow&lt;<a href="#gpu_command_buffer"><a href="#gpu_command_buffer"><code>gpu-command-buffer</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_gpu_command_buffer_label.0"></a> <code>string</code></li>
</ul>
<h4><a id="method_gpu_command_buffer_set_label"></a><code>[method]gpu-command-buffer.set-label: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_command_buffer_set_label.self"></a><code>self</code>: borrow&lt;<a href="#gpu_command_buffer"><a href="#gpu_command_buffer"><code>gpu-command-buffer</code></a></a>&gt;</li>
<li><a id="method_gpu_command_buffer_set_label.label"></a><code>label</code>: <code>string</code></li>
</ul>
<h4><a id="method_gpu_command_encoder_begin_render_pass"></a><code>[method]gpu-command-encoder.begin-render-pass: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_command_encoder_begin_render_pass.self"></a><code>self</code>: borrow&lt;<a href="#gpu_command_encoder"><a href="#gpu_command_encoder"><code>gpu-command-encoder</code></a></a>&gt;</li>
<li><a id="method_gpu_command_encoder_begin_render_pass.descriptor"></a><code>descriptor</code>: <a href="#gpu_render_pass_descriptor"><a href="#gpu_render_pass_descriptor"><code>gpu-render-pass-descriptor</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_gpu_command_encoder_begin_render_pass.0"></a> own&lt;<a href="#gpu_render_pass_encoder"><a href="#gpu_render_pass_encoder"><code>gpu-render-pass-encoder</code></a></a>&gt;</li>
</ul>
<h4><a id="method_gpu_command_encoder_begin_compute_pass"></a><code>[method]gpu-command-encoder.begin-compute-pass: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_command_encoder_begin_compute_pass.self"></a><code>self</code>: borrow&lt;<a href="#gpu_command_encoder"><a href="#gpu_command_encoder"><code>gpu-command-encoder</code></a></a>&gt;</li>
<li><a id="method_gpu_command_encoder_begin_compute_pass.descriptor"></a><code>descriptor</code>: option&lt;<a href="#gpu_compute_pass_descriptor"><a href="#gpu_compute_pass_descriptor"><code>gpu-compute-pass-descriptor</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_gpu_command_encoder_begin_compute_pass.0"></a> own&lt;<a href="#gpu_compute_pass_encoder"><a href="#gpu_compute_pass_encoder"><code>gpu-compute-pass-encoder</code></a></a>&gt;</li>
</ul>
<h4><a id="method_gpu_command_encoder_copy_buffer_to_buffer"></a><code>[method]gpu-command-encoder.copy-buffer-to-buffer: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_command_encoder_copy_buffer_to_buffer.self"></a><code>self</code>: borrow&lt;<a href="#gpu_command_encoder"><a href="#gpu_command_encoder"><code>gpu-command-encoder</code></a></a>&gt;</li>
<li><a id="method_gpu_command_encoder_copy_buffer_to_buffer.source"></a><code>source</code>: borrow&lt;<a href="#gpu_buffer"><a href="#gpu_buffer"><code>gpu-buffer</code></a></a>&gt;</li>
<li><a id="method_gpu_command_encoder_copy_buffer_to_buffer.source_offset"></a><code>source-offset</code>: <a href="#gpu_size64"><a href="#gpu_size64"><code>gpu-size64</code></a></a></li>
<li><a id="method_gpu_command_encoder_copy_buffer_to_buffer.destination"></a><code>destination</code>: borrow&lt;<a href="#gpu_buffer"><a href="#gpu_buffer"><code>gpu-buffer</code></a></a>&gt;</li>
<li><a id="method_gpu_command_encoder_copy_buffer_to_buffer.destination_offset"></a><code>destination-offset</code>: <a href="#gpu_size64"><a href="#gpu_size64"><code>gpu-size64</code></a></a></li>
<li><a id="method_gpu_command_encoder_copy_buffer_to_buffer.size"></a><code>size</code>: <a href="#gpu_size64"><a href="#gpu_size64"><code>gpu-size64</code></a></a></li>
</ul>
<h4><a id="method_gpu_command_encoder_copy_buffer_to_texture"></a><code>[method]gpu-command-encoder.copy-buffer-to-texture: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_command_encoder_copy_buffer_to_texture.self"></a><code>self</code>: borrow&lt;<a href="#gpu_command_encoder"><a href="#gpu_command_encoder"><code>gpu-command-encoder</code></a></a>&gt;</li>
<li><a id="method_gpu_command_encoder_copy_buffer_to_texture.source"></a><code>source</code>: <a href="#gpu_image_copy_buffer"><a href="#gpu_image_copy_buffer"><code>gpu-image-copy-buffer</code></a></a></li>
<li><a id="method_gpu_command_encoder_copy_buffer_to_texture.destination"></a><code>destination</code>: <a href="#gpu_image_copy_texture"><a href="#gpu_image_copy_texture"><code>gpu-image-copy-texture</code></a></a></li>
<li><a id="method_gpu_command_encoder_copy_buffer_to_texture.copy_size"></a><code>copy-size</code>: <a href="#gpu_extent3_d"><a href="#gpu_extent3_d"><code>gpu-extent3-d</code></a></a></li>
</ul>
<h4><a id="method_gpu_command_encoder_copy_texture_to_buffer"></a><code>[method]gpu-command-encoder.copy-texture-to-buffer: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_command_encoder_copy_texture_to_buffer.self"></a><code>self</code>: borrow&lt;<a href="#gpu_command_encoder"><a href="#gpu_command_encoder"><code>gpu-command-encoder</code></a></a>&gt;</li>
<li><a id="method_gpu_command_encoder_copy_texture_to_buffer.source"></a><code>source</code>: <a href="#gpu_image_copy_texture"><a href="#gpu_image_copy_texture"><code>gpu-image-copy-texture</code></a></a></li>
<li><a id="method_gpu_command_encoder_copy_texture_to_buffer.destination"></a><code>destination</code>: <a href="#gpu_image_copy_buffer"><a href="#gpu_image_copy_buffer"><code>gpu-image-copy-buffer</code></a></a></li>
<li><a id="method_gpu_command_encoder_copy_texture_to_buffer.copy_size"></a><code>copy-size</code>: <a href="#gpu_extent3_d"><a href="#gpu_extent3_d"><code>gpu-extent3-d</code></a></a></li>
</ul>
<h4><a id="method_gpu_command_encoder_copy_texture_to_texture"></a><code>[method]gpu-command-encoder.copy-texture-to-texture: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_command_encoder_copy_texture_to_texture.self"></a><code>self</code>: borrow&lt;<a href="#gpu_command_encoder"><a href="#gpu_command_encoder"><code>gpu-command-encoder</code></a></a>&gt;</li>
<li><a id="method_gpu_command_encoder_copy_texture_to_texture.source"></a><code>source</code>: <a href="#gpu_image_copy_texture"><a href="#gpu_image_copy_texture"><code>gpu-image-copy-texture</code></a></a></li>
<li><a id="method_gpu_command_encoder_copy_texture_to_texture.destination"></a><code>destination</code>: <a href="#gpu_image_copy_texture"><a href="#gpu_image_copy_texture"><code>gpu-image-copy-texture</code></a></a></li>
<li><a id="method_gpu_command_encoder_copy_texture_to_texture.copy_size"></a><code>copy-size</code>: <a href="#gpu_extent3_d"><a href="#gpu_extent3_d"><code>gpu-extent3-d</code></a></a></li>
</ul>
<h4><a id="method_gpu_command_encoder_clear_buffer"></a><code>[method]gpu-command-encoder.clear-buffer: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_command_encoder_clear_buffer.self"></a><code>self</code>: borrow&lt;<a href="#gpu_command_encoder"><a href="#gpu_command_encoder"><code>gpu-command-encoder</code></a></a>&gt;</li>
<li><a id="method_gpu_command_encoder_clear_buffer.buffer"></a><code>buffer</code>: borrow&lt;<a href="#gpu_buffer"><a href="#gpu_buffer"><code>gpu-buffer</code></a></a>&gt;</li>
<li><a id="method_gpu_command_encoder_clear_buffer.offset"></a><code>offset</code>: option&lt;<a href="#gpu_size64"><a href="#gpu_size64"><code>gpu-size64</code></a></a>&gt;</li>
<li><a id="method_gpu_command_encoder_clear_buffer.size"></a><code>size</code>: option&lt;<a href="#gpu_size64"><a href="#gpu_size64"><code>gpu-size64</code></a></a>&gt;</li>
</ul>
<h4><a id="method_gpu_command_encoder_resolve_query_set"></a><code>[method]gpu-command-encoder.resolve-query-set: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_command_encoder_resolve_query_set.self"></a><code>self</code>: borrow&lt;<a href="#gpu_command_encoder"><a href="#gpu_command_encoder"><code>gpu-command-encoder</code></a></a>&gt;</li>
<li><a id="method_gpu_command_encoder_resolve_query_set.query_set"></a><code>query-set</code>: borrow&lt;<a href="#gpu_query_set"><a href="#gpu_query_set"><code>gpu-query-set</code></a></a>&gt;</li>
<li><a id="method_gpu_command_encoder_resolve_query_set.first_query"></a><code>first-query</code>: <a href="#gpu_size32"><a href="#gpu_size32"><code>gpu-size32</code></a></a></li>
<li><a id="method_gpu_command_encoder_resolve_query_set.query_count"></a><code>query-count</code>: <a href="#gpu_size32"><a href="#gpu_size32"><code>gpu-size32</code></a></a></li>
<li><a id="method_gpu_command_encoder_resolve_query_set.destination"></a><code>destination</code>: borrow&lt;<a href="#gpu_buffer"><a href="#gpu_buffer"><code>gpu-buffer</code></a></a>&gt;</li>
<li><a id="method_gpu_command_encoder_resolve_query_set.destination_offset"></a><code>destination-offset</code>: <a href="#gpu_size64"><a href="#gpu_size64"><code>gpu-size64</code></a></a></li>
</ul>
<h4><a id="method_gpu_command_encoder_finish"></a><code>[method]gpu-command-encoder.finish: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_command_encoder_finish.self"></a><code>self</code>: borrow&lt;<a href="#gpu_command_encoder"><a href="#gpu_command_encoder"><code>gpu-command-encoder</code></a></a>&gt;</li>
<li><a id="method_gpu_command_encoder_finish.descriptor"></a><code>descriptor</code>: option&lt;<a href="#gpu_command_buffer_descriptor"><a href="#gpu_command_buffer_descriptor"><code>gpu-command-buffer-descriptor</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_gpu_command_encoder_finish.0"></a> own&lt;<a href="#gpu_command_buffer"><a href="#gpu_command_buffer"><code>gpu-command-buffer</code></a></a>&gt;</li>
</ul>
<h4><a id="method_gpu_command_encoder_label"></a><code>[method]gpu-command-encoder.label: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_command_encoder_label.self"></a><code>self</code>: borrow&lt;<a href="#gpu_command_encoder"><a href="#gpu_command_encoder"><code>gpu-command-encoder</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_gpu_command_encoder_label.0"></a> <code>string</code></li>
</ul>
<h4><a id="method_gpu_command_encoder_set_label"></a><code>[method]gpu-command-encoder.set-label: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_command_encoder_set_label.self"></a><code>self</code>: borrow&lt;<a href="#gpu_command_encoder"><a href="#gpu_command_encoder"><code>gpu-command-encoder</code></a></a>&gt;</li>
<li><a id="method_gpu_command_encoder_set_label.label"></a><code>label</code>: <code>string</code></li>
</ul>
<h4><a id="method_gpu_command_encoder_push_debug_group"></a><code>[method]gpu-command-encoder.push-debug-group: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_command_encoder_push_debug_group.self"></a><code>self</code>: borrow&lt;<a href="#gpu_command_encoder"><a href="#gpu_command_encoder"><code>gpu-command-encoder</code></a></a>&gt;</li>
<li><a id="method_gpu_command_encoder_push_debug_group.group_label"></a><code>group-label</code>: <code>string</code></li>
</ul>
<h4><a id="method_gpu_command_encoder_pop_debug_group"></a><code>[method]gpu-command-encoder.pop-debug-group: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_command_encoder_pop_debug_group.self"></a><code>self</code>: borrow&lt;<a href="#gpu_command_encoder"><a href="#gpu_command_encoder"><code>gpu-command-encoder</code></a></a>&gt;</li>
</ul>
<h4><a id="method_gpu_command_encoder_insert_debug_marker"></a><code>[method]gpu-command-encoder.insert-debug-marker: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_command_encoder_insert_debug_marker.self"></a><code>self</code>: borrow&lt;<a href="#gpu_command_encoder"><a href="#gpu_command_encoder"><code>gpu-command-encoder</code></a></a>&gt;</li>
<li><a id="method_gpu_command_encoder_insert_debug_marker.marker_label"></a><code>marker-label</code>: <code>string</code></li>
</ul>
<h4><a id="method_gpu_compute_pass_encoder_set_pipeline"></a><code>[method]gpu-compute-pass-encoder.set-pipeline: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_compute_pass_encoder_set_pipeline.self"></a><code>self</code>: borrow&lt;<a href="#gpu_compute_pass_encoder"><a href="#gpu_compute_pass_encoder"><code>gpu-compute-pass-encoder</code></a></a>&gt;</li>
<li><a id="method_gpu_compute_pass_encoder_set_pipeline.pipeline"></a><code>pipeline</code>: borrow&lt;<a href="#gpu_compute_pipeline"><a href="#gpu_compute_pipeline"><code>gpu-compute-pipeline</code></a></a>&gt;</li>
</ul>
<h4><a id="method_gpu_compute_pass_encoder_dispatch_workgroups"></a><code>[method]gpu-compute-pass-encoder.dispatch-workgroups: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_compute_pass_encoder_dispatch_workgroups.self"></a><code>self</code>: borrow&lt;<a href="#gpu_compute_pass_encoder"><a href="#gpu_compute_pass_encoder"><code>gpu-compute-pass-encoder</code></a></a>&gt;</li>
<li><a id="method_gpu_compute_pass_encoder_dispatch_workgroups.workgroup_count_x"></a><code>workgroup-count-x</code>: <a href="#gpu_size32"><a href="#gpu_size32"><code>gpu-size32</code></a></a></li>
<li><a id="method_gpu_compute_pass_encoder_dispatch_workgroups.workgroup_count_y"></a><code>workgroup-count-y</code>: option&lt;<a href="#gpu_size32"><a href="#gpu_size32"><code>gpu-size32</code></a></a>&gt;</li>
<li><a id="method_gpu_compute_pass_encoder_dispatch_workgroups.workgroup_count_z"></a><code>workgroup-count-z</code>: option&lt;<a href="#gpu_size32"><a href="#gpu_size32"><code>gpu-size32</code></a></a>&gt;</li>
</ul>
<h4><a id="method_gpu_compute_pass_encoder_dispatch_workgroups_indirect"></a><code>[method]gpu-compute-pass-encoder.dispatch-workgroups-indirect: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_compute_pass_encoder_dispatch_workgroups_indirect.self"></a><code>self</code>: borrow&lt;<a href="#gpu_compute_pass_encoder"><a href="#gpu_compute_pass_encoder"><code>gpu-compute-pass-encoder</code></a></a>&gt;</li>
<li><a id="method_gpu_compute_pass_encoder_dispatch_workgroups_indirect.indirect_buffer"></a><code>indirect-buffer</code>: borrow&lt;<a href="#gpu_buffer"><a href="#gpu_buffer"><code>gpu-buffer</code></a></a>&gt;</li>
<li><a id="method_gpu_compute_pass_encoder_dispatch_workgroups_indirect.indirect_offset"></a><code>indirect-offset</code>: <a href="#gpu_size64"><a href="#gpu_size64"><code>gpu-size64</code></a></a></li>
</ul>
<h4><a id="method_gpu_compute_pass_encoder_end"></a><code>[method]gpu-compute-pass-encoder.end: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_compute_pass_encoder_end.self"></a><code>self</code>: borrow&lt;<a href="#gpu_compute_pass_encoder"><a href="#gpu_compute_pass_encoder"><code>gpu-compute-pass-encoder</code></a></a>&gt;</li>
</ul>
<h4><a id="method_gpu_compute_pass_encoder_label"></a><code>[method]gpu-compute-pass-encoder.label: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_compute_pass_encoder_label.self"></a><code>self</code>: borrow&lt;<a href="#gpu_compute_pass_encoder"><a href="#gpu_compute_pass_encoder"><code>gpu-compute-pass-encoder</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_gpu_compute_pass_encoder_label.0"></a> <code>string</code></li>
</ul>
<h4><a id="method_gpu_compute_pass_encoder_set_label"></a><code>[method]gpu-compute-pass-encoder.set-label: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_compute_pass_encoder_set_label.self"></a><code>self</code>: borrow&lt;<a href="#gpu_compute_pass_encoder"><a href="#gpu_compute_pass_encoder"><code>gpu-compute-pass-encoder</code></a></a>&gt;</li>
<li><a id="method_gpu_compute_pass_encoder_set_label.label"></a><code>label</code>: <code>string</code></li>
</ul>
<h4><a id="method_gpu_compute_pass_encoder_push_debug_group"></a><code>[method]gpu-compute-pass-encoder.push-debug-group: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_compute_pass_encoder_push_debug_group.self"></a><code>self</code>: borrow&lt;<a href="#gpu_compute_pass_encoder"><a href="#gpu_compute_pass_encoder"><code>gpu-compute-pass-encoder</code></a></a>&gt;</li>
<li><a id="method_gpu_compute_pass_encoder_push_debug_group.group_label"></a><code>group-label</code>: <code>string</code></li>
</ul>
<h4><a id="method_gpu_compute_pass_encoder_pop_debug_group"></a><code>[method]gpu-compute-pass-encoder.pop-debug-group: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_compute_pass_encoder_pop_debug_group.self"></a><code>self</code>: borrow&lt;<a href="#gpu_compute_pass_encoder"><a href="#gpu_compute_pass_encoder"><code>gpu-compute-pass-encoder</code></a></a>&gt;</li>
</ul>
<h4><a id="method_gpu_compute_pass_encoder_insert_debug_marker"></a><code>[method]gpu-compute-pass-encoder.insert-debug-marker: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_compute_pass_encoder_insert_debug_marker.self"></a><code>self</code>: borrow&lt;<a href="#gpu_compute_pass_encoder"><a href="#gpu_compute_pass_encoder"><code>gpu-compute-pass-encoder</code></a></a>&gt;</li>
<li><a id="method_gpu_compute_pass_encoder_insert_debug_marker.marker_label"></a><code>marker-label</code>: <code>string</code></li>
</ul>
<h4><a id="method_gpu_compute_pass_encoder_set_bind_group"></a><code>[method]gpu-compute-pass-encoder.set-bind-group: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_compute_pass_encoder_set_bind_group.self"></a><code>self</code>: borrow&lt;<a href="#gpu_compute_pass_encoder"><a href="#gpu_compute_pass_encoder"><code>gpu-compute-pass-encoder</code></a></a>&gt;</li>
<li><a id="method_gpu_compute_pass_encoder_set_bind_group.index"></a><code>index</code>: <a href="#gpu_index32"><a href="#gpu_index32"><code>gpu-index32</code></a></a></li>
<li><a id="method_gpu_compute_pass_encoder_set_bind_group.bind_group"></a><code>bind-group</code>: option&lt;borrow&lt;<a href="#gpu_bind_group"><a href="#gpu_bind_group"><code>gpu-bind-group</code></a></a>&gt;&gt;</li>
<li><a id="method_gpu_compute_pass_encoder_set_bind_group.dynamic_offsets"></a><code>dynamic-offsets</code>: option&lt;list&lt;<a href="#gpu_buffer_dynamic_offset"><a href="#gpu_buffer_dynamic_offset"><code>gpu-buffer-dynamic-offset</code></a></a>&gt;&gt;</li>
</ul>
<h4><a id="method_gpu_render_pass_encoder_set_viewport"></a><code>[method]gpu-render-pass-encoder.set-viewport: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_render_pass_encoder_set_viewport.self"></a><code>self</code>: borrow&lt;<a href="#gpu_render_pass_encoder"><a href="#gpu_render_pass_encoder"><code>gpu-render-pass-encoder</code></a></a>&gt;</li>
<li><a id="method_gpu_render_pass_encoder_set_viewport.x"></a><code>x</code>: <code>f32</code></li>
<li><a id="method_gpu_render_pass_encoder_set_viewport.y"></a><code>y</code>: <code>f32</code></li>
<li><a id="method_gpu_render_pass_encoder_set_viewport.width"></a><code>width</code>: <code>f32</code></li>
<li><a id="method_gpu_render_pass_encoder_set_viewport.height"></a><code>height</code>: <code>f32</code></li>
<li><a id="method_gpu_render_pass_encoder_set_viewport.min_depth"></a><code>min-depth</code>: <code>f32</code></li>
<li><a id="method_gpu_render_pass_encoder_set_viewport.max_depth"></a><code>max-depth</code>: <code>f32</code></li>
</ul>
<h4><a id="method_gpu_render_pass_encoder_set_scissor_rect"></a><code>[method]gpu-render-pass-encoder.set-scissor-rect: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_render_pass_encoder_set_scissor_rect.self"></a><code>self</code>: borrow&lt;<a href="#gpu_render_pass_encoder"><a href="#gpu_render_pass_encoder"><code>gpu-render-pass-encoder</code></a></a>&gt;</li>
<li><a id="method_gpu_render_pass_encoder_set_scissor_rect.x"></a><code>x</code>: <a href="#gpu_integer_coordinate"><a href="#gpu_integer_coordinate"><code>gpu-integer-coordinate</code></a></a></li>
<li><a id="method_gpu_render_pass_encoder_set_scissor_rect.y"></a><code>y</code>: <a href="#gpu_integer_coordinate"><a href="#gpu_integer_coordinate"><code>gpu-integer-coordinate</code></a></a></li>
<li><a id="method_gpu_render_pass_encoder_set_scissor_rect.width"></a><code>width</code>: <a href="#gpu_integer_coordinate"><a href="#gpu_integer_coordinate"><code>gpu-integer-coordinate</code></a></a></li>
<li><a id="method_gpu_render_pass_encoder_set_scissor_rect.height"></a><code>height</code>: <a href="#gpu_integer_coordinate"><a href="#gpu_integer_coordinate"><code>gpu-integer-coordinate</code></a></a></li>
</ul>
<h4><a id="method_gpu_render_pass_encoder_set_blend_constant"></a><code>[method]gpu-render-pass-encoder.set-blend-constant: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_render_pass_encoder_set_blend_constant.self"></a><code>self</code>: borrow&lt;<a href="#gpu_render_pass_encoder"><a href="#gpu_render_pass_encoder"><code>gpu-render-pass-encoder</code></a></a>&gt;</li>
<li><a id="method_gpu_render_pass_encoder_set_blend_constant.color"></a><code>color</code>: <a href="#gpu_color"><a href="#gpu_color"><code>gpu-color</code></a></a></li>
</ul>
<h4><a id="method_gpu_render_pass_encoder_set_stencil_reference"></a><code>[method]gpu-render-pass-encoder.set-stencil-reference: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_render_pass_encoder_set_stencil_reference.self"></a><code>self</code>: borrow&lt;<a href="#gpu_render_pass_encoder"><a href="#gpu_render_pass_encoder"><code>gpu-render-pass-encoder</code></a></a>&gt;</li>
<li><a id="method_gpu_render_pass_encoder_set_stencil_reference.reference"></a><code>reference</code>: <a href="#gpu_stencil_value"><a href="#gpu_stencil_value"><code>gpu-stencil-value</code></a></a></li>
</ul>
<h4><a id="method_gpu_render_pass_encoder_begin_occlusion_query"></a><code>[method]gpu-render-pass-encoder.begin-occlusion-query: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_render_pass_encoder_begin_occlusion_query.self"></a><code>self</code>: borrow&lt;<a href="#gpu_render_pass_encoder"><a href="#gpu_render_pass_encoder"><code>gpu-render-pass-encoder</code></a></a>&gt;</li>
<li><a id="method_gpu_render_pass_encoder_begin_occlusion_query.query_index"></a><code>query-index</code>: <a href="#gpu_size32"><a href="#gpu_size32"><code>gpu-size32</code></a></a></li>
</ul>
<h4><a id="method_gpu_render_pass_encoder_end_occlusion_query"></a><code>[method]gpu-render-pass-encoder.end-occlusion-query: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_render_pass_encoder_end_occlusion_query.self"></a><code>self</code>: borrow&lt;<a href="#gpu_render_pass_encoder"><a href="#gpu_render_pass_encoder"><code>gpu-render-pass-encoder</code></a></a>&gt;</li>
</ul>
<h4><a id="method_gpu_render_pass_encoder_execute_bundles"></a><code>[method]gpu-render-pass-encoder.execute-bundles: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_render_pass_encoder_execute_bundles.self"></a><code>self</code>: borrow&lt;<a href="#gpu_render_pass_encoder"><a href="#gpu_render_pass_encoder"><code>gpu-render-pass-encoder</code></a></a>&gt;</li>
<li><a id="method_gpu_render_pass_encoder_execute_bundles.bundles"></a><code>bundles</code>: list&lt;borrow&lt;<a href="#gpu_render_bundle"><a href="#gpu_render_bundle"><code>gpu-render-bundle</code></a></a>&gt;&gt;</li>
</ul>
<h4><a id="method_gpu_render_pass_encoder_end"></a><code>[method]gpu-render-pass-encoder.end: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_render_pass_encoder_end.self"></a><code>self</code>: borrow&lt;<a href="#gpu_render_pass_encoder"><a href="#gpu_render_pass_encoder"><code>gpu-render-pass-encoder</code></a></a>&gt;</li>
</ul>
<h4><a id="method_gpu_render_pass_encoder_label"></a><code>[method]gpu-render-pass-encoder.label: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_render_pass_encoder_label.self"></a><code>self</code>: borrow&lt;<a href="#gpu_render_pass_encoder"><a href="#gpu_render_pass_encoder"><code>gpu-render-pass-encoder</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_gpu_render_pass_encoder_label.0"></a> <code>string</code></li>
</ul>
<h4><a id="method_gpu_render_pass_encoder_set_label"></a><code>[method]gpu-render-pass-encoder.set-label: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_render_pass_encoder_set_label.self"></a><code>self</code>: borrow&lt;<a href="#gpu_render_pass_encoder"><a href="#gpu_render_pass_encoder"><code>gpu-render-pass-encoder</code></a></a>&gt;</li>
<li><a id="method_gpu_render_pass_encoder_set_label.label"></a><code>label</code>: <code>string</code></li>
</ul>
<h4><a id="method_gpu_render_pass_encoder_push_debug_group"></a><code>[method]gpu-render-pass-encoder.push-debug-group: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_render_pass_encoder_push_debug_group.self"></a><code>self</code>: borrow&lt;<a href="#gpu_render_pass_encoder"><a href="#gpu_render_pass_encoder"><code>gpu-render-pass-encoder</code></a></a>&gt;</li>
<li><a id="method_gpu_render_pass_encoder_push_debug_group.group_label"></a><code>group-label</code>: <code>string</code></li>
</ul>
<h4><a id="method_gpu_render_pass_encoder_pop_debug_group"></a><code>[method]gpu-render-pass-encoder.pop-debug-group: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_render_pass_encoder_pop_debug_group.self"></a><code>self</code>: borrow&lt;<a href="#gpu_render_pass_encoder"><a href="#gpu_render_pass_encoder"><code>gpu-render-pass-encoder</code></a></a>&gt;</li>
</ul>
<h4><a id="method_gpu_render_pass_encoder_insert_debug_marker"></a><code>[method]gpu-render-pass-encoder.insert-debug-marker: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_render_pass_encoder_insert_debug_marker.self"></a><code>self</code>: borrow&lt;<a href="#gpu_render_pass_encoder"><a href="#gpu_render_pass_encoder"><code>gpu-render-pass-encoder</code></a></a>&gt;</li>
<li><a id="method_gpu_render_pass_encoder_insert_debug_marker.marker_label"></a><code>marker-label</code>: <code>string</code></li>
</ul>
<h4><a id="method_gpu_render_pass_encoder_set_bind_group"></a><code>[method]gpu-render-pass-encoder.set-bind-group: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_render_pass_encoder_set_bind_group.self"></a><code>self</code>: borrow&lt;<a href="#gpu_render_pass_encoder"><a href="#gpu_render_pass_encoder"><code>gpu-render-pass-encoder</code></a></a>&gt;</li>
<li><a id="method_gpu_render_pass_encoder_set_bind_group.index"></a><code>index</code>: <a href="#gpu_index32"><a href="#gpu_index32"><code>gpu-index32</code></a></a></li>
<li><a id="method_gpu_render_pass_encoder_set_bind_group.bind_group"></a><code>bind-group</code>: option&lt;borrow&lt;<a href="#gpu_bind_group"><a href="#gpu_bind_group"><code>gpu-bind-group</code></a></a>&gt;&gt;</li>
<li><a id="method_gpu_render_pass_encoder_set_bind_group.dynamic_offsets"></a><code>dynamic-offsets</code>: option&lt;list&lt;<a href="#gpu_buffer_dynamic_offset"><a href="#gpu_buffer_dynamic_offset"><code>gpu-buffer-dynamic-offset</code></a></a>&gt;&gt;</li>
</ul>
<h4><a id="method_gpu_render_pass_encoder_set_pipeline"></a><code>[method]gpu-render-pass-encoder.set-pipeline: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_render_pass_encoder_set_pipeline.self"></a><code>self</code>: borrow&lt;<a href="#gpu_render_pass_encoder"><a href="#gpu_render_pass_encoder"><code>gpu-render-pass-encoder</code></a></a>&gt;</li>
<li><a id="method_gpu_render_pass_encoder_set_pipeline.pipeline"></a><code>pipeline</code>: borrow&lt;<a href="#gpu_render_pipeline"><a href="#gpu_render_pipeline"><code>gpu-render-pipeline</code></a></a>&gt;</li>
</ul>
<h4><a id="method_gpu_render_pass_encoder_set_index_buffer"></a><code>[method]gpu-render-pass-encoder.set-index-buffer: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_render_pass_encoder_set_index_buffer.self"></a><code>self</code>: borrow&lt;<a href="#gpu_render_pass_encoder"><a href="#gpu_render_pass_encoder"><code>gpu-render-pass-encoder</code></a></a>&gt;</li>
<li><a id="method_gpu_render_pass_encoder_set_index_buffer.buffer"></a><code>buffer</code>: borrow&lt;<a href="#gpu_buffer"><a href="#gpu_buffer"><code>gpu-buffer</code></a></a>&gt;</li>
<li><a id="method_gpu_render_pass_encoder_set_index_buffer.index_format"></a><code>index-format</code>: <a href="#gpu_index_format"><a href="#gpu_index_format"><code>gpu-index-format</code></a></a></li>
<li><a id="method_gpu_render_pass_encoder_set_index_buffer.offset"></a><code>offset</code>: option&lt;<a href="#gpu_size64"><a href="#gpu_size64"><code>gpu-size64</code></a></a>&gt;</li>
<li><a id="method_gpu_render_pass_encoder_set_index_buffer.size"></a><code>size</code>: option&lt;<a href="#gpu_size64"><a href="#gpu_size64"><code>gpu-size64</code></a></a>&gt;</li>
</ul>
<h4><a id="method_gpu_render_pass_encoder_set_vertex_buffer"></a><code>[method]gpu-render-pass-encoder.set-vertex-buffer: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_render_pass_encoder_set_vertex_buffer.self"></a><code>self</code>: borrow&lt;<a href="#gpu_render_pass_encoder"><a href="#gpu_render_pass_encoder"><code>gpu-render-pass-encoder</code></a></a>&gt;</li>
<li><a id="method_gpu_render_pass_encoder_set_vertex_buffer.slot"></a><code>slot</code>: <a href="#gpu_index32"><a href="#gpu_index32"><code>gpu-index32</code></a></a></li>
<li><a id="method_gpu_render_pass_encoder_set_vertex_buffer.buffer"></a><code>buffer</code>: option&lt;borrow&lt;<a href="#gpu_buffer"><a href="#gpu_buffer"><code>gpu-buffer</code></a></a>&gt;&gt;</li>
<li><a id="method_gpu_render_pass_encoder_set_vertex_buffer.offset"></a><code>offset</code>: option&lt;<a href="#gpu_size64"><a href="#gpu_size64"><code>gpu-size64</code></a></a>&gt;</li>
<li><a id="method_gpu_render_pass_encoder_set_vertex_buffer.size"></a><code>size</code>: option&lt;<a href="#gpu_size64"><a href="#gpu_size64"><code>gpu-size64</code></a></a>&gt;</li>
</ul>
<h4><a id="method_gpu_render_pass_encoder_draw"></a><code>[method]gpu-render-pass-encoder.draw: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_render_pass_encoder_draw.self"></a><code>self</code>: borrow&lt;<a href="#gpu_render_pass_encoder"><a href="#gpu_render_pass_encoder"><code>gpu-render-pass-encoder</code></a></a>&gt;</li>
<li><a id="method_gpu_render_pass_encoder_draw.vertex_count"></a><code>vertex-count</code>: <a href="#gpu_size32"><a href="#gpu_size32"><code>gpu-size32</code></a></a></li>
<li><a id="method_gpu_render_pass_encoder_draw.instance_count"></a><code>instance-count</code>: option&lt;<a href="#gpu_size32"><a href="#gpu_size32"><code>gpu-size32</code></a></a>&gt;</li>
<li><a id="method_gpu_render_pass_encoder_draw.first_vertex"></a><code>first-vertex</code>: option&lt;<a href="#gpu_size32"><a href="#gpu_size32"><code>gpu-size32</code></a></a>&gt;</li>
<li><a id="method_gpu_render_pass_encoder_draw.first_instance"></a><code>first-instance</code>: option&lt;<a href="#gpu_size32"><a href="#gpu_size32"><code>gpu-size32</code></a></a>&gt;</li>
</ul>
<h4><a id="method_gpu_render_pass_encoder_draw_indexed"></a><code>[method]gpu-render-pass-encoder.draw-indexed: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_render_pass_encoder_draw_indexed.self"></a><code>self</code>: borrow&lt;<a href="#gpu_render_pass_encoder"><a href="#gpu_render_pass_encoder"><code>gpu-render-pass-encoder</code></a></a>&gt;</li>
<li><a id="method_gpu_render_pass_encoder_draw_indexed.index_count"></a><code>index-count</code>: <a href="#gpu_size32"><a href="#gpu_size32"><code>gpu-size32</code></a></a></li>
<li><a id="method_gpu_render_pass_encoder_draw_indexed.instance_count"></a><code>instance-count</code>: option&lt;<a href="#gpu_size32"><a href="#gpu_size32"><code>gpu-size32</code></a></a>&gt;</li>
<li><a id="method_gpu_render_pass_encoder_draw_indexed.first_index"></a><code>first-index</code>: option&lt;<a href="#gpu_size32"><a href="#gpu_size32"><code>gpu-size32</code></a></a>&gt;</li>
<li><a id="method_gpu_render_pass_encoder_draw_indexed.base_vertex"></a><code>base-vertex</code>: option&lt;<a href="#gpu_signed_offset32"><a href="#gpu_signed_offset32"><code>gpu-signed-offset32</code></a></a>&gt;</li>
<li><a id="method_gpu_render_pass_encoder_draw_indexed.first_instance"></a><code>first-instance</code>: option&lt;<a href="#gpu_size32"><a href="#gpu_size32"><code>gpu-size32</code></a></a>&gt;</li>
</ul>
<h4><a id="method_gpu_render_pass_encoder_draw_indirect"></a><code>[method]gpu-render-pass-encoder.draw-indirect: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_render_pass_encoder_draw_indirect.self"></a><code>self</code>: borrow&lt;<a href="#gpu_render_pass_encoder"><a href="#gpu_render_pass_encoder"><code>gpu-render-pass-encoder</code></a></a>&gt;</li>
<li><a id="method_gpu_render_pass_encoder_draw_indirect.indirect_buffer"></a><code>indirect-buffer</code>: borrow&lt;<a href="#gpu_buffer"><a href="#gpu_buffer"><code>gpu-buffer</code></a></a>&gt;</li>
<li><a id="method_gpu_render_pass_encoder_draw_indirect.indirect_offset"></a><code>indirect-offset</code>: <a href="#gpu_size64"><a href="#gpu_size64"><code>gpu-size64</code></a></a></li>
</ul>
<h4><a id="method_gpu_render_pass_encoder_draw_indexed_indirect"></a><code>[method]gpu-render-pass-encoder.draw-indexed-indirect: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_render_pass_encoder_draw_indexed_indirect.self"></a><code>self</code>: borrow&lt;<a href="#gpu_render_pass_encoder"><a href="#gpu_render_pass_encoder"><code>gpu-render-pass-encoder</code></a></a>&gt;</li>
<li><a id="method_gpu_render_pass_encoder_draw_indexed_indirect.indirect_buffer"></a><code>indirect-buffer</code>: borrow&lt;<a href="#gpu_buffer"><a href="#gpu_buffer"><code>gpu-buffer</code></a></a>&gt;</li>
<li><a id="method_gpu_render_pass_encoder_draw_indexed_indirect.indirect_offset"></a><code>indirect-offset</code>: <a href="#gpu_size64"><a href="#gpu_size64"><code>gpu-size64</code></a></a></li>
</ul>
<h4><a id="method_gpu_render_bundle_label"></a><code>[method]gpu-render-bundle.label: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_render_bundle_label.self"></a><code>self</code>: borrow&lt;<a href="#gpu_render_bundle"><a href="#gpu_render_bundle"><code>gpu-render-bundle</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_gpu_render_bundle_label.0"></a> <code>string</code></li>
</ul>
<h4><a id="method_gpu_render_bundle_set_label"></a><code>[method]gpu-render-bundle.set-label: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_render_bundle_set_label.self"></a><code>self</code>: borrow&lt;<a href="#gpu_render_bundle"><a href="#gpu_render_bundle"><code>gpu-render-bundle</code></a></a>&gt;</li>
<li><a id="method_gpu_render_bundle_set_label.label"></a><code>label</code>: <code>string</code></li>
</ul>
<h4><a id="method_gpu_render_bundle_encoder_finish"></a><code>[method]gpu-render-bundle-encoder.finish: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_render_bundle_encoder_finish.self"></a><code>self</code>: borrow&lt;<a href="#gpu_render_bundle_encoder"><a href="#gpu_render_bundle_encoder"><code>gpu-render-bundle-encoder</code></a></a>&gt;</li>
<li><a id="method_gpu_render_bundle_encoder_finish.descriptor"></a><code>descriptor</code>: option&lt;<a href="#gpu_render_bundle_descriptor"><a href="#gpu_render_bundle_descriptor"><code>gpu-render-bundle-descriptor</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_gpu_render_bundle_encoder_finish.0"></a> own&lt;<a href="#gpu_render_bundle"><a href="#gpu_render_bundle"><code>gpu-render-bundle</code></a></a>&gt;</li>
</ul>
<h4><a id="method_gpu_render_bundle_encoder_label"></a><code>[method]gpu-render-bundle-encoder.label: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_render_bundle_encoder_label.self"></a><code>self</code>: borrow&lt;<a href="#gpu_render_bundle_encoder"><a href="#gpu_render_bundle_encoder"><code>gpu-render-bundle-encoder</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_gpu_render_bundle_encoder_label.0"></a> <code>string</code></li>
</ul>
<h4><a id="method_gpu_render_bundle_encoder_set_label"></a><code>[method]gpu-render-bundle-encoder.set-label: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_render_bundle_encoder_set_label.self"></a><code>self</code>: borrow&lt;<a href="#gpu_render_bundle_encoder"><a href="#gpu_render_bundle_encoder"><code>gpu-render-bundle-encoder</code></a></a>&gt;</li>
<li><a id="method_gpu_render_bundle_encoder_set_label.label"></a><code>label</code>: <code>string</code></li>
</ul>
<h4><a id="method_gpu_render_bundle_encoder_push_debug_group"></a><code>[method]gpu-render-bundle-encoder.push-debug-group: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_render_bundle_encoder_push_debug_group.self"></a><code>self</code>: borrow&lt;<a href="#gpu_render_bundle_encoder"><a href="#gpu_render_bundle_encoder"><code>gpu-render-bundle-encoder</code></a></a>&gt;</li>
<li><a id="method_gpu_render_bundle_encoder_push_debug_group.group_label"></a><code>group-label</code>: <code>string</code></li>
</ul>
<h4><a id="method_gpu_render_bundle_encoder_pop_debug_group"></a><code>[method]gpu-render-bundle-encoder.pop-debug-group: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_render_bundle_encoder_pop_debug_group.self"></a><code>self</code>: borrow&lt;<a href="#gpu_render_bundle_encoder"><a href="#gpu_render_bundle_encoder"><code>gpu-render-bundle-encoder</code></a></a>&gt;</li>
</ul>
<h4><a id="method_gpu_render_bundle_encoder_insert_debug_marker"></a><code>[method]gpu-render-bundle-encoder.insert-debug-marker: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_render_bundle_encoder_insert_debug_marker.self"></a><code>self</code>: borrow&lt;<a href="#gpu_render_bundle_encoder"><a href="#gpu_render_bundle_encoder"><code>gpu-render-bundle-encoder</code></a></a>&gt;</li>
<li><a id="method_gpu_render_bundle_encoder_insert_debug_marker.marker_label"></a><code>marker-label</code>: <code>string</code></li>
</ul>
<h4><a id="method_gpu_render_bundle_encoder_set_bind_group"></a><code>[method]gpu-render-bundle-encoder.set-bind-group: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_render_bundle_encoder_set_bind_group.self"></a><code>self</code>: borrow&lt;<a href="#gpu_render_bundle_encoder"><a href="#gpu_render_bundle_encoder"><code>gpu-render-bundle-encoder</code></a></a>&gt;</li>
<li><a id="method_gpu_render_bundle_encoder_set_bind_group.index"></a><code>index</code>: <a href="#gpu_index32"><a href="#gpu_index32"><code>gpu-index32</code></a></a></li>
<li><a id="method_gpu_render_bundle_encoder_set_bind_group.bind_group"></a><code>bind-group</code>: option&lt;borrow&lt;<a href="#gpu_bind_group"><a href="#gpu_bind_group"><code>gpu-bind-group</code></a></a>&gt;&gt;</li>
<li><a id="method_gpu_render_bundle_encoder_set_bind_group.dynamic_offsets"></a><code>dynamic-offsets</code>: option&lt;list&lt;<a href="#gpu_buffer_dynamic_offset"><a href="#gpu_buffer_dynamic_offset"><code>gpu-buffer-dynamic-offset</code></a></a>&gt;&gt;</li>
</ul>
<h4><a id="method_gpu_render_bundle_encoder_set_pipeline"></a><code>[method]gpu-render-bundle-encoder.set-pipeline: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_render_bundle_encoder_set_pipeline.self"></a><code>self</code>: borrow&lt;<a href="#gpu_render_bundle_encoder"><a href="#gpu_render_bundle_encoder"><code>gpu-render-bundle-encoder</code></a></a>&gt;</li>
<li><a id="method_gpu_render_bundle_encoder_set_pipeline.pipeline"></a><code>pipeline</code>: borrow&lt;<a href="#gpu_render_pipeline"><a href="#gpu_render_pipeline"><code>gpu-render-pipeline</code></a></a>&gt;</li>
</ul>
<h4><a id="method_gpu_render_bundle_encoder_set_index_buffer"></a><code>[method]gpu-render-bundle-encoder.set-index-buffer: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_render_bundle_encoder_set_index_buffer.self"></a><code>self</code>: borrow&lt;<a href="#gpu_render_bundle_encoder"><a href="#gpu_render_bundle_encoder"><code>gpu-render-bundle-encoder</code></a></a>&gt;</li>
<li><a id="method_gpu_render_bundle_encoder_set_index_buffer.buffer"></a><code>buffer</code>: borrow&lt;<a href="#gpu_buffer"><a href="#gpu_buffer"><code>gpu-buffer</code></a></a>&gt;</li>
<li><a id="method_gpu_render_bundle_encoder_set_index_buffer.index_format"></a><code>index-format</code>: <a href="#gpu_index_format"><a href="#gpu_index_format"><code>gpu-index-format</code></a></a></li>
<li><a id="method_gpu_render_bundle_encoder_set_index_buffer.offset"></a><code>offset</code>: option&lt;<a href="#gpu_size64"><a href="#gpu_size64"><code>gpu-size64</code></a></a>&gt;</li>
<li><a id="method_gpu_render_bundle_encoder_set_index_buffer.size"></a><code>size</code>: option&lt;<a href="#gpu_size64"><a href="#gpu_size64"><code>gpu-size64</code></a></a>&gt;</li>
</ul>
<h4><a id="method_gpu_render_bundle_encoder_set_vertex_buffer"></a><code>[method]gpu-render-bundle-encoder.set-vertex-buffer: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_render_bundle_encoder_set_vertex_buffer.self"></a><code>self</code>: borrow&lt;<a href="#gpu_render_bundle_encoder"><a href="#gpu_render_bundle_encoder"><code>gpu-render-bundle-encoder</code></a></a>&gt;</li>
<li><a id="method_gpu_render_bundle_encoder_set_vertex_buffer.slot"></a><code>slot</code>: <a href="#gpu_index32"><a href="#gpu_index32"><code>gpu-index32</code></a></a></li>
<li><a id="method_gpu_render_bundle_encoder_set_vertex_buffer.buffer"></a><code>buffer</code>: option&lt;borrow&lt;<a href="#gpu_buffer"><a href="#gpu_buffer"><code>gpu-buffer</code></a></a>&gt;&gt;</li>
<li><a id="method_gpu_render_bundle_encoder_set_vertex_buffer.offset"></a><code>offset</code>: option&lt;<a href="#gpu_size64"><a href="#gpu_size64"><code>gpu-size64</code></a></a>&gt;</li>
<li><a id="method_gpu_render_bundle_encoder_set_vertex_buffer.size"></a><code>size</code>: option&lt;<a href="#gpu_size64"><a href="#gpu_size64"><code>gpu-size64</code></a></a>&gt;</li>
</ul>
<h4><a id="method_gpu_render_bundle_encoder_draw"></a><code>[method]gpu-render-bundle-encoder.draw: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_render_bundle_encoder_draw.self"></a><code>self</code>: borrow&lt;<a href="#gpu_render_bundle_encoder"><a href="#gpu_render_bundle_encoder"><code>gpu-render-bundle-encoder</code></a></a>&gt;</li>
<li><a id="method_gpu_render_bundle_encoder_draw.vertex_count"></a><code>vertex-count</code>: <a href="#gpu_size32"><a href="#gpu_size32"><code>gpu-size32</code></a></a></li>
<li><a id="method_gpu_render_bundle_encoder_draw.instance_count"></a><code>instance-count</code>: option&lt;<a href="#gpu_size32"><a href="#gpu_size32"><code>gpu-size32</code></a></a>&gt;</li>
<li><a id="method_gpu_render_bundle_encoder_draw.first_vertex"></a><code>first-vertex</code>: option&lt;<a href="#gpu_size32"><a href="#gpu_size32"><code>gpu-size32</code></a></a>&gt;</li>
<li><a id="method_gpu_render_bundle_encoder_draw.first_instance"></a><code>first-instance</code>: option&lt;<a href="#gpu_size32"><a href="#gpu_size32"><code>gpu-size32</code></a></a>&gt;</li>
</ul>
<h4><a id="method_gpu_render_bundle_encoder_draw_indexed"></a><code>[method]gpu-render-bundle-encoder.draw-indexed: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_render_bundle_encoder_draw_indexed.self"></a><code>self</code>: borrow&lt;<a href="#gpu_render_bundle_encoder"><a href="#gpu_render_bundle_encoder"><code>gpu-render-bundle-encoder</code></a></a>&gt;</li>
<li><a id="method_gpu_render_bundle_encoder_draw_indexed.index_count"></a><code>index-count</code>: <a href="#gpu_size32"><a href="#gpu_size32"><code>gpu-size32</code></a></a></li>
<li><a id="method_gpu_render_bundle_encoder_draw_indexed.instance_count"></a><code>instance-count</code>: option&lt;<a href="#gpu_size32"><a href="#gpu_size32"><code>gpu-size32</code></a></a>&gt;</li>
<li><a id="method_gpu_render_bundle_encoder_draw_indexed.first_index"></a><code>first-index</code>: option&lt;<a href="#gpu_size32"><a href="#gpu_size32"><code>gpu-size32</code></a></a>&gt;</li>
<li><a id="method_gpu_render_bundle_encoder_draw_indexed.base_vertex"></a><code>base-vertex</code>: option&lt;<a href="#gpu_signed_offset32"><a href="#gpu_signed_offset32"><code>gpu-signed-offset32</code></a></a>&gt;</li>
<li><a id="method_gpu_render_bundle_encoder_draw_indexed.first_instance"></a><code>first-instance</code>: option&lt;<a href="#gpu_size32"><a href="#gpu_size32"><code>gpu-size32</code></a></a>&gt;</li>
</ul>
<h4><a id="method_gpu_render_bundle_encoder_draw_indirect"></a><code>[method]gpu-render-bundle-encoder.draw-indirect: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_render_bundle_encoder_draw_indirect.self"></a><code>self</code>: borrow&lt;<a href="#gpu_render_bundle_encoder"><a href="#gpu_render_bundle_encoder"><code>gpu-render-bundle-encoder</code></a></a>&gt;</li>
<li><a id="method_gpu_render_bundle_encoder_draw_indirect.indirect_buffer"></a><code>indirect-buffer</code>: borrow&lt;<a href="#gpu_buffer"><a href="#gpu_buffer"><code>gpu-buffer</code></a></a>&gt;</li>
<li><a id="method_gpu_render_bundle_encoder_draw_indirect.indirect_offset"></a><code>indirect-offset</code>: <a href="#gpu_size64"><a href="#gpu_size64"><code>gpu-size64</code></a></a></li>
</ul>
<h4><a id="method_gpu_render_bundle_encoder_draw_indexed_indirect"></a><code>[method]gpu-render-bundle-encoder.draw-indexed-indirect: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_render_bundle_encoder_draw_indexed_indirect.self"></a><code>self</code>: borrow&lt;<a href="#gpu_render_bundle_encoder"><a href="#gpu_render_bundle_encoder"><code>gpu-render-bundle-encoder</code></a></a>&gt;</li>
<li><a id="method_gpu_render_bundle_encoder_draw_indexed_indirect.indirect_buffer"></a><code>indirect-buffer</code>: borrow&lt;<a href="#gpu_buffer"><a href="#gpu_buffer"><code>gpu-buffer</code></a></a>&gt;</li>
<li><a id="method_gpu_render_bundle_encoder_draw_indexed_indirect.indirect_offset"></a><code>indirect-offset</code>: <a href="#gpu_size64"><a href="#gpu_size64"><code>gpu-size64</code></a></a></li>
</ul>
<h4><a id="method_gpu_queue_submit"></a><code>[method]gpu-queue.submit: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_queue_submit.self"></a><code>self</code>: borrow&lt;<a href="#gpu_queue"><a href="#gpu_queue"><code>gpu-queue</code></a></a>&gt;</li>
<li><a id="method_gpu_queue_submit.command_buffers"></a><code>command-buffers</code>: list&lt;borrow&lt;<a href="#gpu_command_buffer"><a href="#gpu_command_buffer"><code>gpu-command-buffer</code></a></a>&gt;&gt;</li>
</ul>
<h4><a id="method_gpu_queue_on_submitted_work_done"></a><code>[method]gpu-queue.on-submitted-work-done: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_queue_on_submitted_work_done.self"></a><code>self</code>: borrow&lt;<a href="#gpu_queue"><a href="#gpu_queue"><code>gpu-queue</code></a></a>&gt;</li>
</ul>
<h4><a id="method_gpu_queue_write_buffer"></a><code>[method]gpu-queue.write-buffer: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_queue_write_buffer.self"></a><code>self</code>: borrow&lt;<a href="#gpu_queue"><a href="#gpu_queue"><code>gpu-queue</code></a></a>&gt;</li>
<li><a id="method_gpu_queue_write_buffer.buffer"></a><code>buffer</code>: borrow&lt;<a href="#gpu_buffer"><a href="#gpu_buffer"><code>gpu-buffer</code></a></a>&gt;</li>
<li><a id="method_gpu_queue_write_buffer.buffer_offset"></a><code>buffer-offset</code>: <a href="#gpu_size64"><a href="#gpu_size64"><code>gpu-size64</code></a></a></li>
<li><a id="method_gpu_queue_write_buffer.data_offset"></a><code>data-offset</code>: option&lt;<a href="#gpu_size64"><a href="#gpu_size64"><code>gpu-size64</code></a></a>&gt;</li>
<li><a id="method_gpu_queue_write_buffer.data"></a><code>data</code>: list&lt;<code>u8</code>&gt;</li>
<li><a id="method_gpu_queue_write_buffer.size"></a><code>size</code>: option&lt;<a href="#gpu_size64"><a href="#gpu_size64"><code>gpu-size64</code></a></a>&gt;</li>
</ul>
<h4><a id="method_gpu_queue_write_texture"></a><code>[method]gpu-queue.write-texture: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_queue_write_texture.self"></a><code>self</code>: borrow&lt;<a href="#gpu_queue"><a href="#gpu_queue"><code>gpu-queue</code></a></a>&gt;</li>
<li><a id="method_gpu_queue_write_texture.destination"></a><code>destination</code>: <a href="#gpu_image_copy_texture"><a href="#gpu_image_copy_texture"><code>gpu-image-copy-texture</code></a></a></li>
<li><a id="method_gpu_queue_write_texture.data"></a><code>data</code>: list&lt;<code>u8</code>&gt;</li>
<li><a id="method_gpu_queue_write_texture.data_layout"></a><code>data-layout</code>: <a href="#gpu_image_data_layout"><a href="#gpu_image_data_layout"><code>gpu-image-data-layout</code></a></a></li>
<li><a id="method_gpu_queue_write_texture.size"></a><code>size</code>: <a href="#gpu_extent3_d"><a href="#gpu_extent3_d"><code>gpu-extent3-d</code></a></a></li>
</ul>
<h4><a id="method_gpu_queue_label"></a><code>[method]gpu-queue.label: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_queue_label.self"></a><code>self</code>: borrow&lt;<a href="#gpu_queue"><a href="#gpu_queue"><code>gpu-queue</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_gpu_queue_label.0"></a> <code>string</code></li>
</ul>
<h4><a id="method_gpu_queue_set_label"></a><code>[method]gpu-queue.set-label: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_queue_set_label.self"></a><code>self</code>: borrow&lt;<a href="#gpu_queue"><a href="#gpu_queue"><code>gpu-queue</code></a></a>&gt;</li>
<li><a id="method_gpu_queue_set_label.label"></a><code>label</code>: <code>string</code></li>
</ul>
<h4><a id="method_gpu_query_set_destroy"></a><code>[method]gpu-query-set.destroy: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_query_set_destroy.self"></a><code>self</code>: borrow&lt;<a href="#gpu_query_set"><a href="#gpu_query_set"><code>gpu-query-set</code></a></a>&gt;</li>
</ul>
<h4><a id="method_gpu_query_set_type"></a><code>[method]gpu-query-set.type: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_query_set_type.self"></a><code>self</code>: borrow&lt;<a href="#gpu_query_set"><a href="#gpu_query_set"><code>gpu-query-set</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_gpu_query_set_type.0"></a> <a href="#gpu_query_type"><a href="#gpu_query_type"><code>gpu-query-type</code></a></a></li>
</ul>
<h4><a id="method_gpu_query_set_count"></a><code>[method]gpu-query-set.count: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_query_set_count.self"></a><code>self</code>: borrow&lt;<a href="#gpu_query_set"><a href="#gpu_query_set"><code>gpu-query-set</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_gpu_query_set_count.0"></a> <a href="#gpu_size32_out"><a href="#gpu_size32_out"><code>gpu-size32-out</code></a></a></li>
</ul>
<h4><a id="method_gpu_query_set_label"></a><code>[method]gpu-query-set.label: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_query_set_label.self"></a><code>self</code>: borrow&lt;<a href="#gpu_query_set"><a href="#gpu_query_set"><code>gpu-query-set</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_gpu_query_set_label.0"></a> <code>string</code></li>
</ul>
<h4><a id="method_gpu_query_set_set_label"></a><code>[method]gpu-query-set.set-label: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_query_set_set_label.self"></a><code>self</code>: borrow&lt;<a href="#gpu_query_set"><a href="#gpu_query_set"><code>gpu-query-set</code></a></a>&gt;</li>
<li><a id="method_gpu_query_set_set_label.label"></a><code>label</code>: <code>string</code></li>
</ul>
<h4><a id="method_gpu_canvas_context_configure"></a><code>[method]gpu-canvas-context.configure: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_canvas_context_configure.self"></a><code>self</code>: borrow&lt;<a href="#gpu_canvas_context"><a href="#gpu_canvas_context"><code>gpu-canvas-context</code></a></a>&gt;</li>
<li><a id="method_gpu_canvas_context_configure.configuration"></a><code>configuration</code>: <a href="#gpu_canvas_configuration"><a href="#gpu_canvas_configuration"><code>gpu-canvas-configuration</code></a></a></li>
</ul>
<h4><a id="method_gpu_canvas_context_unconfigure"></a><code>[method]gpu-canvas-context.unconfigure: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_canvas_context_unconfigure.self"></a><code>self</code>: borrow&lt;<a href="#gpu_canvas_context"><a href="#gpu_canvas_context"><code>gpu-canvas-context</code></a></a>&gt;</li>
</ul>
<h4><a id="method_gpu_canvas_context_get_current_texture"></a><code>[method]gpu-canvas-context.get-current-texture: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_canvas_context_get_current_texture.self"></a><code>self</code>: borrow&lt;<a href="#gpu_canvas_context"><a href="#gpu_canvas_context"><code>gpu-canvas-context</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_gpu_canvas_context_get_current_texture.0"></a> own&lt;<a href="#gpu_texture"><a href="#gpu_texture"><code>gpu-texture</code></a></a>&gt;</li>
</ul>
<h4><a id="method_gpu_device_lost_info_reason"></a><code>[method]gpu-device-lost-info.reason: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_device_lost_info_reason.self"></a><code>self</code>: borrow&lt;<a href="#gpu_device_lost_info"><a href="#gpu_device_lost_info"><code>gpu-device-lost-info</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_gpu_device_lost_info_reason.0"></a> <a href="#gpu_device_lost_reason"><a href="#gpu_device_lost_reason"><code>gpu-device-lost-reason</code></a></a></li>
</ul>
<h4><a id="method_gpu_device_lost_info_message"></a><code>[method]gpu-device-lost-info.message: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_device_lost_info_message.self"></a><code>self</code>: borrow&lt;<a href="#gpu_device_lost_info"><a href="#gpu_device_lost_info"><code>gpu-device-lost-info</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_gpu_device_lost_info_message.0"></a> <code>string</code></li>
</ul>
<h4><a id="method_gpu_error_message"></a><code>[method]gpu-error.message: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_error_message.self"></a><code>self</code>: borrow&lt;<a href="#gpu_error"><a href="#gpu_error"><code>gpu-error</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_gpu_error_message.0"></a> <code>string</code></li>
</ul>
<h4><a id="method_gpu_validation_error_message"></a><code>[method]gpu-validation-error.message: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_validation_error_message.self"></a><code>self</code>: borrow&lt;<a href="#gpu_validation_error"><a href="#gpu_validation_error"><code>gpu-validation-error</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_gpu_validation_error_message.0"></a> <code>string</code></li>
</ul>
<h4><a id="constructor_gpu_validation_error"></a><code>[constructor]gpu-validation-error: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="constructor_gpu_validation_error.message"></a><code>message</code>: <code>string</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="constructor_gpu_validation_error.0"></a> own&lt;<a href="#gpu_validation_error"><a href="#gpu_validation_error"><code>gpu-validation-error</code></a></a>&gt;</li>
</ul>
<h4><a id="method_gpu_out_of_memory_error_message"></a><code>[method]gpu-out-of-memory-error.message: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_out_of_memory_error_message.self"></a><code>self</code>: borrow&lt;<a href="#gpu_out_of_memory_error"><a href="#gpu_out_of_memory_error"><code>gpu-out-of-memory-error</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_gpu_out_of_memory_error_message.0"></a> <code>string</code></li>
</ul>
<h4><a id="constructor_gpu_out_of_memory_error"></a><code>[constructor]gpu-out-of-memory-error: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="constructor_gpu_out_of_memory_error.message"></a><code>message</code>: <code>string</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="constructor_gpu_out_of_memory_error.0"></a> own&lt;<a href="#gpu_out_of_memory_error"><a href="#gpu_out_of_memory_error"><code>gpu-out-of-memory-error</code></a></a>&gt;</li>
</ul>
<h4><a id="method_gpu_internal_error_message"></a><code>[method]gpu-internal-error.message: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_internal_error_message.self"></a><code>self</code>: borrow&lt;<a href="#gpu_internal_error"><a href="#gpu_internal_error"><code>gpu-internal-error</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_gpu_internal_error_message.0"></a> <code>string</code></li>
</ul>
<h4><a id="constructor_gpu_internal_error"></a><code>[constructor]gpu-internal-error: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="constructor_gpu_internal_error.message"></a><code>message</code>: <code>string</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="constructor_gpu_internal_error.0"></a> own&lt;<a href="#gpu_internal_error"><a href="#gpu_internal_error"><code>gpu-internal-error</code></a></a>&gt;</li>
</ul>
<h4><a id="constructor_gpu_uncaptured_error_event"></a><code>[constructor]gpu-uncaptured-error-event: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="constructor_gpu_uncaptured_error_event.type"></a><code>type</code>: <code>string</code></li>
<li><a id="constructor_gpu_uncaptured_error_event.gpu_uncaptured_error_event_init_dict"></a><code>gpu-uncaptured-error-event-init-dict</code>: <a href="#gpu_uncaptured_error_event_init"><a href="#gpu_uncaptured_error_event_init"><code>gpu-uncaptured-error-event-init</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="constructor_gpu_uncaptured_error_event.0"></a> own&lt;<a href="#gpu_uncaptured_error_event"><a href="#gpu_uncaptured_error_event"><code>gpu-uncaptured-error-event</code></a></a>&gt;</li>
</ul>
<h4><a id="method_gpu_uncaptured_error_event_error"></a><code>[method]gpu-uncaptured-error-event.error: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_gpu_uncaptured_error_event_error.self"></a><code>self</code>: borrow&lt;<a href="#gpu_uncaptured_error_event"><a href="#gpu_uncaptured_error_event"><code>gpu-uncaptured-error-event</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_gpu_uncaptured_error_event_error.0"></a> own&lt;<a href="#gpu_error"><a href="#gpu_error"><code>gpu-error</code></a></a>&gt;</li>
</ul>
<h4><a id="method_non_standard_buffer_get"></a><code>[method]non-standard-buffer.get: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_non_standard_buffer_get.self"></a><code>self</code>: borrow&lt;<a href="#non_standard_buffer"><a href="#non_standard_buffer"><code>non-standard-buffer</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_non_standard_buffer_get.0"></a> list&lt;<code>u8</code>&gt;</li>
</ul>
<h4><a id="method_non_standard_buffer_set"></a><code>[method]non-standard-buffer.set: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_non_standard_buffer_set.self"></a><code>self</code>: borrow&lt;<a href="#non_standard_buffer"><a href="#non_standard_buffer"><code>non-standard-buffer</code></a></a>&gt;</li>
<li><a id="method_non_standard_buffer_set.val"></a><code>val</code>: list&lt;<code>u8</code>&gt;</li>
</ul>

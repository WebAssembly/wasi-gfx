<h1><a id="imports"></a>World imports</h1>
<ul>
<li>Imports:
<ul>
<li>interface <a href="#wasi_graphics_context_graphics_context"><code>wasi:graphics-context/graphics-context</code></a></li>
<li>interface <a href="#wasi_frame_buffer_frame_buffer"><code>wasi:frame-buffer/frame-buffer</code></a></li>
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
<h2><a id="wasi_frame_buffer_frame_buffer"></a>Import interface wasi:frame-buffer/frame-buffer</h2>
<hr />
<h3>Types</h3>
<h4><a id="context"></a><code>type context</code></h4>
<p><a href="#context"><a href="#context"><code>context</code></a></a></p>
<p>
#### <a id="abstract_buffer"></a>`type abstract-buffer`
[`abstract-buffer`](#abstract_buffer)
<p>
#### <a id="device"></a>`resource device`
<h4><a id="buffer"></a><code>resource buffer</code></h4>
<hr />
<h3>Functions</h3>
<h4><a id="constructor_device"></a><code>[constructor]device: func</code></h4>
<h5>Return values</h5>
<ul>
<li><a id="constructor_device.0"></a> own&lt;<a href="#device"><a href="#device"><code>device</code></a></a>&gt;</li>
</ul>
<h4><a id="method_device_connect_graphics_context"></a><code>[method]device.connect-graphics-context: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_device_connect_graphics_context.self"></a><code>self</code>: borrow&lt;<a href="#device"><a href="#device"><code>device</code></a></a>&gt;</li>
<li><a id="method_device_connect_graphics_context.context"></a><a href="#context"><code>context</code></a>: borrow&lt;<a href="#context"><a href="#context"><code>context</code></a></a>&gt;</li>
</ul>
<h4><a id="static_buffer_from_graphics_buffer"></a><code>[static]buffer.from-graphics-buffer: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="static_buffer_from_graphics_buffer.buffer"></a><a href="#buffer"><code>buffer</code></a>: own&lt;<a href="#abstract_buffer"><a href="#abstract_buffer"><code>abstract-buffer</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="static_buffer_from_graphics_buffer.0"></a> own&lt;<a href="#buffer"><a href="#buffer"><code>buffer</code></a></a>&gt;</li>
</ul>
<h4><a id="method_buffer_get"></a><code>[method]buffer.get: func</code></h4>
<p>TODO: This should be replcated with something that doesn't require a copy.</p>
<h5>Params</h5>
<ul>
<li><a id="method_buffer_get.self"></a><code>self</code>: borrow&lt;<a href="#buffer"><a href="#buffer"><code>buffer</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_buffer_get.0"></a> list&lt;<code>u8</code>&gt;</li>
</ul>
<h4><a id="method_buffer_set"></a><code>[method]buffer.set: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_buffer_set.self"></a><code>self</code>: borrow&lt;<a href="#buffer"><a href="#buffer"><code>buffer</code></a></a>&gt;</li>
<li><a id="method_buffer_set.val"></a><code>val</code>: list&lt;<code>u8</code>&gt;</li>
</ul>

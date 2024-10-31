<h1><a id="imports"></a>World imports</h1>
<ul>
<li>Imports:
<ul>
<li>interface <a href="#wasi_graphics_context_graphics_context"><code>wasi:graphics-context/graphics-context</code></a></li>
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

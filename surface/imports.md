<h1><a id="imports"></a>World imports</h1>
<ul>
<li>Imports:
<ul>
<li>interface <a href="#wasi_graphics_context_graphics_context"><code>wasi:graphics-context/graphics-context</code></a></li>
<li>interface <a href="#wasi_io_poll_0_2_0"><code>wasi:io/poll@0.2.0</code></a></li>
<li>interface <a href="#wasi_surface_surface"><code>wasi:surface/surface</code></a></li>
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
<h2><a id="wasi_io_poll_0_2_0"></a>Import interface wasi:io/poll@0.2.0</h2>
<p>A poll API intended to let users wait for I/O events on multiple handles
at once.</p>
<hr />
<h3>Types</h3>
<h4><a id="pollable"></a><code>resource pollable</code></h4>
<h2><a href="#pollable"><code>pollable</code></a> represents a single I/O event which may be ready, or not.</h2>
<h3>Functions</h3>
<h4><a id="method_pollable_ready"></a><code>[method]pollable.ready: func</code></h4>
<p>Return the readiness of a pollable. This function never blocks.</p>
<p>Returns <code>true</code> when the pollable is ready, and <code>false</code> otherwise.</p>
<h5>Params</h5>
<ul>
<li><a id="method_pollable_ready.self"></a><code>self</code>: borrow&lt;<a href="#pollable"><a href="#pollable"><code>pollable</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_pollable_ready.0"></a> <code>bool</code></li>
</ul>
<h4><a id="method_pollable_block"></a><code>[method]pollable.block: func</code></h4>
<p><code>block</code> returns immediately if the pollable is ready, and otherwise
blocks until ready.</p>
<p>This function is equivalent to calling <code>poll.poll</code> on a list
containing only this pollable.</p>
<h5>Params</h5>
<ul>
<li><a id="method_pollable_block.self"></a><code>self</code>: borrow&lt;<a href="#pollable"><a href="#pollable"><code>pollable</code></a></a>&gt;</li>
</ul>
<h4><a id="poll"></a><code>poll: func</code></h4>
<p>Poll for completion on a set of pollables.</p>
<p>This function takes a list of pollables, which identify I/O sources of
interest, and waits until one or more of the events is ready for I/O.</p>
<p>The result <code>list&lt;u32&gt;</code> contains one or more indices of handles in the
argument list that is ready for I/O.</p>
<p>If the list contains more elements than can be indexed with a <code>u32</code>
value, this function traps.</p>
<p>A timeout can be implemented by adding a pollable from the
wasi-clocks API to the list.</p>
<p>This function does not return a <code>result</code>; polling in itself does not
do any I/O so it doesn't fail. If any of the I/O sources identified by
the pollables has an error, it is indicated by marking the source as
being reaedy for I/O.</p>
<h5>Params</h5>
<ul>
<li><a id="poll.in"></a><code>in</code>: list&lt;borrow&lt;<a href="#pollable"><a href="#pollable"><code>pollable</code></a></a>&gt;&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="poll.0"></a> list&lt;<code>u32</code>&gt;</li>
</ul>
<h2><a id="wasi_surface_surface"></a>Import interface wasi:surface/surface</h2>
<hr />
<h3>Types</h3>
<h4><a id="context"></a><code>type context</code></h4>
<p><a href="#context"><a href="#context"><code>context</code></a></a></p>
<p>
#### <a id="pollable"></a>`type pollable`
[`pollable`](#pollable)
<p>
#### <a id="create_desc"></a>`record create-desc`
<h5>Record Fields</h5>
<ul>
<li><a id="create_desc.height"></a><code>height</code>: option&lt;<code>u32</code>&gt;</li>
<li><a id="create_desc.width"></a><code>width</code>: option&lt;<code>u32</code>&gt;</li>
</ul>
<h4><a id="surface"></a><code>resource surface</code></h4>
<h4><a id="resize_event"></a><code>record resize-event</code></h4>
<h5>Record Fields</h5>
<ul>
<li><a id="resize_event.height"></a><code>height</code>: <code>u32</code></li>
<li><a id="resize_event.width"></a><code>width</code>: <code>u32</code></li>
</ul>
<h4><a id="frame_event"></a><code>record frame-event</code></h4>
<h5>Record Fields</h5>
<ul>
<li><a id="frame_event.nothing"></a><code>nothing</code>: <code>bool</code><p>TODO: This field doesn't mean anything.
Can't have empty record. Would like to have a way around this.
</li>
</ul>
<h4><a id="pointer_event"></a><code>record pointer-event</code></h4>
<h5>Record Fields</h5>
<ul>
<li><a id="pointer_event.x"></a><code>x</code>: <code>f64</code></li>
<li><a id="pointer_event.y"></a><code>y</code>: <code>f64</code></li>
</ul>
<h4><a id="key"></a><code>enum key</code></h4>
<p>corresponds with https://w3c.github.io/uievents-code/#code-value-tables
<code>Unidentified</code> is not included, use <code>option&lt;key&gt;</code> instead.</p>
<h5>Enum Cases</h5>
<ul>
<li><a id="key.backquote"></a><code>backquote</code></li>
<li><a id="key.backslash"></a><code>backslash</code></li>
<li><a id="key.bracket_left"></a><code>bracket-left</code></li>
<li><a id="key.bracket_right"></a><code>bracket-right</code></li>
<li><a id="key.comma"></a><code>comma</code></li>
<li><a id="key.digit0"></a><code>digit0</code></li>
<li><a id="key.digit1"></a><code>digit1</code></li>
<li><a id="key.digit2"></a><code>digit2</code></li>
<li><a id="key.digit3"></a><code>digit3</code></li>
<li><a id="key.digit4"></a><code>digit4</code></li>
<li><a id="key.digit5"></a><code>digit5</code></li>
<li><a id="key.digit6"></a><code>digit6</code></li>
<li><a id="key.digit7"></a><code>digit7</code></li>
<li><a id="key.digit8"></a><code>digit8</code></li>
<li><a id="key.digit9"></a><code>digit9</code></li>
<li><a id="key.equal"></a><code>equal</code></li>
<li><a id="key.intl_backslash"></a><code>intl-backslash</code></li>
<li><a id="key.intl_ro"></a><code>intl-ro</code></li>
<li><a id="key.intl_yen"></a><code>intl-yen</code></li>
<li><a id="key.key_a"></a><code>key-a</code></li>
<li><a id="key.key_b"></a><code>key-b</code></li>
<li><a id="key.key_c"></a><code>key-c</code></li>
<li><a id="key.key_d"></a><code>key-d</code></li>
<li><a id="key.key_e"></a><code>key-e</code></li>
<li><a id="key.key_f"></a><code>key-f</code></li>
<li><a id="key.key_g"></a><code>key-g</code></li>
<li><a id="key.key_h"></a><code>key-h</code></li>
<li><a id="key.key_i"></a><code>key-i</code></li>
<li><a id="key.key_j"></a><code>key-j</code></li>
<li><a id="key.key_k"></a><code>key-k</code></li>
<li><a id="key.key_l"></a><code>key-l</code></li>
<li><a id="key.key_m"></a><code>key-m</code></li>
<li><a id="key.key_n"></a><code>key-n</code></li>
<li><a id="key.key_o"></a><code>key-o</code></li>
<li><a id="key.key_p"></a><code>key-p</code></li>
<li><a id="key.key_q"></a><code>key-q</code></li>
<li><a id="key.key_r"></a><code>key-r</code></li>
<li><a id="key.key_s"></a><code>key-s</code></li>
<li><a id="key.key_t"></a><code>key-t</code></li>
<li><a id="key.key_u"></a><code>key-u</code></li>
<li><a id="key.key_v"></a><code>key-v</code></li>
<li><a id="key.key_w"></a><code>key-w</code></li>
<li><a id="key.key_x"></a><code>key-x</code></li>
<li><a id="key.key_y"></a><code>key-y</code></li>
<li><a id="key.key_z"></a><code>key-z</code></li>
<li><a id="key.minus"></a><code>minus</code></li>
<li><a id="key.period"></a><code>period</code></li>
<li><a id="key.quote"></a><code>quote</code></li>
<li><a id="key.semicolon"></a><code>semicolon</code></li>
<li><a id="key.slash"></a><code>slash</code></li>
<li><a id="key.alt_left"></a><code>alt-left</code></li>
<li><a id="key.alt_right"></a><code>alt-right</code></li>
<li><a id="key.backspace"></a><code>backspace</code></li>
<li><a id="key.caps_lock"></a><code>caps-lock</code></li>
<li><a id="key.context_menu"></a><code>context-menu</code></li>
<li><a id="key.control_left"></a><code>control-left</code></li>
<li><a id="key.control_right"></a><code>control-right</code></li>
<li><a id="key.enter"></a><code>enter</code></li>
<li><a id="key.meta_left"></a><code>meta-left</code></li>
<li><a id="key.meta_right"></a><code>meta-right</code></li>
<li><a id="key.shift_left"></a><code>shift-left</code></li>
<li><a id="key.shift_right"></a><code>shift-right</code></li>
<li><a id="key.space"></a><code>space</code></li>
<li><a id="key.tab"></a><code>tab</code></li>
<li><a id="key.convert"></a><code>convert</code></li>
<li><a id="key.kana_mode"></a><code>kana-mode</code></li>
<li><a id="key.lang1"></a><code>lang1</code></li>
<li><a id="key.lang2"></a><code>lang2</code></li>
<li><a id="key.lang3"></a><code>lang3</code></li>
<li><a id="key.lang4"></a><code>lang4</code></li>
<li><a id="key.lang5"></a><code>lang5</code></li>
<li><a id="key.non_convert"></a><code>non-convert</code></li>
<li><a id="key.delete"></a><code>delete</code></li>
<li><a id="key.end"></a><code>end</code></li>
<li><a id="key.help"></a><code>help</code></li>
<li><a id="key.home"></a><code>home</code></li>
<li><a id="key.insert"></a><code>insert</code></li>
<li><a id="key.page_down"></a><code>page-down</code></li>
<li><a id="key.page_up"></a><code>page-up</code></li>
<li><a id="key.arrow_down"></a><code>arrow-down</code></li>
<li><a id="key.arrow_left"></a><code>arrow-left</code></li>
<li><a id="key.arrow_right"></a><code>arrow-right</code></li>
<li><a id="key.arrow_up"></a><code>arrow-up</code></li>
<li><a id="key.num_lock"></a><code>num-lock</code></li>
<li><a id="key.numpad0"></a><code>numpad0</code></li>
<li><a id="key.numpad1"></a><code>numpad1</code></li>
<li><a id="key.numpad2"></a><code>numpad2</code></li>
<li><a id="key.numpad3"></a><code>numpad3</code></li>
<li><a id="key.numpad4"></a><code>numpad4</code></li>
<li><a id="key.numpad5"></a><code>numpad5</code></li>
<li><a id="key.numpad6"></a><code>numpad6</code></li>
<li><a id="key.numpad7"></a><code>numpad7</code></li>
<li><a id="key.numpad8"></a><code>numpad8</code></li>
<li><a id="key.numpad9"></a><code>numpad9</code></li>
<li><a id="key.numpad_add"></a><code>numpad-add</code></li>
<li><a id="key.numpad_backspace"></a><code>numpad-backspace</code></li>
<li><a id="key.numpad_clear"></a><code>numpad-clear</code></li>
<li><a id="key.numpad_clear_entry"></a><code>numpad-clear-entry</code></li>
<li><a id="key.numpad_comma"></a><code>numpad-comma</code></li>
<li><a id="key.numpad_decimal"></a><code>numpad-decimal</code></li>
<li><a id="key.numpad_divide"></a><code>numpad-divide</code></li>
<li><a id="key.numpad_enter"></a><code>numpad-enter</code></li>
<li><a id="key.numpad_equal"></a><code>numpad-equal</code></li>
<li><a id="key.numpad_hash"></a><code>numpad-hash</code></li>
<li><a id="key.numpad_memory_add"></a><code>numpad-memory-add</code></li>
<li><a id="key.numpad_memory_clear"></a><code>numpad-memory-clear</code></li>
<li><a id="key.numpad_memory_recall"></a><code>numpad-memory-recall</code></li>
<li><a id="key.numpad_memory_store"></a><code>numpad-memory-store</code></li>
<li><a id="key.numpad_memory_subtract"></a><code>numpad-memory-subtract</code></li>
<li><a id="key.numpad_multiply"></a><code>numpad-multiply</code></li>
<li><a id="key.numpad_paren_left"></a><code>numpad-paren-left</code></li>
<li><a id="key.numpad_paren_right"></a><code>numpad-paren-right</code></li>
<li><a id="key.numpad_star"></a><code>numpad-star</code></li>
<li><a id="key.numpad_subtract"></a><code>numpad-subtract</code></li>
<li><a id="key.escape"></a><code>escape</code></li>
<li><a id="key.f1"></a><code>f1</code></li>
<li><a id="key.f2"></a><code>f2</code></li>
<li><a id="key.f3"></a><code>f3</code></li>
<li><a id="key.f4"></a><code>f4</code></li>
<li><a id="key.f5"></a><code>f5</code></li>
<li><a id="key.f6"></a><code>f6</code></li>
<li><a id="key.f7"></a><code>f7</code></li>
<li><a id="key.f8"></a><code>f8</code></li>
<li><a id="key.f9"></a><code>f9</code></li>
<li><a id="key.f10"></a><code>f10</code></li>
<li><a id="key.f11"></a><code>f11</code></li>
<li><a id="key.f12"></a><code>f12</code></li>
<li><a id="key.fn"></a><code>fn</code></li>
<li><a id="key.fn_lock"></a><code>fn-lock</code></li>
<li><a id="key.print_screen"></a><code>print-screen</code></li>
<li><a id="key.scroll_lock"></a><code>scroll-lock</code></li>
<li><a id="key.pause"></a><code>pause</code></li>
<li><a id="key.browser_back"></a><code>browser-back</code></li>
<li><a id="key.browser_favorites"></a><code>browser-favorites</code></li>
<li><a id="key.browser_forward"></a><code>browser-forward</code></li>
<li><a id="key.browser_home"></a><code>browser-home</code></li>
<li><a id="key.browser_refresh"></a><code>browser-refresh</code></li>
<li><a id="key.browser_search"></a><code>browser-search</code></li>
<li><a id="key.browser_stop"></a><code>browser-stop</code></li>
<li><a id="key.eject"></a><code>eject</code></li>
<li><a id="key.launch_app1"></a><code>launch-app1</code></li>
<li><a id="key.launch_app2"></a><code>launch-app2</code></li>
<li><a id="key.launch_mail"></a><code>launch-mail</code></li>
<li><a id="key.media_play_pause"></a><code>media-play-pause</code></li>
<li><a id="key.media_select"></a><code>media-select</code></li>
<li><a id="key.media_stop"></a><code>media-stop</code></li>
<li><a id="key.media_track_next"></a><code>media-track-next</code></li>
<li><a id="key.media_track_previous"></a><code>media-track-previous</code></li>
<li><a id="key.power"></a><code>power</code></li>
<li><a id="key.sleep"></a><code>sleep</code></li>
<li><a id="key.audio_volume_down"></a><code>audio-volume-down</code></li>
<li><a id="key.audio_volume_mute"></a><code>audio-volume-mute</code></li>
<li><a id="key.audio_volume_up"></a><code>audio-volume-up</code></li>
<li><a id="key.wake_up"></a><code>wake-up</code></li>
<li><a id="key.hyper"></a><code>hyper</code></li>
<li><a id="key.super"></a><code>super</code></li>
<li><a id="key.turbo"></a><code>turbo</code></li>
<li><a id="key.abort"></a><code>abort</code></li>
<li><a id="key.resume"></a><code>resume</code></li>
<li><a id="key.suspend"></a><code>suspend</code></li>
<li><a id="key.again"></a><code>again</code></li>
<li><a id="key.copy"></a><code>copy</code></li>
<li><a id="key.cut"></a><code>cut</code></li>
<li><a id="key.find"></a><code>find</code></li>
<li><a id="key.open"></a><code>open</code></li>
<li><a id="key.paste"></a><code>paste</code></li>
<li><a id="key.props"></a><code>props</code></li>
<li><a id="key.select"></a><code>select</code></li>
<li><a id="key.undo"></a><code>undo</code></li>
<li><a id="key.hiragana"></a><code>hiragana</code></li>
<li><a id="key.katakana"></a><code>katakana</code></li>
</ul>
<h4><a id="key_event"></a><code>record key-event</code></h4>
<h5>Record Fields</h5>
<ul>
<li><a id="key_event.key"></a><a href="#key"><code>key</code></a>: option&lt;<a href="#key"><a href="#key"><code>key</code></a></a>&gt;</li>
<li><a id="key_event.text"></a><code>text</code>: option&lt;<code>string</code>&gt;</li>
<li><a id="key_event.alt_key"></a><code>alt-key</code>: <code>bool</code></li>
<li><a id="key_event.ctrl_key"></a><code>ctrl-key</code>: <code>bool</code></li>
<li><a id="key_event.meta_key"></a><code>meta-key</code>: <code>bool</code></li>
<li><a id="key_event.shift_key"></a><code>shift-key</code>: <code>bool</code></li>
</ul>
<hr />
<h3>Functions</h3>
<h4><a id="constructor_surface"></a><code>[constructor]surface: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="constructor_surface.desc"></a><code>desc</code>: <a href="#create_desc"><a href="#create_desc"><code>create-desc</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="constructor_surface.0"></a> own&lt;<a href="#surface"><a href="#surface"><code>surface</code></a></a>&gt;</li>
</ul>
<h4><a id="method_surface_connect_graphics_context"></a><code>[method]surface.connect-graphics-context: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_surface_connect_graphics_context.self"></a><code>self</code>: borrow&lt;<a href="#surface"><a href="#surface"><code>surface</code></a></a>&gt;</li>
<li><a id="method_surface_connect_graphics_context.context"></a><a href="#context"><code>context</code></a>: borrow&lt;<a href="#context"><a href="#context"><code>context</code></a></a>&gt;</li>
</ul>
<h4><a id="method_surface_height"></a><code>[method]surface.height: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_surface_height.self"></a><code>self</code>: borrow&lt;<a href="#surface"><a href="#surface"><code>surface</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_surface_height.0"></a> <code>u32</code></li>
</ul>
<h4><a id="method_surface_width"></a><code>[method]surface.width: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_surface_width.self"></a><code>self</code>: borrow&lt;<a href="#surface"><a href="#surface"><code>surface</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_surface_width.0"></a> <code>u32</code></li>
</ul>
<h4><a id="method_surface_request_set_size"></a><code>[method]surface.request-set-size: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_surface_request_set_size.self"></a><code>self</code>: borrow&lt;<a href="#surface"><a href="#surface"><code>surface</code></a></a>&gt;</li>
<li><a id="method_surface_request_set_size.height"></a><code>height</code>: option&lt;<code>u32</code>&gt;</li>
<li><a id="method_surface_request_set_size.width"></a><code>width</code>: option&lt;<code>u32</code>&gt;</li>
</ul>
<h4><a id="method_surface_subscribe_resize"></a><code>[method]surface.subscribe-resize: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_surface_subscribe_resize.self"></a><code>self</code>: borrow&lt;<a href="#surface"><a href="#surface"><code>surface</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_surface_subscribe_resize.0"></a> own&lt;<a href="#pollable"><a href="#pollable"><code>pollable</code></a></a>&gt;</li>
</ul>
<h4><a id="method_surface_get_resize"></a><code>[method]surface.get-resize: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_surface_get_resize.self"></a><code>self</code>: borrow&lt;<a href="#surface"><a href="#surface"><code>surface</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_surface_get_resize.0"></a> option&lt;<a href="#resize_event"><a href="#resize_event"><code>resize-event</code></a></a>&gt;</li>
</ul>
<h4><a id="method_surface_subscribe_frame"></a><code>[method]surface.subscribe-frame: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_surface_subscribe_frame.self"></a><code>self</code>: borrow&lt;<a href="#surface"><a href="#surface"><code>surface</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_surface_subscribe_frame.0"></a> own&lt;<a href="#pollable"><a href="#pollable"><code>pollable</code></a></a>&gt;</li>
</ul>
<h4><a id="method_surface_get_frame"></a><code>[method]surface.get-frame: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_surface_get_frame.self"></a><code>self</code>: borrow&lt;<a href="#surface"><a href="#surface"><code>surface</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_surface_get_frame.0"></a> option&lt;<a href="#frame_event"><a href="#frame_event"><code>frame-event</code></a></a>&gt;</li>
</ul>
<h4><a id="method_surface_subscribe_pointer_up"></a><code>[method]surface.subscribe-pointer-up: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_surface_subscribe_pointer_up.self"></a><code>self</code>: borrow&lt;<a href="#surface"><a href="#surface"><code>surface</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_surface_subscribe_pointer_up.0"></a> own&lt;<a href="#pollable"><a href="#pollable"><code>pollable</code></a></a>&gt;</li>
</ul>
<h4><a id="method_surface_get_pointer_up"></a><code>[method]surface.get-pointer-up: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_surface_get_pointer_up.self"></a><code>self</code>: borrow&lt;<a href="#surface"><a href="#surface"><code>surface</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_surface_get_pointer_up.0"></a> option&lt;<a href="#pointer_event"><a href="#pointer_event"><code>pointer-event</code></a></a>&gt;</li>
</ul>
<h4><a id="method_surface_subscribe_pointer_down"></a><code>[method]surface.subscribe-pointer-down: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_surface_subscribe_pointer_down.self"></a><code>self</code>: borrow&lt;<a href="#surface"><a href="#surface"><code>surface</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_surface_subscribe_pointer_down.0"></a> own&lt;<a href="#pollable"><a href="#pollable"><code>pollable</code></a></a>&gt;</li>
</ul>
<h4><a id="method_surface_get_pointer_down"></a><code>[method]surface.get-pointer-down: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_surface_get_pointer_down.self"></a><code>self</code>: borrow&lt;<a href="#surface"><a href="#surface"><code>surface</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_surface_get_pointer_down.0"></a> option&lt;<a href="#pointer_event"><a href="#pointer_event"><code>pointer-event</code></a></a>&gt;</li>
</ul>
<h4><a id="method_surface_subscribe_pointer_move"></a><code>[method]surface.subscribe-pointer-move: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_surface_subscribe_pointer_move.self"></a><code>self</code>: borrow&lt;<a href="#surface"><a href="#surface"><code>surface</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_surface_subscribe_pointer_move.0"></a> own&lt;<a href="#pollable"><a href="#pollable"><code>pollable</code></a></a>&gt;</li>
</ul>
<h4><a id="method_surface_get_pointer_move"></a><code>[method]surface.get-pointer-move: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_surface_get_pointer_move.self"></a><code>self</code>: borrow&lt;<a href="#surface"><a href="#surface"><code>surface</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_surface_get_pointer_move.0"></a> option&lt;<a href="#pointer_event"><a href="#pointer_event"><code>pointer-event</code></a></a>&gt;</li>
</ul>
<h4><a id="method_surface_subscribe_key_up"></a><code>[method]surface.subscribe-key-up: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_surface_subscribe_key_up.self"></a><code>self</code>: borrow&lt;<a href="#surface"><a href="#surface"><code>surface</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_surface_subscribe_key_up.0"></a> own&lt;<a href="#pollable"><a href="#pollable"><code>pollable</code></a></a>&gt;</li>
</ul>
<h4><a id="method_surface_get_key_up"></a><code>[method]surface.get-key-up: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_surface_get_key_up.self"></a><code>self</code>: borrow&lt;<a href="#surface"><a href="#surface"><code>surface</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_surface_get_key_up.0"></a> option&lt;<a href="#key_event"><a href="#key_event"><code>key-event</code></a></a>&gt;</li>
</ul>
<h4><a id="method_surface_subscribe_key_down"></a><code>[method]surface.subscribe-key-down: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_surface_subscribe_key_down.self"></a><code>self</code>: borrow&lt;<a href="#surface"><a href="#surface"><code>surface</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_surface_subscribe_key_down.0"></a> own&lt;<a href="#pollable"><a href="#pollable"><code>pollable</code></a></a>&gt;</li>
</ul>
<h4><a id="method_surface_get_key_down"></a><code>[method]surface.get-key-down: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_surface_get_key_down.self"></a><code>self</code>: borrow&lt;<a href="#surface"><a href="#surface"><code>surface</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_surface_get_key_down.0"></a> option&lt;<a href="#key_event"><a href="#key_event"><code>key-event</code></a></a>&gt;</li>
</ul>

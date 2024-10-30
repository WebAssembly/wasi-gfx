# WASI WebGPU+

A proposed [WebAssembly System Interface](https://github.com/WebAssembly/WASI) API.

### Current Phase

Phase 1

### Champions

- Mendy Berger
- Sean Isom 

### Portability Criteria
WASI GFX must have host implementations which can pass the testsuite on at least Windows, macOS, and Linux.

WASI GFX must have at least two complete independent implementations.

## Table of Contents [if the explainer is longer than one printed page]

- [Introduction](#introduction)
- [Goals [or Motivating Use Cases, or Scenarios]](#goals-or-motivating-use-cases-or-scenarios)
- [Non-goals](#non-goals)
- [API walk-through](#api-walk-through)
  - [Use case 1](#use-case-1)
  - [Use case 2](#use-case-2)
- [Detailed design discussion](#detailed-design-discussion)
  - [[Tricky design choice 1]](#tricky-design-choice-1)
  - [[Tricky design choice 2]](#tricky-design-choice-2)
- [Considered alternatives](#considered-alternatives)
  - [[Alternative 1]](#alternative-1)
  - [[Alternative 2]](#alternative-2)
- [Stakeholder Interest & Feedback](#stakeholder-interest--feedback)
- [References & acknowledgements](#references--acknowledgements)

### Introduction

This proposal is a collection of multiple packages.

| Package                 | Description                                                                        |
| ------------------------| ---------------------------------------------------------------------------------- |
| `wasi:webgpu`           | WebGPU package for interacting with GPUs                                           |
| `wasi:frame-buffer`     | A CPU based graphics API                                                           |
| `wasi:surface`          | A surface                                                                          |
| `wasi:graphics-context` | A small interface connecting the graphics APIs (webgpu or frame-buffer) to surface |


### Goals [or Motivating Use Cases, or Scenarios]

- Bring some of the benefits that Wasm provides to CPU code, over to GPU code. 
- Enable secure GPU compute.
- Enable wasm-based UI applications.
- Enable wasm-based graphics even in environments with no GPUs.
- Enable wasm-based AI for cases not covered by wasi-nn.

### Non-goals

- A full windowing API.
- VR/AR (subject to change, if we find contributors with expertise in the field).

### API walk-through

The full API documentation can be found in [imports.md](imports.md).

#### `wasi:webgpu`

`wasi:webgpu` is based on the official [webgpu spec](https://www.w3.org/TR/webgpu/).

`wasi:webgpu` does deviate sometimes from the webgpu spec, namely, in cases where the spec makes assumptions about running in a web or JS environment. Wherever `wasi:webgpu` deviates from the spec, a clear explanation should be documented.

#### `wasi:surface`

`wasi:surface` is a basic surface API. It lets you create a surface you can draw to (similar to canvas on web). You can also get basic user events from the surface, like pointer and keyboard events.

`wasi:surface` supports the following event types:
- Pointer: For mouse/touch/pencil/etc.
- Key: For keyboard events.
- Resize: Surface was resized.
- Frame: Fires on each frame. (similar to [`requestAnimationFrame`](https://developer.mozilla.org/en-US/docs/Web/API/window/requestAnimationFrame) on the web)

Other user input, such as scroll wheel, clipboard, drag-over, device motion, device orientation, gamepad input, etc., are all useful in many applications, but since they're a bit less common, they're out of scope for now. However, they might be added eventually.

#### `wasi:frame-buffer`

A simple frame buffer API for CPU-based rendering.

This might be especially useful in embedded.

#### `wasi:graphics-context`

`wasi:graphics-context` is the point of connection between a graphics API (`wasi:webgpu`, `wasi:frame-buffer`, future graphics API) to a windowing system (`wasi:surface`, future windowing system). See diagram.

![image](https://github.com/user-attachments/assets/55bcc436-7bcd-4f02-9090-888106b889ef)

`wasi:graphics-context` has a method to get the next frame as an `abstract-buffer`. Abstract buffer's contents can't be read. Instead, the graphics API in use can turn the `abstract-buffer` into something it can make use of. E.g. webgpu is able to turn the buffer into a `GPUTexture`. The runtime can easily, in the background, represent the `abstract-buffer` as a `GPUTexture`, and the conversion would just be a no-op.

#### [Use case 1]

[Provide example code snippets and diagrams explaining how the API would be used to solve the given problem]

#### [Use case 2]

[etc.]

### Detailed design discussion

[This section should mostly refer to the .wit.md file that specifies the API. This section is for any discussion of the choices made in the API which don't make sense to document in the spec file itself.]

#### [Tricky design choice #1]

[Talk through the tradeoffs in coming to the specific design point you want to make.]

```
// Illustrated with example code.
```

[This may be an open question, in which case you should link to any active discussion threads.]

#### [Tricky design choice 2]

[etc.]

### Considered alternatives

[This section is not required if you already covered considered alternatives in the design discussion above.]

#### [Alternative 1]

[Describe an alternative which was considered, and why you decided against it.]

#### [Alternative 2]

[etc.]

### Stakeholder Interest & Feedback

TODO before entering Phase 3.

[This should include a list of implementers who have expressed interest in implementing the proposal]

### References & acknowledgements

Many thanks for valuable feedback and advice from:

- [Person 1]
- [Person 2]
- [etc.]

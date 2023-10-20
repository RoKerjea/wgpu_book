use std:: {iter, mem};
use wgpu::util::DeviceExt;
use winit::{
	event::*,
	event_loop::{ControlFlow, EventLoop},
	window::{Window, WindowBuilder},
};
use bytemuck:: {Pod, Zeroable, cast_slice};

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
struct Vertex {
	position: [f32; 2],
	color: [f32; 3],
}

const VERTICES: &[Vertex] = &[
	Vertex { //vertex a
		position: [0.0, 0.5],
		color:[1.0, 0.0, 0.0]
	},
	Vertex { //vertex b
		position: [-0.5, -0.5],
		color:[0.0, 1.0, 0.0]
	},
	Vertex { //vertex c
		position: [0.5, -0.5],
		color:[0.0, 0.0, 1.0]
	},
];

impl Vertex {
	
}
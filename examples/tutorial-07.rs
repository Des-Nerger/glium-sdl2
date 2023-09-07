#![warn(clippy::pedantic, elided_lifetimes_in_paths, explicit_outlives_requirements)]
#![allow(
	confusable_idents,
	mixed_script_confusables,
	non_camel_case_types,
	non_snake_case,
	uncommon_codepoints
)]

#[macro_use]
extern crate glium;
extern crate glium_sdl2;
extern crate sdl2;

#[path = "support/tuto-07-teapot.rs"]
mod teapot;

use {
	glium::{index::PrimitiveType::TrianglesList, IndexBuffer, Program, Surface, VertexBuffer},
	glium_sdl2::DisplayBuild,
	sdl2::{event::Event, keyboard::Scancode},
	std::{
		thread,
		time::{Duration, Instant},
	},
};

fn main() {
	let sdl2 = sdl2::init().unwrap();
	let mut eventPump = sdl2.event_pump().unwrap();
	let display = &{
		let video = sdl2.video().unwrap();
		video.gl_attr().set_multisample_samples(16);
		video.window(file!(), 800, 600).resizable().build_glium().unwrap()
	};

	let positions = &VertexBuffer::new(display, &teapot::VERTICES).unwrap();
	let normals = &VertexBuffer::new(display, &teapot::NORMALS).unwrap();
	let indices = &IndexBuffer::new(display, TrianglesList, &teapot::INDICES).unwrap();

	let program = &Program::from_source(
		display,
		r#"
			#version 140

			in vec3 position;
			in vec3 normal;

			uniform mat4 matrix;

			void main() {
				gl_Position = matrix * vec4(position, 1.0);
			}
		"#,
		r#"
			#version 140

			out vec4 color;

			void main() {
				color = vec4(1.0, 0.0, 0.0, 1.0);
			}
    "#,
		None,
	)
	.unwrap();
	const FPS: u32 = 30;
	let frameDuration = Duration::from_secs(1) / FPS;
	let mut nextFrameInstant = Instant::now() + frameDuration;
	loop {
		for event in eventPump.poll_iter() {
			match event {
				Event::Quit { .. } | Event::KeyDown { scancode: Some(Scancode::Escape), .. } => return,
				_ => {}
			}
		}

		{
			let mut frame = display.draw();
			frame.clear_color(0.0, 0.0, 1.0, 1.0);
			frame
				.draw(
					(positions, normals),
					indices,
					program,
					&uniform! {
						matrix: [
							[0.01, 0.00, 0.00, 0.0],
							[0.00, 0.01, 0.00, 0.0],
							[0.00, 0.00, 0.01, 0.0],
							[0.00, 0.00, 0.00, 1.0_f32],
						],
					},
					&default(),
				)
				.unwrap();
			frame.finish().unwrap();
		}
		thread::sleep(nextFrameInstant - Instant::now());
		nextFrameInstant += frameDuration;
	}

	#[inline(always)]
	pub fn default<T: Default>() -> T {
		Default::default()
	}
}

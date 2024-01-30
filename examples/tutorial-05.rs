#![warn(clippy::pedantic, elided_lifetimes_in_paths, explicit_outlives_requirements)]
#![allow(
	confusable_idents,
	mixed_script_confusables,
	non_camel_case_types,
	non_snake_case,
	uncommon_codepoints
)]

extern crate glium;
extern crate glium_sdl2;
extern crate sdl2;

use {
	glium::{
		index::{NoIndices, PrimitiveType::TrianglesList},
		Surface,
	},
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
	let display = &sdl2.video().unwrap().window(file!(), 800, 600).resizable().build_glium().unwrap();
	#[derive(Copy, Clone)]
	struct Vertex {
		position: [f32; 2],
	}
	glium::implement_vertex!(Vertex, position);
	let vertexBuffer = &glium::VertexBuffer::new(
		display,
		&[Vertex { position: [-0.5, -0.5] }, Vertex { position: [0.0, 0.5] }, Vertex { position: [0.5, -0.25] }],
	)
	.unwrap();
	let program = &glium::Program::from_source(
		display,
		r#"
			#version 140

			in vec2 position;
			out vec2 my_attr;      // our new attribute

			uniform mat4 matrix;

			void main() {
				my_attr = position;     // we need to set the value of each `out` variable.
				gl_Position = matrix * vec4(position, 0.0, 1.0);
			}
		"#,
		r#"
			#version 140

			in vec2 my_attr;
			out vec4 color;

			void main() {
				color = vec4(my_attr, 0.0, 1.0);
			}
    "#,
		None,
	)
	.unwrap();
	const FPS: u32 = 30;
	let frameDuration = Duration::from_secs(1) / FPS;
	let (mut t, mut nextFrameInstant) = (-0.5_f32, Instant::now() + frameDuration);
	loop {
		for event in eventPump.poll_iter() {
			match event {
				Event::Quit { .. } | Event::KeyDown { scancode: Some(Scancode::Escape), .. } => return,
				_ => {}
			}
		}

		// we update `t`
		t += 0.0065;
		if t > 0.5 {
			t = -0.5;
		}

		{
			let mut frame = display.draw();
			frame.clear_color(0.0, 0.0, 1.0, 1.0);
			frame
				.draw(
					vertexBuffer,
					&NoIndices(TrianglesList),
					program,
					&glium::uniform! { matrix: [
						[ t.cos(), t.sin(), 0.0, 0.0],
						[-t.sin(), t.cos(), 0.0, 0.0],
						[0.0, 0.0, 1.0, 0.0],
						[0.0, 0.0, 0.0, 1.0_f32],
					] },
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

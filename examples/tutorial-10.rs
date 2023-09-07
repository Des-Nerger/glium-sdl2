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
	glium::{
		draw_parameters::DepthTest::IfLess, index::PrimitiveType::TrianglesList, Depth, DrawParameters,
		IndexBuffer, Program, Surface, VertexBuffer,
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
	let display = &{
		let video = sdl2.video().unwrap();
		{
			let glAttr = video.gl_attr();
			glAttr.set_multisample_samples(16);
			glAttr.set_depth_size(24);
		}
		video.window(file!(), 800, 600).resizable().build_glium().unwrap()
	};

	let positions = &VertexBuffer::new(display, &teapot::VERTICES).unwrap();
	let normals = &VertexBuffer::new(display, &teapot::NORMALS).unwrap();
	let indices = &IndexBuffer::new(display, TrianglesList, &teapot::INDICES).unwrap();

	let program = &Program::from_source(
		display,
		r#"
			#version 150

			in vec3 position;
			in vec3 normal;

			out vec3 v_normal;

			uniform mat4 perspective;       // new
			uniform mat4 matrix;

			void main() {
				v_normal = transpose(inverse(mat3(matrix))) * normal;
				gl_Position = perspective * matrix * vec4(position, 1.0);  // new
			}
		"#,
		r#"
			#version 140

			in vec3 v_normal;
			out vec4 color;
			uniform vec3 u_light;

			void main() {
				float brightness = dot(normalize(v_normal), normalize(u_light));
				vec3 darkColor = vec3(0.6, 0.0, 0.0);
				vec3 regularColor = vec3(1.0, 0.0, 0.0);
				color = vec4(mix(darkColor, regularColor, brightness), 1.0);
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
			let perspective = {
				let (width, height) = frame.get_dimensions();
				let aspectRatio = height as f32 / width as f32;

				let fov: f32 = 3.141592 / 3.0;
				let zfar = 1024.0;
				let znear = 0.1;

				let f = 1.0 / (fov / 2.0).tan();

				[
					[f * aspectRatio, 0.0, 0.0, 0.0],
					[0.0, f, 0.0, 0.0],
					[0.0, 0.0, (zfar + znear) / (zfar - znear), 1.0],
					[0.0, 0.0, -(2.0 * zfar * znear) / (zfar - znear), 0.0],
				]
			};
			frame.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);
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
							[0.00, 0.00, 2.00, 1.0_f32],
						],
						perspective: perspective,
						// the direction of the light
						u_light: [-1.0, 0.4, 0.9_f32],
					},
					&DrawParameters { depth: Depth { test: IfLess, write: true, ..default() }, ..default() },
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

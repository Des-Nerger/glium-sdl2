#![allow(dead_code)]

extern crate clock_ticks;
extern crate genmesh;
extern crate obj;

use glium;
use glium::vertex::VertexBufferAny;
use glium_sdl2::Display;
use std::thread;
use std::time::Duration;

pub mod camera;

pub enum Action {
	Stop,
	Continue,
}

pub fn start_loop<F>(mut callback: F)
where
	F: FnMut() -> Action,
{
	let mut accumulator = 0;
	let mut previous_clock = clock_ticks::precise_time_ns();

	loop {
		match callback() {
			Action::Stop => break,
			Action::Continue => (),
		};

		let now = clock_ticks::precise_time_ns();
		accumulator += now - previous_clock;
		previous_clock = now;

		const FIXED_TIME_STAMP: u64 = 16666667;
		while accumulator >= FIXED_TIME_STAMP {
			accumulator -= FIXED_TIME_STAMP;

			// if you have a game, update the state here
		}

		thread::sleep(Duration::from_millis(((FIXED_TIME_STAMP - accumulator) / 1000000) as u64));
	}
}

/// Returns a vertex buffer that should be rendered as `TrianglesList`.
pub fn load_wavefront(display: &Display, data: &[u8]) -> VertexBufferAny {
	#[derive(Copy, Clone)]
	struct Vertex {
		position: [f32; 3],
		normal: [f32; 3],
		texture: [f32; 2],
	}

	implement_vertex!(Vertex, position, normal, texture);

	let mut data = ::std::io::BufReader::new(data);
	let data = obj::ObjData::load_buf(&mut data).unwrap();

	let mut vertex_data = Vec::new();

	for object in data.objects.iter() {
		for polygon in object.groups.iter().flat_map(|g| g.polys.iter()) {
			match polygon {
				obj::SimplePolygon(indices) => {
					for v in indices.iter() {
						let position = data.position[v.0];
						let texture = v.1.map(|index| data.texture[index]);
						let normal = v.2.map(|index| data.normal[index]);

						let texture = texture.unwrap_or([0.0, 0.0]);
						let normal = normal.unwrap_or([0.0, 0.0, 0.0]);

						vertex_data.push(Vertex { position, normal, texture })
					}
				}
			}
		}
	}

	glium::vertex::VertexBuffer::new(display, &vertex_data).unwrap().into()
}

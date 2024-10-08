#[macro_use]
extern crate glium;

extern crate glium_sdl2;
extern crate sdl2;

use glium::Surface;

mod support;

fn main() {
	use glium_sdl2::DisplayBuild;

	let sdl_context = sdl2::init().unwrap();
	let video_subsystem = sdl_context.video().unwrap();

	video_subsystem.gl_attr().set_depth_size(24);

	// building the display, ie. the main object
	let display = video_subsystem.window("Teapot", 800, 600).build_glium().unwrap();

	// building the vertex and index buffers
	let vertex_buffer = support::load_wavefront(&display, include_bytes!("support/teapot.obj"));

	// the program
	let program = program!(&display,
			140 => {
					vertex: "
                #version 140

                uniform mat4 persp_matrix;
                uniform mat4 view_matrix;

                in vec3 position;
                in vec3 normal;
                out vec3 v_position;
                out vec3 v_normal;

                void main() {
                    v_position = position;
                    v_normal = normal;
                    gl_Position = persp_matrix * view_matrix * vec4(v_position * 0.005, 1.0);
                }
            ",

					fragment: "
                #version 140

                in vec3 v_normal;
                out vec4 f_color;

                const vec3 LIGHT = vec3(-0.2, 0.8, 0.1);

                void main() {
                    float lum = max(dot(normalize(v_normal), normalize(LIGHT)), 0.0);
                    vec3 color = (0.3 + 0.7 * lum) * vec3(1.0, 1.0, 1.0);
                    f_color = vec4(color, 1.0);
                }
            ",
			},

			110 => {
					vertex: "
                #version 110

                uniform mat4 persp_matrix;
                uniform mat4 view_matrix;

                attribute vec3 position;
                attribute vec3 normal;
                varying vec3 v_position;
                varying vec3 v_normal;

                void main() {
                    v_position = position;
                    v_normal = normal;
                    gl_Position = persp_matrix * view_matrix * vec4(v_position * 0.005, 1.0);
                }
            ",

					fragment: "
                #version 110

                varying vec3 v_normal;

                const vec3 LIGHT = vec3(-0.2, 0.8, 0.1);

                void main() {
                    float lum = max(dot(normalize(v_normal), normalize(LIGHT)), 0.0);
                    vec3 color = (0.3 + 0.7 * lum) * vec3(1.0, 1.0, 1.0);
                    gl_FragColor = vec4(color, 1.0);
                }
            ",
			},

			100 => {
					vertex: "
                #version 100

                uniform lowp mat4 persp_matrix;
                uniform lowp mat4 view_matrix;

                attribute lowp vec3 position;
                attribute lowp vec3 normal;
                varying lowp vec3 v_position;
                varying lowp vec3 v_normal;

                void main() {
                    v_position = position;
                    v_normal = normal;
                    gl_Position = persp_matrix * view_matrix * vec4(v_position * 0.005, 1.0);
                }
            ",

					fragment: "
                #version 100

                varying lowp vec3 v_normal;

                const lowp vec3 LIGHT = vec3(-0.2, 0.8, 0.1);

                void main() {
                    lowp float lum = max(dot(normalize(v_normal), normalize(LIGHT)), 0.0);
                    lowp vec3 color = (0.3 + 0.7 * lum) * vec3(1.0, 1.0, 1.0);
                    gl_FragColor = vec4(color, 1.0);
                }
            ",
			},
	)
	.unwrap();

	//
	let mut camera = support::camera::CameraState::new();

	let mut event_pump = sdl_context.event_pump().unwrap();

	// the main loop
	support::start_loop(|| {
		camera.update();

		// building the uniforms
		let uniforms = uniform! {
				persp_matrix: camera.get_perspective(),
				view_matrix: camera.get_view(),
		};

		// draw parameters
		let params = glium::DrawParameters {
			depth: glium::Depth { test: glium::DepthTest::IfLess, write: true, ..Default::default() },
			..Default::default()
		};

		// drawing a frame
		let mut target = display.draw();

		target.clear_color_and_depth((0.0, 0.0, 0.0, 0.0), 1.0);
		target
			.draw(
				&vertex_buffer,
				&glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList),
				&program,
				&uniforms,
				&params,
			)
			.unwrap();
		target.finish().unwrap();

		// polling and handling the events received by the window
		for event in event_pump.poll_iter() {
			use sdl2::event::Event;

			match event {
				Event::Quit { .. } => return support::Action::Stop,
				ev => camera.process_input(&ev),
			}
		}

		support::Action::Continue
	});
}

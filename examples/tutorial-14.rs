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

use {
	glium::{
		draw_parameters::DepthTest::IfLess,
		index::{NoIndices, PrimitiveType::TriangleStrip},
		texture::{RawImage2d, SrgbTexture2d, Texture2d},
		Depth, DrawParameters, Program, Surface, VertexBuffer,
	},
	glium_sdl2::DisplayBuild,
	sdl2::{event::Event, keyboard::Scancode},
	std::{
		io::Cursor,
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

	let diffuseTexture = &{
		let image = image::io::Reader::new(Cursor::new(&include_bytes!("support/tuto-14-diffuse.jpg")))
			.with_guessed_format()
			.unwrap()
			.decode()
			.unwrap()
			.into_rgba8();
		let imageDimensions = image.dimensions();
		let image = RawImage2d::from_raw_rgba_reversed(&image.into_raw(), imageDimensions);
		SrgbTexture2d::new(display, image).unwrap()
	};
	let normalMap = &{
		let image = image::io::Reader::new(Cursor::new(&include_bytes!("support/tuto-14-normal.png")))
			.with_guessed_format()
			.unwrap()
			.decode()
			.unwrap()
			.into_rgba8();
		let imageDimensions = image.dimensions();
		let image = RawImage2d::from_raw_rgba_reversed(&image.into_raw(), imageDimensions);
		Texture2d::new(display, image).unwrap()
	};

	#[derive(Copy, Clone)]
	struct Vertex {
		position: [f32; 3],
		normal: [f32; 3],
		texCoords: [f32; 2],
	}
	implement_vertex!(Vertex, position, normal, texCoords);

	let program = &Program::from_source(
		display,
		r#"
			#version 150

			in vec3 position;
			in vec3 normal;
			in vec2 texCoords;

			out vec3 v_normal;
			out vec3 v_position;
			out vec2 v_texCoords;

			uniform mat4 perspective;
			uniform mat4 view;
			uniform mat4 model;

			void main() {
				mat4 modelview = view * model;
				v_normal = transpose(inverse(mat3(modelview))) * normal;
				gl_Position = perspective * modelview * vec4(position, 1.0);
				v_position = gl_Position.xyz / gl_Position.w;
				v_texCoords = texCoords;
			}
		"#,
		r#"
			#version 140

			in vec3 v_normal;
			in vec3 v_position;
			in vec2 v_texCoords;
			out vec4 color;
			uniform vec3 u_light;
			uniform sampler2D diffuseTex;
			uniform sampler2D normalTex;

			const vec3 specularColor = vec3(1.0, 1.0, 1.0);

			mat3 cotangentFrame(vec3 normal, vec3 pos, vec2 uv) {
					vec3 dp1 = dFdx(pos);
					vec3 dp2 = dFdy(pos);
					vec2 duv1 = dFdx(uv);
					vec2 duv2 = dFdy(uv);

					vec3 dp2perp = cross(dp2, normal);
					vec3 dp1perp = cross(normal, dp1);
					vec3 T = dp2perp * duv1.x + dp1perp * duv2.x;
					vec3 B = dp2perp * duv1.y + dp1perp * duv2.y;

					float invmax = inversesqrt(max(dot(T, T), dot(B, B)));
					return mat3(T * invmax, B * invmax, normal);
			}

			void main() {
				vec3 diffuseColor = texture(diffuseTex, v_texCoords).rgb;
				vec3 ambientColor = diffuseColor * 0.1;

				vec3 v_normalUnit = normalize(v_normal);
				vec3 normalMap = texture(normalTex, v_texCoords).rgb;

				mat3 tbn = cotangentFrame(v_normalUnit, -v_position, v_texCoords);
				vec3 realNormal = normalize(tbn * -(normalMap * 2.0 - 1.0));

				float diffuse = max(dot(realNormal, normalize(u_light)), 0.0);

				vec3 cameraDir = normalize(-v_position);
				vec3 halfDirection = normalize(normalize(u_light) + cameraDir);
				float specular = pow(max(dot(halfDirection, realNormal), 0.0), 16.0);

				color = vec4(ambientColor + diffuse * diffuseColor + specular * specularColor, 1.0);
			}
    "#,
		None,
	)
	.unwrap();
	const FPS: u32 = 30;
	let frameDuration = Duration::from_secs(1) / FPS;
	let startInstant = Instant::now();
	let mut nextFrameInstant = startInstant + frameDuration;
	loop {
		for event in eventPump.poll_iter() {
			match event {
				Event::Quit { .. } | Event::KeyDown { scancode: Some(Scancode::Escape), .. } => return,
				_ => {}
			}
		}

		let t = (Instant::now() - startInstant).as_secs_f32() * 2.0;
		let ang = t.sin();
		let (c, s) = (ang.cos(), ang.sin());

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
					&VertexBuffer::new(
						display,
						&[
							Vertex { position: [-1.0, 1.0, 0.0], normal: [0.0, 0.0, -1.0], texCoords: [0.0, 1.0] },
							Vertex { position: [1.0, 1.0, 0.0], normal: [0.0, 0.0, -1.0], texCoords: [1.0, 1.0] },
							Vertex { position: [-1.0, -1.0, 0.0], normal: [0.0, 0.0, -1.0], texCoords: [0.0, 0.0] },
							Vertex { position: [1.0, -1.0, 0.0], normal: [0.0, 0.0, -1.0], texCoords: [1.0, 0.0] },
						],
					)
					.unwrap(),
					NoIndices(TriangleStrip),
					program,
					&uniform! {
						view: viewMatrix(&[0.5, 0.2, -3.0], &[-0.5, -0.2, 3.0], &[0.0, 1.0, 0.0]),
						model: [
							[  c, 0.0,   s, 0.0],
							[0.0, 1.0, 0.0, 0.0],
							[ -s, 0.0,   c, 0.0],
							[0.0, 0.0, 0.0, 1.0_f32]
						],
						perspective: perspective,
						// the direction of the light
						u_light: [1.4, 0.4, 0.7_f32],
						diffuseTex: diffuseTexture,
						normalTex: normalMap,
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
	fn viewMatrix(position: &[f32; 3], direction: &[f32; 3], up: &[f32; 3]) -> [[f32; 4]; 4] {
		let f = {
			let f = direction;
			let len = f[0] * f[0] + f[1] * f[1] + f[2] * f[2];
			let len = len.sqrt();
			[f[0] / len, f[1] / len, f[2] / len]
		};

		let s = [up[1] * f[2] - up[2] * f[1], up[2] * f[0] - up[0] * f[2], up[0] * f[1] - up[1] * f[0]];

		let sNorm = {
			let len = s[0] * s[0] + s[1] * s[1] + s[2] * s[2];
			let len = len.sqrt();
			[s[0] / len, s[1] / len, s[2] / len]
		};

		let u = [
			f[1] * sNorm[2] - f[2] * sNorm[1],
			f[2] * sNorm[0] - f[0] * sNorm[2],
			f[0] * sNorm[1] - f[1] * sNorm[0],
		];

		let p = [
			-position[0] * sNorm[0] - position[1] * sNorm[1] - position[2] * sNorm[2],
			-position[0] * u[0] - position[1] * u[1] - position[2] * u[2],
			-position[0] * f[0] - position[1] * f[1] - position[2] * f[2],
		];

		[
			[sNorm[0], u[0], f[0], 0.0],
			[sNorm[1], u[1], f[1], 0.0],
			[sNorm[2], u[2], f[2], 0.0],
			[p[0], p[1], p[2], 1.0],
		]
	}
	#[inline(always)]
	pub fn default<T: Default>() -> T {
		Default::default()
	}
}

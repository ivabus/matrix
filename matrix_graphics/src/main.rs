use std::time::Instant;

use log::error;
use pixels::{PixelsBuilder, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{WindowBuilder, WindowButtons};
use winit_input_helper::WinitInputHelper;

use crate::structs::Line;

mod structs;

const WIDTH: u32 = 640;
const HEIGHT: u32 = 480;
const POLYGON: usize = 4;
// POLYGON >= 2
const CENTER_X: u32 = 320;
const CENTER_Y: u32 = 240;
const RADIUS: u32 = 100;
const PI: f64 = std::f64::consts::PI;
const DEFAULT_ANGLE: f64 = 0.;

impl Default for Line {
	fn default() -> Self {
		Line {
			start_x: 0.,
			start_y: 0.,
			end_x: 0.,
			end_y: 0.,
			rotate_angle: 0.,
			rotate_center_x: 0.,
			rotate_center_y: 0.,
		}
	}
}

fn create_polygon() -> [Line; POLYGON] {
	let mut it: [Line; POLYGON] = [Line::default(); POLYGON];
	let mut dots: Vec<(f64, f64)> = Vec::new();
	for i in 0..POLYGON {
		dots.push((
			CENTER_X as f64 + RADIUS as f64 * (2. * PI * i as f64 / POLYGON as f64).cos(),
			CENTER_Y as f64 + RADIUS as f64 * (2. * PI * i as f64 / POLYGON as f64).sin(),
		));
	}
	for i in 0..POLYGON {
		let next = if i + 1 == POLYGON {
			0
		} else {
			i + 1
		};
		it[i] = Line::new(
			dots[i].0,
			dots[i].1,
			dots[next].0,
			dots[next].1,
			DEFAULT_ANGLE,
			CENTER_X as f64,
			CENTER_Y as f64,
		);
	}
	it
}

// Most based render, that goes by existing frame bitmap and just expands it to RGBA.
fn render(frame: &mut [u8], bitmap: &[[bool; WIDTH as usize]; HEIGHT as usize]) {
	for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
		let x = i % WIDTH as usize;
		let y = i / WIDTH as usize;
		if bitmap[y][x] {
			pixel.copy_from_slice(&[0xff, 0xff, 0xff, 0xff]);
		} else {
			pixel.copy_from_slice(&[0x00, 0x00, 0x00, 0xff]);
		}
	}
}

fn main() {
	env_logger::init();
	let event_loop = EventLoop::new();
	let mut input = WinitInputHelper::new();
	let window = {
		let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
		WindowBuilder::new()
			.with_title("matrix_graphics")
			.with_enabled_buttons(WindowButtons::CLOSE.union(WindowButtons::MINIMIZE))
			.with_inner_size(size)
			.with_min_inner_size(size)
			.with_max_inner_size(size)
			.build(&event_loop)
			.unwrap()
	};

	let mut pixels = {
		let window_size = window.inner_size();
		let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
		PixelsBuilder::new(WIDTH, HEIGHT, surface_texture).enable_vsync(true).build().unwrap()
	};

	let mut objects = create_polygon();

	let mut frame_count = 0;
	let mut now = Instant::now();

	let mut bitmap: [[bool; WIDTH as usize]; HEIGHT as usize] =
		[[false; WIDTH as usize]; HEIGHT as usize];
	event_loop.run(move |event, _, control_flow| {
		match event {
			Event::MainEventsCleared => {
				render(pixels.get_frame_mut(), &bitmap);
				if let Err(err) = pixels.render() {
					error!("pixels.render() failed: {err}");
					*control_flow = ControlFlow::Exit;
					return;
				}
			}
			_ => {}
		}

		// Handle input events
		if input.update(&event) {
			if input.key_pressed(VirtualKeyCode::Escape)
				|| input.close_requested()
				|| input.destroyed()
			{
				*control_flow = ControlFlow::Exit;
				return;
			}
			if input.key_pressed(VirtualKeyCode::F) {
				println!("Average FPS: {}", frame_count as f64 / now.elapsed().as_secs_f64());
				frame_count = 0;
				now = Instant::now();
			}
			if input.key_held(VirtualKeyCode::Equals) {
				for i in &mut objects {
					i.rotate_angle += std::f64::consts::PI / 360.;
				}
			}
			if input.key_held(VirtualKeyCode::Space) {
				for i in &mut objects {
					i.rotate_angle = 0.;
				}
			}
			if input.key_held(VirtualKeyCode::Minus) {
				for i in &mut objects {
					i.rotate_angle -= std::f64::consts::PI / 360.;
				}
			}
			if input.key_held(VirtualKeyCode::Up) {
				for i in &mut objects {
					i.start_y -= 1.;
					i.end_y -= 1.;
					i.rotate_center_y -= 1.;
				}
			}
			if input.key_held(VirtualKeyCode::Down) {
				for i in &mut objects {
					i.start_y += 1.;
					i.end_y += 1.;
					i.rotate_center_y += 1.;
				}
			}
			if input.key_held(VirtualKeyCode::Left) {
				for i in &mut objects {
					i.start_x -= 1.;
					i.end_x -= 1.;
					i.rotate_center_x -= 1.;
				}
			}
			if input.key_held(VirtualKeyCode::Right) {
				for i in &mut objects {
					i.start_x += 1.;
					i.end_x += 1.;
					i.rotate_center_x += 1.;
				}
			}

			for i in &mut objects {
				i.update();
			}
			bitmap = [[false; WIDTH as usize]; HEIGHT as usize];
			for object in &objects {
				let current_dots = object.get_points();
				for (x, y) in current_dots {
					if x >= 0 && y >= 0 && x < WIDTH as isize && y < HEIGHT as isize {
						bitmap[y as usize][x as usize] = true;
					}
				}
			}
			frame_count += 1;
		}
	});
}

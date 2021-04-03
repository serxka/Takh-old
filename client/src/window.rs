use crate::{render::Renderer, settings::Settings, Error};

use serde::{Deserialize, Serialize};

use std::collections::HashMap;

pub type EventLoop = winit::event_loop::EventLoop<()>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum GameInput {
	MoveForward,
	MoveBackwards,
	MoveLeft,
	MoveRight,
	FlyUp,
	FlyDown,
	Sprint,
	Escape,
}

impl GameInput {
	pub fn iter() -> impl Iterator<Item = GameInput> {
		[
			GameInput::MoveForward,
			GameInput::MoveBackwards,
			GameInput::MoveLeft,
			GameInput::MoveRight,
			GameInput::FlyUp,
			GameInput::FlyDown,
			GameInput::Sprint,
			GameInput::Escape,
		]
		.iter()
		.copied()
	}
}

#[derive(Debug, Clone, Copy)]
pub enum Event {
	/// A window close was requested
	Close,
	/// The windows was resized
	Resize([u32; 2]),
	/// The mouse was moved
	MouseMove(f64, f64),
	/// Input has changed
	InputChange(GameInput, bool),
}

pub struct Window {
	renderer: Renderer,
	window: winit::window::Window,
	events: Vec<Event>,
	keypress_map: HashMap<GameInput, winit::event::ElementState>,
}

impl Window {
	pub fn new(settings: &Settings) -> Result<(Window, EventLoop), Error> {
		let event_loop = EventLoop::new();

		let size = settings.graphics.window_size;
		let win_builder = winit::window::WindowBuilder::new()
			.with_decorations(true)
			.with_inner_size(winit::dpi::PhysicalSize::new(
				size[0] as f64,
				size[1] as f64,
			))
			.with_resizable(true)
			.with_title("Voxel");

		// Create our Window and OpenGL context
		let wc = glutin::ContextBuilder::new()
			.with_gl(glutin::GlRequest::Specific(glutin::Api::OpenGl, (4, 5)))
			.with_gl_profile(glutin::GlProfile::Core)
			.with_vsync(settings.graphics.vsync)
			.build_windowed(win_builder, &event_loop)
			.map_err(|e| Error::BackendError(Box::new(e)))?;
		// Make our context the current one
		let (context, window) = unsafe {
			let (c, w) = wc.split();
			let c = match c.make_current() {
				Ok(c) => c,
				Err((_, e)) => return Err(Error::BackendError(Box::new(e))),
			};
			(c, w)
		};

		// Generate our keypress state map (pressed or release)
		let mut keypress_map = HashMap::new();
		for input in GameInput::iter() {
			keypress_map.insert(input, winit::event::ElementState::Released);
		}

		Ok((
			Window {
				renderer: Renderer::new(context, size),
				window,
				events: Vec::new(),
				keypress_map,
			},
			event_loop,
		))
	}

	pub fn renderer_mut(&mut self) -> &mut Renderer {
		&mut self.renderer
	}

	pub fn swap_buffers(&mut self) {
		self.renderer
			.context
			.swap_buffers()
			.expect("failed to swap the framebuffer!");
	}

	pub fn set_resolution(&mut self, size: &[u32; 2]) {
		self.window.set_inner_size(winit::dpi::PhysicalSize::new(
			size[0] as f64,
			size[1] as f64,
		));
		self.renderer.viewport(0, 0, size[0], size[1]);
		self.renderer.framebuffer.resize(size[0], size[1]);
	}

	pub fn get_resolution(&self) -> (u32, u32) {
		let size = self.window.inner_size();
		(size.width, size.height)
	}

	/// Set's whether the cursor is locked and if it is visible
	pub fn set_grabbed(&self, grabbed: bool) {
		self.window
			.set_cursor_grab(grabbed)
			.expect("failed to grab cursor");
		self.window.set_cursor_visible(!grabbed);
	}

	/// Handles all relevant `event::WindowEvent`'s in our main event loop
	pub fn handle_window_event(&mut self, event: winit::event::Event<()>, settings: &mut Settings) {
		use winit::event::WindowEvent;
		match event {
			winit::event::Event::WindowEvent { event, .. } => match event {
				WindowEvent::CloseRequested => self.events.push(Event::Close),
				WindowEvent::Resized(s) => {
					self.renderer.viewport(0, 0, s.width, s.height);
					self.renderer.framebuffer.resize(s.width, s.height);
					self.events.push(Event::Resize([s.width, s.height]));
				}
				WindowEvent::KeyboardInput { input, .. } => {
					if let Some(game_input) = settings.input.map_input(&input) {
						self.keypress_map.insert(game_input, input.state);
						self.events.push(Event::InputChange(
							game_input,
							match input.state {
								winit::event::ElementState::Pressed => true,
								winit::event::ElementState::Released => false,
							},
						));
					}
				}
				_ => {}
			},
			_ => {}
		}
	}

	/// Handles all relevant `event::DeviceEvent`'s in our main event loop
	pub fn handle_device_event(&mut self, event: winit::event::Event<()>) {
		use winit::event::DeviceEvent;
		match event {
			winit::event::Event::DeviceEvent { event, .. } => match event {
				DeviceEvent::MouseMotion { delta: (x, y) } => {
					self.events.push(Event::MouseMove(x, y))
				}
				_ => {}
			},
			_ => {}
		}
	}

	/// Returns a bool for wether a key is pressed or not
	pub fn is_key_pressed(&self, input: GameInput) -> bool {
		if let Some(state) = self.keypress_map.get(&input) {
			match state {
				winit::event::ElementState::Pressed => return true,
				_ => {}
			}
		}
		false
	}

	pub fn take_events(&mut self) -> Vec<Event> {
		std::mem::take(&mut self.events)
	}
}

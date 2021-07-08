pub mod error;
pub mod render;
pub mod scene;
pub mod settings;
pub mod state;
pub mod window;

pub use crate::error::Error;

use std::sync::Arc;

use crate::{
	scene::GameScene,
	settings::Settings,
	state::{PlayState, PlayStateNext},
	window::{EventLoop, Window},
};

use tokio::runtime::Runtime;

pub struct GlobalState {
	pub settings: Settings,
	pub window: Window,
	pub runtime: Arc<Runtime>,
}

pub fn run(mut global_state: GlobalState, event_loop: EventLoop) -> ! {
	// Enter our first state
	let mut states: Vec<Box<dyn PlayState>> = vec![Box::new(
		GameScene::new(&mut global_state)
			.map_err(|e| panic!("failed to create GameScene: {:#?}", e))
			.unwrap(),
	)];
	if let Some(top) = states.last_mut() {
		top.enter(&mut global_state);
	}

	event_loop.run(move |event, _, control_flow| {
		*control_flow = winit::event_loop::ControlFlow::Poll;

		match event {
			// This is dispatched after all other events have been completed
			winit::event::Event::MainEventsCleared => {
				handle_main_events_cleared(&mut states, &mut global_state, control_flow);
			}
			winit::event::Event::WindowEvent { .. } => {
				global_state
					.window
					.handle_window_event(event, &mut global_state.settings);
			}
			winit::event::Event::DeviceEvent { .. } => {
				global_state.window.handle_device_event(event);
			}
			// This is the very last event to get dispatched (last code *we* get to run)
			winit::event::Event::LoopDestroyed => {
				if let Err(e) = global_state.settings.save_to_file() {
					log::warn!("failed to save settings file ({:?})", e);
				}
			}
			_ => {}
		}
	})
}

fn handle_main_events_cleared(
	states: &mut Vec<Box<dyn PlayState>>,
	global_state: &mut GlobalState,
	control_flow: &mut winit::event_loop::ControlFlow,
) {
	// Tick the current play state, get the result of that,
	// if playstate::continue; break
	// else loop and figure the next state again
	let mut exit = true;
	while let Some(state_next) = states.last_mut().map(|top| {
		let events = global_state.window.take_events();
		top.tick(global_state, events)
	}) {
		match state_next {
			PlayStateNext::Continue => {
				exit = false;
				break;
			}
			PlayStateNext::Pop => {
				states.pop();
				states.last_mut().map(|new| new.enter(global_state));
			}
			PlayStateNext::Push(mut new) => {
				new.enter(global_state);
				states.push(new);
			}
			PlayStateNext::Quit => while let Some(_) = states.pop() {},
		}
	}

	if exit {
		*control_flow = winit::event_loop::ControlFlow::Exit;
	}

	if let Some(top) = states.last_mut() {
		let renderer = global_state.window.renderer_mut();
		// Bind our framebuffer
		renderer.framebuffer.bind();
		// Draw onto the framebuffer
		top.draw(&global_state.settings);
		// Draw our framebuffer with post processing
		renderer.framebuffer.draw();
		// Flush to the screen
		global_state.window.swap_buffers();
	}
}

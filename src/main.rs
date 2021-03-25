use voxel::{settings::Settings, window::Window, GlobalState};

fn main() {
	// Establish our logger
	simple_logger::SimpleLogger::new()
		.with_level(log::LevelFilter::Warn)
		.init()
		.ok();

	// Load settings
	let settings = Settings::load();
	// Create our window and opengl context
	let (window, event_loop) = match Window::new(&settings) {
		Ok(o) => o,
		Err(e) => {
			log::error!("failed to create window: {:?}", e);
			std::process::exit(1);
		}
	};
	// Create our global state struct and run the event loop
	let global_state = GlobalState { settings, window };
	voxel::run(global_state, event_loop);
}

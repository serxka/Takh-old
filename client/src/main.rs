use std::sync::atomic::{AtomicUsize, Ordering};

use takh_client::{settings::Settings, window::Window, GlobalState};

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
	// Create our Async runtime
	let runtime = std::sync::Arc::new(
		tokio::runtime::Builder::new_multi_thread()
			.enable_io()
			.thread_name_fn(|| {
				static ATOMIC_THREAD_ID: AtomicUsize = AtomicUsize::new(0);
				let id = ATOMIC_THREAD_ID.fetch_add(1, Ordering::SeqCst);
				format!("tokio-runtime-{}", id)
			})
			.build()
			.expect("Failed to build Tokio runtime"),
	);
	// Create our global state struct and run the event loop
	let global_state = GlobalState {
		settings,
		window,
		runtime,
	};
	takh_client::run(global_state, event_loop);
}

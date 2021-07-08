pub mod client;
pub mod settings;

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

use client::Client;
use common::state::State;
use settings::Settings;

use tokio::{
	net::{TcpListener, TcpStream},
	runtime::Runtime,
};

pub struct Server {
	settings: Settings,
	state: State,
	runtime: Arc<Runtime>,
}

impl Server {
	pub fn new(settings: Settings) -> Server {
		let state = State::server();

		let runtime = tokio::runtime::Builder::new_multi_thread()
			.enable_io()
			.thread_name_fn(|| {
				static ATOMIC_THREAD_ID: AtomicUsize = AtomicUsize::new(0);
				let id = ATOMIC_THREAD_ID.fetch_add(1, Ordering::SeqCst);
				format!("tokio-runtime-{}", id)
			})
			.build()
			.expect("Failed to build Tokio runtime");

		let server_address = settings.server_address.clone();
		runtime.spawn(async move {
			let listener = TcpListener::bind(server_address).await.unwrap();
			loop {
				match listener.accept().await {
					Ok((s, _)) => handle_new_connection(s).await,
					Err(e) => log::warn!("Failed to connect new client: {}", e),
				}
			}
		});

		Server {
			settings,
			state,
			runtime: Arc::new(runtime),
		}
	}

	pub fn tick(&mut self) {
		// 1) handle new connections
		// 2) read all network messages from clients
		// 3)
	}
}

async fn handle_new_connection(stream: TcpStream) {}

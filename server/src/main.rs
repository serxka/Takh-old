use takh_server::{settings::Settings, Server};

fn main() {
	let settings = Settings::load();
	let mut server = Server::new(settings);

	loop {
		server.tick();
	}
}

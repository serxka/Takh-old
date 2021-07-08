use std::{
	fs,
	net::SocketAddr,
	path::{Path, PathBuf},
};

use common::config_root;

use serde::{Deserialize, Serialize};

const DEFAULT_PORT: u16 = 7878;

#[derive(Serialize, Deserialize)]
pub struct Settings {
	pub server_address: SocketAddr,
	pub view_distance: u32,
}

impl std::default::Default for Settings {
	fn default() -> Self {
		Settings {
			server_address: SocketAddr::from(([0; 4], DEFAULT_PORT)),
			view_distance: 32,
		}
	}
}

impl Settings {
	pub fn load() -> Settings {
		let mut path = Self::settings_path();

		if let Ok(file) = fs::File::open(&path) {
			match ron::de::from_reader::<_, Self>(file) {
				Ok(s) => s,
				Err(e) => {
					let template_settings = Self::default();
					path.pop();
					path.push("Server_settings.template.ron");
					log::warn!(
						"Failed to parse Server Settings, Falling back to default: {:?}",
						e
					);
					log::warn!(
						"A template settings file will be created for you to migrate, called: {}",
						path.display()
					);
					if let Err(e) = template_settings.save_to_file(&path) {
						log::warn!("Failed to create template server settings file: {:?}", e);
					}
					template_settings
				}
			}
		} else {
			let default_settings = Self::default();
			if let Err(e) = default_settings.save_to_file(&path) {
				log::warn!("Failed to create default server settings file: {:?}", e);
			}
			default_settings
		}
	}

	pub fn save_to_file(&self, path: &Path) -> std::io::Result<()> {
		if let Some(dir) = path.parent() {
			fs::create_dir_all(dir)?;
		}

		let stringify = ron::ser::to_string_pretty(
			self,
			ron::ser::PrettyConfig::default()
				.with_indentor("\t".to_owned())
				.with_decimal_floats(true),
		)
		.unwrap();
		fs::write(path, stringify.as_bytes())
	}

	pub fn settings_path() -> PathBuf {
		let mut path = config_root();
		path.push("Server_settings.ron");
		path
	}
}

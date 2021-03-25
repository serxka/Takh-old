use crate::window::GameInput;

use std::collections::{BTreeMap, HashMap};
use std::fs;
use std::path::PathBuf;

use serde::{Deserialize, Serialize, Serializer};
use winit::event::VirtualKeyCode;

#[derive(Serialize, Deserialize)]
pub struct InputSettings {
	#[serde(serialize_with = "ord_map")]
	pub bindings: HashMap<VirtualKeyCode, GameInput>,
}

fn ord_map<S>(map: &HashMap<VirtualKeyCode, GameInput>, serializer: S) -> Result<S::Ok, S::Error>
where
	S: Serializer,
{
	let ordered: BTreeMap<_, _> = map.iter().collect();
	ordered.serialize(serializer)
}

impl InputSettings {
	pub fn default_binding(input: GameInput) -> VirtualKeyCode {
		match input {
			GameInput::MoveForward => VirtualKeyCode::W,
			GameInput::MoveBackwards => VirtualKeyCode::S,
			GameInput::MoveLeft => VirtualKeyCode::A,
			GameInput::MoveRight => VirtualKeyCode::D,
			GameInput::FlyUp => VirtualKeyCode::Space,
			GameInput::FlyDown => VirtualKeyCode::LShift,
			GameInput::Escape => VirtualKeyCode::Escape,
		}
	}

	pub fn map_input(&self, input: &winit::event::KeyboardInput) -> Option<GameInput> {
		if let Some(virt) = input.virtual_keycode {
			self.bindings.get(&virt).copied()
		} else {
			None
		}
	}
}

impl std::default::Default for InputSettings {
	fn default() -> Self {
		let mut bindings = HashMap::new();
		for input in GameInput::iter() {
			bindings.insert(Self::default_binding(input), input);
		}
		Self { bindings }
	}
}

#[derive(Serialize, Deserialize)]
pub struct GraphicsSettings {
	pub window_size: [u32; 2],
	pub vsync: bool,
	pub fov: f32,
}

impl std::default::Default for GraphicsSettings {
	fn default() -> Self {
		Self {
			window_size: [1280, 720],
			vsync: true,
			fov: 90.0,
		}
	}
}

#[derive(Serialize, Deserialize)]
pub struct Settings {
	pub graphics: GraphicsSettings,
	pub input: InputSettings,
}

impl std::default::Default for Settings {
	fn default() -> Self {
		Settings {
			graphics: GraphicsSettings::default(),
			input: InputSettings::default(),
		}
	}
}

impl Settings {
	pub fn load() -> Settings {
		let path = Self::settings_path();

		if let Ok(file) = fs::File::open(&path) {
			match ron::de::from_reader::<_, Self>(file) {
				Ok(s) => return s,
				Err(e) => {
					log::warn!(
						"failed to parse settings file, falling back to default ({:?})",
						e
					);
					// rename the old one
					let mut rename_path = path.clone();
					rename_path.pop();
					rename_path.push("settings.ron.inv");
					if let Err(e) = fs::rename(&path, &rename_path) {
						log::warn!("failed to rename invalid settings file ({:?})", e);
					}
				}
			}
		}
		let default_settings = Self::default();
		if let Err(e) = default_settings.save_to_file() {
			log::warn!("failed to save settings file ({:?})", e);
		}
		default_settings
	}

	pub fn save_to_file(&self) -> std::io::Result<()> {
		let path = Self::settings_path();
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

	// for when we add proper support for resource paths and shit
	pub fn settings_path() -> PathBuf {
		PathBuf::from("./").join("Settings.ron")
	}
}

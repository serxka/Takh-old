pub mod components;
pub mod ecsres;
pub mod net;
pub mod state;
pub mod sys;
pub mod world;

use std::path::{Path, PathBuf};

#[derive(Copy, Clone)]
pub enum Gamemode {
	Client,
	Server,
}

/// This retures the root of the game data path.
/// For example in it may return `$TAKH_GAMEDATA/`,
/// or `./res/`
///
/// Will attempt ../../gamedata if a debug build for sanity in cargo
pub fn data_root() -> PathBuf {
	match std::env::var("TAKH_GAMEDATA") {
		Ok(env) => {
			if Path::new(&env).exists() {
				let mut path = PathBuf::new();
				path.push(&env);
				return path;
			}
		}
		Err(_) => {}
	}
	match std::env::current_exe() {
		Ok(mut exe) => {
			if cfg!(debug_assertions) {
				exe.pop();
				exe.push("../../res");
			} else {
				exe.pop();
				exe.push("res");
			}
			exe
		}
		Err(_) => {
			let mut path = PathBuf::new();
			path.push("./res");
			path
		}
	}
}

/// This retures the root of the game configuration path.
/// For example in it may return `$TAKH_GAMECONF/`,
/// or `./config/`
///
/// Will attempt ../../config if a debug build for sanity in cargo
pub fn config_root() -> PathBuf {
	match std::env::var("TAKH_GAMECONF") {
		Ok(env) => {
			if Path::new(&env).exists() {
				let mut path = PathBuf::new();
				path.push(&env);
				return path;
			}
		}
		Err(_) => {}
	}
	match std::env::current_exe() {
		Ok(mut exe) => {
			if cfg!(debug_assertions) {
				exe.pop();
				exe.push("../../config");
			} else {
				exe.pop();
				exe.push("config");
			}
			exe
		}
		Err(_) => {
			let mut path = PathBuf::new();
			path.push("./config");
			path
		}
	}
}

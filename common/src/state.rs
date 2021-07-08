use crate::{components, ecsres::*, Gamemode};

use specs::prelude::*;

pub struct State {
	ecs: specs::World,
}

impl State {
	pub fn client() -> Self {
		Self::new(Gamemode::Client)
	}

	pub fn server() -> Self {
		Self::new(Gamemode::Server)
	}

	pub fn new(gamemode: Gamemode) -> Self {
		Self {
			ecs: Self::setup_world(gamemode),
		}
	}

	fn setup_world(_gamemode: Gamemode) -> specs::World {
		let mut world = specs::World::new();

		world.register::<components::Gravity>();
		world.register::<components::Orientation>();
		world.register::<components::Player>();
		world.register::<components::Position>();
		world.register::<components::Last<components::Position>>();

		world.insert(DeltaTime(0.0));
		#[cfg(feature = "client")]
		world.insert(Player(None));

		world
	}

	pub fn ecs(&self) -> &specs::World {
		&self.ecs
	}

	pub fn ecs_mut(&mut self) -> &mut specs::World {
		&mut self.ecs
	}
}

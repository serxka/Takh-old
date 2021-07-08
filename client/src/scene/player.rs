use std::sync::Arc;

use crate::{
	scene::camera::Camera,
	window::{Event, GameInput},
};
use common::{
	components::{Orientation, Player as PlayerComp, Position},
	ecsres,
	state::State,
};

use specs::{Builder, ReadStorage, World, WorldExt, WriteStorage};
use tokio::runtime::Runtime;


pub struct Player {
	runtime: Arc<Runtime>,
	inputs: Vec<Event>,
	state: State,
}

impl Player {
	pub fn new(runtime: Arc<Runtime>) -> Player {
		let mut state = State::client();

		let ecs = state.ecs_mut();
		let player = ecs
			.create_entity()
			.with(Position::default())
			.with(Orientation::default())
			.with(PlayerComp::default())
			.build();

		*ecs.write_resource::<ecsres::Player>() = ecsres::Player(Some(player));

		Player {
			runtime,
			inputs: vec![],
			state,
		}
	}

	pub fn collect_input(&mut self, events: &Vec<Event>) {
		self.inputs.clear();
		let mut events = events.iter();
		while let Some(e) = events.next() {
			match e {
				Event::Input(_) | Event::MouseMove(_, _) => self.inputs.push(*e),
				_ => {}
			}
		}
	}

	pub fn collect_net(&mut self) {
		// unimplemented!()
	}

	pub fn tick(&mut self) {
		self.handle_movement();

		self.ecs_mut().maintain();
	}

	fn handle_movement(&mut self) {
		let mut in_pos = [0.0; 6];
		let mut in_dir = (0.0, 0.0);
		for e in &self.inputs {
			match e {
				Event::Input(GameInput::MoveForward) => in_pos[0] = 1.0,
				Event::Input(GameInput::MoveBackwards) => in_pos[1] = -1.0,
				Event::Input(GameInput::MoveLeft) => in_pos[2] = -1.0,
				Event::Input(GameInput::MoveRight) => in_pos[3] = 1.0,
				Event::Input(GameInput::FlyUp) => in_pos[4] = 1.0,
				Event::Input(GameInput::FlyDown) => in_pos[5] = -1.0,
				Event::MouseMove(x, y) => in_dir = (*x as f64, *y as f64),
				_ => {}
			}
		}

		let world = self.ecs();
		let player = self.ecs_self();

		let mut pos_storage = world.system_data::<WriteStorage<Position>>();
		let pos = pos_storage.get_mut(player).unwrap();
		let mut dir_storage = world.system_data::<WriteStorage<Orientation>>();
		let dir = dir_storage.get_mut(player).unwrap();

		const SPEED: f64 = 0.1;

		// let mut pos_off = DVec3::new(
		// 	in_pos[0] + in_pos[1],
		// 	in_pos[4] + in_pos[5],
		// 	in_pos[2] + in_pos[3],
		// ) * SPEED;

		// let yaw = in_dir.0;
		// let pitch = in_dir.1;

		// let face = DVec3::new(
		// 	yaw.to_radians().cos() * pitch.to_radians().cos(),
		// 	pitch.to_radians().sin(),
		// 	yaw.to_radians().sin() * pitch.to_radians().cos(),
		// );

		// println!("{:?}", face);

		// dir.0 = dir.0.cross(face);
		// pos_off *= dir.0;
		// pos.0 += pos_off;
	}

	pub fn update_camera(&self, camera: &mut Camera, yaw: f32, pitch: f32) {
		let world = self.ecs();
		let player = self.ecs_self();
		let pos = world
			.system_data::<ReadStorage<Position>>()
			.get(player)
			.unwrap()
			.0;

		let x = [pos.x as f32, pos.y as f32, pos.z as f32].into();
		camera.set_pos(x);
		camera.rotate(yaw * 1.5, pitch * 1.5);

		camera.update();
	}

	pub fn ecs_self(&self) -> specs::Entity {
		*self.ecs().fetch::<ecsres::Player>().0.as_ref().unwrap()
	}

	pub fn ecs_self_mut(&mut self) -> specs::Entity {
		*self.ecs_mut().fetch_mut::<ecsres::Player>().0.as_ref().unwrap()
	}

	pub fn ecs(&self) -> &World {
		self.state.ecs()
	}

	pub fn ecs_mut(&mut self) -> &mut World {
		self.state.ecs_mut()
	}
}

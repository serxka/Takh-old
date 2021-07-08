use crate::components::{Gravity, Position};
use crate::ecsres::DeltaTime;

use specs::{Join, Read, ReadStorage, System, WriteStorage};

pub struct Sys;
impl<'a> System<'a> for Sys {
	type SystemData = (
		WriteStorage<'a, Position>,
		ReadStorage<'a, Gravity>,
		Read<'a, DeltaTime>,
	);

	fn run(&mut self, sys_data: Self::SystemData) {
		let (mut pos_storage, grav_storage, delta) = sys_data;

		let delta = delta.0;

		for (pos, _) in (&mut pos_storage, &grav_storage).join() {
			pos.0.y -= delta * 9.8;
		}
	}
}

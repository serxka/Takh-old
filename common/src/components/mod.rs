pub mod physics;
pub mod player;

#[derive(Default)]
pub struct Last<C: specs::Component>(pub C);

impl<C: specs::Component + Sync + Send> specs::Component for Last<C> {
	type Storage = specs::VecStorage<Self>;
}

pub use {
	physics::{Gravity, Orientation, Position},
	player::Player,
};

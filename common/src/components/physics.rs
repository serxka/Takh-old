use specs::prelude::*;
use vek::Vec3;

#[derive(Default)]
pub struct Position(pub Vec3<f64>);

impl Component for Position {
	type Storage = VecStorage<Self>;
}

#[derive(Default)]
pub struct Orientation(pub Vec3<f64>);

impl Component for Orientation {
	type Storage = VecStorage<Self>;
}

#[derive(Default)]
pub struct Gravity;

impl Component for Gravity {
	type Storage = NullStorage<Self>;
}

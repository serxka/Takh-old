pub mod client;
pub mod packet;
pub mod server;
pub mod world;

pub enum NetError {
	ConnectionClosed,
	Deserialize(bincode::Error),
}

pub type EntityID = u32;

pub struct Position {
	pub x: i32,
	pub y: i32,
	pub z: i32,
	pub xf: u8,
	pub yf: u8,
	pub zf: u8,
}

pub struct VPosition {
	pub x: i32,
	pub y: i32,
	pub z: i32,
}

pub struct Rotation {
	pub yaw: u8,
	pub pitch: u8,
}

#[repr(u8)]
pub enum Face {
	Top = 0,
	Bottom = 1,
	North = 2,
	South = 3,
	East = 4,
	West = 5,
}

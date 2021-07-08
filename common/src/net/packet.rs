use crate::net::NetError;
use serde::{de::DeserializeOwned, Serialize};

pub struct Packet {
	data: Vec<u8>,
}

impl Packet {
	pub fn serialize<M: Serialize>(message: &M) -> Self {
		Packet {
			data: bincode::serialize(message).unwrap(),
		}
	}

	pub fn deserialize<M: DeserializeOwned>(self) -> Result<M, NetError> {
		match bincode::deserialize(&self.data) {
			Ok(m) => Ok(m),
			Err(e) => Err(NetError::Deserialize(e)),
		}
	}
}

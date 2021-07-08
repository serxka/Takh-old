use crate::net::{Face, Position, Rotation, VPosition};

pub enum ServerBound {
	Auth(Auth),
	PlayerAction(PlayerAction),
}

pub enum Auth {
	LoginRequest { username: [char; 32] },
}

pub enum PlayerAction {
	/// An absolute position of the player sent to the server
	PlayerTransform { pos: Position, rot: Rotation },
	PlayerMining {
		status: PlayerMiningStatus,
		voxel: VPosition,
		face: Face,
	},
	/// Places a voxel from the players hand
	PlaceVoxel { pos: VPosition, face: Face },
	/// Sets the players hand from index 0-9 in the hotbar
	SetHand(u8),
}

pub enum PlayerMiningStatus {
	/// Players start to mine
	Started,
	/// Player stops mining (looks away)
	Stopped,
	/// Player thinks they have finished mining
	Completed,
}

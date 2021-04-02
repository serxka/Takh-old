pub enum ServerBound {
	Auth(Auth),
	PlayerAction(PlayerAction),
}

pub enum Auth {
	LoginRequest { username: [char; 32] },
}

pub enum PlayerAction {}

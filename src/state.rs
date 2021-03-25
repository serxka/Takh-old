use crate::{window, GlobalState, Settings};

pub enum PlayStateNext {
	/// Continue in the current state
	Continue,
	/// Pop current state
	Pop,
	/// Push a new state onto the stack and continue on in it
	Push(Box<dyn PlayState>),
	/// Pop all states and close the game
	Quit,
}

pub trait PlayState {
	/// Called when state is entered, could be from pushing or popping
	fn enter(&mut self, global_state: &mut GlobalState);
	/// Tick the state forward
	fn tick(&mut self, global_state: &mut GlobalState, events: Vec<window::Event>)
		-> PlayStateNext;
	/// Draw the state
	fn draw(&mut self, settings: &Settings);
}

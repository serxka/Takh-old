pub struct Packet {
	length: u32,
	data: Vec<u8>,
}

impl Packet {
	pub fn new() -> Self {
		Self {
			length: 0,
			data: Vec::new(),
		}
	}

	pub fn clear(&mut self) {
		self.length = 0;
		self.data.clear();
	}

	pub fn append(&mut self, data: &[u8]) {
		self.data.copy_from_slice(data);
	}

	pub fn as_ptr(&self) -> *const u8 {
		self.data.as_ptr()
	}

	pub fn as_slice(&self) -> &[u8] {
		self.data.as_slice()
	}
}

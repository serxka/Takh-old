use ultraviolet::mat::Mat4;
use ultraviolet::vec::Vec3;

pub struct Camera {
	fov: f32,
	aspect: f32,
	pos: Vec3,
	face_dir: Vec3,
	right: Vec3,
	up_dir: Vec3,
	pitch: f32,
	yaw: f32,
}

impl Camera {
	pub fn new(fov: f32, aspect: f32, pos: Vec3) -> Camera {
		let mut camera = Camera {
			fov: 1.0,
			aspect,
			pos,
			face_dir: Vec3::new(0.0, 0.0, -1.0),
			right: Vec3::new(0.0, 0.0, 0.0),
			up_dir: Vec3::new(0.0, 1.0, 0.0),
			pitch: 0.0,
			yaw: 0.0,
		};
		camera.set_fov_deg(fov);
		camera
	}

	pub fn transform_bool(
		&mut self,
		forward: bool,
		backward: bool,
		left: bool,
		right: bool,
		up: bool,
		down: bool,
		speed: f32,
	) {
		let z = (backward as u32 as f32) * -1.0 + (forward as u32 as f32);
		let x = (left as u32 as f32) * -1.0 + (right as u32 as f32);
		let y = (down as u32 as f32) * -1.0 + (up as u32 as f32);
		self.transform(Vec3::new(x * speed, y * speed, z * speed));
	}

	pub fn transform(&mut self, off: Vec3) {
		self.pos += Vec3::new(self.right.x, 0.0, self.right.z) * off.x;
		self.pos.y += off.y;
		self.pos += Vec3::new(self.face_dir.x, 0.0, self.face_dir.z).normalized() * off.z;
	}

	pub fn rotate(&mut self, x: f32, y: f32) {
		self.yaw += x * 0.1;
		self.pitch -= y * 0.1;

		self.pitch = self.pitch.clamp(-89.5, 89.5);
	}

	pub fn update(&mut self) {
		self.face_dir = Vec3::new(
			self.yaw.to_radians().cos() * self.pitch.to_radians().cos(),
			self.pitch.to_radians().sin(),
			self.yaw.to_radians().sin() * self.pitch.to_radians().cos(),
		);

		self.right = self.face_dir.cross(Vec3::new(0.0, 1.0, 0.0)).normalized();
		self.up_dir = self.right.cross(self.face_dir).normalized();
	}

	pub fn view_matrix(&self) -> Mat4 {
		Mat4::look_at(self.pos, self.pos + self.face_dir, self.up_dir)
	}

	pub fn proj_matrix(&self) -> Mat4 {
		ultraviolet::projection::rh_yup::perspective_gl(self.fov, self.aspect, 0.1, 1000.0)
	}

	pub fn set_aspect_ratio(&mut self, aspect: f32) {
		self.aspect = aspect;
	}
	pub fn set_fov(&mut self, fov: f32) {
		self.fov = fov;
	}
	// set fov horizontal degs
	pub fn set_fov_deg(&mut self, fov: f32) {
		self.set_fov((fov / self.aspect) * 0.0174532);
	}
}

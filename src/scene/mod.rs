pub mod camera;
pub mod world;

use crate::render::{
	mesh::Mesh,
	shader::{Program, Shader, ShaderKind},
	texture::TextureAtlas,
};
use crate::{window, window::GameInput, GlobalState, PlayState, PlayStateNext, Settings};

use crate::scene::{
	camera::Camera,
	world::{ChunkVertex, RenderChunks, Chunk, Voxel},
};

pub struct BasicScene {
	world_mesh: RenderChunks,
	camera: Camera,
	cursor_grabbed: bool,
}

impl BasicScene {
	pub fn new(global_state: &mut GlobalState) -> BasicScene {
		let program = Program::from_shaders(&[
			Shader::from_file("res/shaders/chunk.vs", ShaderKind::Vertex).expect(""),
			Shader::from_file("res/shaders/chunk.fs", ShaderKind::Fragement).expect(""),
		])
		.unwrap();

		let atlas = TextureAtlas::new("res/textures/voxel_atlas.png", 1);
		let mut world_mesh = RenderChunks::new(std::rc::Rc::new(program), atlas);
		
		let mut chunk = Chunk::new((0,0,0));
		chunk.set_voxel((0,0,0), Voxel::new(0));
		chunk.set_voxel((0,1,0), Voxel::new(0));
		chunk.set_voxel((1,0,0), Voxel::new(0));
		chunk.set_voxel((0,0,1), Voxel::new(0));
		world_mesh.add_mesh(chunk.mesh_builder().build());

		let win_size = global_state.window.get_resolution();
		let camera = Camera::new(
			global_state.settings.graphics.fov,
			win_size.0 as f32 / win_size.1 as f32,
		);

		BasicScene {
			world_mesh,
			camera,
			cursor_grabbed: false,
		}
	}
}

impl PlayState for BasicScene {
	fn enter(&mut self, _global_state: &mut GlobalState) {
		self.world_mesh.atlas.bind(0);
		self.world_mesh.shader.set_uniform_int("u_tex", 0);
	}

	fn tick(
		&mut self,
		global_state: &mut GlobalState,
		mut events: Vec<window::Event>,
	) -> PlayStateNext {
		let mut next_state = PlayStateNext::Continue;

		while let Some(event) = events.pop() {
			match event {
				crate::window::Event::Close => next_state = PlayStateNext::Quit,
				crate::window::Event::Resize([x, y]) => {
					self.camera.set_aspect_ratio(x as f32 / y as f32)
				}
				crate::window::Event::MouseMove(x, y) => {
					if self.cursor_grabbed {
						self.camera.rotate(x as f32, y as f32)
					}
				}
				crate::window::Event::InputChange(GameInput::Escape, true) => {
					self.cursor_grabbed = !self.cursor_grabbed;
					global_state.window.set_grabbed(self.cursor_grabbed);
				}
				_ => {}
			}
		}

		let window = &global_state.window;
		self.camera.transform_bool(
			window.is_key_pressed(GameInput::MoveForward),
			window.is_key_pressed(GameInput::MoveBackwards),
			window.is_key_pressed(GameInput::MoveLeft),
			window.is_key_pressed(GameInput::MoveRight),
			window.is_key_pressed(GameInput::FlyUp),
			window.is_key_pressed(GameInput::FlyDown),
			0.1,
		);

		self.camera.update();

		next_state
	}

	fn draw(&mut self, _settings: &Settings) {
		let transform = ultraviolet::mat::Mat4::identity();
		self.world_mesh
			.shader
			.set_uniform_mat4("u_model", transform);
		self.world_mesh
			.shader
			.set_uniform_mat4("u_camera", self.camera.view_matrix());
		self.world_mesh
			.shader
			.set_uniform_mat4("u_project", self.camera.proj_matrix());

		self.world_mesh.render();
	}
}

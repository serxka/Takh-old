pub mod camera;
pub mod world;

use crate::render::{
	shader::{Program, Shader, ShaderKind},
	texture::TextureAtlas,
};
use crate::scene::{camera::Camera, world::RenderChunks};
use crate::{window, window::GameInput, Error, GlobalState, PlayState, PlayStateNext, Settings};

use common::world::{
	chunk::{Chunk, Palette},
	voxel,
	voxel::{TextureId, VoxelTexture},
	world::generate_chunk,
};

pub struct BasicScene {
	world_mesh: RenderChunks,
	camera: Camera,
	cursor_grabbed: bool,
	chunks: Vec<Chunk>,
}

impl BasicScene {
	pub fn new(global_state: &mut GlobalState) -> Result<BasicScene, Error> {
		let data_root = common::data_root();

		let program = Program::from_shaders(&[
			Shader::from_file(
				data_root.join("client/shaders/chunk.vs"),
				ShaderKind::Vertex,
			)?,
			Shader::from_file(
				data_root.join("client/shaders/chunk.fs"),
				ShaderKind::Fragement,
			)?,
		])?;

		let atlas = TextureAtlas::new(data_root.join("client/textures/voxel_atlas.png"), 1)?;
		let mut world_mesh = RenderChunks::new(std::rc::Rc::new(program), atlas);

		let tex_id = TextureId::new(world_mesh.atlas.size, 0);

		let mut palette = Palette::new();
		palette.add_voxel(voxel::Voxel::new_full(VoxelTexture::Single {
			faces: tex_id,
		}));

		let mut chunks = Vec::with_capacity(16);
		for x in -2..2 {
			for z in -2..2 {
				chunks.push(generate_chunk(x, 0, z, palette.clone()));
				let chunk = chunks.last().unwrap();
				world_mesh.add_mesh(
					world::mesh_builder(chunk).build(),
					(
						(chunk.coord.0 * Chunk::WIDTH as i32) as f32,
						(chunk.coord.1 * Chunk::HEIGHT as i32) as f32,
						(chunk.coord.2 * Chunk::DEPTH as i32) as f32,
					),
				);
			}
		}
		
		let win_size = global_state.window.get_resolution();
		let camera = Camera::new(
			global_state.settings.graphics.fov,
			win_size.0 as f32 / win_size.1 as f32,
			ultraviolet::vec::Vec3::new(0.0, 20.0, 0.0)
		);

		Ok(BasicScene {
			world_mesh,
			camera,
			cursor_grabbed: false,
			chunks,
		})
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
			if window.is_key_pressed(GameInput::Sprint) {
				0.5
			} else {
				0.1
			},
		);

		self.camera.update();

		next_state
	}

	fn draw(&mut self, _settings: &Settings) {
		self.world_mesh
			.shader
			.set_uniform_mat4("u_camera", self.camera.view_matrix());
		self.world_mesh
			.shader
			.set_uniform_mat4("u_project", self.camera.proj_matrix());

		self.world_mesh.render();
	}
}

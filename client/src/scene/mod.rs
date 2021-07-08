pub mod camera;
pub mod player;
pub mod world;

use std::sync::Arc;

use crate::render::{
	shader::{Program, Shader, ShaderKind},
	texture::TextureAtlas,
};
use crate::scene::{camera::Camera, player::Player, world::RenderChunks};
use crate::{window, window::GameInput, Error, GlobalState, PlayState, PlayStateNext, Settings};

use common::world::{
	chunk::{Chunk, Palette},
	voxel,
	voxel::{TextureId, VoxelTexture},
	world::generate_chunk,
};

use tokio::runtime::Runtime;

pub struct GameScene {
	runtime: Arc<Runtime>,

	player: Player,
	camera: Camera,

	world_mesh: RenderChunks,

	cursor_grabbed: bool,
}

impl GameScene {
	pub fn new(global_state: &mut GlobalState) -> Result<GameScene, Error> {
		let win_size = global_state.window.get_resolution();
		let camera = Camera::new(
			global_state.settings.graphics.fov,
			win_size.0 as f32 / win_size.1 as f32,
		);

		let player = Player::new(global_state.runtime.clone());

		let world_mesh = Self::create_world()?;

		Ok(GameScene {
			cursor_grabbed: false,
			player,
			runtime: global_state.runtime.clone(),
			world_mesh,
			camera,
		})
	}

	fn create_world() -> Result<RenderChunks, Error> {
		let data_root = common::data_root();

		let program = Program::from_shaders(&[
			Shader::from_file(data_root.join("client/shaders/chunk.vs"), ShaderKind::Vertex)?,
			Shader::from_file(data_root.join("client/shaders/chunk.fs"), ShaderKind::Fragement)?,
		])?;

		let atlas = TextureAtlas::new(data_root.join("client/textures/voxel_atlas.png"), 1)?;
		let mut world_mesh = RenderChunks::new(std::rc::Rc::new(program), atlas);

		let tex_id = TextureId::new(world_mesh.atlas.size, 0);

		let mut palette = Palette::new();
		palette.add_voxel(voxel::Voxel::new_full(VoxelTexture::Single { faces: tex_id }));

		let size: i32 = 2;

		let mut chunks = Vec::with_capacity((size * size) as usize);
		for x in -size..size {
			for z in -size..size {
				for y in -size / 2..size / 2 {
					chunks.push(generate_chunk(x, y, z, palette.clone()));
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
		}

		Ok(world_mesh)
	}
}

impl PlayState for GameScene {
	fn enter(&mut self, _global_state: &mut GlobalState) {
		self.world_mesh.atlas.bind(0);
		self.world_mesh.shader.set_uniform_int("u_tex", 0);
	}

	fn tick(&mut self, global_state: &mut GlobalState, mut events: Vec<window::Event>) -> PlayStateNext {
		let mut next_state = PlayStateNext::Continue;
		let mut mouse_delta = (0.0, 0.0);

		self.player.collect_input(&events);
		self.player.collect_net();

		while let Some(event) = events.pop() {
			match event {
				crate::window::Event::Close => next_state = PlayStateNext::Quit,
				crate::window::Event::Resize([_x, _y]) => {}
				crate::window::Event::MouseMove(x, y) => {
					if self.cursor_grabbed {
						mouse_delta = (x as f32, y as f32)
					}
				}
				crate::window::Event::InputChange(GameInput::Escape, true) => {
					self.cursor_grabbed = !self.cursor_grabbed;
					global_state.window.set_grabbed(self.cursor_grabbed);
				}
				_ => {}
			}
		}

		self.player.tick();
		self.player
			.update_camera(&mut self.camera, mouse_delta.0, mouse_delta.1);

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

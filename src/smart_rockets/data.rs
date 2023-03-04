use miscmath::prelude::Vec2;
use raylib::prelude::Color;

pub struct World {
	pub width: f32,
	pub height: f32,
	pub default_pos: Vec2,
	pub default_vel: Vec2,
	pub target_pos: Vec2,
	pub mutation_chance: f64,
	pub gen_done: bool
}

pub struct Attributes {
	pub radius: f32,
	pub color: Color
}
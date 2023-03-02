use miscmath::prelude::Vec2;
use raylib::prelude::Color;

pub struct World {
	pub width: f32,
	pub height: f32,
	pub target_pos: Vec2,
	pub functions: [fn((Vec2, Vec2, Vec2)); 10],
}

pub struct Attributes {
	pub radius: f32,
	pub color: Color,
	pub row: usize,
	pub column: usize,
}
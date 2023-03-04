use std::{thread, time};
use rusty_nature_of_code::smart_rockets::prelude::*;
use miscmath::prelude::*;
use misc_ecs::prelude::*;
use raylib::prelude::*;

fn main() {
	
	/* Creation of a constant tuple for the initial screen size */
	const INIT_SCREEN_SIZE: ( i32, i32 ) = ( 640, 480 );
	let ( width, height ) = ( INIT_SCREEN_SIZE.0 as f32, INIT_SCREEN_SIZE.1 as f32 );
	
	/* Creation of the RayLib handle and thread, sets the screen size, and gives the window a title */
	let ( mut rl, thread ) = init( )
		.size(INIT_SCREEN_SIZE.0, INIT_SCREEN_SIZE.1 )
		.title("random walker" )
		.msaa_4x()
		.build();
	
	/* Sets the target fps of the program */
	rl.set_target_fps( 60 );
	
	/* Place code to be run once here */
	
	/* Creates a World instance, which currently just holds the entity manager which keeps track of entities */
	let mut entity_manager = EntityManager::new();
	
	/* The following four create stores: Position, Velocity, Acceleration, and Attributes,
	   which contains characteristics like color and mass */
	let mut atr_store = HashStore::new();
	let mut pos_store = HashStore::new();
	let mut vel_store = HashStore::new();
	let mut acc_store = HashStore::new();
	let mut mass_store: HashStore<f32> = HashStore::new();
	let mut gene_store = HashStore::new();
	
	let mut world = World{
		width,
		height,
		default_pos: Vec2::new( &(width/2.0), &0.0 ),
		default_vel: Vec2::new(&0.0, &0.0),
		target_pos: Vec2::new(&(width/2.0), &(height)),
		mutation_chance: 0.1,
		gen_done: false
	};
	
	let mut functions: [Box<dyn FnMut(&mut Vec2)>; 10]  = [
		Box::new(|acc| {
			acc.lerp(&world.target_pos, UnitF::new(0.003));
		}),
		Box::new(|acc| {
			*acc = Vec2::new(&5.5, &-5.4);
		}),
		Box::new(|acc| {
			*acc = Vec2::new(&-2.5, &4.4);
		}),
		Box::new(|acc| {
			*acc = Vec2::new(&7.5, &-15.4);
		}),
		Box::new(|acc| {
			*acc = Vec2::new(&-6.5, &-4.4);
		}),
		Box::new(|acc| {
			*acc = Vec2::new(&-3.5, &12.4);
		}),
		Box::new(|acc| {
			*acc = Vec2::new(&12.5, &7.4);
		}),
		Box::new(|acc| {
			*acc = Vec2::new(&1.5, &8.4);
		}),
		Box::new(|acc| {
			*acc = Vec2::new(&-2.5, &5.4);
		}),
		Box::new(|acc| {
			*acc = Vec2::new(&8.5, &2.4);
		}),
	];
	
	while entity_manager.len() < 10 {
		let entity = entity_manager.next();
		atr_store.add(entity, Attributes {
			color: Color::new(random(100..255), random(100..255), random(100..255), 100),
			radius: 10.0 });
		pos_store.add(entity, world.default_pos);
		vel_store.add(entity, world.default_vel);
		acc_store.add(entity, Vec2::default());
		mass_store.add(entity, 1.01);
		gene_store.add(entity, [
			random(0..10),
			random(0..10),
			random(0..10),
			random(0..10),
			random(0..10),
			random(0..10),
			random(0..10),
			random(0..10),
			random(0..10),
			random(0..10),
		]);
	}
	
	/* This is a counter to keep track of how many times the "draw" loop has iterated */
	let mut pass = 0;
	
	/* Draw
	   Loops until the user closes the window, place code to run each loop in following while loop */
	'_draw_loop: while !rl.window_should_close( ) {
		
		/* Creation of the RayLib draw handle. Drawing functions are members of this object, so must be called from this object */
		let mut display = rl.begin_drawing( &thread );
		display.clear_background(Color::BLACK);
		
		if world.gen_done {
			/* Generates a new population */
			genetic_system(&world, &mut pos_store, &mut vel_store, &mut gene_store);
			world.gen_done = false;
		}
		gene_store.for_each(|entity, dna| {
			if let Some(acc) = acc_store.get_mut(entity) {
				functions[dna[pass % 9]](acc);
			}
		});
		/* Moves entities based on their acceleration and velocity */
		movement_system(&mut pos_store, &mut vel_store, &mut acc_store, &mass_store);
		/* Runs the boundary system which checks if the entity has reached the edges of the screen, if they have their velocities are inverted
		   and reduced based on their mass. This makes the entities bounce off of surfaces. Also limits their positions to the screen */
		boundary_system( &world, &mut vel_store, &mut pos_store, &atr_store );
		/* Runs the render system which draws the entities at their positions as circles */
		render_system( &mut display, &world, &pos_store, &atr_store );
		
		if pass % 10 == 0 {
			world.gen_done = true;
			
			//thread::sleep(time::Duration::from_millis(100));
		}
		
		/* Draws the number of passes of the loop to the top left of the screen */
		let x = format!( "Generation = {}", pass );
		display.draw_text( &x, 12, 12, 20, Color::BLACK );
		pass += 1;
		/* Draws the FPS to the top left of the screen */
		let x = format!( "FPS = {}", display.get_fps() );
		display.draw_text( &x, 12, 32, 20, Color::BLACK );
	}
}
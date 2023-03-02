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
		.resizable()
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
	let mut mass_store: HashStore<Option<f32>> = HashStore::new();
	let mut gene_store = HashStore::new();
	
	let mut world = World{
		width,
		height,
		target_pos: Vec2::new(&(width/2.0), &(height)),
		functions: [
			|(pos, vel, acc): (Vec2, Vec2, Vec2)| {},
			|(pos, vel, acc): (Vec2, Vec2, Vec2)| {},
			|(pos, vel, acc): (Vec2, Vec2, Vec2)| {},
			|(pos, vel, acc): (Vec2, Vec2, Vec2)| {},
			|(pos, vel, acc): (Vec2, Vec2, Vec2)| {},
			|(pos, vel, acc): (Vec2, Vec2, Vec2)| {},
			|(pos, vel, acc): (Vec2, Vec2, Vec2)| {},
			|(pos, vel, acc): (Vec2, Vec2, Vec2)| {},
			|(pos, vel, acc): (Vec2, Vec2, Vec2)| {},
			|(pos, vel, acc): (Vec2, Vec2, Vec2)| {}
		],
	};
	
	while entity_manager.len() < 10 {
		let entity = entity_manager.next();
		atr_store.add(entity, Attributes { color: Color::WHITE, radius: 10.0, row: 0, column: 0 } );
		pos_store.add(entity, Vec2::new( &(width/2.0), &(0.0) ) );
		vel_store.add(entity, Vec2::default() );
		acc_store.add(entity, Vec2::default());
		mass_store.add(entity, None);
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
		/* Creation of a tuple for the current screen size */
		let screen_size: ( i32, i32 ) = ( rl.get_screen_width() , rl.get_screen_height() );
		/* Creation of a tuple with two named values, width and height, which is the screen size converted to floats */
		let ( width, height ) = ( screen_size.0 as f32, screen_size.1 as f32 );
		world.width = width;
		world.height = height;
		
		/* Creation of the RayLib draw handle. Drawing functions are members of this object, so must be called from this object */
		let mut display = rl.begin_drawing( &thread );
		
		display.clear_background(Color::BLACK);
		
		for i in 0..10 {
			/* Runs the boundary system which checks if the entity has reached the edges of the screen, if they have their velocities are inverted
			   and reduced based on their mass. This makes the entities bounce off of surfaces. Also limits their positions to the screen */
			boundary_system( &world, &mut vel_store, &mut pos_store, &atr_store );
			/* Runs the render system which draws the entities at their positions as circles */
			render_system( &mut display, &world, &pos_store, &atr_store );
		}
		genetic_system(&world, &mut pos_store, &mut gene_store);
		
		/* Draws the number of passes of the loop to the top left of the screen */
		let x = format!( "Pass = {}", pass );
		display.draw_text( &x, 12, 12, 20, Color::BLACK );
		pass += 1;
		/* Draws the FPS to the top left of the screen */
		let x = format!( "FPS = {}", display.get_fps() );
		display.draw_text( &x, 12, 32, 20, Color::BLACK );
	}
}
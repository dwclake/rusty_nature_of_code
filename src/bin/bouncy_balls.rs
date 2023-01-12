use std::collections::HashMap;
use std::f32::consts::{TAU,PI};
use rusty_nature_of_code::prelude::*;
use miscmath::prelude::*;
use misc_ecs::prelude::*;
use rand::{Rng, thread_rng};
use raylib::prelude::*;

fn main() {
	
	/* Creation of a constant tuple for the initial screen size */
	const INIT_SCREEN_SIZE: ( i32, i32 ) = ( 640, 480 );
	
	/* Creation of the RayLib handle and thread, sets the screen size, and gives the window a title */
	let ( mut rl, thread ) = init( )
		.size(INIT_SCREEN_SIZE.0, INIT_SCREEN_SIZE.1 )
		.title("rusty_nature_of_code" )
		.resizable()
		.msaa_4x()
		.build();
	
	/* Sets the target fps of the program */
	rl.set_target_fps( 60 );
	
	/* Setup
	   Place code to be run once here */
	
	/* Creates a World instance, which currently just holds the entity manager which keeps track of entities */
	let mut world = World::new();
	
	/* Creates a 2d array of "regions", which are hash maps of entities currently in that "region" */
	const COLUMNS: usize = 5;
	let mut regions: [[HashMap<u64,Entity>; COLUMNS]; COLUMNS] = Default::default();
	
	/* The following four create stores: Position, Velocity, Acceleration, and Attributes,
	   which contains characteristics like color and mass */
	let mut atr_store = HashStore::new();
	let mut pos_store = HashStore::new();
	let mut vel_store = HashStore::new();
	let mut acc_store = HashStore::new();
	
	/* This is a counter to keep track of how many times the "draw" loop has iterated */
	let mut pass = 0;
	
	/* Draw
	   Loops until the user closes the window, put code to run each loop in following while loop */
	'_draw_loop: while !rl.window_should_close( ) {
		
		/* Creation of a tuple for the current screen size */
		let screen_size: ( i32, i32 ) = ( rl.get_screen_width() , rl.get_screen_height() );
		/* Creation of a tuple with two named values, width and height, which is the screen size converted to floats */
		let ( width, height ) = ( screen_size.0 as f32, screen_size.1 as f32 );
		
		/* Creation of the RayLib draw handle. Drawing functions are members of this object, so must be called from this object */
		let mut display = rl.begin_drawing( &thread );
		/* Clears the background and sets it's colour to white */
		display.clear_background( Color::WHITE );
		
		/* Creates entities until there are 10 entities active */
		while world.entity_manager().len() < 10 {
			
			/* Creates a new entity id */
			let entity = world.entity_manager_mut().next();
			/* Generates a random color */
			let color = Color::new( thread_rng().gen_range(100..255),
									thread_rng().gen_range(100..255),
									thread_rng().gen_range(100..255), 255);
			/* Adds the entity with a attribute component, which is the same for every entity in this case */
			atr_store.add(entity, Attributes { mass: random( 1.01..1.56 ), radius: random( 10.0..15.0 ), color, row: 0, column: 0 } );
			/* Add the entity with a random position vector, from x: 0.0 to screen width, y: 300.0 to screen height */
			pos_store.add(entity, Vec2::create_random2(&(0.0..width), &((height - 100.0)..height) ) );
			/* Adds the entity with a random velocity vector with a angle from pi (180) to tau (360) and a magnitude of 5.0 */
			vel_store.add(entity, Vec2::from_angle(&thread_rng().gen_range( PI..TAU ), &Some(5.0) ) );
			/* Adds the entity with a acceleration vector with x and y at 0.0 */
			acc_store.add(entity, Vec2::new() );
		}
		
		/* Applies a downward acceleration to every entity with a acceleration component */
		for ( _, a) in acc_store.iter_mut() {
			a.y = -0.9;
		}
		
		/* Runs the acceleration system, which adds the velocity components to the corresponding acceleration components */
		acceleration_system(&mut acc_store, &mut vel_store);
		/* Constrains all the velocities from -25.0 to 25.0 */
		vel_store.for_each_mut( |_, v| {
			v.constrain( &(-25.0..25.0), &(-25.0..25.0) );
		});
		/* Runs the collision system, which checks for collisions in the entities current region, then swaps their directions
		   Not properly detecting the collisions yet though */
		collision_system(&mut vel_store, &pos_store, &mut atr_store, &mut regions );
		/* Runs the movement system which moves applies the velocity to the position vectors,
		   then calculates what region they are currently in */
		movement_system(screen_size, COLUMNS, &mut vel_store, &mut pos_store, &mut atr_store, &mut regions );
		/* Runs the boundary system which checks if the entity has reached the edges of the screen, if they have their velocities are inverted
		   and reduced based on their mass. This makes the entities bounce off of surfaces. Also limits their positions to the screen */
		boundary_system( screen_size, &mut vel_store, &mut pos_store, &atr_store );
		/* Runs the render system which draws the entities at their positions as circles */
		render_system( &mut display, screen_size, &pos_store, &atr_store );
		/* Runs the drop system, which removes entities. CSystem removes them when they go out of bounds and when they stop moving */
		drop_system( screen_size, world.entity_manager_mut(),
					 &mut acc_store, &mut vel_store, &mut pos_store, &mut atr_store );
		
		/* Draws the number of passes of the loop to the top left of the screen */
		let x = format!( "Pass = {}", pass );
		display.draw_text( &x, 12, 12, 20, Color::BLACK );
		pass += 1;
		/* Draws the FPS to the top left of the screen */
		let x = format!( "FPS = {}", display.get_fps() );
		display.draw_text( &x, 12, 32, 20, Color::BLACK );
	}
	/* Prints out the active entities which has position components when the screen was closed */
	for (entity, p) in pos_store {
		println!( "{:?}: {:?}", entity, p);
	}
	dbg!(regions);
}
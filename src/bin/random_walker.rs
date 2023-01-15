use std::collections::HashMap;
use macroquad::{window::{next_frame, screen_width, screen_height, clear_background}, time::get_fps, prelude::{WHITE, BLACK}, text::draw_text};
use rand::{thread_rng, Rng};
use rusty_nature_of_code::prelude::*;
use miscmath::prelude::*;
use misc_ecs::prelude::*;

#[macroquad::main("BasicShapes")]
async fn main() {
	
	/* Place code to be run once here */
	
	/* Creates a World instance, which currently just holds the entity manager which keeps track of entities */
	let mut entity_manager = EntityManager::new();
	
	/* Creates a 2d array of "regions", which are hash maps of entities currently in that "region" */
	const COLUMNS: usize = 10;
	const ROWS: usize = 10;
	let mut regions: [[HashMap<u64,Entity>; COLUMNS]; ROWS] = Default::default();
	
	/* The following four create stores: Position, Velocity, Acceleration, and Attributes,
	   which contains characteristics like color and mass */
	let mut atr_store = HashStore::new();
	let mut pos_store = HashStore::new();
	let mut vel_store = HashStore::new();
    let mut acc_store = HashStore::new();
	
	/* This is a counter to keep track of how many times the "draw" loop has iterated */
	let mut pass = 0;

	clear_background( BLACK );
	
	/* Draw
	   Loops until the user closes the window, place code to run each loop in following while loop */
	'_draw_loop: loop {
		
		/* Creation of a tuple for the current screen size */
		let screen_size: ( f32, f32 ) = ( screen_width() , screen_height() );
		/* Creation of a tuple with two named values, width and height, which is the screen size converted to floats */
		let ( width, height ) = ( screen_size.0, screen_size.1 );
		
		/* Creates entities until there are 10 entities active */
		while entity_manager.len() < 1 {
			
			/* Creates a new entity id */
			let entity = entity_manager.next();
            atr_store.add(entity, Attributes { mass: 1.0, color: WHITE, radius: 2.0, row: 0, column: 0 } );
			/* Add the entity with a random position vector, from x: 0.0 to screen width, y: 300.0 to screen height */
			pos_store.add(entity, Vec2::create( &(width/2.0), &(height/2.0) ) );
			/* Adds the entity with a random velocity vector with a angle from pi (180) to tau (360) and a magnitude of 5.0 */
			vel_store.add(entity, Vec2::new( ) );
		}

        /* Randomly picks a number between 0 and 3, and then moves the entity left, right, up, or down 
            depending on the number picked */
        match thread_rng().gen_range(0..4) {
            0 => {
                for ( _, vel ) in vel_store.iter_mut() {
                    *vel = Vec2::create( &0.0, &2.0);
                };
            },
            1 => {
                for ( _, vel ) in vel_store.iter_mut() {
                    *vel = Vec2::create( &2.0, &0.0);
                };
            },
            2 => {
                for ( _, vel ) in vel_store.iter_mut() {
                    *vel = Vec2::create( &0.0, &-2.0);
                };
            },
            3 => {
                for ( _, vel ) in vel_store.iter_mut() {
                    *vel = Vec2::create( &-2.0, &0.0);
                };
            },
            _ => (),
        }

		/* Runs the movement system which moves applies the velocity to the position vectors,
		   then calculates what region they are currently in */
		movement_system(screen_size, COLUMNS, ROWS, &mut vel_store, &mut pos_store, &mut atr_store, &mut regions );

		/* Runs the boundary system which checks if the entity has reached the edges of the screen, if they have their velocities are inverted
		   and reduced based on their mass. This makes the entities bounce off of surfaces. Also limits their positions to the screen */
		boundary_system( screen_size, &mut vel_store, &mut pos_store, &atr_store );

		/* Runs the render system which draws the entities at their positions as circles */
		render_system( screen_size, &pos_store, &atr_store );

		/* Runs the drop system, which removes entities. CSystem removes them when they go out of bounds and when they stop moving */
		drop_system( screen_size, &mut entity_manager,
					 &mut acc_store, &mut vel_store, &mut pos_store, &mut atr_store );
		
		/* Draws the number of passes of the loop to the top left of the screen */
		let x = format!( "Pass = {}", pass );
		draw_text( &x, 12.0, 12.0, 20.0, WHITE );
		pass += 1;
		/* Draws the FPS to the top left of the screen */
		let x = format!( "FPS = {}", get_fps() );
		draw_text( &x, 12.0, 32.0, 20.0, WHITE );

        pass += 1;

        next_frame().await;
		
	}
}
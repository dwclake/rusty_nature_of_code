use rand::{thread_rng, Rng, RngCore};
use raylib::prelude::*;
use noise::{NoiseFn, Perlin};

fn main() {
	
	/* Creation of a constant tuple for the initial screen size */
	const INIT_SCREEN_SIZE: ( i32, i32 ) = ( 480, 480 );
	
	/* Creation of the RayLib handle and thread, sets the screen size, and gives the window a title */
	let ( mut rl, thread ) = init( )
		.size(INIT_SCREEN_SIZE.0, INIT_SCREEN_SIZE.1 )
		.title("perlin 1d" )
		.resizable()
		.msaa_4x()
		.build();
	
	/* Sets the target fps of the program */
	rl.set_target_fps( 60 );
	
	/* Place code to be run once here */
	
	/* Creation of a tuple with two named values, width and height, which is the screen size converted to floats */
	let ( width, height ) = ( INIT_SCREEN_SIZE.0 as f32, INIT_SCREEN_SIZE.1 as f32 );
	
	/* Creation of a perlin object, with a random seed */
	let perlin = Perlin::new(thread_rng().next_u32() );
	
	/* Creation of the x offset, which is used to get values from the perlin object */
	let mut x_off = thread_rng().gen_range(0.0..100.0);
	
	/* Draw
	   Loops until the user closes the window, place code to run each loop in following while loop */
	'_draw_loop: while !rl.window_should_close( ) {
		
		/* Creation of the RayLib draw handle. Drawing functions are members of this object, so must be called from this object */
		let mut display = rl.begin_drawing( &thread );
		/* Clears the background and sets it's colour to black */
		display.clear_background( Color::BLACK );
		
		/* Get a value out of perlin, using the current x offset, with y offset set to 0 */
		let val = (perlin.get([x_off, 0.0]).abs() as f32) * width;
		/* Increments offset by a small value to get get smooth randomness */
		x_off += 0.01;
		
		/* Draws a circle with a x pos of the perlin value, and a y pos of 1/2 the screen height */
		display.draw_circle(val as i32, (height / 2.0) as i32, 10.0, Color::WHITE);
	}
}
use rand::{thread_rng, Rng};
use miscmath::prelude::{*, vector::Vec2};
use raylib::prelude::*;
use noise::{NoiseFn, Perlin, Seedable};

fn main() {
	
	/* Creation of a constant tuple for the initial screen size */
	const INIT_SCREEN_SIZE: ( i32, i32 ) = ( 640, 480 );
	
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
	
	/* Creation of a tuple with two named values, width and height, which is the screen size converted to floats */
	let ( width, height ) = ( INIT_SCREEN_SIZE.0 as f32, INIT_SCREEN_SIZE.1 as f32 );
	
	/* This is a counter to keep track of how many times the "draw" loop has iterated */
	let mut pass = 0;
	
	let perlin = Perlin::new(100101 );
	let mut num1 = thread_rng().gen_range(0.0..100.0);
	let mut num2 = thread_rng().gen_range(0.0..100.0);
	
	/* Draw
	   Loops until the user closes the window, place code to run each loop in following while loop */
	'_draw_loop: while !rl.window_should_close( ) {
		
		/* Creation of a tuple for the current screen size */
		//let screen_size: ( i32, i32 ) = ( rl.get_screen_width() , rl.get_screen_height() );
		
		/* Creation of the RayLib draw handle. Drawing functions are members of this object, so must be called from this object */
		let mut display = rl.begin_drawing( &thread );
		/* Clears the background and sets it's colour to black */
		display.clear_background( Color::BLACK );
		
		pass += 1;
		
		num1 += 0.01;
		num2 -= 0.01;
		
		let val = perlin.get([num1, num2]).abs() as f32 * (width - 10.0) + 50.0;
		
		//let val = map(val, -1.0..1.0, 0.0..1.0 ) as i32;
		dbg!(val);
		display.draw_circle(val as i32, (height / 2.0) as i32, 5.0, Color::WHITE);
	}
}
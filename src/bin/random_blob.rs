use miscmath::prelude::*;
use rand::thread_rng;
use raylib::prelude::*;
use rand::prelude::SliceRandom;

fn main() {
	
	/* Creation of a constant tuple for the initial screen size */
	const INIT_SCREEN_SIZE: ( i32, i32 ) = ( 480, 480 );
	
	/* Creation of the RayLib handle and thread, sets the screen size, and gives the window a title */
	let ( mut rl, thread ) = init( )
		.size(INIT_SCREEN_SIZE.0, INIT_SCREEN_SIZE.1 )
		.title("random blob" )
		.resizable()
		.msaa_4x()
		.build();
	
	/* Sets the target fps of the program */
	rl.set_target_fps( 60 );
	
	/* Place code to be run once here */
	
	/* Creation of a tuple with two named values, width and height, which is the screen size converted to floats */
	let ( width, height ) = ( INIT_SCREEN_SIZE.0 as f32, INIT_SCREEN_SIZE.1 as f32 );
	
	/* Creates an array of tuples, each tuple containing a position and a velocity */
	let mut circles = [(Vec2::new( &(width / 2.0), &(height/ 2.0)), Vec2::default()); 500];
	/* Creates an array of closures, each closure applies the velocity to the position differently */
	let functions = [
		|(pos, vel): &mut(Vec2, Vec2)| {
			pos.x *= vel.x;
			pos.y *= vel.y;
		},
		|(pos, vel): &mut(Vec2, Vec2)| {
			pos.x *= vel.x;
			pos.y /= vel.y;
		},
		|(pos, vel): &mut(Vec2, Vec2)| {
			pos.x /= vel.x;
			pos.y *= vel.y;
		},
		|(pos, vel): &mut(Vec2, Vec2)| {
			pos.x /= vel.x;
			pos.y *= vel.y;
		},
		|(pos, vel): &mut(Vec2, Vec2)| {
			pos.x *= vel.y.sqrt();
			pos.y /= vel.x.powf(2.0);
		},
		|(pos, vel): &mut(Vec2, Vec2)| {
			pos.x /= vel.y.sqrt();
			pos.y *= vel.y.powf(2.0);
		},
		|(pos, vel): &mut(Vec2, Vec2)| {
			pos.x -= vel.y.cos();
			pos.y += vel.x.sin();
		},
		|(pos, vel): &mut(Vec2, Vec2)| {
			pos.x += vel.x.cos();
			pos.y -= vel.x.sin();
		}
	];
	/* Instantiates a random number generator */
	let mut rng = thread_rng();
	
	/* Draw
	   Loops until the user closes the window, place code to run each loop in following while loop */
	'_draw_loop: while !rl.window_should_close( ) {
		
		/* Creation of the RayLib draw handle. Drawing functions are members of this object, so must be called from this object */
		let mut display = rl.begin_drawing( &thread );
		/* Clears the background and sets it's colour to black */
		display.clear_background( Color::BLACK );
		
		/* Generate random velocities */
		circles.iter_mut().for_each( |(_, vel)| {
			*vel = Vec2::create_random2(&(0.995..1.005), &(0.995..1.005));
		});
		/* Chooses random closures from the functions array and applies one to each circle */
		circles.iter_mut().for_each(
			functions.choose(&mut rng).unwrap()
		);
		
		/* Draws every circle with a random color */
		for (pos, _) in circles {
			display.draw_circle(pos.x as i32, pos.y as i32, 10.0, Color::WHITE);
		}
	}
}
use rand::{thread_rng, Rng};
use miscmath::prelude::{*, vector::Vec2};
use raylib::prelude::*;

struct Walker {
    pos: Vec2,
    color: Color,
    radius: f32,
}

impl Walker {

    // Creates a new instance of Walker initialized with a radius of 2, position of (0,0), and a color of white
    fn new() -> Walker {
        Walker{
            pos: Vec2::default(),
            color: Color::WHITE,
            radius: 2.0,
        }
    }

    // Adds a Vec2 to the position of the entity
    fn update( &mut self, vel: &Vec2 ) {
        self.pos.add( vel );
    }

    // Draws Walker instance to the screen at the instance's position, with it's color and radius
    fn draw( &self, display: &mut RaylibDrawHandle, screen_size: (i32, i32) ){

        let screen_size = ( screen_size.0 as f32, screen_size.1 as f32 );
        /* Maps the y value, from the range 0->screen_height, to the range screen_height->0, so that the coordinate (0,0) is the bottom left
			   instead of the top left */
        let y = map( self.pos.y, 0.0..screen_size.1, screen_size.1..0.0 );
        /* Draws a circle at the entities position, with the entities radius and color */
        display.draw_circle( self.pos.x as i32, y as i32, self.radius, self.color );
    }
}

fn main() {
	
	/* Creation of a constant tuple for the initial screen size */
	const INIT_SCREEN_SIZE: ( i32, i32 ) = ( 640, 480 );
	
	/* Creation of the RayLib handle and thread, sets the screen size, and gives the window a title */
	let ( mut rl, thread ) = init( )
		.size(INIT_SCREEN_SIZE.0, INIT_SCREEN_SIZE.1 )
		.title("random walker oop" )
		.resizable()
		.msaa_4x()
		.build();
	
	/* Sets the target fps of the program */
	rl.set_target_fps( 60 );
	
	/* Place code to be run once here */

	/* Creation of a tuple with two named values, width and height, which is the screen size converted to floats */
	let ( width, height ) = ( INIT_SCREEN_SIZE.0 as f32, INIT_SCREEN_SIZE.1 as f32 );

	/* Creation of a walker entity positioned at the center of the screen */
	let mut walker = Walker::new();
    walker.pos = Vec2::new( &(width/2.0), &(height/2.0) );

	/* This is a counter to keep track of how many times the "draw" loop has iterated */
	let mut pass = 0;
	
	/* Draw
	   Loops until the user closes the window, place code to run each loop in following while loop */
	'_draw_loop: while !rl.window_should_close( ) {
		
		/* Creation of a tuple for the current screen size */
	    let screen_size: ( i32, i32 ) = ( rl.get_screen_width() , rl.get_screen_height() );
		
		/* Creation of the RayLib draw handle. Drawing functions are members of this object, so must be called from this object */
		let mut display = rl.begin_drawing( &thread );
		/* Clears the background and sets it's colour to black for the first two iterations, as internally the drawing is done on two
            separate "canvases" which are swapped every iteration so background needs to be applied to both, then the background is no
            longer cleared so anything drawn to the screen stays. 
            If only done once with any color other than black, there will be severe flickering */
        if pass < 2 {
		    display.clear_background( Color::BLACK );
        }

        /* Randomly picks a number between 0 and 3, and then moves the entity left, right, up, or down 
            depending on the number picked */
        match thread_rng().gen_range(0..4) {
            0 => {
                    walker.update(&Vec2::new( &0.0, &2.0) );
            },
            1 => {
                    walker.update( &Vec2::new( &2.0, &0.0) );
            },
            2 => {
                    walker.update( &Vec2::new( &0.0, &-2.0) );
            },
            3 => {
                    walker.update( &Vec2::new( &-2.0, &0.0) );
            },
            _ => (),
        }

        walker.draw( &mut display, screen_size);

		pass += 1;
	}
}
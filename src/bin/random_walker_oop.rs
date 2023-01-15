use macroquad::{prelude::{BLACK, WHITE, Color}, window::{clear_background, screen_width, screen_height, next_frame}, shapes::draw_circle, text::draw_text, time::get_fps};
use rand::{thread_rng, Rng};
use miscmath::prelude::*;

struct Walker {
    pos: Vec2,
    color: Color,
    radius: f32,
}

impl Walker {

    // Creates a new instance of Walker initialized with a radius of 2, position of (0,0), and a color of white
    fn new() -> Walker {
        Walker{
            pos: Vec2::new(),
            color: WHITE,
            radius: 2.0,
        }
    }

    // Adds a Vec2 to the position of the entity
    fn update( &mut self, vel: &Vec2 ) {
        self.pos.add( vel );
    }

    // Draws Walker instance to the screen at the instance's position, with it's color and radius
    fn draw( &self, screen_size: (f32, f32) ){

        /* Maps the y value, from the range 0->screen_height, to the range screen_height->0, so that the coordinate (0,0) is the bottom left
			   instead of the top left */
        let y = map( self.pos.y, 0.0..screen_size.1, screen_size.1..0.0 );
        /* Draws a circle at the entities position, with the entities radius and color */
        draw_circle( self.pos.x, y, self.radius, self.color );
    }
}

#[macroquad::main("BasicShapes")]
async fn main() {
	
	/* Place code to be run once here */

	/* Creation of a walker entity positioned at the center of the screen */
	let mut walker = Walker::new();
    walker.pos = Vec2::create( &(screen_width()/2.0), &(screen_height()/2.0) );

	/* This is a counter to keep track of how many times the "draw" loop has iterated */
	let mut pass = 0;

    clear_background( BLACK );
	
	'_draw_loop: loop {
        /* Draw
	        Loops until the user closes the window, place code to run each loop in following while loop */
		
		/* Creation of a tuple for the current screen size */
	    let screen_size: ( f32, f32 ) = ( screen_width() , screen_height() );
		
		/* Clears the background and sets it's colour to black for the first two iterations, as internally the drawing is done on two
            separate "canvases" which are swapped every iteration so background needs to be applied to both, then the background is no
            longer cleared so anything drawn to the screen stays. 
            If only done once with any color other than black, there will be severe flickering */

        /* Randomly picks a number between 0 and 3, and then moves the entity left, right, up, or down 
            depending on the number picked */
        match thread_rng().gen_range(0..4) {
            0 => {
                    walker.update(&Vec2::create( &0.0, &2.0) );
            },
            1 => {
                    walker.update( &Vec2::create( &2.0, &0.0) );
            },
            2 => {
                    walker.update( &Vec2::create( &0.0, &-2.0) );
            },
            3 => {
                    walker.update( &Vec2::create( &-2.0, &0.0) );
            },
            _ => (),
        }

        walker.draw( screen_size );

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
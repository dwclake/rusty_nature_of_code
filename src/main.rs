use rusty_nature_of_code::prelude::*;
use raylib::prelude::*;

fn main() {
    
    // Creation of a constant tuple for the screen size, so it can be passed as an argument easily
    const SCREEN_SIZE: ( i32, i32 ) = ( 640, 480 );
    // Creation of a tuple with two named values, width and height, which is the screen size converted to floats
    let ( _width, _height ) = ( SCREEN_SIZE.0 as f32, SCREEN_SIZE.1 as f32 );

    // Creation of the RayLib handle and thread, sets the screen size, and gives the window a title
    let ( mut rl, thread ) = init( )
            .size(SCREEN_SIZE.0, SCREEN_SIZE.1 )
            .title("rusty_nature_of_code" )
            .build();
    
    // Sets the target fps of the program
    rl.set_target_fps( 60 );
    
    // Setup
    // Place code to be run once here
    let perlin = Noise::new( DEFAULT_NOISE_SEED );
    dbg!( perlin );
    
    // Draw
    // Loops until the user closes the window, put code to run each loop in following while loop
    while !rl.window_should_close( ) {
        
        // Creation of the RayLib draw handle. Drawing functions are members of this object, so must be called from this object
        let mut display = rl.begin_drawing( &thread );
        
        // Clears the background and sets it's colour to white
        display.clear_background( Color::WHITE );
        
        
        
    }
}
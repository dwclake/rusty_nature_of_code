use raylib::prelude::*;

fn main() {
    
    const SCREEN_SIZE: ( i32, i32 ) = ( 640, 480 );
    let ( width, height ) = ( SCREEN_SIZE.0 as f32, SCREEN_SIZE.1 as f32 );

    let ( mut rl, thread ) = init( )
            .size(SCREEN_SIZE.0, SCREEN_SIZE.1)
            .title("rusty_nature_of_code")
            .build();
    
    rl.set_target_fps( 60 );
    
    while !rl.window_should_close( ) {
        
        let mut display = rl.begin_drawing( &thread );
        display.clear_background( Color::WHITE );
        
        
    }
}
pub mod data;
pub mod systems;

use std::ops::Range;
use miscmath::prelude::Vec2;
use rand::{Rng, thread_rng};
use rand::distributions::uniform::{SampleRange, SampleUniform};

pub const DEFAULT_NOISE_SEED: usize = 0;

/// Generates a random number in the range provided
///
/// # Examples
///
/// ```
/// use rusty_nature_of_code::noc::random;
///
/// let a = random( 0..10 );
/// let b = random( 0.0..10.0 );
///
/// assert!( ( a < 10 ) && ( a >= 0 ) );
/// assert!( (b < 10.0 ) && ( b > 0.0 ) );
/// ```
///
pub fn random<T: SampleUniform>( rng: Range<T> ) -> T
		                       where Range<T>: SampleRange<T>,{
	
	thread_rng().gen_range( rng )
}

/// Perlin Noise struct
///
/// # Examples
///
/// ```
/// use rusty_nature_of_code::prelude::*;
///
/// let perlin = Noise::new( DEFAULT_NOISE_SEED );
/// ```
///
#[derive(Debug)]
pub struct Noise {
	pub seed: usize,
	permutation_table: [i16; 256]
}

impl Noise {
	
	/// Initializes and returns a new Noise object
	///
	/// # Examples
	///
	/// ```
	/// use rusty_nature_of_code::prelude::*;
	///
	/// let perlin = Noise::new( DEFAULT_NOISE_SEED );
	///
	/// ```
	///
	pub fn new( seed: usize ) -> Noise {
		let mut noise = Noise{
			seed,
			permutation_table: [-1; 256],
		};
		
		// Sets each element to a number from 0 to 256, with no repeating values
		for i in 0..256 {
			
			// Finds a element that hasn't been set yet to assign to i
			Noise::permutation_gen( &mut noise.permutation_table, i );
		}
		
		// todo: implement the rest of the perlin noise algorithm, using the randomly generated permutation table to generate a array of interpolated
		//       pseudo-random values
		
		noise
	}
	
	/// RECURSIVE: Takes i and adds it to the permutation table at a random index, as long as that element hasn't been set yet (is still -1).
	/// If the element picked has been set already this function is called recursively until it picks a non-set element.
	fn permutation_gen(table: &mut [i16; 256], i: i16 ) {
		// Pick a random int between 0 and 255
		let index = random( 0..256 );
		
		// Base case: If a number has not been assigned to that element yet, assign it i
		if table[index] == -1 {
			table[index] = i;
		} else if i <= 256 { // Recursive case: If a number has been assigned to that element, try again
			Noise::permutation_gen( table, i );
		}
	}
	
	/// Generates a value between 0 and 1 using perlin Noise
	///
	/// # Examples
	///
	/// ```
	///
	/// ```
	///
	pub fn get<T>(_time: T) {
		todo!()
	}
	
}
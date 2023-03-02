use misc_ecs::prelude::{Entity, Store};
use miscmath::{map, random};
use miscmath::prelude::Vec2;
use raylib::prelude::{Color, RaylibDraw, RaylibDrawHandle};
use crate::smart_rockets::choose_two;
use crate::smart_rockets::prelude::{Attributes, World};

pub fn render_system<P: Store<Vec2>, A: Store<Attributes>>(display: &mut RaylibDrawHandle,
														   world: &World,
														   pos_store: &P,
														   atr_store: &A ) {
	/* Applies a closure for each entity with a position */
	pos_store.for_each( | entity, pos| {
		
		/* If the current entity has a attributes component, assign it to atr and run the following code block */
		if let Some( atr ) = atr_store.get( entity ) {
			
			/* Maps the y value, from the range 0->screen_height, to the range screen_height->0, so that the coordinate (0,0) is the bottom left
			   instead of the top left */
			let y = map( pos.y, 0.0..world.height, world.height..0.0 );
			
			/* Draws a circle at the entities position, with the entities radius and color */
			display.draw_circle(pos.x as i32, y as i32, atr.radius, atr.color );
		}
	});
	
	let y = map( world.target_pos.y, 0.0..world.height, world.height..0.0 );
	display.draw_circle(world.target_pos.x as i32, y as i32, 25.0, Color::RED);
}

/// Bounds entities position to be within the screen, and inverts their velocities when they hit the edges
///
/// # Examples
///
/// ```
///
/// ```
///
pub fn boundary_system<V: Store<Vec2>, P: Store<Vec2>, A: Store<Attributes>>(world: &World,
																			 vel_store: &mut V,
																			 pos_store: &mut P,
																			 atr_store: &A ) {
	/* Applies a closure to each entity with a pos component */
	pos_store.for_each_mut( | entity, pos | {
		
		/* Gets current entities attributes component */
		let atr = atr_store.get( entity ).unwrap();
		
		/* If current entity has a velocity component, assign it to vel then run the following code block */
		if let Some( vel ) = vel_store.get_mut( entity ) {
			
			/* Constrain entities position to within the screen, factoring in the size of the entity */
			pos.constrain( &((atr.radius)..(world.width - atr.radius)), &((atr.radius)..(world.height - atr.radius)) );
			
			/* If entity hits the edges, invert their velocity and reduce it based on mass, then reduce it based on surface friction */
			if pos.x - atr.radius < 0.00000001 { vel.x *= -1.0; vel.y *= 0.99; }
			if pos.y - atr.radius < 0.00000001 { vel.y *= -1.0; vel.x *= 0.9; }
			if pos.x > world.width - atr.radius - 0.0001 { vel.x *= -1.0; vel.y *= 0.99; }
			if pos.y > world.height - atr.radius - 0.0001 { vel.y *= -1.0; vel.x *= 0.95; }
		}
	});
	
}

/// Generates the new population
///
/// # Examples
///
/// ```
///
/// ```
///
pub fn genetic_system<P: Store<Vec2>, G: Store<[i8; 10]>>(world: &World, pos_store: &mut P, gene_store: &mut G) {
	
	let mut weights: [(Entity, f32);10] = Default::default();
	let mut i = 0;
	
	/* Calculates the entities distance from the target, and assigns a weight respectively */
	gene_store.for_each( |entity, dna| {
		
		if let Some( pos ) = pos_store.get( entity ) {
			let distance = pos.dist( &world.target_pos );
			let weight = map(distance, 0.0..world.width, 1.0..0.0);
			weights[i] = (entity, weight);
		}
		i = i + 1;
	});
	
	/* Clones the dna of two randomly chosen entities based on their weights */
	let (entity_a, entity_b) = choose_two(&weights);
	let dna_a = gene_store.get( entity_a ).unwrap().clone();
	let dna_b = gene_store.get( entity_b ).unwrap().clone();
	
	/* Iterates through each entity and generates new dna */
	gene_store.for_each_mut( |entity, dna| {
		/* Resets the position of the rockets */
		if let Some(pos) = pos_store.get_mut( entity ) {
			*pos = Vec2::new( &(world.width/2.0), &(0.0));
		}
		
		/* Generates a temp dna */
		let mut temp_dna: [i8; 10] = Default::default();
		let mut i = 0;
		/* Assigns each element of temp_dna the value of either dna_a or dna_b's element at the same index */
		for gene in &mut temp_dna {
			match random(0..2) {
				0 => {
					*gene = dna_a[i];
				},
				1 => {
					*gene = dna_b[i];
				},
				_ => {}
			}
			i = i + 1;
		}
		/* Assigns entities dna the newly generated dna */
		*dna = temp_dna;
	});
	
}
use super::data::*;
use miscmath::prelude::*;
use misc_ecs::prelude::*;
use raylib::prelude::*;
use std::collections::HashMap;

/// Applies the vel to each corresponding pos, then calculates the region, or grid that current entity is in
///
/// # Examples
///
/// ```
///
/// ```
///
pub fn movement_system< V: Store<Vec2>, P: Store<Vec2>, A: Store<Attributes> >(screen_size: (i32, i32),
																			   columns: usize,
																			   vel_store: &mut V,
																			   pos_store: &mut P,
																			   atr_store: &mut A,
																			   regions: &mut [HashMap<u64,Entity>] ) {
	/* Converts the screen_size tuple to f32 to be used in calculations */
	let screen_size = ( screen_size.0 as f32, screen_size.1 as f32 );
	
	/* Apply a closure to each pos in the pos component store */
	pos_store.for_each_mut( | entity, pos | {
		
		/* Gets the attribute component for the current entity */
		let atr = atr_store.get_mut( entity ).unwrap();
		
		/* If current entity has a velocity component, assign it to vel and run the following code block */
		if let Some( vel ) = vel_store.get_mut( entity ) {
			
			/* Adds the velocity component to the position component */
			pos.add( vel );
			
			/* Removes the current entity from it's current region (it may still be in this region and will be
			   added back or it may have moved to a new region) */
			regions[atr.region].remove( &entity.id() );
			/* Calculates the column number of the current region or grid and clamps that value to the
			   max number of columns based on the number of regions */
			let mut col = ( (pos.x / screen_size.0) * 10.0 ) as usize;
			col = col.clamp( 1, columns - 1 );
			/* Calculates the row number of the current region or grid and clamps that value to the
			   max number of rows based on the number of regions */
			let mut row = ( (pos.y / screen_size.1) * 10.0 ) as usize;
			row = row.clamp( 1, columns - 1 );
			/* Uses the row and column to calculate the index number for that region in the vector of regions and
			   clamps the value so it can't go out of bounds */
			let mut region_index: usize = col + row * columns;
			region_index = region_index.clamp( 0, regions.len() - 1 );
			/* Assigns the entities new region */
			atr.region = region_index;
			/* Inserts the entity into the map, with its u64 id value as the key */
			regions[region_index].insert( entity.id(), entity );
		}
	});
}

/// Bounds entities position to be within the screen, and inverts their velocities when they hit the edges
///
/// # Examples
///
/// ```
///
/// ```
///
pub fn boundary_system< V: Store<Vec2>, P: Store<Vec2>, A: Store<Attributes> >( screen_size: (i32, i32),
																				vel_store: &mut V,
																				pos_store: &mut P,
																				atr_store: &A ) {
	/* Converts screen_size tuple to f32 for use in calculations */
	let screen_size = ( screen_size.0 as f32, screen_size.1 as f32 );
	
	/* Applies a closure to each entity with a pos component */
	pos_store.for_each_mut( | entity, pos | {
		
		/* Gets current entities attributes component */
		let atr = atr_store.get( entity ).unwrap();
		/* If current entity has a velocity component, assign it to vel then run the following code block */
		if let Some( vel ) = vel_store.get_mut( entity ) {
			
			/* Constrain entities position to within the screen, factoring in the size of the entity */
			pos.constrain( &((atr.radius)..(screen_size.0 - atr.radius)), &((atr.radius)..(screen_size.1 - atr.radius)) );
			/* If entity hits the edges, invert their velocity and reduce it based on mass, then reduce it based on surface friction */
			if pos.x - atr.radius < 0.00000001 { vel.x *= -1.0 / atr.mass; vel.y *= 0.99; }
			if pos.y - atr.radius < 0.00000001 { vel.y *= -1.0 / atr.mass; vel.x *= 0.9; }
			if pos.x > screen_size.0 - atr.radius - 0.0001 { vel.x *= -1.0 / atr.mass; vel.y *= 0.99; }
			if pos.y > screen_size.1 - atr.radius - 0.0001 { vel.y *= -1.0 / atr.mass; vel.x *= 0.95; }
		}
	});
	
}

/// Applies the acceleration components to the velocity components
///
/// # Examples
///
/// ```
///
/// ```
///
pub fn acceleration_system< A: Store<Vec2>, V: Store<Vec2> >(acc_store: &mut A, vel_store: &mut V ) {
	
	/* Defines a closure, taking a Entity and a mutable reference of a Vec2, which adds the acc to the vel */
	let apply_force = | entity: Entity, vel: &mut Vec2 | {
		/* If current entity has a acceleration component, assign it to acc and run the following code block */
		if let Some( acc) = acc_store.get_mut( entity ) {
			
			/* Adds acc to vel, then zeroes out the acceleration */
			vel.add(acc );
			/* Zero out the acceleration afterwards */
			acc.x = 0.0;
			acc.y = 0.0;
		}
	};
	/* Iterates through every entity with vel components and adds their acc to it using the closure above */
	vel_store.for_each_mut( apply_force );
}

/// Detects collisions and swaps the velocity directions of the entities which collide
///
/// # Examples
///
/// ```
///
/// ```
///
pub fn collision_system< V: Store<Vec2>, P: Store<Vec2>, AT: Store<Attributes> >(vel_store: &mut V,
																				 pos_store: &P,
																				 atr_store: &mut AT,
																				 regions: &mut [HashMap<u64,Entity>] ) {
	/* Create a vector to store entities which collide with each other */
	let mut collisions = Vec::new( );
	
	/* Apply a closure to each entity with a pos component */
	pos_store.for_each( | entity_a, pos_a| {
		
		/* Get current entity A's attributes */
		let atr_a = atr_store.get( entity_a ).unwrap();
		/* Apply a closure to each entity in the same region as entity A */
		regions[atr_a.region].iter().for_each( | entity_b | {
			
			/* Get the pos of another entity, entity B, in entity A's region */
			let pos_b = pos_store.get( *entity_b.1 ).unwrap();
			/* Calculate the combined radius of entity A and entity B */
			let rad = atr_a.radius + atr_store.get( *entity_b.1 ).unwrap().radius;
			/* If the distance squared between entity A and B is equal to the combined radius squared,
			   add entity A and B to the collisions list as a tuple */
			if pos_a.dist_sq( pos_b ).abs() - rad.powf( 2.0 ) < 0.0001 && ( entity_b.1.id() != entity_a.id() ) {
				collisions.push((entity_a, *entity_b.1));
			}
		});
	});
	
	/* Iterate through each collision pair tuple in the collisions vector */
	for ( entity_a, entity_b ) in collisions {
		
		/* Create some copies of entity A's and entity B's velocity components */
		let mut vel_a1 = *vel_store.get( entity_a ).unwrap();
		let mut vel_b = *vel_store.get( entity_b ).unwrap();
		let vel_a2 = *vel_store.get( entity_a ).unwrap();
		
		/* Get the attributes components for the entities, will be used to reduce velocity based on their respective masses */
		let _atr_a = atr_store.get( entity_a ).unwrap();
		let _atr_b = atr_store.get( entity_b ).unwrap();
		
		/* Swaps the angles of the velocity component of the entities */
		vel_a1.set_theta( &vel_b.theta() );
		vel_b.set_theta( &vel_a2.theta() );
		
		/* Assigns the swapped velocities back to the entities */
		*vel_store.get_mut( entity_a ).unwrap() = vel_a1;
		*vel_store.get_mut( entity_b ).unwrap() = vel_b;
	}
}

/// Draws circles at the position component of the entities
///
/// # Examples
///
/// ```
///
/// ```
///
pub fn render_system< P: Store<Vec2>, A: Store<Attributes> >(display: &mut RaylibDrawHandle,
															 screen_size: (i32, i32),
															 pos_store: &P,
															 atr_store: &A ) {
	/* Converts screen_size tuple to f32, for use in calculations */
	let screen_size = ( screen_size.0 as f32, screen_size.1 as f32 );
	
	/* Applies a closure for each entity with a position */
	pos_store.for_each( | entity, pos| {
		
		/* If the current entity has a attributes component, assign it to atr and run the following code block */
		if let Some( atr ) = atr_store.get( entity ) {
			
			/* Maps the y value, from the range 0->screen_height, to the range screen_height->0, so that the coordinate (0,0) is the bottom left
			   instead of the top left */
			let y = map( pos.y, 0.0..screen_size.1, screen_size.1..0.0 );
			
			/* Draws a circle at the entities position, with the entities radius and color */
			display.draw_circle(pos.x as i32, y as i32, atr.radius, atr.color );
		}
	});
}

/// Drops entities which go out of bounds
///
/// # Examples
///
/// ```
///
/// ```
///
pub fn drop_system< AT: Store<Attributes>, P: Store<Vec2>, V: Store<Vec2>, A: Store<Vec2> >(screen_size: (i32, i32),
																							entity_manager: &mut EntityManager,
																							acc_store: &mut A,
																							vel_store: &mut V,
																							pos_store: &mut P,
																							atr_store: &mut AT ) {
	/* Creates a vector of entities which will be dropped */
	let mut to_drop = Vec::new( );
	
	/* Converts screen_size tuple to f32, for use in calculations */
	let screen_size = ( screen_size.0 as f32, screen_size.1 as f32 );
	
	/* Checks each entity to see if it's position is out of bounds. If it is, the entity is added to the to_drop vector */
	pos_store.for_each( | entity, pos | {
		if pos.x > screen_size.0 + 2.0 || pos.x < -2.0 || pos.y > screen_size.1 + 2.0 || pos.y < -2.0 {
			to_drop.push( entity );
		}
	});
	
	/* Checks if they are resting on the bottom edge of screen, if they are adds them to the drop list */
	vel_store.for_each( | entity, vel | {
		let pos = pos_store.get( entity ).unwrap();
		let atr = atr_store.get( entity ).unwrap();
		if vel.x < 0.38 && vel.y < 0.38 && pos.y - atr.radius < 0.40 {
			to_drop.push(entity);
		}
	});

	/* For each entity in the to_drop vector, drop it from the entity manager and each components store */
	for mut td in to_drop {
		entity_manager.drop( &mut td );
		pos_store.drop( td );
		vel_store.drop( td );
		acc_store.drop( td );
		atr_store.drop( td );
	}
}
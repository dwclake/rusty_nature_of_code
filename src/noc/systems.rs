use super::data::*;
use miscmath::prelude::*;
use misc_ecs::prelude::*;
use raylib::prelude::*;
use std::collections::HashMap;

///
///
/// # Examples
///
/// ```
///
/// ```
///
pub fn movement_system< V: Store<Vec2>, P: Store<Vec2>, A: Store<Attributes> >(screen_size: (i32, i32), vel: &mut V, pos: &mut P, atr: &mut A, regions: &mut Vec<HashMap<u64,EntityID>> ) {
	
	let screen_size = ( screen_size.0 as f32, screen_size.1 as f32 );
	pos.for_each_mut( | entity, p | {
		
		let at = atr.get_mut( entity ).unwrap();
		if let Some( v ) = vel.get_mut( entity ) {
			
			p.add( v );
			
			regions[at.region].remove( &entity.id() );
			
			let mut col = ( (p.x / screen_size.0) * 10.0 ) as usize;
			col = col.clamp( 1, regions.len() / ( regions.len() / 4 ) + 1 );
			
			let mut row = ( (p.y / screen_size.1) * 10.0 ) as usize;
			row = row.clamp( 1, regions.len() / ( regions.len() / 4 ) + 1 );
			
			let mut region_index: usize = col * row - 1;
			region_index = region_index.clamp( 0, regions.len() - 1 );
			at.region = region_index;
			
			regions[region_index].insert( entity.id(), entity );
		}
	});
}

///
///
/// # Examples
///
/// ```
///
/// ```
///
pub fn boundary_system< V: Store<Vec2>, P: Store<Vec2>, A: Store<Attributes> >( screen_size: (i32, i32), vel: &mut V, pos: &mut P, atr: &A ) {
	
	let screen_size = ( screen_size.0 as f32, screen_size.1 as f32 );
	pos.for_each_mut( | entity, p | {
		
		let at = atr.get( entity ).unwrap();
		if let Some( v ) = vel.get_mut( entity ) {
			
			p.constrain( &((at.radius)..(screen_size.0 - at.radius)), &((at.radius)..(screen_size.1 - at.radius)) );
			
			if p.x - at.radius < 0.00000001 { v.x *= -1.0 / at.mass; v.y *= 0.99; }
			if p.y - at.radius < 0.00000001 { v.y *= -1.0 / at.mass; v.x *= 0.99; }
			if p.x > screen_size.0 - at.radius - 0.0001 { v.x *= -1.0 / at.mass; v.y *= 0.99; }
			if p.y > screen_size.1 - at.radius - 0.0001 { v.y *= -1.0 / at.mass; v.x *= 0.99; }
		}
	});
	
}

///
///
/// # Examples
///
/// ```
///
/// ```
///
pub fn acceleration_system< A: Store<Vec2>, V: Store<Vec2> >(acc: &mut A, vel: &mut V ) {
	
	// Defines a closure, taking a EntityID and a mutable reference ot a Vec2, which adds the acceleration to the velocity
	let apply_force = | entity: EntityID, velocity: &mut Vec2 | {
		if let Some(a) = acc.get_mut( entity ) {
			// Adds acceleration to velocity, then zeroes out the acceleration
			velocity.add(a);
			a.x = 0.0;
			a.y = 0.0;
		}
	};
	
	// Iterates through every entity with velocity components and adds their acceleration to it using the closure above
	vel.for_each_mut( apply_force );
}

///
///
/// # Examples
///
/// ```
///
/// ```
///
pub fn collision_system< V: Store<Vec2>, P: Store<Vec2>, AT: Store<Attributes> >(vel: &mut V, pos: &P, atr: &mut AT, regions: &mut Vec<HashMap<u64,EntityID>>) {
	
	let mut collisions = Vec::new( );
	pos.for_each( |entity, position| {
		let at = atr.get( entity ).unwrap();
		regions[at.region].iter().for_each( |ent| {
			
			let pos2 = pos.get( *ent.1 ).unwrap();
			let rad = at.radius + atr.get( *ent.1 ).unwrap().radius;
			
			if position.dist_sq( pos2 ).abs() - rad.powf( 2.0 ) < 0.0001 && ( ent.1.id() != entity.id() ) {
				collisions.push((entity, *ent.1));
			}
		});
	});
	
	for ( out_entity, in_entity ) in collisions {
		
		let mut v_out1 = *vel.get( out_entity ).unwrap();
		let mut v_in = *vel.get( in_entity ).unwrap();
		let v_out2 = *vel.get( out_entity ).unwrap();
		
		let _atr_out = atr.get( out_entity ).unwrap();
		let _atr_in = atr.get( in_entity ).unwrap();
		
		v_out1.set_theta( &v_in.theta() );
		v_in.set_theta( &v_out2.theta() );
		
		*vel.get_mut( out_entity ).unwrap() = v_out1;
		*vel.get_mut( in_entity ).unwrap() = v_in;
	}
}

///
///
/// # Examples
///
/// ```
///
/// ```
///
pub fn render_system< P: Store<Vec2>, A: Store<Attributes> >(display: &mut RaylibDrawHandle, screen_size: (i32, i32), pos: &P, atr: &A ) {
	
	let screen_size = ( screen_size.0 as f32, screen_size.1 as f32 );
	
	pos.for_each( |entity, p| {
		if let Some( st ) = atr.get( entity ) {
			let y = map( p.y, 0.0..screen_size.1, screen_size.1..0.0 );
			display.draw_circle(p.x as i32, y as i32, st.radius as f32, st.color );
		}
	});
}

///
///
/// # Examples
///
/// ```
///
/// ```
///
pub fn drop_system< AT: Store<Attributes>, P: Store<Vec2>, V: Store<Vec2>, A: Store<Vec2> >(screen_size: (i32, i32), entity_manager: &mut EntityManager, acc: &mut A, vel: &mut V, pos: &mut P, atr: &mut AT ) {
	
	let mut to_drop = Vec::new( );
	let screen_size = ( screen_size.0 as f32, screen_size.1 as f32 );
	
	pos.for_each( | entity, p | {
		if p.x > screen_size.0 + 2.0 || p.x < -2.0 || p.y > screen_size.1 + 2.0 || p.x < -2.0 {
			to_drop.push( entity );
		}
	});

	for mut td in to_drop {
		entity_manager.drop( &mut td );
		pos.drop( td );
		vel.drop( td );
		acc.drop( td );
		atr.drop( td );
	}
}
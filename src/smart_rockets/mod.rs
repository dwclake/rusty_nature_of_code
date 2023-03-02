use misc_ecs::prelude::Entity;
use rand::prelude::SliceRandom;
use rand::thread_rng;

pub mod data;
pub mod systems;
pub mod prelude;

/// Chooses two entities based on their weights
///
/// # Examples
///
/// ```
///
/// ```
///
pub fn choose_two(weights: &[(Entity, f32);10]) -> (Entity, Entity) {
	let mut rng = thread_rng();
	let chosen = weights.choose_multiple_weighted(
		&mut rng,
		2,
		|entity| { entity.1 }
	);
	
	match chosen {
		Ok(mut entities) => {
			(entities.next().unwrap().0, entities.next().unwrap().0)
		}
		Err(_) => {
			(Entity::default(), Entity::default())
		}
	}
}
use misc_ecs::prelude::Entity;
use rand::distributions::WeightedError;
use rand::prelude::{SliceRandom, ThreadRng};
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
pub fn choose_two(weights: &[(Entity, u32);10]) -> Result<(&Entity, &Entity), WeightedError> {
	let mut rng = thread_rng();
	let first_chosen = choose_one(weights, &mut rng)?;
	let mut second_chosen = choose_one(weights, &mut rng)?;
	while first_chosen == second_chosen {
		second_chosen = choose_one(weights, &mut rng)?;
	}
	Ok((first_chosen, second_chosen))
}

fn choose_one<'a>(weights: &'a [(Entity, u32);10], rng: &mut ThreadRng) -> Result<&'a Entity, WeightedError> {
	
	return match weights.choose_weighted(rng, |entity| { entity.1 }) {
		Ok((entity, _)) => {
			Ok(entity)
		}
		Err(e) => {
			Err(e)
		}
	};
}
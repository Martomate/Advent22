use std::collections::HashMap;
use std::hash::Hash;

use super::{
    blueprint::Blueprint,
    resource::{Resource, ResourceSlice},
};

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct SimulationState {
    pub resources: ResourceSlice<u8>,
    pub robots: ResourceSlice<u8>,
    pub steps_left: u8,
}

fn arithmetic_sum(start: u8, count: u8) -> u8 {
    ((start + (start + count - 1)) as u16 * count as u16 / 2).min(255) as u8
}

impl Hash for SimulationState {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_u8(self.resources[Resource::Ore]);
        state.write_u8(self.resources[Resource::Clay]);
        state.write_u8(self.resources[Resource::Obsidian]);
        state.write_u8(self.robots[Resource::Ore]);
        state.write_u8(self.robots[Resource::Clay]);
        state.write_u8(self.robots[Resource::Obsidian]);
        state.write_u8(self.steps_left);
    }
}

impl SimulationState {
    /// Returns an upper bound on how many Geodes there can be when the simulation ends.
    /// This bound is impossible to exceed (that's the whole point).
    ///
    /// Assume one geode robot is built every timestep;
    /// then we get `e + r + (r+1) + (r+2) + ... + (r + s - 1)` (e: existing, g: robots, s: steps)
    fn geode_upper_bound(&self) -> u8 {
        if self.steps_left == 0 {
            self.resources[Resource::Geode]
        } else {
            self.resources[Resource::Geode]
                + arithmetic_sum(self.robots[Resource::Geode], self.steps_left)
        }
    }
}

const CACHE_LIMIT: u8 = 4;

pub fn simulate(
    blueprint: &Blueprint,
    state: SimulationState,
    cache: &mut HashMap<SimulationState, u8>,
    next_states: &mut [[SimulationState; 4]],
    best_so_far: u8,
) -> u8 {
    if state.steps_left == 0 {
        return state.resources[Resource::Geode];
    }

    if state.steps_left > CACHE_LIMIT {
        if let Some(&value) = cache.get(&state) {
            return value;
        }
    }

    let mut best_result: u8 = best_so_far;

    let (candidates, next_states) = next_states.split_first_mut().unwrap();
    let num_candidates = simulation_candidates(blueprint, &state, candidates);

    #[allow(clippy::needless_range_loop)]
    for i in 0..num_candidates {
        let new_state = &candidates[i];
        // only try to find something better if it's not impossible
        let upper_bound = new_state.geode_upper_bound();
        if upper_bound > best_result {
            let result = simulate(
                blueprint,
                new_state.clone(),
                cache,
                next_states,
                best_result,
            );

            if result > best_result {
                best_result = result;
            }
        }
    }

    // for each existing robot, queue one unit of production
    // maybe start building a new robot, and queue one robot of production
    // for each item in the queue add it to the stock

    if state.steps_left > CACHE_LIMIT {
        cache.insert(state.clone(), best_result);
    }

    best_result
}

fn simulation_candidates(
    blueprint: &Blueprint,
    current_state: &SimulationState,
    candidates: &mut [SimulationState; 4],
) -> usize {
    let SimulationState {
        resources,
        robots,
        steps_left,
    } = *current_state;

    let mut num_candidates = 0;

    for r in [
        Resource::Geode,
        Resource::Obsidian,
        Resource::Clay,
        Resource::Ore,
    ] {
        if blueprint.recipes[r].affordable(resources) {
            let new_state = SimulationState {
                resources: resources + robots - blueprint.recipes[r].input,
                robots: robots.with(r, robots[r] + 1),
                steps_left: steps_left - 1,
            };

            candidates[num_candidates] = new_state;
            num_candidates += 1;
        }
    }

    // Don't build anything yet (but only if there is something better to save resources for)
    if num_candidates < 4 {
        let new_state = SimulationState {
            resources: resources + robots,
            robots,
            steps_left: steps_left - 1,
        };

        candidates[num_candidates] = new_state;
        num_candidates += 1;
    }

    num_candidates
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn arithmetic_sum_from_5_to_6_is_11() {
        assert_eq!(arithmetic_sum(5, 2), 11);
    }

    #[test]
    fn arithmetic_sum_from_5_to_7_is_18() {
        assert_eq!(arithmetic_sum(5, 3), 18);
    }
}

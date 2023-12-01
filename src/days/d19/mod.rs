use std::collections::HashMap;

use blueprint::Blueprint;
use resource::{Resource, ResourceSlice};
use simulation::SimulationState;

mod blueprint;
mod parser;
mod resource;
mod simulation;

pub struct Day;

impl super::Runner for Day {
    type T = u32;
    
    fn run(input: &str, basic: bool) -> Self::T {
        let lines: Vec<_> = input.lines().collect();
        run_program(&lines, if basic { 24 } else { 32 }, !basic)
    }
}

const MAX_DEPTH: usize = 32;

pub fn run_program(lines: &[&str], steps: u8, perform_sum: bool) -> u32 {
    assert!(
        steps as usize <= MAX_DEPTH,
        "steps may not be more than {}",
        MAX_DEPTH
    );

    let blueprints: Vec<Blueprint> = parser::parse_blueprints(lines).unwrap();

    // TODO: use rayon thread pool for the blueprints

    let result = if perform_sum {
        blueprints
            .iter()
            .map(|b| {
                let mut cache = HashMap::new();
                let mut next_states: [_; MAX_DEPTH] = Default::default();
                let quality = simulation::simulate(
                    b,
                    SimulationState {
                        resources: ResourceSlice::new(),
                        robots: ResourceSlice::new().with(Resource::Ore, 1),
                        steps_left: steps,
                    },
                    &mut cache,
                    &mut next_states,
                    0,
                );
                println!("{}: {}", b.id, quality);
                quality as u32 * b.id as u32
            })
            .sum()
    } else {
        blueprints
            .iter()
            .take(3)
            .map(|b| {
                let mut cache = HashMap::new();
                let mut next_states: [_; MAX_DEPTH] = Default::default();
                let quality = simulation::simulate(
                    b,
                    SimulationState {
                        resources: ResourceSlice::new(),
                        robots: ResourceSlice::new().with(Resource::Ore, 1),
                        steps_left: steps,
                    },
                    &mut cache,
                    &mut next_states,
                    0,
                );
                println!("{}: {}", b.id, quality);
                quality as u32
            })
            .product()
    };

    result
}

#[cfg(test)]
mod tests {
    use super::run_program;

    #[test]
    fn example_works_part_1() {
        let lines = small_example();

        assert_eq!(run_program(&lines, 24, true), 33);
    }

    #[test]
    #[ignore]
    fn big_example_works_part_1() {
        let lines = big_example();

        assert_eq!(run_program(&lines, 24, true), 1725);
    }

    #[test]
    #[ignore]
    fn big_example_works_part_2() {
        let lines = big_example();

        assert_eq!(run_program(&lines, 32, false), 15510);
    }

    fn small_example() -> Vec<&'static str> {
        include_str!("ex1.txt")
            .split('\n')
            .collect()
    }

    fn big_example() -> Vec<&'static str> {
        include_str!("ex2.txt")
            .split('\n')
            .collect()
    }
}

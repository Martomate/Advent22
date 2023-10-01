use std::{
    collections::HashMap,
    hash::Hash,
    ops::{Add, Index, IndexMut, Sub},
};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Resource {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

impl Resource {
    #[inline(always)]
    fn all() -> [Resource; 4] {
        use Resource::*;
        [Ore, Clay, Obsidian, Geode]
    }
}

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy, Hash)]
struct ResourceSlice<T>([T; 4]);

impl<T> ResourceSlice<T> {
    fn populate<F: Fn(Resource) -> T>(f: F) -> ResourceSlice<T> {
        ResourceSlice(Resource::all().map(f))
    }
}

impl<T: Default> ResourceSlice<T> {
    fn new() -> ResourceSlice<T> {
        Default::default()
    }
}

impl<T: Clone> ResourceSlice<T> {
    fn with(mut self, r: Resource, v: T) -> Self {
        self[r] = v;
        self
    }
}

impl<T> Index<Resource> for ResourceSlice<T> {
    type Output = T;

    fn index(&self, index: Resource) -> &Self::Output {
        &self.0[index as usize]
    }
}

impl<T> IndexMut<Resource> for ResourceSlice<T> {
    fn index_mut(&mut self, index: Resource) -> &mut Self::Output {
        &mut self.0[index as usize]
    }
}

impl<T> Add for ResourceSlice<T>
where
    T: Add + Default + Copy,
    T::Output: Default,
{
    type Output = ResourceSlice<T::Output>;

    fn add(self, rhs: Self) -> Self::Output {
        let mut res: ResourceSlice<T::Output> = Default::default();
        for r in Resource::all() {
            res[r] = self[r] + rhs[r];
        }
        res
    }
}

impl<T> Sub for ResourceSlice<T>
where
    T: Sub + Default + Copy,
    T::Output: Default,
{
    type Output = ResourceSlice<T::Output>;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut res: ResourceSlice<T::Output> = Default::default();
        for r in Resource::all() {
            res[r] = self[r] - rhs[r];
        }
        res
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
struct SimulationState {
    resources: ResourceSlice<u8>,
    robots: ResourceSlice<u8>,
    steps_left: u8,
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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Recipe {
    input: ResourceSlice<u8>,
}

impl Recipe {
    fn new(ore: u8, clay: u8, obsidian: u8) -> Recipe {
        Recipe {
            input: ResourceSlice::new()
                .with(Resource::Ore, ore)
                .with(Resource::Clay, clay)
                .with(Resource::Obsidian, obsidian),
        }
    }

    fn affordable(&self, res: ResourceSlice<u8>) -> bool {
        Resource::all().iter().all(|&r| self.input[r] <= res[r])
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Blueprint {
    id: u8,
    recipes: ResourceSlice<Recipe>,
}

const CACHE_LIMIT: u8 = 4;
const MAX_DEPTH: usize = 32;

impl Blueprint {
    fn simulate(
        &self,
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
        let num_candidates = self.simulation_candidates(&state, candidates);

        for i in 0..num_candidates {
            let new_state = &candidates[i];
            // only try to find something better if it's not impossible
            let upper_bound = new_state.geode_upper_bound();
            if upper_bound > best_result {
                let result = self.simulate(
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
        &self,
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
            if self.recipes[r].affordable(resources) {
                let new_state = SimulationState {
                    resources: resources + robots - self.recipes[r].input,
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
}

#[derive(Debug, PartialEq, Eq)]
enum BlueprintParseError {
    MissingRecipe(Resource),
}

pub fn run_program(lines: Vec<String>, steps: u8, perform_sum: bool) -> u32 {
    assert!(
        steps as usize <= MAX_DEPTH,
        "steps may not be more than {}",
        MAX_DEPTH
    );

    let blueprints: Vec<Blueprint> = parse_blueprints(lines).unwrap();

    // TODO: use rayon thread pool for the blueprints

    let result = if perform_sum {
        blueprints
            .iter()
            .map(|b| {
                let mut cache = HashMap::new();
                let mut next_states: [_; MAX_DEPTH] = Default::default();
                let quality = b.simulate(
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
                let quality = b.simulate(
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

fn parse_recipe(s: &str) -> Option<Recipe> {
    let parts: Vec<_> = s.split_whitespace().collect();

    if parts.len() < 6 {
        return None; // the first ingredient is mandatory
    }

    let mut inputs = Vec::new();
    inputs.push((parts[4], parts[5]));
    if parts.len() >= 9 {
        inputs.push((parts[7], parts[8]));
    }

    let mut recipe = Recipe::new(0, 0, 0);
    for (amt_str, res) in inputs {
        let amt = amt_str.parse::<u8>().ok()?;
        let res = match res {
            "ore" => Some(Resource::Ore),
            "clay" => Some(Resource::Clay),
            "obsidian" => Some(Resource::Obsidian),
            _ => None,
        };
        if let Some(res) = res {
            recipe.input = recipe.input.with(res, amt);
        } else {
            return None;
        }
    }

    Some(recipe)
}

fn parse_blueprints(lines: Vec<String>) -> Result<Vec<Blueprint>, BlueprintParseError> {
    let mut blueprints: Vec<Blueprint> = Vec::new();

    for pieces in lines
        .iter()
        .flat_map(|l| l.chars())
        .collect::<String>()
        .split('.')
        .collect::<Vec<_>>()
        .chunks(4)
    {
        if pieces.len() < 4 {
            continue;
        }
        use BlueprintParseError::*;
        if let Some((_, a)) = pieces[0].split_once(':') {
            let ore = parse_recipe(a).ok_or(MissingRecipe(Resource::Ore))?;
            let clay = parse_recipe(pieces[1]).ok_or(MissingRecipe(Resource::Clay))?;
            let obsidian = parse_recipe(pieces[2]).ok_or(MissingRecipe(Resource::Obsidian))?;
            let geode = parse_recipe(pieces[3]).ok_or(MissingRecipe(Resource::Geode))?;

            blueprints.push(Blueprint {
                id: (blueprints.len() + 1) as u8,
                recipes: ResourceSlice::populate(|r| match r {
                    Resource::Ore => ore,
                    Resource::Clay => clay,
                    Resource::Obsidian => obsidian,
                    Resource::Geode => geode,
                }),
            });
        }
    }

    Ok(blueprints)
}

#[cfg(test)]
mod tests {
    use super::{
        arithmetic_sum, parse_blueprints, run_program, Blueprint, Recipe, Resource, ResourceSlice,
    };

    #[test]
    fn arithmetic_sum_from_5_to_6_is_11() {
        assert_eq!(arithmetic_sum(5, 2), 11);
    }

    #[test]
    fn arithmetic_sum_from_5_to_7_is_18() {
        assert_eq!(arithmetic_sum(5, 3), 18);
    }

    #[test]
    fn parse_blueprints_on_single_lines() {
        let blueprints = parse_blueprints(vec![
            "Blueprint 1: Each ore robot costs 3 ore. Each clay robot costs 3 ore. Each obsidian robot costs 2 ore and 20 clay. Each geode robot costs 2 ore and 20 obsidian.".to_owned(),
            "Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 2 ore and 20 clay. Each geode robot costs 2 ore and 20 obsidian.".to_owned()
        ]);

        assert_eq!(
            blueprints,
            Ok(vec![
                Blueprint {
                    id: 1,
                    recipes: ResourceSlice::populate(|r| match r {
                        Resource::Ore => Recipe::new(3, 0, 0),
                        Resource::Clay => Recipe::new(3, 0, 0),
                        Resource::Obsidian => Recipe::new(2, 20, 0),
                        Resource::Geode => Recipe::new(2, 0, 20),
                    })
                },
                Blueprint {
                    id: 2,
                    recipes: ResourceSlice::populate(|r| match r {
                        Resource::Ore => Recipe::new(2, 0, 0),
                        Resource::Clay => Recipe::new(3, 0, 0),
                        Resource::Obsidian => Recipe::new(2, 20, 0),
                        Resource::Geode => Recipe::new(2, 0, 20),
                    })
                }
            ])
        )
    }

    #[test]
    fn parse_blueprints_on_separate_lines() {
        let blueprints = parse_blueprints(
            "
Blueprint 1:
  Each ore robot costs 4 ore.
  Each clay robot costs 2 ore.
  Each obsidian robot costs 3 ore and 14 clay.
  Each geode robot costs 2 ore and 7 obsidian.

Blueprint 2:
  Each ore robot costs 2 ore.
  Each clay robot costs 3 ore.
  Each obsidian robot costs 3 ore and 8 clay.
  Each geode robot costs 3 ore and 12 obsidian.
"
            .split('\n')
            .map(|s| s.to_owned())
            .collect(),
        );

        assert_eq!(
            blueprints,
            Ok(vec![
                Blueprint {
                    id: 1,
                    recipes: ResourceSlice::populate(|r| match r {
                        Resource::Ore => Recipe::new(4, 0, 0),
                        Resource::Clay => Recipe::new(2, 0, 0),
                        Resource::Obsidian => Recipe::new(3, 14, 0),
                        Resource::Geode => Recipe::new(2, 0, 7),
                    })
                },
                Blueprint {
                    id: 2,
                    recipes: ResourceSlice::populate(|r| match r {
                        Resource::Ore => Recipe::new(2, 0, 0),
                        Resource::Clay => Recipe::new(3, 0, 0),
                        Resource::Obsidian => Recipe::new(3, 8, 0),
                        Resource::Geode => Recipe::new(3, 0, 12),
                    })
                }
            ])
        )
    }

    #[test]
    fn example_works_part_1() {
        let lines = small_example();

        assert_eq!(run_program(lines, 24, true), 33);
    }

    #[test]
    #[ignore]
    fn big_example_works_part_1() {
        let lines = big_example();

        assert_eq!(run_program(lines, 24, true), 1725);
    }

    #[test]
    #[ignore]
    fn big_example_works_part_2() {
        let lines = big_example();

        assert_eq!(run_program(lines, 32, false), 15510);
    }

    fn small_example() -> Vec<String> {
        include_str!("ex1.txt")
            .split('\n')
            .map(|s| s.to_owned())
            .collect()
    }

    fn big_example() -> Vec<String> {
        include_str!("ex2.txt")
            .split('\n')
            .map(|s| s.to_owned())
            .collect()
    }
}

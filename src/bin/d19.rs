use std::{
    collections::HashMap,
    io::{self, BufRead},
    ops::{Add, Index, IndexMut, Sub}, time::Instant, env,
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
        ResourceSlice(Resource::all().map(|r| f(r)))
    }
}

impl<T: Default> ResourceSlice<T> {
    fn new() -> ResourceSlice<T> {
        Default::default()
    }
}

impl<T: Clone> ResourceSlice<T> {
    fn with(self, r: Resource, v: T) -> Self {
        let mut res = self.clone();
        res[r] = v;
        res
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

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
struct SimulationState {
    resources: ResourceSlice<u32>,
    robots: ResourceSlice<u32>,
    steps_left: u32,
}

fn arithmetic_sum(start: u32, count: u32) -> u32 {
    (start + (start + count - 1)) * count / 2
}

impl SimulationState {
    /// Returns an upper bound on how many Geodes there can be when the simulation ends.
    /// This bound is impossible to exceed (that's the whole point).
    /// 
    /// Assume one geode robot is built every timestep;
    /// then we get `e + r + (r+1) + (r+2) + ... + (r + s - 1)` (e: existing, g: robots, s: steps)
    fn geode_upper_bound(&self) -> u32 {
        if self.steps_left == 0 {
            self.resources[Resource::Geode]
        } else {
            self.resources[Resource::Geode] + arithmetic_sum(self.robots[Resource::Geode], self.steps_left)
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Recipe {
    input: ResourceSlice<u32>,
}

impl Recipe {
    fn new(ore: u32, clay: u32, obsidian: u32) -> Recipe {
        return Recipe {
            input: ResourceSlice::new()
                .with(Resource::Ore, ore)
                .with(Resource::Clay, clay)
                .with(Resource::Obsidian, obsidian),
        };
    }

    fn affordable(&self, res: ResourceSlice<u32>) -> bool {
        Resource::all().iter().all(|&r| self.input[r] <= res[r])
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Blueprint {
    id: u32,
    recipes: ResourceSlice<Recipe>,
}

impl Blueprint {
    fn simulate(&self, state: SimulationState, cache: &mut HashMap<SimulationState, u32>, best_so_far: u32) -> u32 {
        let SimulationState {
            resources,
            robots,
            steps_left,
        } = state;

        if steps_left <= 0 {
            return resources[Resource::Geode];
        }

        if steps_left > 6 {
            if let Some(&value) = cache.get(&state) {
                return value;
            }
        }

        let added_resources = robots;

        let mut best_result: u32 = best_so_far;
        let mut recipes_afforded: u32 = 0;

        for r in [Resource::Geode, Resource::Obsidian, Resource::Clay, Resource::Ore] {
            if self.recipes[r].affordable(resources) {
                recipes_afforded += 1;

                let new_state = SimulationState {
                    resources: resources - self.recipes[r].input + added_resources,
                    robots: robots.with(r, robots[r] + 1),
                    steps_left: steps_left - 1,
                };

                // only try to find something better if it's not impossible
                if new_state.geode_upper_bound() > best_result {
                    let result = self.simulate(
                        new_state,
                        cache,
                        best_result,
                    );

                    if result > best_result {
                        best_result = result;
                    }
                }
            }
        }

        // Don't build anything yet (but only if there is something better to save resources for)
        if recipes_afforded < 4 {
            let new_state = SimulationState {
                resources: resources + added_resources,
                robots,
                steps_left: steps_left - 1,
            };

            // only try to find something better if it's not impossible
            if new_state.geode_upper_bound() > best_result {
                let result = self.simulate(
                    new_state,
                    cache,
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

        if steps_left > 6 {
            cache.insert(state, best_result);
        }

        best_result
    }
}

#[derive(Debug, PartialEq, Eq)]
enum BlueprintParseError {
    MissingRecipe(Resource),
}

fn run_program(lines: Vec<String>, steps: u32, perform_sum: bool) -> u32 {
    let blueprints: Vec<Blueprint> = parse_blueprints(lines).unwrap();

    if perform_sum {
        return blueprints
            .iter()
            .map(|b| {
                let mut cache = HashMap::new();
                let quality = b.simulate(
                    SimulationState {
                        resources: ResourceSlice::new(),
                        robots: ResourceSlice::new().with(Resource::Ore, 1),
                        steps_left: steps,
                    },
                    &mut cache,
                    0,
                );
                println!("{}: {}", b.id, quality);
                quality * b.id
            })
            .sum();
    } else {
        return blueprints
            .iter()
            .take(3)
            .map(|b| {
                let mut cache = HashMap::new();
                let quality = b.simulate(
                    SimulationState {
                        resources: ResourceSlice::new(),
                        robots: ResourceSlice::new().with(Resource::Ore, 1),
                        steps_left: steps,
                    },
                    &mut cache,
                    0,
                );
                println!("{}: {}", b.id, quality);
                quality
            })
            .product();
    }
}

fn parse_recipe(s: &str) -> Option<Recipe> {
    let parts: Vec<_> = s.trim().split_whitespace().map(|s| s.to_string()).collect();

    let mut recipe = Recipe::new(0, 0, 0);

    let mut inputs = Vec::new();
    for (a, r) in vec![(parts.get(4), parts.get(5)), (parts.get(7), parts.get(8))] {
        if let (Some(a), Some(r)) = (a, r) {
            inputs.push((a, r));
        }
    }

    if inputs.len() == 0 {
        return None; // the first ingredient is mandatory
    }

    for (amt_str, res) in inputs {
        let amt = amt_str.parse::<u32>().ok()?;
        let res = match res.as_str() {
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
                id: (blueprints.len() + 1) as u32,
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

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let steps = args.iter().find_map(|s| match s.split_once('=') {
        Some(("steps", v)) => v.parse::<u32>().ok(),
        _ => None,
    }).unwrap_or(24);

    let perform_sum = args.iter().find_map(|s| match s.split_once("=") {
        Some(("mode", v)) => Some(v == "sum"),
        _ => None,
    }).unwrap_or(true);

    let stdin = io::stdin();

    let mut lines: Vec<String> = Vec::new();

    for l in stdin.lock().lines() {
        let line = l.unwrap();

        if line.len() == 0 && lines.last().filter(|&s| s.len() == 0).is_some() {
            break;
        }

        lines.push(line);
    }

    println!("Calculating {} steps in '{}' mode...", steps, if perform_sum { "sum" } else { "product" });

    let now = Instant::now();

    let res = run_program(lines, steps, perform_sum);

    let elapsed_time = now.elapsed();

    println!("Time: {} seconds.", elapsed_time.as_secs_f32());
    println!("Answer: {}", res);
}

#[cfg(test)]
mod tests {
    use crate::{parse_blueprints, run_program, Blueprint, Recipe, ResourceSlice, Resource, arithmetic_sum};

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
        "\
Blueprint 1:
  Each ore robot costs 4 ore.
  Each clay robot costs 2 ore.
  Each obsidian robot costs 3 ore and 14 clay.
  Each geode robot costs 2 ore and 7 obsidian.

Blueprint 2:
  Each ore robot costs 2 ore.
  Each clay robot costs 3 ore.
  Each obsidian robot costs 3 ore and 8 clay.
  Each geode robot costs 3 ore and 12 obsidian."
            .split('\n')
            .map(|s| s.to_owned())
            .collect()
    }

    fn big_example() -> Vec<String> {
        "\
Blueprint 1: Each ore robot costs 3 ore. Each clay robot costs 3 ore. Each obsidian robot costs 2 ore and 20 clay. Each geode robot costs 2 ore and 20 obsidian.
Blueprint 2: Each ore robot costs 4 ore. Each clay robot costs 4 ore. Each obsidian robot costs 4 ore and 9 clay. Each geode robot costs 3 ore and 9 obsidian.
Blueprint 3: Each ore robot costs 3 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 2 ore and 12 obsidian.
Blueprint 4: Each ore robot costs 4 ore. Each clay robot costs 3 ore. Each obsidian robot costs 2 ore and 19 clay. Each geode robot costs 3 ore and 10 obsidian.
Blueprint 5: Each ore robot costs 4 ore. Each clay robot costs 4 ore. Each obsidian robot costs 4 ore and 14 clay. Each geode robot costs 3 ore and 16 obsidian.
Blueprint 6: Each ore robot costs 3 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 17 clay. Each geode robot costs 4 ore and 8 obsidian.
Blueprint 7: Each ore robot costs 3 ore. Each clay robot costs 4 ore. Each obsidian robot costs 4 ore and 14 clay. Each geode robot costs 4 ore and 10 obsidian.
Blueprint 8: Each ore robot costs 4 ore. Each clay robot costs 4 ore. Each obsidian robot costs 4 ore and 12 clay. Each geode robot costs 4 ore and 19 obsidian.
Blueprint 9: Each ore robot costs 4 ore. Each clay robot costs 3 ore. Each obsidian robot costs 4 ore and 18 clay. Each geode robot costs 4 ore and 11 obsidian.
Blueprint 10: Each ore robot costs 4 ore. Each clay robot costs 3 ore. Each obsidian robot costs 4 ore and 5 clay. Each geode robot costs 3 ore and 10 obsidian.
Blueprint 11: Each ore robot costs 4 ore. Each clay robot costs 4 ore. Each obsidian robot costs 4 ore and 7 clay. Each geode robot costs 2 ore and 19 obsidian.
Blueprint 12: Each ore robot costs 4 ore. Each clay robot costs 4 ore. Each obsidian robot costs 2 ore and 11 clay. Each geode robot costs 3 ore and 14 obsidian.
Blueprint 13: Each ore robot costs 2 ore. Each clay robot costs 2 ore. Each obsidian robot costs 2 ore and 8 clay. Each geode robot costs 2 ore and 14 obsidian.
Blueprint 14: Each ore robot costs 2 ore. Each clay robot costs 2 ore. Each obsidian robot costs 2 ore and 17 clay. Each geode robot costs 2 ore and 10 obsidian.
Blueprint 15: Each ore robot costs 4 ore. Each clay robot costs 4 ore. Each obsidian robot costs 2 ore and 9 clay. Each geode robot costs 3 ore and 15 obsidian.
Blueprint 16: Each ore robot costs 4 ore. Each clay robot costs 4 ore. Each obsidian robot costs 2 ore and 7 clay. Each geode robot costs 4 ore and 13 obsidian.
Blueprint 17: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 9 clay. Each geode robot costs 3 ore and 9 obsidian.
Blueprint 18: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 2 ore and 14 clay. Each geode robot costs 3 ore and 20 obsidian.
Blueprint 19: Each ore robot costs 4 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 17 clay. Each geode robot costs 3 ore and 13 obsidian.
Blueprint 20: Each ore robot costs 3 ore. Each clay robot costs 4 ore. Each obsidian robot costs 4 ore and 5 clay. Each geode robot costs 4 ore and 8 obsidian.
Blueprint 21: Each ore robot costs 4 ore. Each clay robot costs 4 ore. Each obsidian robot costs 3 ore and 10 clay. Each geode robot costs 2 ore and 14 obsidian.
Blueprint 22: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 20 obsidian.
Blueprint 23: Each ore robot costs 4 ore. Each clay robot costs 4 ore. Each obsidian robot costs 4 ore and 16 clay. Each geode robot costs 2 ore and 15 obsidian.
Blueprint 24: Each ore robot costs 4 ore. Each clay robot costs 3 ore. Each obsidian robot costs 2 ore and 13 clay. Each geode robot costs 2 ore and 9 obsidian.
Blueprint 25: Each ore robot costs 4 ore. Each clay robot costs 4 ore. Each obsidian robot costs 2 ore and 12 clay. Each geode robot costs 3 ore and 15 obsidian.
Blueprint 26: Each ore robot costs 4 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 18 clay. Each geode robot costs 4 ore and 8 obsidian.
Blueprint 27: Each ore robot costs 4 ore. Each clay robot costs 4 ore. Each obsidian robot costs 3 ore and 19 clay. Each geode robot costs 4 ore and 15 obsidian.
Blueprint 28: Each ore robot costs 3 ore. Each clay robot costs 4 ore. Each obsidian robot costs 3 ore and 20 clay. Each geode robot costs 3 ore and 14 obsidian.
Blueprint 29: Each ore robot costs 2 ore. Each clay robot costs 4 ore. Each obsidian robot costs 4 ore and 16 clay. Each geode robot costs 4 ore and 17 obsidian.
Blueprint 30: Each ore robot costs 3 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 9 clay. Each geode robot costs 3 ore and 7 obsidian."
            .split('\n')
            .map(|s| s.to_owned())
            .collect()
    }
}

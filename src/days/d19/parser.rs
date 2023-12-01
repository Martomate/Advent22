use super::{
    blueprint::{Blueprint, Recipe},
    resource::{Resource, ResourceSlice},
};

#[derive(Debug, PartialEq, Eq)]
pub enum BlueprintParseError {
    MissingRecipe(Resource),
}

pub fn parse_blueprints(lines: &[&str]) -> Result<Vec<Blueprint>, BlueprintParseError> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_blueprints_on_single_lines() {
        let blueprints = parse_blueprints(&[
            "Blueprint 1: Each ore robot costs 3 ore. Each clay robot costs 3 ore. Each obsidian robot costs 2 ore and 20 clay. Each geode robot costs 2 ore and 20 obsidian.",
            "Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 2 ore and 20 clay. Each geode robot costs 2 ore and 20 obsidian."
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
        let input = "
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
";
        let lines: Vec<_> = input.split('\n').collect();

        let blueprints = parse_blueprints(&lines,);

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
}

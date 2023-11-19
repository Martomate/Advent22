use super::resource::{ResourceSlice, Resource};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Recipe {
    pub input: ResourceSlice<u8>,
}

impl Recipe {
    pub fn new(ore: u8, clay: u8, obsidian: u8) -> Recipe {
        Recipe {
            input: ResourceSlice::new()
                .with(Resource::Ore, ore)
                .with(Resource::Clay, clay)
                .with(Resource::Obsidian, obsidian),
        }
    }

    pub fn affordable(&self, res: ResourceSlice<u8>) -> bool {
        Resource::all().iter().all(|&r| self.input[r] <= res[r])
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Blueprint {
    pub id: u8,
    pub recipes: ResourceSlice<Recipe>,
}

use std::ops::{Add, Index, IndexMut, Sub};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Resource {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

impl Resource {
    #[inline(always)]
    pub fn all() -> [Resource; 4] {
        use Resource::*;
        [Ore, Clay, Obsidian, Geode]
    }
}

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy, Hash)]
pub struct ResourceSlice<T>([T; 4]);

impl<T> ResourceSlice<T> {
    pub fn populate<F: Fn(Resource) -> T>(f: F) -> ResourceSlice<T> {
        ResourceSlice(Resource::all().map(f))
    }
}

impl<T: Default> ResourceSlice<T> {
    pub fn new() -> ResourceSlice<T> {
        Default::default()
    }
}

impl<T: Clone> ResourceSlice<T> {
    pub fn with(mut self, r: Resource, v: T) -> Self {
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

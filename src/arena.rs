use std::marker::PhantomData;

#[derive(Debug, Clone, Hash)]
pub struct Id<T> {
    index: usize,
    _marker: PhantomData<fn() -> T>,
}

impl<T: Clone> Copy for Id<T> {}

impl<T> PartialEq for Id<T> {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index
    }
}

#[derive(Debug, Clone)]
pub struct Arena<T> {
    nodes: Vec<T>,
}

impl<T> Arena<T> {
    pub fn new() -> Self {
        Self { nodes: Vec::new() }
    }

    pub fn alloc(&mut self, value: T) -> Id<T> {
        let index = self.nodes.len();
        self.nodes.push(value);
        Id {
            index,
            _marker: PhantomData,
        }
    }

    pub fn get(&self, id: &Id<T>) -> &T {
        &self.nodes[id.index]
    }

    pub fn get_mut(&mut self, id: &Id<T>) -> &mut T {
        &mut self.nodes[id.index]
    }

    pub fn clear(&mut self) {
        self.nodes.clear();
    }
}

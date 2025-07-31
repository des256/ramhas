use {
    crate::*,
    generational_arena::{Arena as GenArena, Index as GenIndex},
};

pub struct Arena {
    arena: GenArena<Node>,
}

impl Arena {
    pub fn new() -> Self {
        Self {
            arena: GenArena::new(),
        }
    }

    pub fn print_state(&self) {
        for (index, node) in self.arena.iter() {
            println!("{:?}: {}", index, node);
        }
    }

    pub fn insert(&mut self, node: Node) -> GenIndex {
        for (index, existing_node) in self.arena.iter() {
            if existing_node.equals(&node) {
                return index;
            }
        }
        self.arena.insert(node)
    }

    pub fn remove(&mut self, index: GenIndex) {
        self.arena.remove(index);
    }

    pub fn clear(&mut self) {
        self.arena.clear();
    }

    pub fn get_ref(&self, index: GenIndex) -> &Node {
        &self.arena[index]
    }

    pub fn get_mut(&mut self, index: GenIndex) -> &mut Node {
        &mut self.arena[index]
    }

    pub fn get2_mut(
        &mut self,
        index: GenIndex,
        index2: GenIndex,
    ) -> (Option<&mut Node>, Option<&mut Node>) {
        self.arena.get2_mut(index, index2)
    }
}

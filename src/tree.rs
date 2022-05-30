pub struct Tree<T>
where
    T: PartialEq + Clone
{
    arena: Vec<Node<T>>
}

impl<T> Tree<T>
where
    T: PartialEq + Clone
{
    pub fn new(root: T) -> Self {
        let node = Node::new(root);
        Self {
            arena: Vec::from([node])
        }
    }

    pub fn insert(&mut self, parent: usize, val: T) -> usize {
        let idx = self.arena.len();
        let mut node = Node::new(val);
        node.set_parent(parent);
        self.arena.push(node);

        idx
    }

    pub fn get(&self, index: usize) -> &T {
        self.arena[index].inner()
    }

    pub fn path_to(&self, index: usize) -> Vec<T> {
        let mut index_path = Vec::from([index]);

        let mut node = &self.arena[index];

        while let Some(parent) = node.parent {
            index_path.push(parent);
            node = &self.arena[parent];
        }

        index_path.iter().rev().map(|i| self.arena[*i].inner().clone()).collect()
    }
}

pub struct Node<T>
where
    T: PartialEq + Clone
{
    parent: Option<usize>,
    val: T
}

impl<T> Node<T>
where
    T: PartialEq + Clone
{
    pub fn new(val: T) -> Self {
        Self {
            parent: None,
            val
        }
    }

    pub fn set_parent(&mut self, index: usize) {
        self.parent = Some(index);
    }

    pub fn inner(&self) -> &T {
        &self.val
    }
}


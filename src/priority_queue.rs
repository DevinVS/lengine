use crate::pathfinding::Node;

#[derive(Debug)]
pub struct PriorityQueue {
    nodes: Vec<Node>
}

impl PriorityQueue {
    pub fn new() -> PriorityQueue {
        Self {
            nodes: Vec::new()
        }
    }

    // Add n to the list so that the list is guaranteed to
    // still be sorted.
    pub fn push(&mut self, n: Node) {
        let insert_at = self.binary_search(&n);
        self.nodes.insert(insert_at, n);
    }

    // Search for an element in the vec
    fn binary_search(&self, n: &Node) -> usize {
        match self.nodes.binary_search_by(|node| {
            if n.1 < node.1 {
                std::cmp::Ordering::Less
            } else if n.1 > node.1 {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        }) {
            Ok(e) => e,
            Err(e) => e
        }
    }

    // remove the last item from the list and return it
    pub fn pop(&mut self) -> Option<Node> {
        self.nodes.pop()
    }

    // Index of node with the same points
    pub fn index_of(&self, n: Node) -> Option<usize> {
        self.nodes.iter().enumerate().find(|(_,node)| node.2==n.2 && node.3==n.3).map(|a| a.0)
    }

    // Remove all nodes with the same point
    pub fn remove(&mut self, i: usize) {
        self.nodes.remove(i);
    }

    // Insert node or replace if cost is less
    pub fn insert_or_replace(&mut self, n: Node) {
        if let Some(i) = self.index_of(n) {
            if n.1 < self.nodes[i].1 {
                self.remove(i);
                self.push(n);
            }
        } else {
            self.push(n);
        }
    }
}

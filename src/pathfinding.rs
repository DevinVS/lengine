use std::cmp::Ordering;
use std::collections::HashSet;
use crate::priority_queue::PriorityQueue;

use crate::tree::Tree;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Node (pub usize, pub u32, pub i32, pub i32);

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.0.cmp(&self.0)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn shortest_path_segment(from: (i32, i32), to: (i32, i32), delta: i32) -> Option<(i32, i32)> {
    let mut tree = Tree::new(from);

    let mut queue = PriorityQueue::new();
    queue.push(Node(0, 0, from.0, from.1));

    let mut visits = HashSet::new();

    while let Some(Node(node, curr_cost, x, y)) = queue.pop() {
        visits.insert((x, y));

        // If we have reached our destination return the next step
        if dist(x, y, to.0, to.1) < delta as u32 {
            let path = tree.path_to(node);
            return Some(path[0]);
        }

        // For each direction add an adjacent node and its cost
        // f(n) = g(n) + h(n)
        for i in -1..=1 {
            for j in -1..=1 {
                if i==0 && j==0 { continue; }

                let new_x = x + delta * i;
                let new_y = y + delta * j;

                if visits.contains(&(new_x, new_y)) {
                    continue;
                }

                let cost = curr_cost + 1 + dist(x, y, to.0, to.1);
                let id = tree.insert(node, (new_x, new_y));
                let new_node = Node(id, cost, new_x, new_y);
                queue.insert_or_replace(new_node);
            }
        }
    }

    None
}

fn dist(x0: i32, y0: i32, x1: i32, y1: i32) -> u32 {
    ((y1-y0).pow(2) as f32 + (x1-x0).pow(2) as f32).sqrt() as u32
}

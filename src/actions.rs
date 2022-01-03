use std::collections::HashSet;
use crate::geometry::PositionComponent;
use crate::physics::PhysicsComponent;
use crate::graphics::GraphicsComponent;
use crate::animation::AnimationComponent;

pub trait Action {
    fn tick(&mut self, s: &mut HashSet<String>, p: &Option<&mut PositionComponent>, ph: &Option<&mut PhysicsComponent>, g: &Option<&mut GraphicsComponent>, a: &Option<&mut AnimationComponent>);
}

struct AddState {
    state: String
}

impl Action for AddState {
    fn tick(&mut self, s: &mut HashSet<String>, p: &Option<&mut PositionComponent>, ph: &Option<&mut PhysicsComponent>, g: &Option<&mut GraphicsComponent>, a: &Option<&mut AnimationComponent>) {
        s.insert(self.state.clone());
    }
}

struct RemoveState {
    state: String
}

impl Action for RemoveState {
    fn tick(&mut self, s: &mut HashSet<String>, p: &Option<&mut PositionComponent>, ph: &Option<&mut PhysicsComponent>, g: &Option<&mut GraphicsComponent>, a: &Option<&mut AnimationComponent>) {
        s.remove(&self.state);
    }
}

// TODO
struct AddEffect {}

struct ShowDialog {
    dialog: String
}

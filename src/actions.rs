use std::collections::HashSet;
use crate::effect::Effect;

pub trait Action {
    fn tick(&mut self, states: &mut HashSet<String>, effects: &mut Vec<Effect>, dialog: &mut Option<String>);
}

pub struct AddState {
    pub state: String
}

impl Action for AddState {
    fn tick(&mut self, states: &mut HashSet<String>, effects: &mut Vec<Effect>, dialogs: &mut Option<String>) {
        states.insert(self.state.clone());
    }
}

pub struct RemoveState {
    pub state: String
}

impl Action for RemoveState {
    fn tick(&mut self, states: &mut HashSet<String>, effects: &mut Vec<Effect>, dialogs: &mut Option<String>) {
        states.remove(&self.state);
    }
}

pub struct AddEffect {
    pub effect: Effect
}

impl Action for AddEffect {
    fn tick(&mut self, states: &mut HashSet<String>, effects: &mut Vec<Effect>, dialog: &mut Option<String>) {
        effects.push(self.effect.clone())
    }
}

pub struct ShowDialog {
    pub dialog: String
}

impl Action for ShowDialog {
    fn tick(&mut self, states: &mut HashSet<String>, effects: &mut Vec<Effect>, dialog: &mut Option<String>) {
        *dialog = Some(self.dialog.clone())
    }
}

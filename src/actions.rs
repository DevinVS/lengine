use std::collections::HashSet;
use crate::effect::Effect;

/// An action is activated by the ActionSystem.
/// When an entity has a state which maps to an action,
/// the action is run
pub trait Action {
    /// Run the desired action
    fn tick(&mut self, states: &mut HashSet<String>, effects: &mut Vec<Effect>, dialog: &mut Option<String>);
}

/// Add an additional state to the current entity
pub struct AddState {
    /// State to add
    pub state: String
}

impl Action for AddState {
    fn tick(&mut self, states: &mut HashSet<String>, _: &mut Vec<Effect>, _: &mut Option<String>) {
        states.insert(self.state.clone());
    }
}

/// Remove a state from the current entity
pub struct RemoveState {
    /// State to remove
    pub state: String
}

impl Action for RemoveState {
    fn tick(&mut self, states: &mut HashSet<String>, _: &mut Vec<Effect>, _: &mut Option<String>) {
        states.remove(&self.state);
    }
}

/// Add an effect to the world
pub struct AddEffect {
    /// Effect to add
    pub effect: Effect
}

impl Action for AddEffect {
    fn tick(&mut self, _: &mut HashSet<String>, effects: &mut Vec<Effect>, _: &mut Option<String>) {
        effects.push(self.effect.clone())
    }
}

/// Show a dialog box
pub struct ShowDialog {
    /// Name of dialog to display
    pub dialog: String
}

impl Action for ShowDialog {
    fn tick(&mut self, _: &mut HashSet<String>, _: &mut Vec<Effect>, dialog: &mut Option<String>) {
        *dialog = Some(self.dialog.clone())
    }
}

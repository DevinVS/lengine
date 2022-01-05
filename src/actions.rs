use std::{collections::HashSet, fmt::Debug};
use crate::effect::Effect;

/// An action is activated by the ActionSystem.
/// When an entity has a state which maps to an action,
/// the action is run
pub trait Actionable {
    /// Run the desired action
    fn tick(&mut self, states: &mut HashSet<String>, effects: &mut Vec<Effect>, dialog: &mut Option<String>);
}

pub trait Action: Actionable + Debug {}

/// Add an additional state to the current entity
#[derive(Debug)]
pub struct AddState {
    /// State to add
    pub state: String
}

impl Actionable for AddState {
    fn tick(&mut self, states: &mut HashSet<String>, _: &mut Vec<Effect>, _: &mut Option<String>) {
        states.insert(self.state.clone());
    }
}

impl Action for AddState {}

/// Remove a state from the current entity
#[derive(Debug)]
pub struct RemoveState {
    /// State to remove
    pub state: String
}

impl Actionable for RemoveState {
    fn tick(&mut self, states: &mut HashSet<String>, _: &mut Vec<Effect>, _: &mut Option<String>) {
        states.remove(&self.state);
    }
}

impl Action for RemoveState {}

/// Add an effect to the world
#[derive(Debug)]
pub struct AddEffect {
    /// Effect to add
    pub effect: Effect
}

impl Actionable for AddEffect {
    fn tick(&mut self, _: &mut HashSet<String>, effects: &mut Vec<Effect>, _: &mut Option<String>) {
        effects.push(self.effect.clone())
    }
}

impl Action for AddEffect {}

/// Show a dialog box
#[derive(Debug)]
pub struct ShowDialog {
    /// Name of dialog to display
    pub dialog: String
}

impl Actionable for ShowDialog {
    fn tick(&mut self, _: &mut HashSet<String>, _: &mut Vec<Effect>, dialog: &mut Option<String>) {
        *dialog = Some(self.dialog.clone());
    }
}

impl Action for ShowDialog {}

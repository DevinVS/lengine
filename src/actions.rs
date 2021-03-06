use std::{collections::HashSet, fmt::Debug, process::exit};
use crate::effect::{Effect, EffectSpawner};

/// Trait to define an action caused by a change in state or world event
///
/// An action is able to modify the states of the entity who spawned it, the global effects in the
/// current world, or the currently displayed Dialog.
/// Most commonly actions coincide with a set of states defined on an entity in an ActionComponent,
/// but actions can also be spawned after certain events have finished, such as an animation
pub trait Actionable {
    /// Run the desired action, modifying entity state, world effects, or the current dialog
    fn tick(&mut self, states: &mut HashSet<String>, effects: &mut Vec<Effect>, dialog: &mut Option<String>);
}

/// Wrapper trait to allow printing of actions
pub trait Action: Actionable + Debug {}

/// An action which adds a state to the entity who spawned it
#[derive(Debug, Clone)]
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

/// An action which remove a state from the entity who spawned it
#[derive(Debug, Clone)]
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

/// An action which adds an effect to the world
#[derive(Debug, Clone)]
pub struct AddEffect {
    /// Effect to add
    pub effect: EffectSpawner
}

impl Actionable for AddEffect {
    fn tick(&mut self, _: &mut HashSet<String>, effects: &mut Vec<Effect>, _: &mut Option<String>) {
        effects.push(self.effect.spawn())
    }
}

impl Action for AddEffect {}

/// An action which shows a dialog box
#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct ExitGame;

impl Actionable for ExitGame {
    fn tick(&mut self, _: &mut HashSet<String>, _: &mut Vec<Effect>, _: &mut Option<String>) {
        exit(0);
    }
}

impl Action for ExitGame {}

#[derive(Debug)]
pub struct Goto {
    pub location: String
}

impl Actionable for Goto {
    fn tick(&mut self, states: &mut HashSet<String>, _: &mut Vec<Effect>, _: &mut Option<String>) {
        states.insert(format!("__MOVE_TO__={}", self.location));
    }
}
impl Action for Goto {}

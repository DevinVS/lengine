use std::collections::HashSet;
use crate::effect::Effect;

use crate::state::Sequence;

/// Represents a Dialog interaction with the player
#[derive(Debug)]
pub struct Dialog {
    /// Messages to display, in order that they will be displayed
    messages: Vec<String>,
    /// Index of the current message to display
    curr_msg: usize,
    /// Actions to run after
    /// Note: state changes are nonsensical and have no effect when run after a dialog
    /// Use an effect instead
    after: Option<Sequence>
}

impl Dialog {
    /// Create a new Dialog
    pub fn new(messages: Vec<String>, after: Option<Sequence>) -> Dialog {
        Dialog {
            messages,
            curr_msg: 0,
            after
        }
    }

    /// Switch the dialog to the next message
    pub fn next(&mut self) -> String {
        let msg = self.messages[self.curr_msg].clone();
        self.curr_msg = (self.curr_msg + 1) % self.messages.len();
        msg
    }

    /// Check if the dialog box has shown all messages
    pub fn finished(&self) -> bool {
        self.messages.len()-1 == self.curr_msg
    }

    /// Get current message
    pub fn msg(&self) -> String {
        self.messages[self.curr_msg].clone()
    }

    pub fn run_after(&mut self, effects: &mut Vec<Effect>, curr_dialog: &mut Option<String>) {
        if let Some(sequence) = &mut self.after {
            sequence.run_all(&mut HashSet::new(), effects, curr_dialog);
        }
    }
}

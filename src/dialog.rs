/// Represents a Dialog interaction with the player
#[derive(Debug)]
pub struct Dialog {
    /// Messages to display, in order that they will be displayed
    messages: Vec<String>,
    /// Index of the current message to display
    curr_msg: usize,
}

impl Dialog {
    /// Create a new Dialog
    pub fn new(messages: Vec<String>) -> Dialog {
        Dialog {
            messages,
            curr_msg: 0,
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
}

pub struct Dialog {
    messages: Vec<String>,
    curr_msg: usize
}

impl Dialog {
    pub fn new(messages: Vec<String>) -> Dialog {
        Dialog {
            messages,
            curr_msg: 0
        }
    }

    pub fn next(&mut self) -> String {
        let msg = self.messages[self.curr_msg].clone();
        self.curr_msg = (self.curr_msg + 1) % self.messages.len();
        msg
    }

    pub fn finished(&self) -> bool {
        self.messages.len()-1 == self.curr_msg
    }

    pub fn msg(&self) -> String {
        self.messages[self.curr_msg].clone()
    }
}

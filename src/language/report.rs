pub struct Report {
    actions: Vec<Action>
}

impl Report {
    pub fn new(actions: Vec<Action>) -> Report {
        Report {
            actions: actions
        }
    }
}

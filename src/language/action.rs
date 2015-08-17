pub struct Action {
    words: Vec<Word>,
    details: Vec<Detail>
}

impl Action {
    pub fn new(words: Vec<Word>, details: Vec<Detail>) -> Action {
        Action {
            words: words,
            details: details
        }
    }
}

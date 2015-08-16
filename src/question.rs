pub struct Question {
    words: Vec<Word>
}

impl Question {
    pub fn new(words: Vec<Word>) -> Question {
        Question {
            words: words
        }
    }
}

pub struct Answer {
    words: Vec<Word>
}

impl Answer {
    pub fn new(words: Vec<Word>) -> Answer {
        Answer {
            words: words
        }
    }
}

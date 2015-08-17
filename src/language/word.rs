pub struct Word {
    letters: Vec<Letter>
}

impl Word {
    pub fn new(letters: Vec<Letter>) -> Word {
        Word {
            letters: letters
        }
    }
}

pub struct Detail {
    question: Question,
    answer: Answer
}

impl Detail {
    pub fn new(question: Question, answer: Answer) -> Detail {
        Detail {
            question: question,
            answer: answer
        }
    }
}

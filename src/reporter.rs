pub struct Reporter {
    reports: Vec<Report>
}

impl Reporter {
    pub fn new(reports: Vec<Report>) -> Reporter {
        Reporter {
            reports: reports
        }
    }
}

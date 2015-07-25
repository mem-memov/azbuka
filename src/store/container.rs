use std::error::Error;
use std::fs::{OpenOptions, File};
use std::io::Write;

pub struct Container<'a> {
    path: &'a str,
}

impl<'a> Container<'a> {

    pub fn new<'b>(path: &'b str) -> Container<'b> {
        Container {
            path: path,
        }
    }

    pub fn append(&self) {

        let mut val = [5,5,5,5,5];

        let mut file = match OpenOptions::new().append(true).open(self.path) {
            Err(_) => self.create(),
            Ok(file) => file,
        };

        file.write(&val);

    }

    pub fn getAt() {

    }

    fn create(&self) -> File {

        let file = match OpenOptions::new().create(true).open(self.path) {
            Err(why) => panic!("couldn't open {}: {}", self.path, Error::description(&why)),
            Ok(file) => file,
        };

        file

    }

    fn delete() {

    }



}

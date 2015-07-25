//use std::error::Error;
//use std::fs::File;
//use std::io::prelude::*;
use std::path::Path;

pub struct Container<'a> {
    path: &'a str,
}

impl<'a> Container<'a> {

    pub fn new<'b>(path: &'b str) -> Container<'b> {
        Container {
            path: path,
        }
    }

    pub fn append() {

    }

    pub fn getAt() {

    }

    fn create() {

    }

    fn delete() {

    }



}

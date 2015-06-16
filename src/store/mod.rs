use std::io;
use std::fs;

pub fn write() -> io::Result<()> {

	let mut f = try!(fs::File::create("foo.txt"));
	Ok(())

}
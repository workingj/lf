use std::error::Error;
use std::fmt;

pub struct MyError {
	pub emsg: String,
}

pub fn my_error(emsg: String) -> MyError {
	MyError { emsg }
}

impl MyError {
	pub fn new(emsg: String) -> MyError {
		MyError { emsg }
	}
}

impl fmt::Display for MyError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", self.emsg)
	}
}
impl fmt::Debug for MyError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", self.emsg)
	}
}

impl Error for MyError {}

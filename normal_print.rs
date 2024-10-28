use std::io;
use std::io::Write;

pub(crate) fn printf(message: &str) {
    print!("{}", message);
    io::stdout().flush().unwrap();
}
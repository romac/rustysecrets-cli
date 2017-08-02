
// https://gist.github.com/ayosec/2ee0993247e003b42c5c

use std::{env, fs, io};
use std::io::prelude::*;

pub enum Input {
    Standard(io::Stdin),
    File(fs::File),
}

impl Input {
    pub fn stdin() -> Input {
        Input::Standard(io::stdin())
    }

    pub fn file(path: String) -> io::Result<Input> {
        Ok(Input::File(try!(fs::File::open(path))))
    }

    pub fn from_arg(arg: Option<String>) -> io::Result<Input> {
        Ok(match arg {
            None => Input::stdin(),
            Some(path) => try!(Input::file(path)),
        })
    }
}

impl io::Read for Input {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        match *self {
            Input::Standard(ref mut s) => s.read(buf),
            Input::File(ref mut f) => f.read(buf),
        }
    }
}

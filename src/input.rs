
// https://gist.github.com/ayosec/2ee0993247e003b42c5c

use std::{fmt, fs, io};

#[derive(Debug)]
pub enum Input {
    Standard(io::Stdin),
    File(fs::File, String),
}

impl Input {
    pub fn stdin() -> Input {
        Input::Standard(io::stdin())
    }

    pub fn file(path: String) -> io::Result<Input> {
        Ok(Input::File(try!(fs::File::open(path.clone())), path))
    }

    // pub fn from_arg(arg: Option<String>) -> io::Result<Input> {
    //     Ok(match arg {
    //         None => Input::stdin(),
    //         Some(path) => try!(Input::file(path)),
    //     })
    // }
}

impl io::Read for Input {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        match *self {
            Input::Standard(ref mut s) => s.read(buf),
            Input::File(ref mut f, _) => f.read(buf),
        }
    }
}

impl fmt::Display for Input {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Input::Standard(_) => write!(f, "<stdin>"),
            Input::File(_, ref path) => write!(f, "{}", path),
        }
    }
}

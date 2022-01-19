use std::ffi::{OsStr, OsString};
use std::fmt::{Display, Formatter};

fn main() {
    let arg = exit_code_arg()
        .and_then(|v| parse_utf8(&v))
        .and_then(|v| parse_i32(&v));

    std::process::exit(match arg {
        Ok(ok) => ok,
        Err(e) => {
            eprintln!("{}", e);
            255
        }
    });
}

static USAGE: &'static str = concat!(
    "ewc ",
    env!("CARGO_PKG_VERSION"),
    "
Martijn Gribnau <garm@ilumeo.com>

Returns the exit code, provided by you as the first argument, unless:
* Too many or too few arguments were given
* The argument could not be parsed as UTF-8
* The argument could not be parsed as an integer (i32)
* Platform specific behaviour, such as the maximum accepted range of integers, interfered with the
    execution behaviour of the program

When the program fails with an unexpected error, as above, it will output a message to stderr,
and exit with exit code 255. The program will not write to stderr in case of successful execution
of the program. NB: A non-zero exit code on its own does not indicate failure for this program.

Usage:
    ewc [EXIT_CODE]

Args:
    EXIT_CODE   The exit code which this program will return
"
);

fn exit_code_arg() -> Result<OsString, Error> {
    fn error() -> Error {
        let n = std::env::args_os().skip(1).count();
        Error::InvalidAmountOfArgs(n)
    }

    let mut iter = std::env::args_os().into_iter();

    let arg = iter.nth(1).ok_or_else(error)?;

    if let Some(_) = iter.peekable().peek() {
        Err(error())
    } else {
        Ok(arg)
    }
}

fn parse_utf8(arg: &OsStr) -> Result<String, Error> {
    arg.to_str()
        .map(ToString::to_string)
        .ok_or_else(|| Error::NotUTF8(arg.to_owned()))
}

fn parse_i32(arg: &str) -> Result<i32, Error> {
    arg.parse()
        .map_err(|_| Error::InvalidInteger(arg.to_owned()))
}

#[derive(Debug)]
enum Error {
    InvalidAmountOfArgs(usize),
    InvalidInteger(String),
    NotUTF8(OsString),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidAmountOfArgs(n) => f.write_fmt(format_args!(
                "Expected 1 argument (the exit code), but {} were given",
                *n
            )),
            Self::InvalidInteger(arg) => f.write_fmt(format_args!(
                "Argument '{}' could not be parsed as a valid integer (i32)",
                arg
            )),
            Self::NotUTF8(arg) => f.write_fmt(format_args!(
                "Argument '{:?}' could not be parsed to a UTF-8 formatted string",
                arg
            )),
        }?;

        f.write_fmt(format_args!("\n\n---\n\n{}", USAGE))
    }
}

impl std::error::Error for Error {}

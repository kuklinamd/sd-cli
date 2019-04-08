#[derive(PartialEq, Debug)]
pub enum ShellError {
    /// Input/Output error.
    IOError(IOErr),
    /// Other error.
    Error(&'static str)
}

#[derive(PartialEq, Debug)]
pub enum IOErr {
    /// Unable to write data.
    Write,
    /// Unable to read data.
    Read,
    /// Unable to execute the file.
    Exec,
    /// Unable to open the file.
    Open
}

/// Print errors.
pub fn eprint(err: ShellError) {
    match err {
        ShellError::IOError(io) => eprintio(io),
        ShellError::Error(s)    => eprintln!("{}", s)
    };
}

fn eprintio(err: IOErr) {
    match err {
        IOErr::Write => eprintln!("Unable to write to the given source."),
        IOErr::Read  => eprintln!("Unable to read the given source."),
        IOErr::Exec  => eprintln!("Unable to execute the given source."),
        IOErr::Open  => eprintln!("Unable to open the given source.")
    };
}

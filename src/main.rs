use std::{
    io,
    io::Write,
    os::unix::process::CommandExt,
    ffi,
    fmt,
    error::Error,
    convert::{
        AsRef,
    },
    env,
    fs,
    iter::Iterator,
    path::{
        self,
        Path,
        PathBuf
    },
    process::{
        self,
        Command,
        Stdio,
    },
};

#[derive(Debug)]
enum MethodError {
    IO(io::Error),
    String(ffi::OsString),
}

impl fmt::Display for MethodError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MethodError::IO(err) => err.fmt(f),
            MethodError::String(_) => write!(f, "Problem converting text to printable string...")
        }
    }
}

impl Error for MethodError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            MethodError::IO(err) => Some(err),
            MethodError::String(_) => None
        }
    }
}

impl From<ffi::OsString> for MethodError {
    fn from(error: ffi::OsString) -> Self {
	MethodError::String(error)
    }
}

impl From<io::Error> for MethodError {
    fn from(error: io::Error) -> Self {
	MethodError::IO(error)
    }
}


fn methods<P: AsRef<Path>>(enum_dir: P) -> io::Result<impl Iterator<Item=Result<String, MethodError>>> {

    fn to_string(result: io::Result<fs::DirEntry>) -> Result<String, MethodError> {
        Ok(result?.file_name().into_string()?)
    }

    fn file_only(entry_result: &io::Result<fs::DirEntry>) -> bool {
        match entry_result {
            Ok(entry) => entry.file_type()
                .map(|t| t.is_file())
                .unwrap_or(true),
            Err(_) => true
        }
    }

    let methods = fs::read_dir(enum_dir)?
        .filter(file_only)
        .map(to_string);

    Ok(methods)
}

fn list_methods<P: AsRef<Path>>(enum_dir: P) -> Result<(), MethodError> {
    let mut stdout = io::stdout();

    for method in methods(enum_dir)? {
        writeln!(&mut stdout, "{}", method?);
    }

    Ok(())
}

fn main() {

    let mut args = env::args()
        .skip(1);

    // Since this is an 'interpreter', the path command that's run is passed as the second
    // argument.
    let invoker = args.next()
        .expect("Pass script as first argument, and subcommand as second");


    let path = path::PathBuf::from(invoker);
    let name = path.file_name();
    let enum_dir = format!("{}.d", path.display());

    let mut command = match args.next() {
        Some(sub) => {
            let possible: Vec<String> = methods(&enum_dir)
                .expect("There was a problem reading the subcommand directory")
                .filter_map(Result::ok)
                .filter(|file| file.starts_with(&sub))
                .collect();

            if let Some(name) = possible.iter().find(|&f| f == &sub) {

                Command::new(format!("{}/{}", enum_dir, name))

            } else if possible.len() == 1 {

                let name = possible.get(0)
                    .unwrap();

                Command::new(format!("{}/{}", enum_dir, name))
                
            } else {

                eprintln!("Ambiguous subcommand. Possible matches:");

                for file in possible {
                    println!("{}", file);
                }

                process::exit(1);

            }
            
        },
        None => {
            list_methods(enum_dir)
                .expect("There was a problem printing the subcommands: {}");

            process::exit(0);
        }
    };

    command.stdin(Stdio::inherit())
        .stdout(Stdio::inherit());

    while let Some(arg) = args.next() {
        command.arg(arg);
    }

    command.exec();

    println!("Trouble forking command");
    process::exit(1);
}

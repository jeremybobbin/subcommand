use std::{
    io,
    convert::{
        AsRef,
    },
    env,
    fs,
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

struct Builder {

}

struct Config {
    directory: PathBuf,
    subcommand: String,
    menu: bool,
    cache: bool,
    strict: bool
}

fn list_methods<P: AsRef<Path>>(enum_dir: P) -> io::Result<()> {
    for dir in fs::read_dir(enum_dir)? {

        let method = dir?
            .file_name()
            .into_string()
            .unwrap();

        println!("{}", method );
    }

    Ok(())
}

fn parse_args() -> Option<Config> {
    let mut args = env::args();

    let script = args.next()
        .expect("Pass script as first argument, and subcommand as second");


    let path = path::PathBuf::from(script);
    let name = path.file_name();
    let enum_dir = format!("{}.d", path.display());

    let subcommand = args.next() {
        Some(cmd) => cmd,
        None => {
            eprintln!("Please use one of the following subcommands:");
            list_methods(enum_dir);
            process::exit(1);
        },
    };

    let mut cache = false;
    let mut menu = false;
    let mut strict = false;
    
    let mut rest: Vec<String> = Vec::new();

    if let Some(arg) = args.next() {
        if args.starts_with("-") {
            let mut chars = arg[1..].chars();

            while let Some(c) = chars.next() {
                match c {
                    'd' => menu = true,
                    's' => strict = true,
                    'c' => cache = true,
                    _ => panic!("Unrecognized.."),

                }
            }

        } else {
            rest.push(arg)
        }
    }

    if strict || cache {
        menu = true;
    }



    //if subcommand == "-l" {
    //    list_methods(enum_dir);
    //    process::exit(0);
    //}

    //let to_execute = format!("{}/{}", enum_dir, subcommand);
    
    None
}

fn main() {

    //let status = Command::new(to_execute)
    //    .stdin(Stdio::inherit())
    //    .stdout(Stdio::inherit())
    //    .args(&args[3..])
    //    .status()
    //    .expect("FAIL");
}

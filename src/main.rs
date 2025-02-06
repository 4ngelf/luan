use std::{
    env, ffi::{OsStr, OsString}, fs::File, io::{self, prelude::*}, path::PathBuf, process::{self, Command}
};

use tempfile::NamedTempFile;

mod error;
use error::Result;

type Arguments = Vec<OsString>;

fn main() -> Result<()> {
    let args = match parse_args() {
        Ok(args) => args,
        Err(err) => {
            eprintln!("Error: {err}");
            print_help(true)?;
            process::exit(1);
        }
    };

    let mut temp_info = NamedTempFile::new()?;
    let temp = temp_info.as_file_mut();

    for line in args.lua_run {
        temp.write_all(line.as_ref())?;
    }

    let source: &mut dyn Read = match args.lua_script {
        Source::Stdin => &mut io::stdin(),
        Source::File(path) => &mut File::open(path)?
    };

    stream_into(source, temp)?;

    let arguments = ["-l".as_ref(), temp_info.path().as_os_str()];
    let arguments = arguments
        .iter()
        .copied()
        .chain(args.lua_args.iter().map(|s| s.as_os_str()));

    neovim_run(arguments)?;

    //println!("Arguments: {args:?}");
    Ok(())
}

#[derive(Default, Debug)]
pub struct Args {
    lua_run: Vec<String>,
    lua_script: Source,
    lua_args: Arguments,
    #[allow(unused)]
    interactive: bool,
    #[allow(unused)]
    neovim_args: Option<Arguments>,
}

/// Should we execute code from stdin or a file?
#[derive(Default, Debug)]
enum Source {
    #[default]
    Stdin,
    File(PathBuf),
}

pub fn parse_args() -> Result<Args> {
    use lexopt::prelude::*;

    let mut parameters = Args::default();

    let mut parser = lexopt::Parser::from_env();

    // Get source while handling options
    parameters.lua_script = loop {
        let Some(arg) = parser.next()? else {
            break Source::Stdin;
        };
        match arg {
            // Execute commands
            Short('e') => {
                let value: String = parser.value()?.parse()?;
                parameters.lua_run.push(format!("do {value} end;"));
            }
            Short('l') => {
                let value: String = parser.value()?.parse()?;
                parameters.lua_run.push(format!("require(\"{value}\");"));
            }

            // Modifiers
            Short('i') | Long("interactive") => {
                return Err("interactive option not implemented yet".into());
            }
            Short('n') | Long("neovim-args") => {
                return Err("neovim-args option not implemented yet".into());
            }

            // Information
            Short('v') | Long("version") => {
                const NAME: &str = env!("CARGO_BIN_NAME");
                const VERSION: &str = env!("CARGO_PKG_VERSION");
                println!("{NAME} v{VERSION}");

                neovim_run(["--version"])?;

                process::exit(0);
            }
            Short('h') | Long("help") => {
                print_help(false)?;
                process::exit(0);
            }

            // stop handling options
            Value(source) => {
                if source == "-" {
                    break Source::Stdin;
                } else {
                    break Source::File(PathBuf::from(source));
                }
            }

            _ => return Err(arg.unexpected().into()),
        }
    };

    // The rest are arguments for lua_script
    while let Ok(arg) = parser.value() {
        parameters.lua_args.push(arg);
    }

    Ok(parameters)
}

fn neovim_run<S: AsRef<OsStr>, I: IntoIterator<Item = S>>(args: I) -> Result<()> {
    let _cmd = Command::new("nvim")
        .args(["-u", "NONE", "-i", "NONE"])
        .args(args)
        .status()?;

    Ok(())
}

fn stream_into<R: Read + ?Sized, W: Write + ?Sized>(from: &mut R, into: &mut W) -> io::Result<()> {
    let mut buf = io::BufReader::new(from);

    loop {
        let data = buf.fill_buf()?;

        if !data.is_empty() {
            into.write_all(data)?;
            
            let len = data.len();
            buf.consume(len);
        } else {
            break
        }
    }

    Ok(())
}


fn print_help(to_stderr: bool) -> io::Result<()> {
    let exe = env::current_exe();
    let exe = exe
        .as_ref()
        .ok()
        .and_then(|path| path.to_str())
        .unwrap_or(env!("CARGO_BIN_NAME"));

    let stream: &mut dyn Write = if !to_stderr {
        &mut io::stdout()
    } else {
        &mut io::stderr()
    };

    write!(
        stream,
        r##"usage: {exe} [options] [script [args]].
Available options are:
  -e statement      execute string 'statement'
  -l name           require library 'name'
  -i,--interactive  enter interactive mode after executing 'script'
  -n,--neovim-args  list of arguments to pass to neovim.
                    Default: "-u NONE -i NONE"
  -h,--help         show this help
  -v,--version      show version information
  --                stop handling options
  -                 execute stdin and stop handling options
"##
    )
}

//impl FromIterator<Execute> for String {
//    type IntoIterator
//    fn from_iter<T: IntoIterator<Item = A>>(iter: T) -> Self {
//
//    }
//}

//fn neovim_exists() -> bool {
//    false
//}

use std::{
    ffi::OsStr,
    fs::File,
    io::{self, prelude::*},
    process::{self, Command},
};

use anyhow::Result;
use tempfile::NamedTempFile;

mod cli;
use cli::{Action, ScriptSource};

fn main() -> Result<()> {
    let args = match cli::parse_args() {
        Ok(args) => args,
        Err(err) => {
            eprintln!("Error: {}", err);
            print_help(&mut io::stderr());
            process::exit(1)
        }
    };

    match args.action {
        Action::Normal => {
            let temp_script = make_temp_lua_script(&args.options)?;
            let temp_script_path = temp_script.path().as_os_str();

            let neovim_args = ["-l".as_ref(), temp_script_path].into_iter();
            let script_args = args.options.script_args.iter().map(AsRef::as_ref);

            neovim_run(neovim_args.chain(script_args))?;
        }
        Action::Interactive => unimplemented!("interactive mode"),
        Action::Help => print_help(&mut io::stdout()),
        Action::Version => {
            const NAME: &str = env!("CARGO_BIN_NAME");
            const VERSION: &str = env!("CARGO_PKG_VERSION");
            println!("{NAME} v{VERSION}");

            neovim_run(["--version"])?;
        }
    }

    Ok(())
}

fn print_help<W: Write + ?Sized>(stream: &mut W) {
    let exe = std::env::current_exe();
    let exe = exe
        .as_ref()
        .ok()
        .and_then(|path| path.to_str())
        .unwrap_or(env!("CARGO_BIN_NAME"));

    write!(
        stream,
        r##"usage: {exe} [options] [script [args]].
Available options are:
  -e statement      execute string 'statement'
  -l name           require library 'name'
  -i,--interactive  enter interactive mode after executing 'script'
  -h,--help         show this help
  -v,--version      show version information
  --                stop handling options
  -                 execute stdin and stop handling options
"##
    )
    .expect("failed writing help");
}

fn neovim_run<S, I>(args: I) -> io::Result<process::ExitStatus>
where
    S: AsRef<OsStr>,
    I: IntoIterator<Item = S>,
{
    Command::new("nvim")
        .args(["-u", "NONE", "-i", "NONE"])
        .args(args)
        .status()
}

fn make_temp_lua_script(opts: &cli::Options) -> Result<NamedTempFile> {
    let mut temp = NamedTempFile::new()?;
    let temp_file = temp.as_file_mut();

    temp_file.write_all(opts.lua_run_before.as_bytes())?;

    let source: &mut dyn Read = match &opts.script {
        ScriptSource::Stdin => &mut io::stdin(),
        ScriptSource::File(path) => &mut File::open(path)?,
    };

    stream_into(source, temp_file)?;

    Ok(temp)
}

fn stream_into<R: Read + ?Sized, W: Write + ?Sized>(src: &mut R, dst: &mut W) -> io::Result<()> {
    let mut buf = io::BufReader::new(src);

    loop {
        let data = buf.fill_buf()?;

        if !data.is_empty() {
            dst.write_all(data)?;

            let len = data.len();
            buf.consume(len);
        } else {
            break;
        }
    }

    Ok(())
}

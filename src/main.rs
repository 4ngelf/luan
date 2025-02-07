use std::{
    fs::File,
    io::{self, BufRead, Read, Write},
    process,
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
            print_help();
            process::exit(1)
        }
    };

    match args.action {
        Action::Normal => {
            let temp_script = make_temp_lua_script(&args)?;

            neovim()
                .arg("-l")
                .arg(temp_script.path())
                .args(args.script_args)
                .status()
                .map_err(neovim_error)?;
        }
        Action::Interactive => eprintln!("Error: interactive mode not available"),
        Action::Help => print_help(),
        Action::Version => {
            const NAME: &str = env!("CARGO_BIN_NAME");
            const VERSION: &str = env!("CARGO_PKG_VERSION");
            println!("{NAME} v{VERSION}");

            neovim().arg("--version").status().map_err(neovim_error)?;
        }
    }

    Ok(())
}

fn print_help() {
    let exe = std::env::current_exe().ok();
    let exe = exe
        .as_ref()
        .and_then(|path| path.file_name())
        .and_then(|path| path.to_str())
        .unwrap_or(env!("CARGO_BIN_NAME"));

    eprintln!(
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
}

fn neovim() -> process::Command {
    let mut nvim = process::Command::new("nvim");
    nvim.args(["-u", "NONE", "-i", "NONE"]);
    nvim
}

fn neovim_error(error: io::Error) -> anyhow::Error {
    match &error.kind() {
        io::ErrorKind::NotFound => anyhow::anyhow!("Neovim not found!"),
        _ => anyhow::anyhow!(error),
    }
}

fn make_temp_lua_script(opts: &cli::Args) -> Result<NamedTempFile> {
    let mut temp = NamedTempFile::new()?;
    let temp_file = temp.as_file_mut();

    temp_file.write_all(opts.lua_run_before.as_bytes())?;

    let source: &mut dyn Read = match &opts.script {
        ScriptSource::Unspecified => &mut io::empty(),
        ScriptSource::Stdin => &mut io::stdin(),
        ScriptSource::File(path) => &mut File::open(path)?,
    };

    stream_into(source, temp_file)?;

    Ok(temp)
}

fn stream_into<R: Read + ?Sized, W: Write + ?Sized>(src: &mut R, dst: &mut W) -> io::Result<()> {
    let mut reader = io::BufReader::new(src);

    loop {
        let data = reader.fill_buf()?;

        if !data.is_empty() {
            dst.write_all(data)?;

            let len = data.len();
            reader.consume(len);
        } else {
            break;
        }
    }

    Ok(())
}

//! Command line interface
use anyhow::{bail, Result};
use std::{ffi::OsString, path::PathBuf};

#[derive(Default, Debug)]
pub struct Args {
    pub action: Action,
    pub lua_run_before: String,
    pub script: ScriptSource,
    pub script_args: Vec<OsString>,
    // TODO: Add an option to modify neovim options?
    #[allow(unused)]
    pub neovim_args: Vec<OsString>,
}

#[derive(Default, Debug)]
pub enum Action {
    /// Execute lua code
    #[default]
    Normal,
    /// Run interactive mode
    #[allow(unused)]
    Interactive,
    /// Show help
    Help,
    /// Show version
    Version,
}

/// Should we execute code from stdin or a file?
#[derive(Default, Debug)]
pub enum ScriptSource {
    #[default]
    Unspecified,
    Stdin,
    File(PathBuf),
}

pub fn parse_args() -> Result<Args> {
    use lexopt::prelude::*;

    let mut args = Args::default();

    let mut parser = lexopt::Parser::from_env();
    while let Some(arg) = parser.next()? {
        match arg {
            // Execute commands
            Short('e') => {
                let expr: String = parser.value()?.parse()?;
                let statement = format!("do {expr} end\n");
                args.lua_run_before.push_str(&statement);
            }
            Short('l') => {
                let expr: String = parser.value()?.parse()?;
                let statement = format!("require'{expr}'\n");
                args.lua_run_before.push_str(&statement);
            }

            // Modifiers
            Short('i') | Long("interactive") => {
                args.action = Action::Interactive;
                break;
            }
            Short('n') | Long("neovim-args") => {
                bail!("neovim-args option not implemented yet");
            }

            // Information
            Short('v') | Long("version") => {
                args.action = Action::Version;
                break;
            }
            Short('h') | Long("help") => {
                args.action = Action::Help;
                break;
            }

            // stop handling options
            Value(source) => {
                args.script = if source == "-" {
                    ScriptSource::Stdin
                } else {
                    ScriptSource::File(PathBuf::from(source))
                };
                break;
            }

            _ => bail!(arg.unexpected()),
        }
    }

    if let ScriptSource::Unspecified = &args.script {
        let is_terminal = is_terminal();
        let no_statements = args.lua_run_before.is_empty();

        if is_terminal && no_statements {
            args.action = Action::Interactive;
        }

        if !is_terminal || no_statements {
            args.script = ScriptSource::Stdin;
        }
    }

    // The rest are arguments for script
    while let Ok(arg) = parser.value() {
        args.script_args.push(arg);
    }

    Ok(args)
}

fn is_terminal() -> bool {
    use std::io::{stdin, IsTerminal};
    stdin().is_terminal()
}

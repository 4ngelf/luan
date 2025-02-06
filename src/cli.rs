//! Command line interface
use anyhow::{bail, Result};
use std::{ffi::OsString, path::PathBuf};

pub type Arguments = Vec<OsString>;

#[derive(Default, Debug)]
pub struct ParseArgs {
    pub action: Action,
    pub options: Options,
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

#[derive(Default, Debug)]
pub struct Options {
    pub lua_run_before: String,
    pub script: ScriptSource,
    pub script_args: Arguments,
    // TODO: Add an option to modify neovim options?
    #[allow(unused)]
    pub neovim_args: Arguments,
}

/// Should we execute code from stdin or a file?
#[derive(Default, Debug)]
pub enum ScriptSource {
    #[default]
    Stdin,
    File(PathBuf),
}

pub fn parse_args() -> Result<ParseArgs> {
    use lexopt::prelude::*;

    let mut action = Action::Normal;
    let mut options = Options::default();

    let mut parser = lexopt::Parser::from_env();
    while let Some(arg) = parser.next()? {
        match arg {
            // Execute commands
            Short('e') => {
                let expr: String = parser.value()?.parse()?;
                let statement = format!("do {expr} end\n");
                options.lua_run_before.push_str(&statement);
            }
            Short('l') => {
                let expr: String = parser.value()?.parse()?;
                let statement = format!("require'{expr}'\n");
                options.lua_run_before.push_str(&statement);
            }

            // Modifiers
            Short('i') | Long("interactive") => {
                action = Action::Interactive;
                break;
            }
            Short('n') | Long("neovim-args") => {
                bail!("neovim-args option not implemented yet");
            }

            // Information
            Short('v') | Long("version") => {
                action = Action::Version;
                break;
            }
            Short('h') | Long("help") => {
                action = Action::Help;
                break;
            }

            // stop handling options
            Value(source) => {
                options.script = if source == "-" {
                    ScriptSource::Stdin
                } else {
                    ScriptSource::File(PathBuf::from(source))
                };
                break;
            }

            _ => bail!(arg.unexpected()),
        }
    }

    // The rest are arguments for script
    while let Ok(arg) = parser.value() {
        options.script_args.push(arg);
    }

    Ok(ParseArgs { action, options })
}

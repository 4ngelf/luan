// Test global vim is available
use std::process::Command;

type Result = anyhow::Result<()>;

fn luan() -> Command {
    Command::new(env!("CARGO_BIN_EXE_luan"))
}

#[test]
fn vim_global() -> Result {
    let status = luan().args(["-e", "assert(vim)"]).status()?;

    assert!(status.success());
    Ok(())
}

#[test]
fn e_option_isolation() -> Result {
    let status = luan()
        .args(["-e", "local foo = true"])
        .args(["-e", "assert(foo == nil)"])
        .status()?;

    assert!(status.success());
    Ok(())
}

/// code in -l and -e options must run in order and before the script code
#[test]
fn execution_order() -> Result {
    const LUA_PACKAGE_PATH: &str = "-epackage.path='./tests/lua/?.lua'";
    const LUA_INIT: &str = "tests/lua/init.lua";

    macro_rules! assert_output {
        ($expected:expr, $cmd:expr) => {
            let out = $cmd.arg(LUA_INIT).output()?;
            let out = std::str::from_utf8(&out.stderr)?.trim();

            assert_eq!(out, $expected);
        };
    }

    assert_output!("luan",
        luan()
            .arg(LUA_PACKAGE_PATH)
            .arg("-llib")
            .args(["-e", "foo = 'luan'"])
    );

    assert_output!("library",
        luan()
            .arg(LUA_PACKAGE_PATH)
            .args(["-e", "foo = 'luan'"])
            .arg("-llib")
    );

    Ok(())
}

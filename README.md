# 🌙 Luan &emsp; [![ci-badge]][ci] [![latest-badge]][latest]

[ci-badge]: https://img.shields.io/github/actions/workflow/status/4ngelf/luan/ci.yaml?branch=main&label=Tests
[ci]: https://github.com/4ngelf/luan/actions/workflows/ci.yaml
[latest-badge]: https://img.shields.io/github/v/release/4ngelf/luan?label=latest
[latest]: https://github.com/4ngelf/luan/releases/latest

Neovim as a lua interpreter.

## Usage

luan interface is 100% compatible with the regular lua executable. That means you can use it as
a direct replacement as long as neovim is available on the system.

```sh
usage: luan [options] [script [args]].
Available options are:
  -e statement      execute string 'statement'
  -l name           require library 'name'
  -i,--interactive  enters interactive mode after running script
  -h,--help         show this help
  -v,--version      show version information
  --                stop handling options
  -                 execute stdin and stop handling options
```

## Installation

Ensure that [cargo][cargo] is installed and run:

[cargo]: https://doc.rust-lang.org/cargo/getting-started/installation.html

```sh
cargo install --git https://github.com/4ngelf/luan
```

### License

[MIT License](./LICENSE)

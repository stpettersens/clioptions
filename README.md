### clioptions

> :heavy_dollar_sign: A very thin wrapper for command line arguments in Rust.

[![Build Status](https://travis-ci.org/stpettersens/clioptions.png?branch=master)](https://travis-ci.org/stpettersens/clioptions)
[![Build status](https://ci.appveyor.com/api/projects/status/04vxn0sjoi3ufev6?svg=true)](https://ci.appveyor.com/project/stpettersens/clioptions)
[![Crates.io](https://img.shields.io/crates/v/clioptions.svg)](https://crates.io/crates/clioptions)

##### Usage:

- Add this to your `Cargo.toml` file.

```toml
[dependencies]
clioptions = { git = "https://github.com/stpettersens/clioptions.git" }
```

- Implement your command line arguments.

```rust
extern crate clioptions;
use clioptions::CliOptions;

fn main() {
    let cli = CliOptions::new("program_name"); // "program_name" is the fallback for argv[0].
    let program = cli.get_program();
    let mut filename = String::new();
    if cli.get_num() > 1 {
        for (i, a) in cli.get_args().iter().enumerate() {
            match a.trim() {
                "-h" | "--help" => display_usage(&program, 0),
                "-v" | "--version" => display_version(),
                "-f" | "--file" => filename = cli.next_argument(i), 
                // next_argument(i) gets the argument after i.
                _ => continue,
            }
        }
    }
    if(!filename.is_empty()) {
        do_something_with_filename(&filename);
    }
}
```

# clioptions
A very thin wrapper for command line arguments in Rust.

Usage:

- Add this to your `Cargo.toml` file.

```toml
[dependencies]
clioptions = { git = "https://github.com/stpettersens/clioptions.git" }
```

- Add something like this to your source code.

```rust
fn main() {
    let cli = CliOptions::new("program_name");
    let program = cli.get_program();
    let mut filename = String::new();
    if cli.get_num() > 1 {
        for (i, a) in cli.get_args().iter().enumerate() {

            if a == "-h" || a == "--help" {
                display_usage(&program, 0);
            }
            else if a == "-v" || a == "--version" {
                display_version();
            }
            
            if f == "-f" || a == "--file" {
                filename = cli.next_argument(i);
            }
        }
    }
}
```

# Practice Lab: Build a Command Line tool to list files in a directory

In this lab, you will build a command line tool that lists files in a directory. The tool will take a directory path as an argument and list all the files in the directory. By default, if no directory path is provided, the tool will list files in the current directory.

**Learning Objectives**

- Create a command line tool using the Rust programming language
- Use the `Clap` framework to parse command line arguments
- Use the `std::fs` module to read files and directories

**Steps:**

For an easy setup with all dependencies installed and a pre-configured Rust and Cargo, [open the repository with Codespaces](https://codespaces.new/alfredodeza/rust-cli-example?quickstart=1)

Alternate setup: Create a new repository in your account for your Rust project. Use this link to [create it in one step](https://github.com/alfredodeza/rust-template/generate).

1. Use the Clap framework to parse command line arguments. Use the [simple example in this repository](./examples/simple/src/main.rs) as a reference
1. Use the `std::fs` module to read files. For example:

```rust
use std::fs;

fn main() {
    let path = ".";
    let paths = fs::read_dir(path).unwrap();
    for path in paths {
        println!("Name: {}", path.unwrap().path().display())
    }
}
```

**Bonus challenge:** Add options to your tool to list files recursively and to list hidden files.

**Concepts Covered:**

- [x] Build a command line tool using the Rust programming language
- [x] Use the `Clap` framework to parse command line arguments
- [x] Using the Rust standard library to read files and directories

By completing this lab, you have demonstrated that you can create a command line tool using the Rust programming language. You have also demonstrated that you can use the `Clap` framework to parse command line arguments and the Rust standard library to read files and directories.

# Rust CLI Example

A small Rust CLI example you can use to build on. With an emphasis on Linux and creating automation tools that solve a problem for you. This is the basis for DevOps principles that you can apply in day-to-day work.

## Setting up your environment
Rust development requires certain tools to be installed on your system. The easiest way to do this is to use the [rustup](https://rustup.rs/) tool. This will install the Rust compiler and Cargo, the Rust package manager. Although you can install it in Linux using the package manager, I recommend using `rustup`. Use the following command or go through the [rustup.rs](https://rustup.rs/) website to install it.

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

This repository and video course focuses on the development side of command-line tools in Rust. It uses [Visual Studio Code](https://code.visualstudio.com/?WT.mc_id=academic-0000-alfredodeza) as the editor of choice. You can use any editor you like, but the instructions in this repository will be for VS Code.

These are all the tools and editor extensions I recommend you install to get started:

- [Visual Studio Code](https://code.visualstudio.com/?WT.mc_id=academic-0000-alfredodeza)
- [Rust and Cargo tools](https://rustup.rs/)
- [Rust Analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer&WT.mc_id=academic-0000-alfredodeza)
- [GitHub Copilot](https://marketplace.visualstudio.com/items?itemName=GitHub.copilot&WT.mc_id=academic-0000-alfredodeza)

As part of your development workflow, I highly suggest you use the following programs in the terminal regularly:

- `cargo fmt` - Formats your code to the Rust standard
- `cargo clippy` - Lints your code and helps you find errors and potential issues
- `cargo check` - Checks your code for errors and allows you to fix them before compiling (which means its faster!)

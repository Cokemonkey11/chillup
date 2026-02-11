# Gemini CLI Configuration for chillup

This document provides instructions and configurations for using the Gemini CLI with the `chillup` project.

## About This File

This `GEMINI.md` file is intended to be used with the Gemini CLI. It provides a centralized place for project-specific commands and instructions that can be used by the Gemini CLI to assist with development tasks.

## Getting Started with Gemini CLI

To get started with the Gemini CLI, you can use the following commands:

*   `gemini`: The main command for the Gemini CLI.
*   `gemini --help`: Displays a list of available commands and options.

## Project Commands

This project uses `just` as a command runner. The following commands are available in the `justfile`:

*   `just`: Lists the available commands.
*   `just check`: Runs `cargo check`, `cargo test`, `cargo clippy`, `cargo fmt`, `cargo update`, and `cargo outdated`.
*   `just cargo-check`: Runs `cargo check` to quickly check the code for errors.
*   `just test`: Runs the test suite using `cargo test`.
*   `just clippy`: Lints the code using `cargo clippy` to catch common mistakes and improve the code.
*   `just fmt`: Formats the code using `cargo fmt`.
*   `just update`: Updates the dependencies using `cargo update`.
*   `just outdated`: Shows outdated dependencies using `cargo outdated -R`.

## Building and Running

To build the project, you can use the following command:

```bash
cargo build
```

To run the project, you can use the following command:

```bash
cargo run
```

## Testing

To run the tests, you can use the following command:

```bash
just test
```

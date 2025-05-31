# RISE (Rust Interactive SHell Environment)

RISE is yet anothe interactive shell environment written in rust that implements promintent features of BASH

## Features

* **Interactive Prompt:** Displays a `🦀 >>` prompt for user input.
* **Command Execution:** Executes external programs with arguments.
* **Built-in Commands:**
    * `cd <directory>`: Changes the current working directory. Defaults to `/` if no directory is specified.
    * `exit`: Terminates the RISE shell.
* **Basic Error Handling:** Prints errors for command not found or issues with built-ins like `cd`.
* **Current Directory Information**: Displays the current working directory at all times.

## Getting Started

### Prerequisites

* Rust programming language toolchain (Rustc and Cargo). You can install it from [rust-lang.org](https://www.rust-lang.org/tools/install).

### Building

1.  Clone the repository (or navigate to your project directory):
    ```bash
    # git clone <your-repo-url> # If you have it on Git
    cd path/to/your/rise-project
    ```
2.  Build the project using Cargo:
    ```bash
    cargo build
    ```
    You can also build an optimized release version:
    ```bash
    cargo build --release
    ```

### Running

1.  After building, you can run RISE using Cargo:
    ```bash
    cargo run
    ```

## Basic Usage

Once RISE is running, you'll see the `>> 🦀 ` prompt. Type your commands and press Enter.


## Future Enhancements

RISE is currently a work in progress. Potential future features could include:

* More robust error handling and reporting.
* Additional built-in commands (e.g., `pwd`, `echo`, `export`).
* Support for pipes
* Support for environment variables.
* Command history.
* Input/output redirection (`<`, `>`, `>>`).
* Background processes (`&`).
* Tab completion.
* Scripting capabilities.
* Globbing (`*`, `?`).

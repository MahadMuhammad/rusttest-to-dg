//! This module contains the command-line arguments `struct` and related functions for the tool.

use std::{fs, path};

use anyhow::{Context, Result};
use clap::Parser;

/// # Command-line arguments `struct` for the tool
///
/// This struct defines the command-line arguments that the tool accepts,
/// using the [`clap`](https://docs.rs/clap/latest/clap/) crate with `derive` feature for argument parsing.
///
/// # Arguments
///
/// * `source_file` - The Rust source file to convert into DejaGnu format.
/// * `stderr_file` - Optional. The file used to extract rustc error codes, line/column numbers, and convert them into DejaGnu format.
///
/// # Examples
///
/// ```rust
/// // Example of parsing argument
/// let args = Arguments::parse();
/// // Example of accessing the parsed argument
/// println!("Source file: {}", args.source_file.display());
/// if let Some(stderr_file) = &args.stderr_file {
///    println!("Stderr file: {}", stderr_file.display());
/// }
/// ```
#[derive(Parser, Debug)]
#[command(
    name = "rust test to DejaGnu",
    long_about = "A tool to convert rust tests into DejaGnu tests format"
)]
pub struct Arguments {
    /// The `Rustc` source file to convert into `DejaGnu` format.
    #[arg(
        short = 'f',
        long = "file",
        value_name = "FILE",
        help = "The rust source file to convert into DejaGnu format"
    )]
    pub source_file: path::PathBuf,

    /// Optional. `Rustc` stores error output along with error codes in `<file_name>.stderr`.
    /// The file used to extract rustc error codes
    #[arg(
        short = 'e',
        long = "stderr",
        value_name = "STDERR_FILE",
        help = "These file are used to extract rustc error codes, line/column numbers and convert them into DejaGnu format",
        required = false
    )]
    pub stderr_file: Option<path::PathBuf>,
}

/// **Helper function:** Takes the parsed arguments and returns the source code and `stderr` code, if provided in `String` format.
///
/// This function takes the parsed command-line arguments, reads the contents of the specified
/// source file, and optional stderr file. And it returns the content of the source file and
/// the content of the `stderr` file (if provided).
///
/// # Arguments
///
/// * `args` - A reference to the `Arguments` struct containing the parsed command-line arguments.
///
/// # Returns
///
/// A `Result` containing a tuple with the content of the source file as a `String` and an
/// `Option<String>` with the content of the stderr file (if provided). If an error occurs
/// while reading the files, an `anyhow::Error` is returned.
///
/// # Errors
///
/// This function will return an error if it fails to read the source file or the stderr file.
///
/// # Examples
///
/// ```rust
/// let args = Arguments::parse();
/// match parse_arguments_and_read_file(&args) {
///     Ok((source_code, err_file)) => {
///         println!("Source code: {}", source_code);
///         if let Some(err) = err_file {
///             println!("Stderr file content: {}", err);
///         }
///     }
///     Err(e) => eprintln!("Error: {}", e),
/// }
/// ```
pub fn parse_arguments_and_read_file(args: &Arguments) -> Result<(String, Option<String>)> {
    //TODO: maybe to use sanitization to prevent reading files outside the project directory
    let source_code = fs::read_to_string(&args.source_file)
        .with_context(|| format!("could not read sourcefile `{}`", args.source_file.display()))?;

    let err_file =
        match &args.stderr_file {
            Some(stderr_file) => Some(fs::read_to_string(stderr_file).with_context(|| {
                format!("could not read stderr file `{}`", stderr_file.display())
            })?),
            None => None,
        };

    Ok((source_code, err_file))
}

/// Prints the provided source code to the standard output.
///
/// # Arguments
///
/// * `source_code` - A reference to a string slice that holds the source code.
///
/// # Examples
///
/// ```rust
/// let source_code = "fn main() { println!(\"Hello, world!\"); }";
/// print_source_code(source_code);
/// ```
pub fn print_source_code(source_code: &str) {
    println!("{source_code}");
}

/// Unit tests for the `Arguments` struct and related functions.
#[cfg(test)]
mod tests {
    use super::*;

    /// Tests that the required `source_file` argument is parsed correctly.
    ///
    /// This test verifies that when only the required `-f` argument is provided,
    /// the `source_file` field is set correctly and the `stderr_file` field is `None`.
    #[test]
    fn test_required_argument_file() {
        let args = Arguments::parse_from(["test", "-f", "test.rs"]);
        assert_eq!(args.source_file, path::PathBuf::from("test.rs"));
        assert_eq!(args.stderr_file, None);
    }

    /// Tests that the optional `stderr_file` argument is parsed correctly.
    ///
    /// This test verifies that when both the required `-f` argument and the optional
    /// `-e` argument are provided, the `source_file` and `stderr_file` fields are set correctly.
    #[test]
    fn test_optional_argument_file() {
        let args = Arguments::parse_from(["test", "-f", "test.rs", "-e", "test.stderr"]);
        assert_eq!(args.source_file, path::PathBuf::from("test.rs"));
        assert_eq!(args.stderr_file, Some(path::PathBuf::from("test.stderr")));
    }

    /// clap reports most development errors as `debug_assert!`s. Rather than checking every subcommand,
    /// you should have a test that calls `Command::debug_assert`
    ///
    /// See [docs.rs/clap/latest/clap/_derive/_tutorial/chapter_4](https://docs.rs/clap/latest/clap/_derive/_tutorial/chapter_4)
    #[test]
    fn debug_args() {
        use clap::CommandFactory;
        let command = Arguments::command();
        command.debug_assert();
    }
}

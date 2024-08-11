//! This module contains the command line interface for the tool

use {
    anyhow::{Context, Result},
    clap::Parser,
    std::{fs, path},
};

/// Command line arguments for the tool
#[derive(Parser, Debug)]
#[command(
    name = "rust test to DejaGnu",
    long_about = "A tool to convert rust tests into DejaGnu tests format"
)]
pub struct Arguments {
    /// The rust source file to convert into `DejaGnu` format
    #[arg(
        short = 'f',
        long = "file",
        value_name = "FILE",
        help = "The rust source file to convert into DejaGnu format"
    )]
    pub source_file: path::PathBuf,

    /// `optional argument`: The `stderr` file to extract rustc error codes, column numbers and convert them into `DejaGnu` format
    #[arg(
        short = 'e',
        long = "stderr",
        value_name = "STDERR_FILE",
        help = "These file are used to extract rustc error codes, line/column numbers and convert them into DejaGnu format",
        required = false
    )]
    pub stderr_file: Option<path::PathBuf>,
}

/// Parses the command line arguments and reads the input file.
///
/// # Arguments
///
/// * `args` - A reference to the `Arguments` struct containing the parsed command line arguments.
///
/// # Returns
///
/// * `Result<(String, Option<String>)>` - Returns a tuple containing the source code and optionally the stderr file content if successful, otherwise returns an error.
pub fn parse_arguments_and_read_file(args: &Arguments) -> Result<(String, Option<String>)> {
    //TODO: maybe to use sanitization to prevent reading files outside the project directory
    let source_code = fs::read_to_string(&args.source_file)
        .with_context(|| format!("could not read sourcefile `{}`", args.source_file.display()))?;

    // Read the stderr file if it exists
    let err_file =
        match &args.stderr_file {
            Some(stderr_file) => Some(fs::read_to_string(stderr_file).with_context(|| {
                format!("could not read stderr file `{}`", stderr_file.display())
            })?),
            None => None,
        };

    Ok((source_code, err_file))
}

/// Prints the source code to the standard output.
///
/// # Arguments
///
/// * `source_code` - A reference to the source code string to be printed.
pub fn print_source_code(source_code: &str) {
    println!("{source_code}");
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Tests the required argument `file`.
    #[test]
    fn test_required_argument_file() {
        let args = Arguments::parse_from(["test", "-f", "test.rs"]);
        assert_eq!(args.source_file, path::PathBuf::from("test.rs"));
        assert_eq!(args.stderr_file, None);
    }

    /// Tests the optional argument `stderr_file`.
    #[test]
    fn test_optional_argument_file() {
        let args = Arguments::parse_from(["test", "-f", "test.rs", "-e", "test.stderr"]);
        assert_eq!(args.source_file, path::PathBuf::from("test.rs"));
        assert_eq!(args.stderr_file, Some(path::PathBuf::from("test.stderr")));
    }

    /// Tests the debug assertions for the command line arguments.
    /// clap reports most development errors as `debug_assert!`s
    /// See this for more details, [here](https://docs.rs/clap/4.5.15/clap/_derive/_tutorial/chapter_4/index.html)
    #[test]
    fn debug_args() {
        use clap::CommandFactory;
        let command = Arguments::command();
        command.debug_assert();
    }
}

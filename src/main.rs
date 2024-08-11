//! The main entry point of the program.

use {
    anyhow::{Context, Result},
    clap::Parser,
};

mod cli;
mod errors;
mod transform;

/// The main function of the program.
///
/// # Returns
///
/// * `Result<()>` - Returns an `Ok(())` if the program runs successfully, otherwise returns an error.
fn main() -> Result<()> {
    try_parse()
}

/// Parses the command line arguments, reads the input file, transforms the code, and prints the transformed code.
///
/// # Returns
///
/// * `Result<()>` - Returns an `Ok(())` if the operations are successful, otherwise returns an error.
fn try_parse() -> Result<()> {
    let args = cli::Arguments::parse();

    let (code, stderr_code) = cli::parse_arguments_and_read_file(&args)?;

    let new_code = transform::transform_code(&code, stderr_code.as_deref()).with_context(|| {
        format!(
            "could not transform code from file `{}`",
            args.source_file.display()
        )
    })?;

    cli::print_source_code(&new_code);

    Ok(())
}

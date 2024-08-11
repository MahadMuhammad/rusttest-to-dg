//! This module contains the logic for parsing rustc error messages.

use {
    self::WhichLine::*,
    std::{fmt, str::FromStr},
};

/// A macro to lazily compile a regular expression.
///
/// # Arguments
///
/// * `$re` - A string literal representing the regular expression pattern.
///
/// # Example
///
/// This example:
/// ```rust
/// regex!(r"^E\d{4}$")
/// ```
/// was expanded to
/// ```rust
///    {
///         static RE: once_cell::sync::OnceCell<regex::Regex> = once_cell::sync::OnceCell::new();
///         RE.get_or_init(|| regex::Regex::new(r"^E\d{4}$").unwrap())
///    }
/// ```
/// Another example:
/// ```rust
/// let re = regex!(r"^E\d{4}$");
/// assert!(re.is_match("E1234"));
/// ```
#[macro_export]
macro_rules! regex {
    ($re:literal $(,)?) => {{
        static RE: once_cell::sync::OnceCell<regex::Regex> = once_cell::sync::OnceCell::new();
        RE.get_or_init(|| regex::Regex::new($re).unwrap())
    }};
}

/// Represents the different kinds of Rustc compiler messages.
///
/// This enum is used to categorize the types of messages that the Rust compiler can produce,
/// See [rustc dev guide](https://rustc-dev-guide.rust-lang.org/tests/ui.html#error-levels)
///
/// # Variants
///
/// * `Help` - Represents a help message.
/// * `Error` - Represents an error message.
/// * `Note` - Represents a note message.
/// * `Suggestion` - Represents a suggestion message.
/// * `Warning` - Represents a warning message.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum RustcErrorKind {
    Help,
    Error,
    Note,
    Suggestion,
    Warning,
}

impl FromStr for RustcErrorKind {
    type Err = ();

    /// Converts a string slice to a `RustcErrorKind`.
    ///
    /// # Arguments
    ///
    /// * `s` - A case-insensitive String slice representing the error kind.
    ///
    /// # Returns
    ///
    /// * `Result<Self, Self::Err>` - Returns the corresponding `RustcErrorKind` variant if successful,
    ///   otherwise returns an error.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use std::str::FromStr;
    /// assert_eq!(RustcErrorKind::from_str("help").unwrap(), RustcErrorKind::Help);
    /// assert_eq!(RustcErrorKind::from_str("error").unwrap(), RustcErrorKind::Error);
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.to_uppercase();
        // Some RustcErrorKinds has this colon, so we need to split it
        // See this for example:
        // https://github.com/rust-lang/rust/blob/master/tests/ui/async-await/in-trait/fn-not-async-err.rs#L9
        let part0: &str = s
            .split(':')
            .next()
            .expect("split always returns at least one element");
        match part0 {
            "HELP" => Ok(RustcErrorKind::Help),
            "ERROR" => Ok(RustcErrorKind::Error),
            "NOTE" => Ok(RustcErrorKind::Note),
            "SUGGESTION" => Ok(RustcErrorKind::Suggestion),
            "WARN" | "WARNING" => Ok(RustcErrorKind::Warning),
            _ => Err(()),
        }
    }
}

impl fmt::Display for RustcErrorKind {
    /// Formats the `RustcErrorKind` for display.
    ///
    /// This method implements the `fmt` function from the `fmt::Display` trait,
    /// allowing `RustcErrorKind` to be formatted according to rust compiletest.
    ///
    /// # Arguments
    ///
    /// * `f` - A mutable reference to a `fmt::Formatter` used to build the formatted string.
    ///
    /// # Returns
    ///
    /// * `fmt::Result` - Returns `fmt::Result` indicating whether the formatting was successful.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use std::fmt;
    /// let kind = RustcErrorKind::Help;
    /// assert_eq!(format!("{}", kind), "help message");
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            RustcErrorKind::Help => write!(f, "help message"),
            RustcErrorKind::Error => write!(f, "error"),
            RustcErrorKind::Note => write!(f, "note"),
            RustcErrorKind::Suggestion => write!(f, "suggestion"),
            RustcErrorKind::Warning => write!(f, "warning"),
        }
    }
}

/// For representing an error in the Rustc source file.
///
/// This struct was use to store information from rustc source file
#[derive(Debug)]
pub struct Error {
    /// The line number where the error occurred.
    pub line_num: usize,
    /// We also need to take into account the relative line number.
    /// - `1` if the error is on the previous line
    /// - `0` if the error is on the same line
    /// - `-1` if the error is on the next line
    pub relative_line_num: i32,

    /// What kind of message we expect (e.g., warning, error, suggestion).
    /// `None` if not specified or unknown message kind.
    pub kind: Option<RustcErrorKind>,

    /// The error message. (if we are loading this from rustc source file, this might be incomplete)
    pub msg: String,

    /// An optional error code associated with the error.
    pub error_code: Option<String>,
}

impl fmt::Display for Error {
    /// Formats the `Error` for display according to `DejaGnu` format
    /// See `DejaGnu` documentation [here](https://gcc.gnu.org/onlinedocs/gccint/testsuites/directives-used-within-dejagnu-tests/syntax-and-descriptions-of-test-directives.html)
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use RustcErrorKind::*;

        let error_code = self.error_code.as_ref().map_or("", |code| &code[..]);

        let error_type = match &self.kind {
            Some(Help) => "help",
            Some(Error) => "dg-error",
            Some(Note) => "dg-note",
            Some(Suggestion) => "suggestion",
            Some(Warning) => "dg-warning",
            None => "dg-error",
        };

        let error_code = if error_code.is_empty() {
            error_code.to_owned()
        } else {
            format!(".{}.", error_code)
        };

        let rel_line_number = if self.relative_line_num == 0 {
            "".to_owned()
        } else {
            format!(".{} ", self.relative_line_num)
        };

        write!(
            f,
            "// {{ {error_type} \"{error_code}\" \"\" {{ target *-*-* }} {rel_line_number}}}"
        )
    }
}

/// Represents the line in the rustc source code where an error occurred.
///
/// This enum is used to determine the relative position of the error line
/// in relation to the current line being processed.
///
/// Luckily, rust compile test only stores error messages on and after the line where the error occurred.
/// But `DejaGnu` can process error messages on the previous line, the current line, or the next line.
#[derive(PartialEq, Debug)]
enum WhichLine {
    /// The error is on the current line.
    ThisLine,

    /// The error follows the previous line.
    ///
    /// # Arguments
    ///
    /// * `usize` - The number of lines to follow.
    FollowPrevious(usize),

    /// The error is adjusted backward by a certain number of lines.
    ///
    /// # Arguments
    ///
    /// * `usize` - The number of lines to adjust backward.
    AdjustBackward(usize),
}

/// The main function for loading errors from source file and from optional stderr file.
///
/// # Arguments
///
/// * `text_file` - A string slice containing rustc error messages.
/// * `stderr_file` - An optional string slice containing error codes.
///
/// # Returns
///
/// * `Vec<Error>` - A vector of `Error` structs containing the parsed error information.
pub fn load_error(text_file: &str, stderr_file: Option<&str>) -> Vec<Error> {
    let mut last_unfollow_error = None;
    // For storing the errors
    let mut errors = Vec::new();

    for (line_num, line) in text_file.lines().enumerate() {
        if let Some((which, error)) = parse_expected(last_unfollow_error, line_num + 1, line) {
            match which {
                FollowPrevious(_) => {}
                _ => last_unfollow_error = Some(line_num),
            }
            errors.push(error);
        }
    }

    // If stderr file is not provided, return the errors
    if stderr_file.is_none() {
        return errors;
    }
    // TODO: improve this code incrementally
    // parsing error related information from `.stderr` file
    let error_code_stderr = parse_error_code(stderr_file.expect("stderr file is not found"));

    // TODO: We need to load error messages from `.stderr` instead of source file become sometimes source file contains incomplete error messages
    // finding the error code w.r.t line number and error message
    // TODO: sometimes, the error message might not be same but this doesn't matter as we are not comparing the row number for the message
    for error in errors.iter_mut() {
        for error_code in error_code_stderr.iter() {
            if error.line_num == error_code.line_number
                || error.msg == error_code.error_message_detail
            {
                error.error_code = Some(error_code.error_code.clone());
            }
        }
    }
    // return error detail with error code
    errors
}

/// Represents the result of parsing an error from the stderr file.
#[derive(Debug)]
struct StderrResult {
    /// The error code associated with the error.
    /// We only consider error codes that match the pattern `E\d{4}`.
    error_code: String,

    /// The complete error message.
    error_message_detail: String,

    /// The line number in the source code where the error occurred.
    line_number: usize,
}

/// Checks if the given string is a valid rustc error code.
///
/// # Arguments
///
/// * `s` - A string slice representing the error code to be checked.
///
/// # Returns
///
/// * `bool` - Returns `true` if the string matches the error code pattern, otherwise `false`.
fn is_error_code(s: &str) -> bool {
    regex!(r"^E\d{4}$").is_match(s)
}

/// Parses error codes from the stderr file.
///
/// # Arguments
///
/// * `stderr_content` - A string slice representing the content of the stderr file.
///
/// # Returns
///
/// * `Vec<StderrResult>` - A vector of `StderrResult` structs containing the parsed error information.
fn parse_error_code(stderr_content: &str) -> Vec<StderrResult> {
    // Modified regex pattern with named capture groups
    let error_pattern = regex!(
        r"error\[(?P<error_code>E\d{4})\]: (?P<error_message_detail>.+?)\n\s+-->.+:(?P<line_number>\d+):"
    );

    let mut results = Vec::new();

    for caps in error_pattern.captures_iter(stderr_content) {
        let error_code = caps.name("error_code").map_or_else(
            || "Error code not found".to_string(),
            |m| m.as_str().to_string(),
        );
        let error_message_detail = caps.name("error_message_detail").map_or_else(
            || "Error message detail not found".to_string(),
            |m| m.as_str().to_string(),
        );
        let line_number = caps.name("line_number").map_or_else(
            || "Line number not found".to_string(),
            |m| m.as_str().to_string(),
        );
        // We only consider error codes that match the pattern `E\d{4}`
        if !is_error_code(&error_code) {
            continue;
        }
        results.push(StderrResult {
            error_code,
            error_message_detail,
            line_number: line_number
                .parse::<usize>()
                .expect("expected line number to be a valid number"),
        });
    }

    results
}

/// Parses expected error comments from a source line.
///
/// # Arguments
///
/// * `last_nonfollow_error` - An optional `usize` representing the line number of the last non-follow error.
/// * `line_num` - A `usize` representing the current line number being processed.
/// * `line` - A string slice representing the content of the current line.
///
/// # Returns
///
/// * `Option<(WhichLine, Error)>` - Returns an `Option` containing a tuple with `WhichLine` and `Error` if a match is found, otherwise `None`.
fn parse_expected(
    last_nonfollow_error: Option<usize>,
    line_num: usize,
    line: &str,
) -> Option<(WhichLine, Error)> {
    // Matches comments like:
    //     //~
    //     //~|
    //     //~^
    //     //~^^^^^

    let captures = regex!(r"//(?:\[(?P<revs>[\w\-,]+)])?~(?P<adjust>\||\^*)").captures(line)?;

    let (follow, adjusts) = match &captures["adjust"] {
        "|" => (true, 0),
        circumflexes => (false, circumflexes.len()),
    };

    // Get the part of the comment after the sigil (e.g. `~^^` or ~|).
    let whole_match = captures
        .get(0)
        .expect("Failed to parse comments like \"//~\" \"//~^\" \"//~^^^^^\" ");
    let (_, mut msg) = line.split_at(whole_match.end());

    let first_word = msg
        .split_whitespace()
        .next()
        .expect("Encountered unexpected empty comment");

    // If we find `//~ ERROR foo` or something like that, skip the first word.
    let kind = first_word.parse::<RustcErrorKind>().ok();
    if kind.is_some() {
        msg = msg.trim_start().split_at(first_word.len()).1;
    }

    let msg = msg.trim().to_owned();

    // If we find `//~|` or `//~^`, we need to adjust the line number.
    let mut relative_line_num = line_num as i32;
    let (which, line_num) = if follow {
        assert_eq!(adjusts, 0, "use either //~| or //~^, not both.");
        let line_num = last_nonfollow_error.expect(
            "encountered //~| without \
             preceding //~^ line.",
        );
        relative_line_num = (line_num as i32) - relative_line_num;
        (FollowPrevious(line_num), line_num)
    } else {
        let which = if adjusts > 0 {
            AdjustBackward(adjusts)
        } else {
            ThisLine
        };
        let line_num = line_num - adjusts;
        relative_line_num = -(adjusts as i32);
        (which, line_num)
    };

    Some((
        which,
        Error {
            line_num,
            kind,
            msg,
            error_code: None,
            relative_line_num,
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test that `RustcErrorKind::from_str` correctly parses "help" and "help:" into `RustcErrorKind::Help`.
    #[test]
    fn from_str_help_returns_help() {
        assert_eq!(
            RustcErrorKind::from_str("help").unwrap(),
            RustcErrorKind::Help
        );
        assert_eq!(
            RustcErrorKind::from_str("help:").unwrap(),
            RustcErrorKind::Help
        );
    }

    /// Tests that `RustcErrorKind::from_str` correctly parses "error" into `RustcErrorKind::Error`.
    #[test]
    fn from_str_error_returns_error() {
        assert_eq!(
            RustcErrorKind::from_str("error").unwrap(),
            RustcErrorKind::Error
        );
    }

    /// Tests that `RustcErrorKind::from_str` correctly parses "note" into `RustcErrorKind::Note`.
    #[test]
    fn from_str_note_returns_note() {
        assert_eq!(
            RustcErrorKind::from_str("note").unwrap(),
            RustcErrorKind::Note
        );
    }

    /// Tests that `RustcErrorKind::from_str` correctly parses "suggestion" into `RustcErrorKind::Suggestion`.
    #[test]
    fn from_str_suggestion_returns_suggestion() {
        assert_eq!(
            RustcErrorKind::from_str("suggestion").unwrap(),
            RustcErrorKind::Suggestion
        );
    }

    /// Tests that `RustcErrorKind::from_str` correctly parses "warning" into `RustcErrorKind::Warning`.
    #[test]
    fn from_str_warning_returns_warning() {
        assert_eq!(
            RustcErrorKind::from_str("warning").unwrap(),
            RustcErrorKind::Warning
        );
    }

    /// Tests that `RustcErrorKind::from_str` correctly parses "warn" into `RustcErrorKind::Warning`.
    #[test]
    fn from_str_warn_returns_warning() {
        assert_eq!(
            RustcErrorKind::from_str("warn").unwrap(),
            RustcErrorKind::Warning
        );
    }

    /// Tests that `RustcErrorKind::from_str` returns an error for unrecognized strings.
    #[test]
    fn from_str_unrecognized_returns_err() {
        assert!(RustcErrorKind::from_str("unrecognized").is_err());
    }

    /// Tests that `RustcErrorKind::from_str` returns an error for an empty string.
    #[test]
    fn from_str_empty_string_returns_err() {
        // split always returns at least one element
        assert!(RustcErrorKind::from_str("").is_err());
    }

    /// Tests that `RustcErrorKind::Help` is formatted correctly as "help message" according to rust compiletest tool
    #[test]
    fn display_help_outputs_correct_string() {
        assert_eq!(format!("{}", RustcErrorKind::Help), "help message");
    }

    /// Tests that `RustcErrorKind::Error` is formatted correctly as "error" according to rust compiletest tool
    #[test]
    fn display_error_outputs_correct_string() {
        assert_eq!(format!("{}", RustcErrorKind::Error), "error");
    }

    /// Tests that `RustcErrorKind::Note` is formatted correctly as "note" according to rust compiletest tool
    #[test]
    fn display_note_outputs_correct_string() {
        assert_eq!(format!("{}", RustcErrorKind::Note), "note");
    }

    /// Tests that `RustcErrorKind::Suggestion` is formatted correctly as "suggestion" according to rust compiletest tool
    #[test]
    fn display_suggestion_outputs_correct_string() {
        assert_eq!(format!("{}", RustcErrorKind::Suggestion), "suggestion");
    }

    /// Tests that `RustcErrorKind::Warning` is formatted correctly as "warning" according to rust compiletest tool
    #[test]
    fn display_warning_outputs_correct_string() {
        assert_eq!(format!("{}", RustcErrorKind::Warning), "warning");
    }
}

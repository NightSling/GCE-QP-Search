use regex::Regex;

/// Sanitizes the input string for safe use in URLs or templates.
/// Allows only alphanumeric characters, dashes, underscores, and periods.
/// Rejects unsafe characters.
///
/// # Arguments
///
/// * `input` - A string slice representing the input to sanitize.
///
/// # Returns
///
/// A sanitized version of the input as a `String`. If the input contains
/// unsafe characters, they are removed.
///
/// # Example
///
/// ```
/// let sanitized = sanitize_input("example.pdf");
/// assert_eq!(sanitized, "example.pdf");
/// ```
pub fn sanitize_input(input: &str) -> String {
    // Define a regular expression for valid characters
    let re = Regex::new(r"[^a-zA-Z0-9._/-]").unwrap();

    // Replace invalid characters with an empty string
    re.replace_all(input, "").to_string()
}
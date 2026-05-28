#[doc = include_str!("../mdocs/index.md")]

/// Greets the user by name.
///
/// # Examples
///
/// ```
/// use @project@::greet;
///
/// assert_eq!(greet("World"), "Hello, World!");
/// ```
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

// DOCUMENTATION TESTS
// Documentation comments are written in Markdown.
// "//!" denotes module-level documentation
// "///" denotes function-level documentation
//! The 'adder' crate provides functions that
//!
//! # Examples
//!
//! ```
//! assert_eq!(4, adder::add_two(2));
//! ```


/// This function adds two to its argument.
///
/// # Examples
///
/// ```
/// use adder::add_two;
///
/// assert_eq!(4, add_two(2));
/// ```
pub fn add_two(a: i32) -> i32 {
    a + 2
}

// Grouping tests. Use cfg attribute to only compile test code if
// we're trying to run tests. Saves on compile time, and ensures
// tests are left out of a normal build.
// This is good for unit tests.
#[cfg(test)]
mod tests {
    // Bring our test function and other things into scope.
    use super::*;

    #[test]
    fn add_two_test_normal() {
        assert_eq!(4, add_two(2));
    }

    // Negative Test, use expected to filter on panic message.
    #[test]
    #[should_panic(expected = "assertion failed")]
    fn add_two_test_negative() {
        assert_eq!(4, add_two(3));
    }

    // Use #[ignore] to ignore a test.
    // Run "cargo test -- --ignore" to run ignored tests separately.
    #[test]
    #[ignore]
    fn test_test_please_ignore() {
        while { true } {
            assert_eq!(4, add_two(2));
        }
    }
}


// CONDITIONAL COMPILATION
// Only compile this if build was run with: 'cargo build --features "foo"'
#[cfg(feature = "foo")]
mod foo {
}

// If a is set by cfg attribute, produces #[b]
#[cfg_attr(a, b)]
mod bar{
}

/// Prints "Think Different!" if running this on an Apple OS.
/// Prints "Think Same!" otherwise.
/// Detect compile flags at runtime with cfg! macro.
fn is_apple_os() -> bool {
    if cfg!(target_os = "macos") || cfg!(target_os = "ios") {
        println!("Think Different!");
        return true;
    } else {
        println!("Think Same!");
        return false;
    }
}

#[test]
#[cfg(unix)]
fn test_is_apple_os() {
    assert!(!is_apple_os());
}


// DOCUMENTATION
// Example. Run "cargo doc" to generate documentation.

use std::rc::Rc;

// First line of documentation should be a short summary of its functionality.
/// Constructs a new `Rc<T>`.
// If more explanation is needed, make it in a new paragraph.
// Section Headers (conventional): Panics, Failures, Safety, Examples
///
/// # Panics
/// Panics caused by misuses of the function
/// # Failures
/// If function returns a Result<T, E> then describe under which conditions this occurs.
/// # Safety
/// Explain which invariants of the caller is responsible for upholding
/// # Examples
/// Use triple graves to compile and run on for testing.
/// (Can annotate inital set of graves with language if not using rust)
/// ```
/// use std::rc::Rc;
/// # // Code followed by a hash will be run but not shown in the documentation.
/// # println!("hi");
/// let five = Rc::new(5);
/// ```
pub fn new<T>(value: T) -> Rc<T> {
    // implementation goes here
    return Rc::new(value);
}

// Don't put documentation comments in line with code.
/// The `Option` type. See [the module level documentation](../) for more.
enum Option<T> {
    /// No value
    None,
    /// Some value `T`
    Some(T),
}

// Note, testing binary crates requires specifying the file name with a "test" flag.
// Example:
// rustdoc --test path/to/my/crate/root.rs
// code annotations:
// ```ignore
// ignore code within this block
// ```
// ```should_panic
// // Code within this block should compile but not actually pass as a test.
// assert!(false);
// ```
// ```no_run
// // Compiles code within this block, but does not run.
// ```

// Controlling HTML
#![doc(html_logo_url = "https://www.rust-lang.org/logos/rust-logo-128x128-blk-v2.png",
       html_favicon_url = "https://www.rust-lang.org/favicon.ico",
       html_root_url = "https://doc.rust-lang.org/")]
mod foo {
    //! This is documentation for the `foo` module.
    //!
    //! This blurb here explains the general purpose of the `foo` module.
}

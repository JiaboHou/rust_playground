// DOCUMENTATION TESTS
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

// Detect compile flags at runtime with cfg! macro.
fn is_apple_os() {
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
    // My OS is linux.
    assert!(!is_apple_os());
}

# Notes

# General Principles of Rust
    - 3 Goals: safety, speed, concurrency
    - Make things explicity
    - "ownership"

# Macros
    - Invoked with "!"
    - "can do significantly more complicated things than function calls"
    - "!" also helps with parsing, making tooling easier to write

# Mutation
    - Variable bindings are immutable by default
    - "mut" makes bindings mutable. (e.g. "let mut x")

# Typing
    - Static type, with type inference. No need to annotate the type.

# Memory Allocation
    - Preference: Stack allocation > Heap allocation.
    - Declare a variable binding and it is placed on the stack.
    - However, types like Vec<T> allocates space for the elements of the vector on the heap.

# Ownership
    - variable bindings "own" their value.
    - When a variable binding goes out of scope, the value it owned is de-allocated from memory.
    - This is deterministic, not through GC. (no need for malloc or free)

# References
    - immutable by default.

# Error Handling
    - Rust handles throwing exceptions by returning them, in a Result container. This Result type encodes error handling information.

# Arc<T>
    - atomic reference count: Automatically keeps track of the number of references of the binding across multiple threads.

# Mutex
    - Used to control concurrency. Only one thread can access the contents of a Mutex at any given time.

# Foreign Function Interface (FFI)
    - Often times a programming language is weak in runtime performance, with the trade off of providing more convenience and productivity. To help mitigate the slower runtime, we can write some of a system in C and have a higher-level language call that C code.
    - Rust has bi-directional support for FFI. It can call into C code easily, and more importantly, have C code call into Rust.

# Concurrency
    - Dealing with concurrency is where Rust shines. Many languages, place numbers on the heap, rather than on the stack (esp. OOP and languages with GC. Granted some optimizations can stack allocate).
    - Some languages may have a "global interpreter lock" (GIL) which limits concurrency in many situations, for safety's sake.

# Cargo.toml
    - To compile a library into a standard dynamic library, do:
        '[lib]
        name = "embed"
        crate-type = ["dylib"]'
    - Rust creates rlib's by default.

# Return code
"$ echo $?" prints the status code. (Useful for integrating "cargo test" into other tooling.)

# Testing
## Unit Tests
Grouping tests: mod tests {}
Asserts: assert!(exp), assert_eq!(x, y);
Test: #[test]
Negative test: #[should_panic(expected = "msg to expect upon panic")]
Ignore a test: #[ignore]
## Integration Tests
Located in a separate "tests" dir, need to import the crate you are testing.
## Documentation Tests
When you use examples in your documentation, make sure they actually work when you update your code.
Use //! for module-level documentation, /// for function-level documentation

# Conditional Compilation
Rust has an attribute "#[cfg]" which allows for conditional compilation.
Examples:
    - #[cfg(foo)]
    - #[cfg(bar = "baz")]
    - #[cfg(any(not(unix), all(target_os="macos", target_arch="powerpc")))]

These switches can be enabled/disabled in the features section of the Cargo.toml. When you specify a feature, it is passed in as a flag to rustc:
```
--cfg feature="${feature_name}"
```

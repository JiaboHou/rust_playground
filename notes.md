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

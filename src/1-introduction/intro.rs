fn ex1() {
    // Mutable variable binding x.
    // x is stored on stack, Vec<T>'s elements are on the heap.
    // x "owns" the vector. When x falls out of scope, the vector
    // is de-allocated.
    let mut x = vec!["Hello", "world"];

    // y is a reference to x.
    // A safer version of a pointer in C/C++.
    // y is "borrowing" from x. When y goes out of scope,
    // the vector owned by x is NOT de-allocated.
    let y = &x[0];

    // Append element to x. This will cause a compiler error.
    // Mutating something while another reference exists (in this case, y)
    // is dangerous, because we may invalidate the reference.
    // (e.g. could create a dangling pointer)
    // x.push("foo"); // COMPILER ERROR

    // How do we solve this?
}

// APPROACH 1: Copy it, instead of referencing it.
fn ex1_1() {
    let mut x = vec!["Hello", "world"];
    let y = &x[0];

    // y is a copy of x's first element.
    let y = x[0].clone();

    x.push("foo"); // COMPILER ERROR

}

// APPROACH 2: Scoping.
fn ex1_2() {
    let mut x = vec!["Hello", "world"];
    let y = &x[0];

    {
        let y = &x[0];
    }
    // y is now out of scope.
    x.push("foo"); // COMPILER ERROR
}

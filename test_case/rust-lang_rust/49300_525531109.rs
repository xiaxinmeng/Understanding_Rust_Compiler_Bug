rust
mod m {
    fn foo() { // def-path: m::foo
        // Here's how the closures' `NodeId`s get a stable representation.
        let closure1 = || {}; // m::foo(1)
        let closure2 = || {}; // m::foo(2)

        // Same idea, but for macro invocations and their `ExpnId`s instead of `NodeId`s
        mac1!(); // m::foo!(1)
        mac2!(); // m::foo!(2)
    }
}

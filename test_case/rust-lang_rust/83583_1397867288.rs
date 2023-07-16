rust
mod outer {
    struct O;
    pub fn foo() {
        // This type is local to this fn implementation. Furthermore,
        // it might have been injected *into* the body of the fn implementation
        // by a macro exapnsion. (For example, that is part of what happens
        // with doc tests that include struct definitions.) So, we cannot
        // try to force the struct definition to live outside of here.
        //
        // and remember, there may be a derive on this thing too.
        #[derive()]
        struct FooS;
        
        // This code is macro-generated, e.g. from a derive on FooS.
        mod inner {
            use super::*; // this use enables us to see `O`, but not `FooS` (!)
            
            fn om(x: O) -> O { x }

            // so how can one reference the type `FooS` from here?
            #[cfg(this_is_the_code_that_breaks_with_a_resolve_error)]
            fn g(f: FooS) -> FooS { f }
        }
    }
}

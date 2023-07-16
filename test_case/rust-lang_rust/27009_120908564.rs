 rust
<anon>:23:41: 23:48 error: no method named `clone` found for type `Test` in the current scope
<anon>:23     let test = (*tests.last().unwrap()).clone();
                                                  ^~~~~~~
<anon>:23:41: 23:48 help: items from traits can only be used if the trait is implemented and in scope; the following trait defines an item `clone`, perhaps you need to implement it:
<anon>:23:41: 23:48 help: candidate #1: `core::clone::Clone`

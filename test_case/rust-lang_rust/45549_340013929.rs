
$ rg just -g diagnostics.rs
src/librustc/diagnostics.rs
214:We don't just need to create a table of all implementations of all methods of
1423:[RFC 1156]: https://github.com/rust-lang/rfcs/blob/master/text/1156-adjust-default-object-bounds.md
1651:These attributes do not work on typedefs, since typedefs are just aliases.
1738:you just have to implement `Copy` and `Clone` traits on `struct X` and it'll
1853:The intention is typically to describe a function pointer, but just `fn()`
1855:(Since these values are typically just passed to C code, however, this rarely
1878:No `main` function was found in a binary crate. To fix this error, just add a
1901:Maybe you just misspelled the lint name or the lint doesn't exist anymore.

src/librustc_mir/diagnostics.rs
794:case, `StaticMutex` would work just fine, but it isn't stable yet:
1675:created. So to fix the previous example, just make the `y` lifetime greater than
1708:resolve the previous example by removing the borrow and just storing

src/librustc_resolve/diagnostics.rs
459:Items inside functions are basically just like top-level items, except
727:To fix this error, just use the same mode in both cases.
1533:To fix this error, just change the binding's name in order to avoid shadowing

src/librustc_typeck/diagnostics.rs
590:It can be fixed by adjusting the trait bound like this:
781:Since `return;` is just like `return ();`, there is a mismatch between the
3287:default parameter, this parameter can just be substituted in. However, if the
3904:To fix this issue, just remove the return keyword or move the expression into a
4494:To fix this error, just specify the type of the variable. Example:
4546:error, just declare a function.
4664://  E0319, // trait impls for defaulted traits allowed just for structs/enums

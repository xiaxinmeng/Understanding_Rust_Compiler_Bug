
fn f2() {
    let x = 0;
    fn bar() {
        x += 1; // error: unresolved name `x`
                // note: hey, maybe you wanted a closure and not a function
                // ^ the note shouldn't even be precise and may be reported on best-effort basis
    }
}

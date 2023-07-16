 rust
struct Foo { x: int }

extern {
    // This one should fail because you've supplied a literal as a fn arg with
    // no type
    fn foo(1: ());
    // This one should fail because you've specified '()' as a variable name for
    // the fn args
    fn bar((): int);
    // This one should pass because you've declared a deconstructed variable name
    // for the input parm 'x' and you've declared a type. This still should trigger
    // the warning for not using a 'libc::c_int'.
    fn baz(Foo {x}: int);
    // This should pass because you've specified input parameters and rust will 
    // infer the type?
    fn qux((x,y): ());
}

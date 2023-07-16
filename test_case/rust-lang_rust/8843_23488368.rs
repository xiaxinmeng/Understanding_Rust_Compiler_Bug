 rust
pub fn foo() {
    static x: int = 42;
    {
        static x; int = 42;
        printfln!(x);
    }
    printfln!(x);
}

rust
fn main() {
    let x = { struct S; }; // WARN
    let _ = { struct S; }; // WARN
    const CX: () = { struct S; }; // NO WARN
    const _: () = { struct S; }; // NO WARN
    static SX: () = { struct S; }; // NO WARN
    static _SX: () = { struct S; }; // NO WARN
}

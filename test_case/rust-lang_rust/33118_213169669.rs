 rust
fn foo() {}
fn f1() {
    macro_rules_v2! bar { () => { foo() } }
    let foo = 0;
    bar!(); // `foo` in this expansion would resolve to the item
}
fn f2() {
    let foo = 0;
    macro_rules_v2! bar { () => { foo() } }
    bar!(); // Should `foo` in this expansion resolve to the item or the local?
}

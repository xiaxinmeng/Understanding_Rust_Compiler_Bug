Rust
fn if_let_chain_let(v: u32) {
    macro_rules! if_let_chain {
        ($arg:pat, $exp:expr) => {
            if true && let $arg = $exp && ({
                print!("postfix,");
                true
            },).0 {
                print!("body,");
            } else {
                print!("else,");
            }
        };
    }
    nestings!(if_let_chain, "if let chain (let)", v);
    print!("\n  ref &:        ");
    if_let_chain!(0, &Droppy(v).0);

    print!("\n  ref mut &mut: ");
    if_let_chain!(0, &mut Droppy(v).0);

    println!("");
}

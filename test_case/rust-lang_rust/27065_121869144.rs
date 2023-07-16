 rust
fn main() {
    let _: i32 =
        'a: //~ ERROR mismatched types
        loop { break };
    let _: i32 =
        'b: //~ ERROR mismatched types
        while true { break };
    let _: i32 =
        'c: //~ ERROR mismatched types
        for _ in None { break };
}

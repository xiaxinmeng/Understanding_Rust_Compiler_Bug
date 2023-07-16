
~/workspace/rust (master) » cat ~/qux.rs 
fn foo<'a>(_: &'a int) -> &Option<int> {
    let v: &Option<int> = &None;
    v
}

fn main() {
    let a = 4;
    let b = foo(&a);
    let c = [1, ..1000]; // fill some stack space
    printfln!(b);
}

~/workspace/rust (master) » rust run ~/qux.rs 
/home/ermolovd/qux.rs:9:8: 9:9 warning: unused variable: `c` [-W unused-variable (default)]
/home/ermolovd/qux.rs:9     let c = [1, ..1000]; // fill some stack space
                                ^
rust: task failed at 'enum value matched no variant', /home/ermolovd/workspace/rust/src/libstd/repr.rs:521
rust: domain main @0x232a110 root task failed

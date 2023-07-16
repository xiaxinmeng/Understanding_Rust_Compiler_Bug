rust
fn bar(x: &mut i32) { println!("x = {}", x);}
fn foo(a: &mut i32) {
    let ref y = a; // a is borrowed as immutable.
    bar(a); // error: cannot borrow `*a` as mutable because `a` is also borrowed
            //        as immutable
    println!("y = {}", y); // only add this statement, compiler will make a compilation.
}

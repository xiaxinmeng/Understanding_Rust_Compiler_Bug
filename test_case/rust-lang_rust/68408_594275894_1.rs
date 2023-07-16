
   Compiling playground v0.0.1 (/playground)
warning: variant is never constructed: `A`
 --> src/main.rs:5:5
  |
5 |     A { x: () },
  |     ^^^^^^^^^^^
  |
  = note: `#[warn(dead_code)]` on by default

    Finished dev [unoptimized + debuginfo] target(s) in 0.43s
     Running `target/debug/playground`
[src/main.rs:41] E::mk_e() = A {
    x: (),
}
[src/main.rs:42] F::mk_f() = B {
    x: (),
}
[src/main.rs:43] G::mk_g() = C(
    (),
)

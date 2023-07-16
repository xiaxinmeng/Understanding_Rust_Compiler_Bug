none
┌ 0 ─ bob_twinkles@cheesecake ─ ~/Code/rust/rust-issue-47055
└╼ 1 rust-issue-47055 >> rustup toolchain link issue-47055 ./build/x86_64-unknown-linux-gnu/stage2
┌ 0 ─ bob_twinkles@cheesecake ─ ~/Code/rust/rust-issue-47055
└╼ rust-issue-47055 >> rustup override set issue-47055
info: override toolchain for '/home/bob_twinkles/Code/rust/rust-issue-47055' set to 'issue-47055'
┌ 0 ─ bob_twinkles@cheesecake ─ ~/Code/rust/rust-issue-47055
└╼ rust-issue-47055 >> cat << EOF > test-code.rs
> #![crate_type = "lib"]
> #![feature(conservative_impl_trait, generators, generator_trait, nll)]
> use std::ops::Generator;
> 
> pub fn render<'a>(input: &'a str) -> impl Generator<Yield = (), Return = ()> + 'a {
>     move || {
>         let _t = input;
>         yield ();
>     }
> }
> EOF
┌ 0 ─ bob_twinkles@cheesecake ─ ~/Code/rust/rust-issue-47055
└╼ rust-issue-47055 >> rustc ./test-code.rs 
┌ 0 ─ bob_twinkles@cheesecake ─ ~/Code/rust/rust-issue-47055
└╼ rust-issue-47055 >> ls
build  libtest_code.rlib  test-code.rs
┌ 0 ─ bob_twinkles@cheesecake ─ ~/Code/rust/rust-issue-47055
└╼ rust-issue-47055 >> file libtest_code.rlib 
libtest_code.rlib: current ar archive

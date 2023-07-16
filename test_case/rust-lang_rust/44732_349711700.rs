
luser@eye7:/tmp$ rustc +nightly --version
rustc 1.24.0-nightly (560a5da9f 2017-11-27)
luser@eye7:/tmp$ cat > test1.rs <<EOF
> #[allow(unused)]
> const s: &str = include_str!("other-file");
> EOF
luser@eye7:/tmp$ cat > other-file <<EOF
> hello!
> EOF
luser@eye7:/tmp$ rustc +nightly test1.rs --emit=dep-info -o test1.d
luser@eye7:/tmp$ cat test1.d
test1.d: test1.rs other-file

test1.rs:
other-file:
luser@eye7:/tmp$ cat > test2.rs <<EOF
> #![feature(external_doc)]
> #![doc(include="other-file")]
> EOF
luser@eye7:/tmp$ rustc +nightly test2.rs --emit=dep-info -o test2.d
cluser@eye7:/tmp$ cat test2.d
test2.d: test2.rs

test2.rs:


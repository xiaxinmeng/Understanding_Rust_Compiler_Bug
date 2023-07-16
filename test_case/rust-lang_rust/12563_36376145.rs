
$ echo 'fn main() { ' 'let _x: int; /*'{1..5000}'*/' '}' | rustc - -Z time-passes
...
time: 6.745 s   expansion
$ echo 'fn main() { ' 'let _x: int; /*'{1..5000}'*/' '}' | /patched/rustc - -Z time-passes
...
time: 3.723 s   expansion

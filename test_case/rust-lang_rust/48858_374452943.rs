
zmd@ReflectiveCoherence:~/Code/rust$ cat correct.rs 
fn main() {
    let not = 2;
    let _really = not;
}
zmd@ReflectiveCoherence:~/Code/rust$ rustc correct.rs # sane rustc
zmd@ReflectiveCoherence:~/Code/rust$ echo $?
0
zmd@ReflectiveCoherence:~/Code/rust$ rustc +stage1 correct.rs # my delusional work-in-progress build
error: expected one of `!`, `.`, `::`, `;`, `?`, `{`, or an operator, found `}`
 --> correct.rs:4:1
  |
3 |     let _really = not;
  |                      - expected one of 7 possible tokens here
4 | }
  | ^ unexpected token

error: aborting due to previous error

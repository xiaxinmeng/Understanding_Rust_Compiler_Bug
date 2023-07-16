
% cat /tmp/g.rs
fn main() {
    let mut x = 1;
    let y = &mut x;
    fn bar(b: &mut int, c: &mut int) {*b += 0; *c += 0; }
    bar(y, y);
}
% rustc /tmp/g.rs
/tmp/g.rs:5:11: 5:12 error: cannot borrow `*y` as mutable more than once at a time
/tmp/g.rs:5     bar(y, y);
                       ^
/tmp/g.rs:5:8: 5:9 note: previous borrow of `*y` as mutable occurs here
/tmp/g.rs:5     bar(y, y);
                    ^
error: aborting due to previous error
task 'rustc' failed at 'explicit failure', /Users/fklock/Dev/Mozilla/rust.git/src/libsyntax/diagnostic.rs:102
task '<main>' failed at 'explicit failure', /Users/fklock/Dev/Mozilla/rust.git/src/librustc/lib.rs:443

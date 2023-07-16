\n\nWith this approach, x and y share ownership of the data via the `Rc` (reference\ncount type). `RefCell` essentially performs runtime borrow checking: ensuring\nthat at most one writer or multiple readers can access the data at any one time.\n\nIf you wish to learn more about ownership in Rust, start with the chapter in the\nBook:\n\nhttps://doc.rust-lang.org/book/first-edition/ownership.html\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/borrowck/borrowck-issue-48962.rs","byte_start":660,"byte_end":663,"line_start":25,"line_end":25,"column_start":6,"column_end":9,"is_primary":false,"text":[{"text":"    {src};","highlight_start":6,"highlight_end":9}],"label":"value moved here","suggested_replacement":null,"sugting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:47:58] {"message":"For more information about this error, try `rustc --explain E0382`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0382`.\n"}
[00:47:58] ------------------------------------------
[00:47:58] 
[00:47:58] thread '[ui] ui/borrowck/borrowck-move-moved-value-into-closure.rs#mir' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3252:9
[00:47:58] 
[00:47:58] 
[00:47:58] ---- [ui] ui/borrowck/borrowck-move-out-from-array.rs#mir stdout ----
[00:47:58] diff of stderr:
[00:47:58] 
[00:47:58] 4 LL |     let [_, _x] = a;
[00:47:58] 5    |             -- value moved here
[00:47:58] 6 LL |     let [.., _y] = a; //[ast]~ ERROR [E0382]
[00:47:58] -    |              ^^ value used here after move
[00:47:58] +    |              ^^ value used here after partial move
[00:47:58] 8 
[00:47:58] 9 error[E0382]: use of moved value: `a[..]`
[00:47:58] 
[00:47:58] 
[00:47:58] 12 LL |     let [_x, _] = a;
[00:47:58] 13    |          -- value moved here
[00:47:58] 14 LL |     let [_y..] = a; //[ast]~ ERROR [E0382]
[00:47:58] -    |          ^^ value used here after move
[00:47:58] +    |          ^^ value used here after partial move
[00:47:58] 17 error: aborting due to 2 previous errors
[00:47:58] 18 
[00:47:58] 
[00:47:58] 
[00:47:58] 
[00:47:58] The actual stderr differed from the expected stderr.
[00:47:58] Actual stderr the example below, we implement a `Point` type. Because it only stores two\nintegers, we opt-out of ownership semantics with `Copy`. Then we can\n`let p2 = p1` without `p1` being moved.\n\n
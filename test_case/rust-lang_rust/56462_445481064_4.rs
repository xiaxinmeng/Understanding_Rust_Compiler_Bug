\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/resolve/issue-23305.rs","byte_start":530,"byte_end":534,"line_start":15,"line_end":15,"column_start":12,"column_end":16,"is_primary":true,"text":[{"text":"impl ToNbt<Self> {}","highlight_start":12,"highlight_end":16}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"...which again requires finding type of <impl at /checkout/src/test/ui/resolve/issue-23305.rs:15:1: 15:20>, completing the cycle","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0391]: cycle detected when finding type of <impl at /checkout/src/test/ui/resolve/issue-23305.rs:15:1: 15:20>\n  --> /checkout/src/test/ui/resolve/issue-23305.rs:15:12\n   |\nLL | impl ToNbt<Self> {}\n   |            ^^^^\n   |\n   = note: ...which again requires finding type of <impl at /checkout/src/test/ui/resolve/issue-23305.rs:15:1: 15:20>, completing the cycle\n\n"}
[00:47:57] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:47:57] {"message":"For more information about this error, try `rustc --e -    = note: ...which again requires processing `<impl at $DIR/resolve-self-in-impl.rs:25:1: 25:23>`, completing the cycle
[00:47:57] +    = note: ...which again requires finding type of <impl at $DIR/resolve-self-in-impl.rs:25:1: 25:23>, completing the cycle
[00:47:57] 16 
[00:47:57] - error[E0391]: cycle detected when processing `<impl at $DIR/resolve-self-in-impl.rs:26:1: 26:13>`
[00:47:57] + error[E0391]: cycle detected when finding type of <impl at $DIR/resolve-self-in-impl.rs:26:1: 26:13>
[00:47:57] 19    |
[00:47:57] 19    |
[00:47:57] 20 LL | impl Self {} //~ ERROR cycle detected
[00:47:57] 21    |      ^^^^
[00:47:57] 22    |
[00:47:57] 22    |
[00:47:57] -    = note: ...which again requires processing `<impl at $DIR/resolve-self-in-impl.rs:26:1: 26:13>`, completing the cycle
[00:47:57] +    = note: ...which again requires finding type of <impl at $DIR/resolve-self-in-impl.rs:26:1: 26:13>, completing the cycle
[00:47:57] 24 
[00:47:57] - error[E0391]: cycle detected when processing `<impl at $DIR/resolve-self-in-impl.rs:27:1: 27:16>`
[00:47:57] + error[E0391]: cycle detected when finding type of <impl at $DIR/resolve-self-in-impl.rs:27:1: 27:16>
[00:47:57] 27    |
[00:47:57] 27    |
[00:47:57] 28 LL | impl S<Self> {} //~ ERROR cycle detected
[00:47:57] 29    |        ^^^^
[00:47:57] 30    |
[00:47:57] 30    |
[00:47:57] -    = note: ...which again requires processing `<impl at $DIR/resolve-self-in-impl.rs:27:1: 27:16>`, completing the cycle
[00:47:57] +    = note: ...which again requires finding type of <impl at $DIR/resolve-self-in-impl. on each other\nand therefore cannot be constructed.\n\nThe following example contains a circular dependency between two traits:\n\n
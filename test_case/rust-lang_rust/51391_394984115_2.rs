
warning: [BarA] cannot be resolved, ignoring it...
 --> 1.rs:3:1
  |
3 | / /// Foo
4 | | /// bar [BarA] bar
5 | | /// baz
  | |_______^
  |
  = note: the link appears in this line:
          
           bar [BarA] bar
                ^^^^

warning: [BarB] cannot be resolved, ignoring it...
  --> 1.rs:8:1
   |
8  | / /**
9  | |  * Foo
10 | |  * bar [BarB] bar
11 | |  * baz
12 | |  */
   | |___^
   |
   = note: the link appears in this line:
           
            bar [BarB] bar
                 ^^^^

warning: [BarC] cannot be resolved, ignoring it...
  --> 1.rs:15:1
   |
15 | / /** Foo
16 | |
17 | | bar [BarC] bar
18 | | baz
...  |
24 | |
25 | | */
   | |__^
   |
   = note: the link appears in this line:
           
           bar [BarC] bar
                ^^^^

warning: [BarD] cannot be resolved, ignoring it...
  --> 1.rs:28:1
   |
28 | #[doc = "Foo\nbar [BarD] bar\nbaz"]
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: the link appears in this line:
           
           bar [BarD] bar
                ^^^^

warning: [BarE] cannot be resolved, ignoring it...
  --> 1.rs:31:1
   |
31 | #[doc(include = "file.md")]
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: the link appears in this line:
           
           bar [BarE] bar
                ^^^^

warning: [BarF] cannot be resolved, ignoring it...
  --> 1.rs:36:9
   |
36 |         #[doc = $f]
   |         ^^^^^^^^^^^
...
40 | f!("Foo\nbar [BarF] bar\nbaz");
   | ------------------------------- in this macro invocation
   |
   = note: the link appears in this line:
           
           bar [BarF] bar
                ^^^^

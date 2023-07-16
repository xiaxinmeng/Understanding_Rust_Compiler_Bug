plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between bba9785dd73f61aacd301a2cb379e1e85a129047 and 24e3a22bcb98798347c3e3a8440aac90738c69e4
src/ci/scripts/should-skip-this.sh: line 23: library/std/src/sys: Is a directory
Tool subtrees were updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---

---- compile_test stdout ----
diff of stderr:

-error: use Option::map_or instead of an if let/else
+error[E0716]: temporary value dropped while borrowed
+  --> $DIR/option_if_let_else.rs:36:14
    |
    |
-LL | /     if let Some(x) = string {
-LL | |         (true, x)
-LL | |     } else {
-LL | |         (false, "hello")
-LL | |     }
-   | |_____^ help: try: `string.map_or((false, "hello"), |x| (true, x))`
+LL |     let _ = if let Some(s) = &mut num {
+...
+LL |         &mut 0
+   |              ^ creates a temporary which is freed while still in use
+LL |     };
+LL |     };
+   |     - temporary value is freed at the end of this statement
    |
-   = note: `-D clippy::option-if-let-else` implied by `-D warnings`
 
 
-error: use Option::map_or instead of an if let/else
+error[E0716]: temporary value dropped while borrowed
+  --> $DIR/option_if_let_else.rs:49:14
    |
    |
-LL |     let _ = if let Some(s) = *string { s.len() } else { 0 };
-   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `string.map_or(0, |s| s.len())`
-
-error: use Option::map_or instead of an if let/else
-  --> $DIR/option_if_let_else.rs:31:13
+LL |     let _ = if let Some(ref mut s) = num {
+...
+LL |         &mut 0
+   |              ^ creates a temporary which is freed while still in use
+LL |     };
+LL |     };
+   |     - temporary value is freed at the end of this statement
    |
-LL |     let _ = if let Some(s) = &num { s } else { &0 };
-   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `num.as_ref().map_or(&0, |s| s)`
 
 
-error: use Option::map_or instead of an if let/else
-   |
-   |
-LL |       let _ = if let Some(s) = &mut num {
-   |  _____________^
-LL | |         *s += 1;
-LL | |         s
-LL | |     } else {
-LL | |         &mut 0
-LL | |     };
-   |
-help: try
-   |
-   |
-LL ~     let _ = num.as_mut().map_or(&mut 0, |s| {
-LL +         *s += 1;
-LL +         s
-LL ~     });
+error: aborting due to 2 previous errors
 
 
-error: use Option::map_or instead of an if let/else
-   |
-   |
-LL |     let _ = if let Some(ref s) = num { s } else { &0 };
-   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `num.as_ref().map_or(&0, |s| s)`
-
-error: use Option::map_or instead of an if let/else
-   |
-   |
-LL |       let _ = if let Some(mut s) = num {
-LL | |         s += 1;
-LL | |         s
-LL | |     } else {
-LL | |         0
-LL | |         0
-LL | |     };
-   | |_____^
-   |
-help: try
-   |
-LL ~     let _ = num.map_or(0, |mut s| {
-LL +         s += 1;
-LL +         s
-LL ~     });
-
-
-error: use Option::map_or instead of an if let/else
-   |
-   |
-LL |       let _ = if let Some(ref mut s) = num {
-   |  _____________^
-LL | |         *s += 1;
-LL | |         s
-LL | |     } else {
-LL | |         &mut 0
-LL | |     };
-   |
-help: try
-   |
-   |
-LL ~     let _ = num.as_mut().map_or(&mut 0, |s| {
-LL +         *s += 1;
-LL +         s
-LL ~     });
-
-
-error: use Option::map_or instead of an if let/else
-   |
-   |
-LL | /     if let Some(x) = arg {
-LL | |         let y = x * x;
-LL | |         y * y
-LL | |     } else {
-LL | |         13
-LL | |     }
-   |
-help: try
-   |
-   |
-LL ~     arg.map_or(13, |x| {
-LL +         let y = x * x;
-LL +         y * y
-LL +     })
-
-
-error: use Option::map_or_else instead of an if let/else
-   |
-   |
-LL |       let _ = if let Some(x) = arg {
-LL | |         x
-LL | |     } else {
-LL | |         // map_or_else must be suggested
-LL | |         side_effect()
-LL | |         side_effect()
-LL | |     };
-   | |_____^ help: try: `arg.map_or_else(|| side_effect(), |x| x)`
-
-error: use Option::map_or_else instead of an if let/else
-   |
-   |
-LL |       let _ = if let Some(x) = arg {
-LL | |         x * x * x * x
-LL | |     } else {
-LL | |         let mut y = 1;
-...  |
-...  |
-LL | |         y
-LL | |     };
-   | |_____^
-   |
-help: try
-   |
-LL ~     let _ = arg.map_or_else(|| {
-LL +         let mut y = 1;
-LL +         y = (y + 2 / y) / 2;
-LL +         y = (y + 2 / y) / 2;
-LL +         y
-LL ~     }, |x| x * x * x * x);
-
-
-error: use Option::map_or_else instead of an if let/else
-   |
-   |
-LL | /             if let Some(idx) = s.find('.') {
-LL | |                 vec![s[..idx].to_string(), s[idx..].to_string()]
-LL | |             } else {
-LL | |                 vec![s.to_string()]
-LL | |             }
-   | |_____________^ help: try: `s.find('.').map_or_else(|| vec![s.to_string()], |idx| vec![s[..idx].to_string(), s[idx..].to_string()])`
-
-error: use Option::map_or instead of an if let/else
-   |
-   |
-LL |     let _ = if let Some(x) = optional { x + 2 } else { 5 };
-   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `optional.map_or(5, |x| x + 2)`
-
-error: use Option::map_or instead of an if let/else
-   |
-   |
-LL |       let _ = if let Some(x) = Some(0) {
-LL | |         loop {
-LL | |         loop {
-LL | |             if x == 0 {
-LL | |                 break x;
-LL | |         0
-LL | |     };
-   | |_____^
-   |
-   |
-help: try
-   |
-LL ~     let _ = Some(0).map_or(0, |x| loop {
-LL +             if x == 0 {
-LL +                 break x;
-LL +             }
-LL ~         });
-
-
-error: use Option::map_or instead of an if let/else
-   |
-   |
-LL |     let _ = if let Some(x) = Some(0) { s.len() + x } else { s.len() };
-   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `Some(0).map_or(s.len(), |x| s.len() + x)`
-
-error: use Option::map_or instead of an if let/else
-   |
-   |
-LL |       let _ = if let Some(x) = Some(0) {
-LL | |         let s = s;
-LL | |         let s = s;
-LL | |         s.len() + x
-LL | |     } else {
-LL | |         1
-LL | |     };
-   |
-help: try
-   |
-   |
-LL ~     let _ = Some(0).map_or(1, |x| {
-LL +         let s = s;
-LL +         s.len() + x
-LL ~     });
-
-
-error: use Option::map_or instead of an if let/else
-   |
-LL |       let _ = match s {
-   |  _____________^
-   |  _____________^
-LL | |         Some(string) => string.len(),
-LL | |         None => 1,
-LL | |     };
-   | |_____^ help: try: `s.map_or(1, |string| string.len())`
-
-error: use Option::map_or instead of an if let/else
-   |
-LL |       let _ = match Some(10) {
-   |  _____________^
-   |  _____________^
-LL | |         Some(a) => a + 1,
-LL | |         None => 5,
-LL | |     };
-   | |_____^ help: try: `Some(10).map_or(5, |a| a + 1)`
-
-error: use Option::map_or instead of an if let/else
-   |
-LL |       let _ = match res {
-   |  _____________^
-   |  _____________^
-LL | |         Ok(a) => a + 1,
-LL | |         _ => 1,
-LL | |     };
-LL | |     };
-   | |_____^ help: try: `res.map_or(1, |a| a + 1)`
-
-error: use Option::map_or instead of an if let/else
-   |
-LL |       let _ = match res {
-   |  _____________^
-LL | |         Err(_) => 1,
-LL | |         Err(_) => 1,
-LL | |         Ok(a) => a + 1,
-LL | |     };
-   | |_____^ help: try: `res.map_or(1, |a| a + 1)`
-
-error: use Option::map_or instead of an if let/else
-   |
-   |
-LL |     let _ = if let Ok(a) = res { a + 1 } else { 5 };
-   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `res.map_or(5, |a| a + 1)`
-error: aborting due to 20 previous errors
-
+For more information about this error, try `rustc --explain E0716`.
 
 

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/option_if_let_else.stage-id.stderr
diff of fixed:

 // run-rustfix
 #![warn(clippy::option_if_let_else)]
     unused_tuple_struct_fields,
     clippy::redundant_closure,
     clippy::ref_option_ref,
     clippy::equatable_if_let,
     clippy::equatable_if_let,
     clippy::let_unit_value
 )]
 
 fn bad1(string: Option<&str>) -> (bool, &str) {
-    string.map_or((false, "hello"), |x| (true, x))
+    if let Some(x) = string {
+        (true, x)
+        (false, "hello")
+    }
 }
 
 
 fn else_if_option(string: Option<&str>) -> Option<(bool, &str)> {
     if string.is_none() {
         None
     } else if let Some(x) = string {
         Some((true, x))
         Some((false, ""))
     }
 }
 
 
 fn unop_bad(string: &Option<&str>, mut num: Option<i32>) {
-    let _ = string.map_or(0, |s| s.len());
-    let _ = num.as_ref().map_or(&0, |s| s);
-    let _ = num.as_mut().map_or(&mut 0, |s| {
+    let _ = if let Some(s) = *string { s.len() } else { 0 };
+    let _ = if let Some(s) = &num { s } else { &0 };
+    let _ = if let Some(s) = &mut num {
         *s += 1;
-    });
-    });
-    let _ = num.as_ref().map_or(&0, |s| s);
-    let _ = num.map_or(0, |mut s| {
+        &mut 0
+    };
+    };
+    let _ = if let Some(ref s) = num { s } else { &0 };
+    let _ = if let Some(mut s) = num {
         s += 1;
-    });
-    });
-    let _ = num.as_mut().map_or(&mut 0, |s| {
+        0
+    };
+    };
+    let _ = if let Some(ref mut s) = num {
         *s += 1;
-    });
+    } else {
+        &mut 0
+    };
+    };
 }
 
 fn longer_body(arg: Option<u32>) -> u32 {
-    arg.map_or(13, |x| {
+    if let Some(x) = arg {
         let y = x * x;
         y * y
+    } else {
+        13
+    }
 }
 }
 
 fn impure_else(arg: Option<i32>) {
     let side_effect = || {
         println!("return 1");
     };
     };
-    let _ = arg.map_or_else(|| side_effect(), |x| x);
+    let _ = if let Some(x) = arg {
+    } else {
+        // map_or_else must be suggested
+        side_effect()
+    };
+    };
 }
 
 fn test_map_or_else(arg: Option<u32>) {
-    let _ = arg.map_or_else(|| {
+    let _ = if let Some(x) = arg {
+        x * x * x * x
         let mut y = 1;
         let mut y = 1;
         y = (y + 2 / y) / 2;
         y = (y + 2 / y) / 2;
         y
-    }, |x| x * x * x * x);
 }
 
 
 fn negative_tests(arg: Option<u32>) -> u32 {
     let _ = if let Some(13) = arg { "unlucky" } else { "lucky" };
     for _ in 0..10 {
         let _ = if let Some(x) = arg {
         } else {
             continue;
         };
     }
     }
     let _ = if let Some(x) = arg {
         return x;
         5
     };
     7
 }
 }
 
 // #7973
 fn pattern_to_vec(pattern: &str) -> Vec<String> {
     pattern
         .trim_matches('/')
         .split('/')
         .flat_map(|s| {
-            s.find('.').map_or_else(|| vec![s.to_string()], |idx| vec![s[..idx].to_string(), s[idx..].to_string()])
+            if let Some(idx) = s.find('.') {
+                vec![s[..idx].to_string(), s[idx..].to_string()]
+            } else {
+                vec![s.to_string()]
         })
         .collect::<Vec<_>>()
 }
 
 
 enum DummyEnum {
     One(u8),
 }
 
 // should not warn since there is a compled complex subpat
 // see #7991
 // see #7991
 fn complex_subpat() -> DummyEnum {
     let x = Some(DummyEnum::One(1));
     let _ = if let Some(_one @ DummyEnum::One(..)) = x { 1 } else { 2 };
     DummyEnum::Two
 
 fn main() {
     let optional = Some(5);
     let optional = Some(5);
-    let _ = optional.map_or(5, |x| x + 2);
+    let _ = if let Some(x) = optional { x + 2 } else { 5 };
     let _ = bad1(None);
     let _ = else_if_option(None);
     unop_bad(&None, None);
     let _ = longer_body(None);
     test_map_or_else(None);
     let _ = negative_tests(None);
     let _ = impure_else(None);
 
-    let _ = Some(0).map_or(0, |x| loop {
+    let _ = if let Some(x) = Some(0) {
+        loop {
             if x == 0 {
                 break x;
-        });
+        }
+    } else {
+        0
+        0
+    };
 
     // #7576
     const fn _f(x: Option<u32>) -> u32 {
         // Don't lint, `map_or` isn't const
         if let Some(x) = x { x } else { 10 }
 
     // #5822
     let s = String::new();
     let s = String::new();
     // Don't lint, `Some` branch consumes `s`, but else branch uses `s`
     let _ = if let Some(x) = Some(0) {
         let s = s;
         s.len() + x
         s.len()
     };
 
     let s = String::new();
     let s = String::new();
     // Lint, both branches immutably borrow `s`.
-    let _ = Some(0).map_or(s.len(), |x| s.len() + x);
+    let _ = if let Some(x) = Some(0) { s.len() + x } else { s.len() };
     let s = String::new();
     let s = String::new();
     // Lint, `Some` branch consumes `s`, but else branch doesn't use `s`.
-    let _ = Some(0).map_or(1, |x| {
+    let _ = if let Some(x) = Some(0) {
         let s = s;
         s.len() + x
+    } else {
+        1
+    };
 
 
     let s = Some(String::new());
     // Don't lint, `Some` branch borrows `s`, but else branch consumes `s`
     let _ = if let Some(x) = &s {
         x.len()
         let _s = s;
         10
     };
 
 

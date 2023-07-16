plain
i................................................................................................... 100/1183
.................................................................................................... 200/1183
..............................iii......i......i...i.........i....................................... 300/1183
.................................................................................................... 400/1183
.........................................................................F..........FF...F.F.F.iF... 500/1183
.................................................................................................... 700/1183
.................................................................................................... 800/1183
.................................................................................................... 900/1183
..............................................i............iii...................................... 1000/1183
---
---- src/io/stdio.rs - io::stdio::Stdin (line 218) stdout ----
error: unused import: `Read`
 --> src/io/stdio.rs:219:21
  |
3 | use std::io::{self, Read};
  |
note: the lint level is defined here
 --> src/io/stdio.rs:217:9
  |
---
---- src/io/stdio.rs - io::stdio::stdin (line 279) stdout ----
error: unused import: `Read`
 --> src/io/stdio.rs:280:21
  |
3 | use std::io::{self, Read};
  |
note: the lint level is defined here
 --> src/io/stdio.rs:278:9
  |
---
---- src/io/stdio.rs - io::stdio::Stdin::into_locked (line 433) stdout ----
error: unused import: `Read`
 --> src/io/stdio.rs:435:21
  |
4 | use std::io::{self, Read};
  |
note: the lint level is defined here
 --> src/io/stdio.rs:432:9
  |
  |
1 | #![deny(warnings)]
  |         ^^^^^^^^
  = note: `#[deny(unused_imports)]` implied by `#[deny(warnings)]`

error[E0599]: no method named `read_line` found for struct `StdinLock` in the current scope
     |
     |
10   |     handle.read_line(&mut buffer)?;
     |            ^^^^^^^^^ method not found in `StdinLock<'_>`
    ::: /checkout/library/std/src/io/mod.rs:2183:8
     |
     |
2183 |     fn read_line(&mut self, buf: &mut String) -> Result<usize> {
     |        --------- the method is available for `StdinLock<'_>` here
     = help: items from traits can only be used if the trait is in scope
help: the following trait is implemented but not in scope; perhaps add a `use` for it:
     |
4    | use std::io::BufRead;
---
---- src/io/stdio.rs - io::stdio::Stdin::lock (line 365) stdout ----
error: unused import: `Read`
 --> src/io/stdio.rs:366:21
  |
3 | use std::io::{self, Read};
  |
note: the lint level is defined here
 --> src/io/stdio.rs:364:9
  |
  |
1 | #![deny(warnings)]
  |         ^^^^^^^^
  = note: `#[deny(unused_imports)]` implied by `#[deny(warnings)]`

error[E0599]: no method named `read_line` found for struct `StdinLock` in the current scope
     |
     |
10   |     handle.read_line(&mut buffer)?;
     |            ^^^^^^^^^ method not found in `StdinLock<'_>`
    ::: /checkout/library/std/src/io/mod.rs:2183:8
     |
     |
2183 |     fn read_line(&mut self, buf: &mut String) -> Result<usize> {
     |        --------- the method is available for `StdinLock<'_>` here
     = help: items from traits can only be used if the trait is in scope
help: the following trait is implemented but not in scope; perhaps add a `use` for it:
     |
3    | use std::io::BufRead;
---
---- src/io/stdio.rs - io::stdio::StdinLock (line 246) stdout ----
error: unused import: `Read`
 --> src/io/stdio.rs:247:21
  |
3 | use std::io::{self, Read};
  |
note: the lint level is defined here
 --> src/io/stdio.rs:245:9
  |
  |
1 | #![deny(warnings)]
  |         ^^^^^^^^
  = note: `#[deny(unused_imports)]` implied by `#[deny(warnings)]`

error[E0599]: no method named `read_line` found for struct `StdinLock` in the current scope
     |
     |
10   |         handle.read_line(&mut buffer)?;
     |                ^^^^^^^^^ method not found in `StdinLock<'_>`
    ::: /checkout/library/std/src/io/mod.rs:2183:8
     |
     |
2183 |     fn read_line(&mut self, buf: &mut String) -> Result<usize> {
     |        --------- the method is available for `StdinLock<'_>` here
     = help: items from traits can only be used if the trait is in scope
help: the following trait is implemented but not in scope; perhaps add a `use` for it:
     |
3    | use std::io::BufRead;
---
---- src/io/stdio.rs - io::stdio::stdin (line 291) stdout ----
error: unused import: `Read`
 --> src/io/stdio.rs:292:21
  |
3 | use std::io::{self, Read};
  |
note: the lint level is defined here
 --> src/io/stdio.rs:290:9
  |
  |
1 | #![deny(warnings)]
  |         ^^^^^^^^
  = note: `#[deny(unused_imports)]` implied by `#[deny(warnings)]`

error[E0599]: no method named `read_line` found for struct `StdinLock` in the current scope
     |
     |
10   |     handle.read_line(&mut buffer)?;
     |            ^^^^^^^^^ method not found in `StdinLock<'_>`
    ::: /checkout/library/std/src/io/mod.rs:2183:8
     |
     |
2183 |     fn read_line(&mut self, buf: &mut String) -> Result<usize> {
     |        --------- the method is available for `StdinLock<'_>` here
     = help: items from traits can only be used if the trait is in scope
help: the following trait is implemented but not in scope; perhaps add a `use` for it:
     |
3    | use std::io::BufRead;
---
---- src/io/stdio.rs - io::stdio::stdin_locked (line 338) stdout ----
error: unused import: `Read`
 --> src/io/stdio.rs:340:21
  |
4 | use std::io::{self, Read};
  |
note: the lint level is defined here
 --> src/io/stdio.rs:337:9
  |
  |
1 | #![deny(warnings)]
  |         ^^^^^^^^
  = note: `#[deny(unused_imports)]` implied by `#[deny(warnings)]`

error[E0599]: no method named `read_line` found for struct `StdinLock` in the current scope
     |
     |
10   |     handle.read_line(&mut buffer)?;
     |            ^^^^^^^^^ method not found in `StdinLock<'_>`
    ::: /checkout/library/std/src/io/mod.rs:2183:8
     |
     |
2183 |     fn read_line(&mut self, buf: &mut String) -> Result<usize> {
     |        --------- the method is available for `StdinLock<'_>` here
     = help: items from traits can only be used if the trait is in scope
help: the following trait is implemented but not in scope; perhaps add a `use` for it:
     |
4    | use std::io::BufRead;
---

error: test failed, to rerun pass '--doc'


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "std" "--" "--quiet"


Build completed unsuccessfully in 0:19:23

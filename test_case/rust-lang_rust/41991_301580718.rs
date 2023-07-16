
./x.py test src/test/rustdoc/ --stage 1
    Finished dev [unoptimized] target(s) in 0.0 secs
Synchronizing submodule url for 'src/compiler-rt'
Synchronizing submodule url for 'src/doc/book'
Synchronizing submodule url for 'src/doc/nomicon'
Synchronizing submodule url for 'src/doc/reference'
Synchronizing submodule url for 'src/jemalloc'
Synchronizing submodule url for 'src/liblibc'
Synchronizing submodule url for 'src/llvm'
Synchronizing submodule url for 'src/rt/hoedown'
Synchronizing submodule url for 'src/rust-installer'
Synchronizing submodule url for 'src/tools/cargo'
Synchronizing submodule url for 'src/tools/rls'
HEAD is now at c8a8767 Merge pull request #26 from TimNN/arm-cc
HEAD is now at ad7de19 Merge pull request #631 from frewsxcv/new-unstable-book-links
HEAD is now at 6fa139b Merge pull request #18 from Gankro/master
HEAD is now at 6b0de90 Merge pull request #43 from frewsxcv/fix-link
HEAD is now at 3288e06 Merge pull request #18 from arthurprs/update-to-4.5.0
HEAD is now at 03562b0 Auto merge of #585 - jonhoo:res_init, r=alexcrichton
HEAD is now at 1ef3b91 Merge pull request #78 from pftbest/msp430_libcalls
Removing utils/llvm-build/llvmbuild/__init__.pyc
Removing utils/llvm-build/llvmbuild/componentinfo.pyc
Removing utils/llvm-build/llvmbuild/configutil.pyc
Removing utils/llvm-build/llvmbuild/main.pyc
Removing utils/llvm-build/llvmbuild/util.pyc
HEAD is now at da282f1 Merge pull request #8 from GuillaumeGomez/line_information
HEAD is now at 2e6417f Merge pull request #61 from TimNN/tar-sort-default
HEAD is now at cf17c9f Auto merge of #4010 - jonhoo:better-incremental-nightly-test, r=alexcrichton
HEAD is now at 38ca9b7 Merge pull request #314 from martinschleiss/master
Building stage0 std artifacts (x86_64-apple-darwin -> x86_64-apple-darwin)
    Finished release [optimized] target(s) in 0.0 secs
Copying stage0 std from stage0 (x86_64-apple-darwin -> x86_64-apple-darwin / x86_64-apple-darwin)
Building stage0 test artifacts (x86_64-apple-darwin -> x86_64-apple-darwin)
    Finished release [optimized] target(s) in 0.0 secs
Copying stage0 test from stage0 (x86_64-apple-darwin -> x86_64-apple-darwin / x86_64-apple-darwin)
Building stage0 compiler artifacts (x86_64-apple-darwin -> x86_64-apple-darwin)
warning: ../rustllvm/PassWrapper.cpp:285:3: warning: default label in switch which covers all enumeration values [-Wcovered-switch-default]
warning:   default:
warning:   ^
warning: 1 warning generated.
   Compiling html5ever v0.13.1
   Compiling rustc_back v0.0.0 (file:///Users/imperio/rust/rust/src/librustc_back)
   Compiling rustc_const_math v0.0.0 (file:///Users/imperio/rust/rust/src/librustc_const_math)
   Compiling proc_macro v0.0.0 (file:///Users/imperio/rust/rust/src/libproc_macro)
error: use of unstable library feature 'rustc_private' (see issue #27812)
  --> /Users/imperio/.cargo/registry/src/github.com-1ecc6299db9ec823/html5ever-0.13.1/src/lib.rs:21:1
   |
21 | extern crate log;
   | ^^^^^^^^^^^^^^^^^
   |
   = help: add #![feature(rustc_private)] to the crate attributes to enable

error: use of unstable library feature 'rustc_private' (see issue #27812)
  --> /Users/imperio/.cargo/registry/src/github.com-1ecc6299db9ec823/html5ever-0.13.1/src/serialize/mod.rs:71:13
   |
71 |             warn!("node with weird namespace {:?}", ns);
   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = help: add #![feature(rustc_private)] to the crate attributes to enable
   = note: this error originates in a macro outside of the current crate

error: use of unstable library feature 'rustc_private' (see issue #27812)
  --> /Users/imperio/.cargo/registry/src/github.com-1ecc6299db9ec823/html5ever-0.13.1/src/serialize/mod.rs:71:13
   |
71 |             warn!("node with weird namespace {:?}", ns);
   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = help: add #![feature(rustc_private)] to the crate attributes to enable
   = note: this error originates in a macro outside of the current crate

error: use of unstable library feature 'rustc_private' (see issue #27812)
  --> /Users/imperio/.cargo/registry/src/github.com-1ecc6299db9ec823/html5ever-0.13.1/src/serialize/mod.rs:71:13
   |
71 |             warn!("node with weird namespace {:?}", ns);
   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = help: add #![feature(rustc_private)] to the crate attributes to enable
   = note: this error originates in a macro outside of the current crate

error: use of unstable library feature 'rustc_private' (see issue #27812)
  --> /Users/imperio/.cargo/registry/src/github.com-1ecc6299db9ec823/html5ever-0.13.1/src/serialize/mod.rs:71:13
   |
71 |             warn!("node with weird namespace {:?}", ns);
   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = help: add #![feature(rustc_private)] to the crate attributes to enable
   = note: this error originates in a macro outside of the current crate

error: use of unstable library feature 'rustc_private' (see issue #27812)
  --> /Users/imperio/.cargo/registry/src/github.com-1ecc6299db9ec823/html5ever-0.13.1/src/serialize/mod.rs:71:13
   |
71 |             warn!("node with weird namespace {:?}", ns);
   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = help: add #![feature(rustc_private)] to the crate attributes to enable
   = note: this error originates in a macro outside of the current crate

error: use of unstable library feature 'rustc_private' (see issue #27812)
  --> /Users/imperio/.cargo/registry/src/github.com-1ecc6299db9ec823/html5ever-0.13.1/src/serialize/mod.rs:71:13
   |
71 |             warn!("node with weird namespace {:?}", ns);
   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = help: add #![feature(rustc_private)] to the crate attributes to enable
   = note: this error originates in a macro outside of the current crate

error: use of unstable library feature 'rustc_private' (see issue #27812)
   --> /Users/imperio/.cargo/registry/src/github.com-1ecc6299db9ec823/html5ever-0.13.1/src/serialize/mod.rs:144:21
    |
144 |                     warn!("attr with weird namespace {:?}", ns);
    |                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: add #![feature(rustc_private)] to the crate attributes to enable
    = note: this error originates in a macro outside of the current crate

error: use of unstable library feature 'rustc_private' (see issue #27812)
   --> /Users/imperio/.cargo/registry/src/github.com-1ecc6299db9ec823/html5ever-0.13.1/src/serialize/mod.rs:144:21
    |
144 |                     warn!("attr with weird namespace {:?}", ns);
    |                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: add #![feature(rustc_private)] to the crate attributes to enable
    = note: this error originates in a macro outside of the current crate

error: use of unstable library feature 'rustc_private' (see issue #27812)
   --> /Users/imperio/.cargo/registry/src/github.com-1ecc6299db9ec823/html5ever-0.13.1/src/tokenizer/char_ref/mod.rs:126:9
    |
126 |         debug!("char ref tokenizer stepping in state {:?}", self.state);
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: add #![feature(rustc_private)] to the crate attributes to enable
    = note: this error originates in a macro outside of the current crate

error: use of unstable library feature 'rustc_private' (see issue #27812)
   --> /Users/imperio/.cargo/registry/src/github.com-1ecc6299db9ec823/html5ever-0.13.1/src/tokenizer/char_ref/mod.rs:126:9
    |
126 |         debug!("char ref tokenizer stepping in state {:?}", self.state);
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: add #![feature(rustc_private)] to the crate attributes to enable
    = note: this error originates in a macro outside of the current crate

error: use of unstable library feature 'rustc_private' (see issue #27812)
   --> /Users/imperio/.cargo/registry/src/github.com-1ecc6299db9ec823/html5ever-0.13.1/src/tokenizer/mod.rs:289:9
    |
289 |         debug!("got character {}", c);
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: add #![feature(rustc_private)] to the crate attributes to enable
    = note: this error originates in a macro outside of the current crate

error: use of unstable library feature 'rustc_private' (see issue #27812)
   --> /Users/imperio/.cargo/registry/src/github.com-1ecc6299db9ec823/html5ever-0.13.1/src/tokenizer/mod.rs:289:9
    |
289 |         debug!("got character {}", c);
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: add #![feature(rustc_private)] to the crate attributes to enable
    = note: this error originates in a macro outside of the current crate

error: use of unstable library feature 'rustc_private' (see issue #27812)
   --> /Users/imperio/.cargo/registry/src/github.com-1ecc6299db9ec823/html5ever-0.13.1/src/tokenizer/mod.rs:315:9
    |
315 |         debug!("got characters {:?}", d);
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: add #![feature(rustc_private)] to the crate attributes to enable
    = note: this error originates in a macro outside of the current crate

error: use of unstable library feature 'rustc_private' (see issue #27812)
   --> /Users/imperio/.cargo/registry/src/github.com-1ecc6299db9ec823/html5ever-0.13.1/src/tokenizer/mod.rs:315:9
    |
315 |         debug!("got characters {:?}", d);
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: add #![feature(rustc_private)] to the crate attributes to enable
    = note: this error originates in a macro outside of the current crate

error: use of unstable library feature 'rustc_private' (see issue #27812)
   --> /Users/imperio/.cargo/registry/src/github.com-1ecc6299db9ec823/html5ever-0.13.1/src/tokenizer/mod.rs:694:9
    |
694 |         debug!("processing in state {:?}", self.state);
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: add #![feature(rustc_private)] to the crate attributes to enable
    = note: this error originates in a macro outside of the current crate

error: use of unstable library feature 'rustc_private' (see issue #27812)
   --> /Users/imperio/.cargo/registry/src/github.com-1ecc6299db9ec823/html5ever-0.13.1/src/tokenizer/mod.rs:694:9
    |
694 |         debug!("processing in state {:?}", self.state);
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: add #![feature(rustc_private)] to the crate attributes to enable
    = note: this error originates in a macro outside of the current crate

error: use of unstable library feature 'rustc_private' (see issue #27812)
    --> /Users/imperio/.cargo/registry/src/github.com-1ecc6299db9ec823/html5ever-0.13.1/src/tokenizer/mod.rs:1331:9
     |
1331 |         debug!("processing EOF in state {:?}", self.state);
     |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
     = help: add #![feature(rustc_private)] to the crate attributes to enable
     = note: this error originates in a macro outside of the current crate

error: use of unstable library feature 'rustc_private' (see issue #27812)
    --> /Users/imperio/.cargo/registry/src/github.com-1ecc6299db9ec823/html5ever-0.13.1/src/tokenizer/mod.rs:1331:9
     |
1331 |         debug!("processing EOF in state {:?}", self.state);
     |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
     = help: add #![feature(rustc_private)] to the crate attributes to enable
     = note: this error originates in a macro outside of the current crate

error: use of unstable library feature 'rustc_private' (see issue #27812)
   --> /Users/imperio/.cargo/registry/src/github.com-1ecc6299db9ec823/html5ever-0.13.1/src/tree_builder/actions.rs:170:9
    |
170 |         warn!("stop_parsing not implemented, full speed ahead!");
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: add #![feature(rustc_private)] to the crate attributes to enable
    = note: this error originates in a macro outside of the current crate

error: use of unstable library feature 'rustc_private' (see issue #27812)
   --> /Users/imperio/.cargo/registry/src/github.com-1ecc6299db9ec823/html5ever-0.13.1/src/tree_builder/actions.rs:170:9
    |
170 |         warn!("stop_parsing not implemented, full speed ahead!");
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: add #![feature(rustc_private)] to the crate attributes to enable
    = note: this error originates in a macro outside of the current crate

error: use of unstable library feature 'rustc_private' (see issue #27812)
   --> /Users/imperio/.cargo/registry/src/github.com-1ecc6299db9ec823/html5ever-0.13.1/src/tree_builder/actions.rs:672:9
    |
672 |         warn!("foster parenting not implemented");
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: add #![feature(rustc_private)] to the crate attributes to enable
    = note: this error originates in a macro outside of the current crate

error: use of unstable library feature 'rustc_private' (see issue #27812)
   --> /Users/imperio/.cargo/registry/src/github.com-1ecc6299db9ec823/html5ever-0.13.1/src/tree_builder/actions.rs:672:9
    |
672 |         warn!("foster parenting not implemented");
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: add #![feature(rustc_private)] to the crate attributes to enable
    = note: this error originates in a macro outside of the current crate

error: use of unstable library feature 'rustc_private' (see issue #27812)
   --> /Users/imperio/.cargo/registry/src/github.com-1ecc6299db9ec823/html5ever-0.13.1/src/tree_builder/mod.rs:308:9
    |
308 |         debug!("processing {} in insertion mode {:?}", to_escaped_string(token), mode);
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: add #![feature(rustc_private)] to the crate attributes to enable
    = note: this error originates in a macro outside of the current crate

error: use of unstable library feature 'rustc_private' (see issue #27812)
   --> /Users/imperio/.cargo/registry/src/github.com-1ecc6299db9ec823/html5ever-0.13.1/src/tree_builder/mod.rs:308:9
    |
308 |         debug!("processing {} in insertion mode {:?}", to_escaped_string(token), mode);
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: add #![feature(rustc_private)] to the crate attributes to enable
    = note: this error originates in a macro outside of the current crate

error: use of unstable library feature 'rustc_private' (see issue #27812)
   --> /Users/imperio/.cargo/registry/src/github.com-1ecc6299db9ec823/html5ever-0.13.1/src/serialize/mod.rs:144:21
    |
144 |                     warn!("attr with weird namespace {:?}", ns);
    |                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: add #![feature(rustc_private)] to the crate attributes to enable
    = note: this error originates in a macro outside of the current crate

error: use of unstable library feature 'rustc_private' (see issue #27812)
   --> /Users/imperio/.cargo/registry/src/github.com-1ecc6299db9ec823/html5ever-0.13.1/src/serialize/mod.rs:144:21
    |
144 |                     warn!("attr with weird namespace {:?}", ns);
    |                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: add #![feature(rustc_private)] to the crate attributes to enable
    = note: this error originates in a macro outside of the current crate

error: use of unstable library feature 'rustc_private' (see issue #27812)
   --> /Users/imperio/.cargo/registry/src/github.com-1ecc6299db9ec823/html5ever-0.13.1/src/serialize/mod.rs:144:21
    |
144 |                     warn!("attr with weird namespace {:?}", ns);
    |                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: add #![feature(rustc_private)] to the crate attributes to enable
    = note: this error originates in a macro outside of the current crate

error: use of unstable library feature 'rustc_private' (see issue #27812)
   --> /Users/imperio/.cargo/registry/src/github.com-1ecc6299db9ec823/html5ever-0.13.1/src/serialize/mod.rs:144:21
    |
144 |                     warn!("attr with weird namespace {:?}", ns);
    |                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: add #![feature(rustc_private)] to the crate attributes to enable
    = note: this error originates in a macro outside of the current crate

error: use of unstable library feature 'rustc_private' (see issue #27812)
   --> /Users/imperio/.cargo/registry/src/github.com-1ecc6299db9ec823/html5ever-0.13.1/src/tokenizer/char_ref/mod.rs:126:9
    |
126 |         debug!("char ref tokenizer stepping in state {:?}", self.state);
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: add #![feature(rustc_private)] to the crate attributes to enable
    = note: this error originates in a macro outside of the current crate

error: use of unstable library feature 'rustc_private' (see issue #27812)
   --> /Users/imperio/.cargo/registry/src/github.com-1ecc6299db9ec823/html5ever-0.13.1/src/tokenizer/char_ref/mod.rs:126:9
    |
126 |         debug!("char ref tokenizer stepping in state {:?}", self.state);
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: add #![feature(rustc_private)] to the crate attributes to enable
    = note: this error originates in a macro outside of the current crate

error: use of unstable library feature 'rustc_private' (see issue #27812)
   --> /Users/imperio/.cargo/registry/src/github.com-1ecc6299db9ec823/html5ever-0.13.1/src/tokenizer/char_ref/mod.rs:126:9
    |
126 |         debug!("char ref tokenizer stepping in state {:?}", self.state);
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: add #![feature(rustc_private)] to the crate attributes to enable
    = note: this error originates in a macro outside of the current crate

error: use of unstable library feature 'rustc_private' (see issue #27812)
   --> /Users/imperio/.cargo/registry/src/github.com-1ecc6299db9ec823/html5ever-0.13.1/src/tokenizer/char_ref/mod.rs:126:9
    |
126 |         debug!("char ref tokenizer stepping in state {:?}", self.state);
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: add #![feature(rustc_private)] to the crate attributes to enable
    = note: this error originates in a macro outside of the current crate

error: use of unstable library feature 'rustc_private' (see issue #27812)
   --> /Users/imperio/.cargo/registry/src/github.com-1ecc6299db9ec823/html5ever-0.13.1/src/tokenizer/mod.rs:289:9
    |
289 |         debug!("got character {}", c);
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: add #![feature(rustc_private)] to the crate attributes to enable
    = note: this error originates in a macro outside of the current crate

error: use of unstable library feature 'rustc_private' (see issue #27812)
   --> /Users/imperio/.cargo/registry/src/github.com-1ecc6299db9ec823/html5ever-0.13.1/src/tokenizer/mod.rs:289:9
    |
289 |         debug!("got character {}", c);
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: add #![feature(rustc_private)] to the crate attributes to enable
    = note: this error originates in a macro outside of the current crate

error: use of unstable library feature 'rustc_private' (see issue #27812)
   --> /Users/imperio/.cargo/registry/src/github.com-1ecc6299db9ec823/html5ever-0.13.1/src/tokenizer/mod.rs:289:9
    |
289 |         debug!("got character {}", c);
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: add #![feature(rustc_private)] to the crate attributes to enable
    = note: this error originates in a macro outside of the current crate

error: use of unstable library feature 'rustc_private' (see issue #27812)
   --> /Users/imperio/.cargo/registry/src/github.com-1ecc6299db9ec823/html5ever-0.13.1/src/tokenizer/mod.rs:289:9
    |
289 |         debug!("got character {}", c);
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: add #![feature(rustc_private)] to the crate attributes to enable
    = note: this error originates in a macro outside of the current crate

error: use of unstable library feature 'rustc_private' (see issue #27812)
   --> /Users/imperio/.cargo/registry/src/github.com-1ecc6299db9ec823/html5ever-0.13.1/src/tokenizer/mod.rs:315:9
    |
315 |         debug!("got characters {:?}", d);
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: add #![feature(rustc_private)] to the crate attributes to enable
    = note: this error originates in a macro outside of the current crate

error: use of unstable library feature 'rustc_private' (see issue #27812)
   --> /Users/imperio/.cargo/registry/src/github.com-1ecc6299db9ec823/html5ever-0.13.1/src/tokenizer/mod.rs:315:9
    |
315 |         debug!("got characters {:?}", d);
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: add #![feature(rustc_private)] to the crate attributes to enable
    = note: this error originates in a macro outside of the current crate

error: use of unstable library feature 'rustc_private' (see issue #27812)
   --> /Users/imperio/.cargo/registry/src/github.com-1ecc6299db9ec823/html5ever-0.13.1/src/tokenizer/mod.rs:315:9
    |
315 |         debug!("got characters {:?}", d);
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: add #![feature(rustc_private)] to the crate attributes to enable
    = note: this error originates in a macro outside of the current crate

error: use of unstable library feature 'rustc_private' (see issue #27812)
   --> /Users/imperio/.cargo/registry/src/github.com-1ecc6299db9ec823/html5ever-0.13.1/src/tokenizer/mod.rs:315:9
    |
315 |         debug!("got characters {:?}", d);
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: add #![feature(rustc_private)] to the crate attributes to enable
    = note: this error originates in a macro outside of the current crate

error: use of unstable library feature 'rustc_private' (see issue #27812)
   --> /Users/imperio/.cargo/registry/src/github.com-1ecc6299db9ec823/html5ever-0.13.1/src/tokenizer/mod.rs:694:9
    |
694 |         debug!("processing in state {:?}", self.state);
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: add #![feature(rustc_private)] to the crate attributes to enable
    = note: this error originates in a macro outside of the current crate

error: use of unstable library feature 'rustc_private' (see issue #27812)
   --> /Users/imperio/.cargo/registry/src/github.com-1ecc6299db9ec823/html5ever-0.13.1/src/tokenizer/mod.rs:694:9
    |
694 |         debug!("processing in state {:?}", self.state);
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: add #![feature(rustc_private)] to the crate attributes to enable
    = note: this error originates in a macro outside of the current crate

error: use of unstable library feature 'rustc_private' (see issue #27812)
   --> /Users/imperio/.cargo/registry/src/github.com-1ecc6299db9ec823/html5ever-0.13.1/src/tokenizer/mod.rs:694:9
    |
694 |         debug!("processing in state {:?}", self.state);
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: add #![feature(rustc_private)] to the crate attributes to enable
    = note: this error originates in a macro outside of the current crate

error: use of unstable library feature 'rustc_private' (see issue #27812)
   --> /Users/imperio/.cargo/registry/src/github.com-1ecc6299db9ec823/html5ever-0.13.1/src/tokenizer/mod.rs:694:9
    |
694 |         debug!("processing in state {:?}", self.state);
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: add #![feature(rustc_private)] to the crate attributes to enable
    = note: this error originates in a macro outside of the current crate

error: use of unstable library feature 'rustc_private' (see issue #27812)
    --> /Users/imperio/.cargo/registry/src/github.com-1ecc6299db9ec823/html5ever-0.13.1/src/tokenizer/mod.rs:1331:9
     |
1331 |         debug!("processing EOF in state {:?}", self.state);
     |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
     = help: add #![feature(rustc_private)] to the crate attributes to enable
     = note: this error originates in a macro outside of the current crate

error: use of unstable library feature 'rustc_private' (see issue #27812)
    --> /Users/imperio/.cargo/registry/src/github.com-1ecc6299db9ec823/html5ever-0.13.1/src/tokenizer/mod.rs:1331:9
     |
1331 |         debug!("processing EOF in state {:?}", self.state);
     |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
     = help: add #![feature(rustc_private)] to the crate attributes to enable
     = note: this error originates in a macro outside of the current crate

error: use of unstable library feature 'rustc_private' (see issue #27812)
    --> /Users/imperio/.cargo/registry/src/github.com-1ecc6299db9ec823/html5ever-0.13.1/src/tokenizer/mod.rs:1331:9
     |
1331 |         debug!("processing EOF in state {:?}", self.state);
     |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
     = help: add #![feature(rustc_private)] to the crate attributes to enable
     = note: this error originates in a macro outside of the current crate

error: use of unstable library feature 'rustc_private' (see issue #27812)
    --> /Users/imperio/.cargo/registry/src/github.com-1ecc6299db9ec823/html5ever-0.13.1/src/tokenizer/mod.rs:1331:9
     |
1331 |         debug!("processing EOF in state {:?}", self.state);
     |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
     = help: add #![feature(rustc_private)] to the crate attributes to enable
     = note: this error originates in a macro outside of the current crate

error: use of unstable library feature 'rustc_private' (see issue #27812)
   --> /Users/imperio/.cargo/registry/src/github.com-1ecc6299db9ec823/html5ever-0.13.1/src/tree_builder/actions.rs:170:9
    |
170 |         warn!("stop_parsing not implemented, full speed ahead!");
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: add #![feature(rustc_private)] to the crate attributes to enable
    = note: this error originates in a macro outside of the current crate

error: use of unstable library feature 'rustc_private' (see issue #27812)
   --> /Users/imperio/.cargo/registry/src/github.com-1ecc6299db9ec823/html5ever-0.13.1/src/tree_builder/actions.rs:170:9
    |
170 |         warn!("stop_parsing not implemented, full speed ahead!");
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: add #![feature(rustc_private)] to the crate attributes to enable
    = note: this error originates in a macro outside of the current crate

error: use of unstable library feature 'rustc_private' (see issue #27812)
   --> /Users/imperio/.cargo/registry/src/github.com-1ecc6299db9ec823/html5ever-0.13.1/src/tree_builder/actions.rs:170:9
    |
170 |         warn!("stop_parsing not implemented, full speed ahead!");
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: add #![feature(rustc_private)] to the crate attributes to enable
    = note: this error originates in a macro outside of the current crate

error: use of unstable library feature 'rustc_private' (see issue #27812)
   --> /Users/imperio/.cargo/registry/src/github.com-1ecc6299db9ec823/html5ever-0.13.1/src/tree_builder/actions.rs:170:9
    |
170 |         warn!("stop_parsing not implemented, full speed ahead!");
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: add #![feature(rustc_private)] to the crate attributes to enable
    = note: this error originates in a macro outside of the current crate

error: use of unstable library feature 'rustc_private' (see issue #27812)
   --> /Users/imperio/.cargo/registry/src/github.com-1ecc6299db9ec823/html5ever-0.13.1/src/tree_builder/actions.rs:672:9
    |
672 |         warn!("foster parenting not implemented");
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: add #![feature(rustc_private)] to the crate attributes to enable
    = note: this error originates in a macro outside of the current crate

error: use of unstable library feature 'rustc_private' (see issue #27812)
   --> /Users/imperio/.cargo/registry/src/github.com-1ecc6299db9ec823/html5ever-0.13.1/src/tree_builder/actions.rs:672:9
    |
672 |         warn!("foster parenting not implemented");
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: add #![feature(rustc_private)] to the crate attributes to enable
    = note: this error originates in a macro outside of the current crate

error: use of unstable library feature 'rustc_private' (see issue #27812)
   --> /Users/imperio/.cargo/registry/src/github.com-1ecc6299db9ec823/html5ever-0.13.1/src/tree_builder/actions.rs:672:9
    |
672 |         warn!("foster parenting not implemented");
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: add #![feature(rustc_private)] to the crate attributes to enable
    = note: this error originates in a macro outside of the current crate

error: use of unstable library feature 'rustc_private' (see issue #27812)
   --> /Users/imperio/.cargo/registry/src/github.com-1ecc6299db9ec823/html5ever-0.13.1/src/tree_builder/actions.rs:672:9
    |
672 |         warn!("foster parenting not implemented");
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: add #![feature(rustc_private)] to the crate attributes to enable
    = note: this error originates in a macro outside of the current crate

error: use of unstable library feature 'rustc_private' (see issue #27812)
   --> /Users/imperio/.cargo/registry/src/github.com-1ecc6299db9ec823/html5ever-0.13.1/src/tree_builder/mod.rs:308:9
    |
308 |         debug!("processing {} in insertion mode {:?}", to_escaped_string(token), mode);
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: add #![feature(rustc_private)] to the crate attributes to enable
    = note: this error originates in a macro outside of the current crate

error: use of unstable library feature 'rustc_private' (see issue #27812)
   --> /Users/imperio/.cargo/registry/src/github.com-1ecc6299db9ec823/html5ever-0.13.1/src/tree_builder/mod.rs:308:9
    |
308 |         debug!("processing {} in insertion mode {:?}", to_escaped_string(token), mode);
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: add #![feature(rustc_private)] to the crate attributes to enable
    = note: this error originates in a macro outside of the current crate

error: use of unstable library feature 'rustc_private' (see issue #27812)
   --> /Users/imperio/.cargo/registry/src/github.com-1ecc6299db9ec823/html5ever-0.13.1/src/tree_builder/mod.rs:308:9
    |
308 |         debug!("processing {} in insertion mode {:?}", to_escaped_string(token), mode);
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: add #![feature(rustc_private)] to the crate attributes to enable
    = note: this error originates in a macro outside of the current crate

error: use of unstable library feature 'rustc_private' (see issue #27812)
   --> /Users/imperio/.cargo/registry/src/github.com-1ecc6299db9ec823/html5ever-0.13.1/src/tree_builder/mod.rs:308:9
    |
308 |         debug!("processing {} in insertion mode {:?}", to_escaped_string(token), mode);
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: add #![feature(rustc_private)] to the crate attributes to enable
    = note: this error originates in a macro outside of the current crate

error: use of unstable library feature 'rustc_private' (see issue #27812)
  --> /Users/imperio/.cargo/registry/src/github.com-1ecc6299db9ec823/html5ever-0.13.1/src/serialize/mod.rs:71:13
   |
71 |             warn!("node with weird namespace {:?}", ns);
   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = help: add #![feature(rustc_private)] to the crate attributes to enable
   = note: this error originates in a macro outside of the current crate

error: use of unstable library feature 'rustc_private' (see issue #27812)
  --> /Users/imperio/.cargo/registry/src/github.com-1ecc6299db9ec823/html5ever-0.13.1/src/serialize/mod.rs:71:13
   |
71 |             warn!("node with weird namespace {:?}", ns);
   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = help: add #![feature(rustc_private)] to the crate attributes to enable
   = note: this error originates in a macro outside of the current crate

error: use of unstable library feature 'rustc_private' (see issue #27812)
  --> /Users/imperio/.cargo/registry/src/github.com-1ecc6299db9ec823/html5ever-0.13.1/src/serialize/mod.rs:71:13
   |
71 |             warn!("node with weird namespace {:?}", ns);
   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = help: add #![feature(rustc_private)] to the crate attributes to enable
   = note: this error originates in a macro outside of the current crate

error: use of unstable library feature 'rustc_private' (see issue #27812)
   --> /Users/imperio/.cargo/registry/src/github.com-1ecc6299db9ec823/html5ever-0.13.1/src/serialize/mod.rs:144:21
    |
144 |                     warn!("attr with weird namespace {:?}", ns);
    |                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: add #![feature(rustc_private)] to the crate attributes to enable
    = note: this error originates in a macro outside of the current crate

error: use of unstable library feature 'rustc_private' (see issue #27812)
   --> /Users/imperio/.cargo/registry/src/github.com-1ecc6299db9ec823/html5ever-0.13.1/src/serialize/mod.rs:144:21
    |
144 |                     warn!("attr with weird namespace {:?}", ns);
    |                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: add #![feature(rustc_private)] to the crate attributes to enable
    = note: this error originates in a macro outside of the current crate

error: use of unstable library feature 'rustc_private' (see issue #27812)
   --> /Users/imperio/.cargo/registry/src/github.com-1ecc6299db9ec823/html5ever-0.13.1/src/serialize/mod.rs:144:21
    |
144 |                     warn!("attr with weird namespace {:?}", ns);
    |                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: add #![feature(rustc_private)] to the crate attributes to enable
    = note: this error originates in a macro outside of the current crate

error: use of unstable library feature 'rustc_private' (see issue #27812)
   --> /Users/imperio/.cargo/registry/src/github.com-1ecc6299db9ec823/html5ever-0.13.1/src/tokenizer/char_ref/mod.rs:126:9
    |
126 |         debug!("char ref tokenizer stepping in state {:?}", self.state);
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: add #![feature(rustc_private)] to the crate attributes to enable
    = note: this error originates in a macro outside of the current crate

error: use of unstable library feature 'rustc_private' (see issue #27812)
   --> /Users/imperio/.cargo/registry/src/github.com-1ecc6299db9ec823/html5ever-0.13.1/src/tokenizer/char_ref/mod.rs:126:9
    |
126 |         debug!("char ref tokenizer stepping in state {:?}", self.state);
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: add #![feature(rustc_private)] to the crate attributes to enable
    = note: this error originates in a macro outside of the current crate

error: use of unstable library feature 'rustc_private' (see issue #27812)
   --> /Users/imperio/.cargo/registry/src/github.com-1ecc6299db9ec823/html5ever-0.13.1/src/tokenizer/char_ref/mod.rs:126:9
    |
126 |         debug!("char ref tokenizer stepping in state {:?}", self.state);
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: add #![feature(rustc_private)] to the crate attributes to enable
    = note: this error originates in a macro outside of the current crate

error: use of unstable library feature 'rustc_private' (see issue #27812)
   --> /Users/imperio/.cargo/registry/src/github.com-1ecc6299db9ec823/html5ever-0.13.1/src/tokenizer/mod.rs:289:9
    |
289 |         debug!("got character {}", c);
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: add #![feature(rustc_private)] to the crate attributes to enable
    = note: this error originates in a macro outside of the current crate

error: use of unstable library feature 'rustc_private' (see issue #27812)
   --> /Users/imperio/.cargo/registry/src/github.com-1ecc6299db9ec823/html5ever-0.13.1/src/tokenizer/mod.rs:289:9
    |
289 |         debug!("got character {}", c);
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: add #![feature(rustc_private)] to the crate attributes to enable
    = note: this error originates in a macro outside of the current crate

error: use of unstable library feature 'rustc_private' (see issue #27812)
   --> /Users/imperio/.cargo/registry/src/github.com-1ecc6299db9ec823/html5ever-0.13.1/src/tokenizer/mod.rs:289:9
    |
289 |         debug!("got character {}", c);
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: add #![feature(rustc_private)] to the crate attributes to enable
    = note: this error originates in a macro outside of the current crate

error: use of unstable library feature 'rustc_private' (see issue #27812)
   --> /Users/imperio/.cargo/registry/src/github.com-1ecc6299db9ec823/html5ever-0.13.1/src/tokenizer/mod.rs:315:9
    |
315 |         debug!("got characters {:?}", d);
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: add #![feature(rustc_private)] to the crate attributes to enable
    = note: this error originates in a macro outside of the current crate

error: use of unstable library feature 'rustc_private' (see issue #27812)
   --> /Users/imperio/.cargo/registry/src/github.com-1ecc6299db9ec823/html5ever-0.13.1/src/tokenizer/mod.rs:315:9
    |
315 |         debug!("got characters {:?}", d);
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: add #![feature(rustc_private)] to the crate attributes to enable
    = note: this error originates in a macro outside of the current crate

error: use of unstable library feature 'rustc_private' (see issue #27812)
   --> /Users/imperio/.cargo/registry/src/github.com-1ecc6299db9ec823/html5ever-0.13.1/src/tokenizer/mod.rs:315:9
    |
315 |         debug!("got characters {:?}", d);
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: add #![feature(rustc_private)] to the crate attributes to enable
    = note: this error originates in a macro outside of the current crate

error: use of unstable library feature 'rustc_private' (see issue #27812)
   --> /Users/imperio/.cargo/registry/src/github.com-1ecc6299db9ec823/html5ever-0.13.1/src/tokenizer/mod.rs:694:9
    |
694 |         debug!("processing in state {:?}", self.state);
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: add #![feature(rustc_private)] to the crate attributes to enable
    = note: this error originates in a macro outside of the current crate

error: use of unstable library feature 'rustc_private' (see issue #27812)
   --> /Users/imperio/.cargo/registry/src/github.com-1ecc6299db9ec823/html5ever-0.13.1/src/tokenizer/mod.rs:694:9
    |
694 |         debug!("processing in state {:?}", self.state);
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: add #![feature(rustc_private)] to the crate attributes to enable
    = note: this error originates in a macro outside of the current crate

error: use of unstable library feature 'rustc_private' (see issue #27812)
   --> /Users/imperio/.cargo/registry/src/github.com-1ecc6299db9ec823/html5ever-0.13.1/src/tokenizer/mod.rs:694:9
    |
694 |         debug!("processing in state {:?}", self.state);
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: add #![feature(rustc_private)] to the crate attributes to enable
    = note: this error originates in a macro outside of the current crate

error: use of unstable library feature 'rustc_private' (see issue #27812)
    --> /Users/imperio/.cargo/registry/src/github.com-1ecc6299db9ec823/html5ever-0.13.1/src/tokenizer/mod.rs:1331:9
     |
1331 |         debug!("processing EOF in state {:?}", self.state);
     |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
     = help: add #![feature(rustc_private)] to the crate attributes to enable
     = note: this error originates in a macro outside of the current crate

error: use of unstable library feature 'rustc_private' (see issue #27812)
    --> /Users/imperio/.cargo/registry/src/github.com-1ecc6299db9ec823/html5ever-0.13.1/src/tokenizer/mod.rs:1331:9
     |
1331 |         debug!("processing EOF in state {:?}", self.state);
     |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
     = help: add #![feature(rustc_private)] to the crate attributes to enable
     = note: this error originates in a macro outside of the current crate

error: use of unstable library feature 'rustc_private' (see issue #27812)
    --> /Users/imperio/.cargo/registry/src/github.com-1ecc6299db9ec823/html5ever-0.13.1/src/tokenizer/mod.rs:1331:9
     |
1331 |         debug!("processing EOF in state {:?}", self.state);
     |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
     = help: add #![feature(rustc_private)] to the crate attributes to enable
     = note: this error originates in a macro outside of the current crate

error: use of unstable library feature 'rustc_private' (see issue #27812)
   --> /Users/imperio/.cargo/registry/src/github.com-1ecc6299db9ec823/html5ever-0.13.1/src/tree_builder/actions.rs:170:9
    |
170 |         warn!("stop_parsing not implemented, full speed ahead!");
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: add #![feature(rustc_private)] to the crate attributes to enable
    = note: this error originates in a macro outside of the current crate

error: use of unstable library feature 'rustc_private' (see issue #27812)
   --> /Users/imperio/.cargo/registry/src/github.com-1ecc6299db9ec823/html5ever-0.13.1/src/tree_builder/actions.rs:170:9
    |
170 |         warn!("stop_parsing not implemented, full speed ahead!");
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: add #![feature(rustc_private)] to the crate attributes to enable
    = note: this error originates in a macro outside of the current crate

error: use of unstable library feature 'rustc_private' (see issue #27812)
   --> /Users/imperio/.cargo/registry/src/github.com-1ecc6299db9ec823/html5ever-0.13.1/src/tree_builder/actions.rs:170:9
    |
170 |         warn!("stop_parsing not implemented, full speed ahead!");
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: add #![feature(rustc_private)] to the crate attributes to enable
    = note: this error originates in a macro outside of the current crate

error: use of unstable library feature 'rustc_private' (see issue #27812)
   --> /Users/imperio/.cargo/registry/src/github.com-1ecc6299db9ec823/html5ever-0.13.1/src/tree_builder/actions.rs:672:9
    |
672 |         warn!("foster parenting not implemented");
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: add #![feature(rustc_private)] to the crate attributes to enable
    = note: this error originates in a macro outside of the current crate

error: use of unstable library feature 'rustc_private' (see issue #27812)
   --> /Users/imperio/.cargo/registry/src/github.com-1ecc6299db9ec823/html5ever-0.13.1/src/tree_builder/actions.rs:672:9
    |
672 |         warn!("foster parenting not implemented");
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: add #![feature(rustc_private)] to the crate attributes to enable
    = note: this error originates in a macro outside of the current crate

error: use of unstable library feature 'rustc_private' (see issue #27812)
   --> /Users/imperio/.cargo/registry/src/github.com-1ecc6299db9ec823/html5ever-0.13.1/src/tree_builder/actions.rs:672:9
    |
672 |         warn!("foster parenting not implemented");
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: add #![feature(rustc_private)] to the crate attributes to enable
    = note: this error originates in a macro outside of the current crate

error: use of unstable library feature 'rustc_private' (see issue #27812)
   --> /Users/imperio/.cargo/registry/src/github.com-1ecc6299db9ec823/html5ever-0.13.1/src/tree_builder/mod.rs:308:9
    |
308 |         debug!("processing {} in insertion mode {:?}", to_escaped_string(token), mode);
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: add #![feature(rustc_private)] to the crate attributes to enable
    = note: this error originates in a macro outside of the current crate

error: use of unstable library feature 'rustc_private' (see issue #27812)
   --> /Users/imperio/.cargo/registry/src/github.com-1ecc6299db9ec823/html5ever-0.13.1/src/tree_builder/mod.rs:308:9
    |
308 |         debug!("processing {} in insertion mode {:?}", to_escaped_string(token), mode);
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: add #![feature(rustc_private)] to the crate attributes to enable
    = note: this error originates in a macro outside of the current crate

error: use of unstable library feature 'rustc_private' (see issue #27812)
   --> /Users/imperio/.cargo/registry/src/github.com-1ecc6299db9ec823/html5ever-0.13.1/src/tree_builder/mod.rs:308:9
    |
308 |         debug!("processing {} in insertion mode {:?}", to_escaped_string(token), mode);
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: add #![feature(rustc_private)] to the crate attributes to enable
    = note: this error originates in a macro outside of the current crate

error: aborting due to 30 previous errors

error: Could not compile `html5ever`.
Build failed, waiting for other jobs to finish...
error: build failed


command did not execute successfully: "/Users/imperio/rust/rust/build/x86_64-apple-darwin/stage0/bin/cargo" "build" "-j" "8" "--target" "x86_64-apple-darwin" "--release" "--features" " jemalloc" "--manifest-path" "/Users/imperio/rust/rust/src/rustc/Cargo.toml"
expected success, got: exit code: 101


Build completed unsuccessfully in 0:00:17

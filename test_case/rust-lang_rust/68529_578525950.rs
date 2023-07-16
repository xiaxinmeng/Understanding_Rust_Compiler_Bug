plain
2020-01-26T16:41:02.8212687Z ---- /checkout/src/doc/nomicon/src/destructors.md - Destructors (line 128) stdout ----
2020-01-26T16:41:02.8212979Z error[E0432]: unresolved import `std::alloc::Alloc`
2020-01-26T16:41:02.8213302Z  --> /checkout/src/doc/nomicon/src/destructors.md:131:18
2020-01-26T16:41:02.8213388Z   |
2020-01-26T16:41:02.8213567Z 4 | use std::alloc::{Alloc, GlobalAlloc, Global, Layout};
2020-01-26T16:41:02.8213754Z   |                  |
2020-01-26T16:41:02.8213833Z   |                  no `Alloc` in `alloc`
2020-01-26T16:41:02.8213940Z   |                  help: a similar name exists in the module: `alloc`
2020-01-26T16:41:02.8214001Z 
2020-01-26T16:41:02.8214001Z 
2020-01-26T16:41:02.8214106Z error[E0599]: no method named `dealloc` found for struct `std::alloc::Global` in the current scope
2020-01-26T16:41:02.8214419Z   --> /checkout/src/doc/nomicon/src/destructors.md:142:20
2020-01-26T16:41:02.8214518Z    |
2020-01-26T16:41:02.8214599Z 15 |             Global.dealloc(c.cast(), Layout::new::<T>());
2020-01-26T16:41:02.8214823Z    |
2020-01-26T16:41:02.8214906Z    = help: items from traits can only be used if the trait is in scope
2020-01-26T16:41:02.8214906Z    = help: items from traits can only be used if the trait is in scope
2020-01-26T16:41:02.8215035Z help: the following trait is implemented but not in scope; perhaps add a `use` for it:
2020-01-26T16:41:02.8215212Z 4  | use std::alloc::AllocRef;
2020-01-26T16:41:02.8215287Z    |
2020-01-26T16:41:02.8215516Z 
2020-01-26T16:41:02.8215615Z error[E0599]: no method named `dealloc` found for struct `std::alloc::Global` in the current scope
2020-01-26T16:41:02.8215615Z error[E0599]: no method named `dealloc` found for struct `std::alloc::Global` in the current scope
2020-01-26T16:41:02.8216106Z   --> /checkout/src/doc/nomicon/src/destructors.md:157:20
2020-01-26T16:41:02.8216194Z    |
2020-01-26T16:41:02.8216289Z 30 |             Global.dealloc(c.cast(), Layout::new::<T>());
2020-01-26T16:41:02.8216489Z    |
2020-01-26T16:41:02.8216572Z    = help: items from traits can only be used if the trait is in scope
2020-01-26T16:41:02.8216572Z    = help: items from traits can only be used if the trait is in scope
2020-01-26T16:41:02.8216702Z help: the following trait is implemented but not in scope; perhaps add a `use` for it:
2020-01-26T16:41:02.8216877Z 4  | use std::alloc::AllocRef;
2020-01-26T16:41:02.8216966Z    |
2020-01-26T16:41:02.8217007Z 
2020-01-26T16:41:02.8217086Z error: aborting due to 3 previous errors
---
2020-01-26T16:41:02.8218115Z ---- /checkout/src/doc/nomicon/src/destructors.md - Destructors (line 28) stdout ----
2020-01-26T16:41:02.8218218Z error[E0432]: unresolved import `std::alloc::Alloc`
2020-01-26T16:41:02.8218513Z  --> /checkout/src/doc/nomicon/src/destructors.md:31:18
2020-01-26T16:41:02.8218609Z   |
2020-01-26T16:41:02.8218688Z 4 | use std::alloc::{Alloc, Global, GlobalAlloc, Layout};
2020-01-26T16:41:02.8218871Z   |                  |
2020-01-26T16:41:02.8218962Z   |                  no `Alloc` in `alloc`
2020-01-26T16:41:02.8219053Z   |                  help: a similar name exists in the module: `alloc`
2020-01-26T16:41:02.8219134Z 
2020-01-26T16:41:02.8219134Z 
2020-01-26T16:41:02.8219225Z error[E0599]: no method named `dealloc` found for struct `std::alloc::Global` in the current scope
2020-01-26T16:41:02.8219530Z   --> /checkout/src/doc/nomicon/src/destructors.md:42:20
2020-01-26T16:41:02.8220047Z    |
2020-01-26T16:41:02.8220142Z 15 |             Global.dealloc(c.cast(), Layout::new::<T>())
2020-01-26T16:41:02.8220336Z    |
2020-01-26T16:41:02.8220417Z    = help: items from traits can only be used if the trait is in scope
2020-01-26T16:41:02.8220417Z    = help: items from traits can only be used if the trait is in scope
2020-01-26T16:41:02.8220536Z help: the following trait is implemented but not in scope; perhaps add a `use` for it:
2020-01-26T16:41:02.8220793Z 4  | use std::alloc::AllocRef;
2020-01-26T16:41:02.8220883Z    |
2020-01-26T16:41:02.8220922Z 
2020-01-26T16:41:02.8220993Z error: aborting due to 2 previous errors
---
2020-01-26T16:41:02.8222080Z ---- /checkout/src/doc/nomicon/src/destructors.md - Destructors (line 55) stdout ----
2020-01-26T16:41:02.8222181Z error[E0432]: unresolved import `std::alloc::Alloc`
2020-01-26T16:41:02.8222475Z  --> /checkout/src/doc/nomicon/src/destructors.md:58:18
2020-01-26T16:41:02.8222571Z   |
2020-01-26T16:41:02.8222649Z 4 | use std::alloc::{Alloc, Global, GlobalAlloc, Layout};
2020-01-26T16:41:02.8222824Z   |                  |
2020-01-26T16:41:02.8222916Z   |                  no `Alloc` in `alloc`
2020-01-26T16:41:02.8223017Z   |                  help: a similar name exists in the module: `alloc`
2020-01-26T16:41:02.8223091Z 
2020-01-26T16:41:02.8223091Z 
2020-01-26T16:41:02.8223182Z error[E0599]: no method named `dealloc` found for struct `std::alloc::Global` in the current scope
2020-01-26T16:41:02.8223497Z   --> /checkout/src/doc/nomicon/src/destructors.md:69:20
2020-01-26T16:41:02.8223581Z    |
2020-01-26T16:41:02.8223673Z 15 |             Global.dealloc(c.cast(), Layout::new::<T>());
2020-01-26T16:41:02.8223872Z    |
2020-01-26T16:41:02.8223952Z    = help: items from traits can only be used if the trait is in scope
2020-01-26T16:41:02.8223952Z    = help: items from traits can only be used if the trait is in scope
2020-01-26T16:41:02.8224072Z help: the following trait is implemented but not in scope; perhaps add a `use` for it:
2020-01-26T16:41:02.8224245Z 4  | use std::alloc::AllocRef;
2020-01-26T16:41:02.8224333Z    |
2020-01-26T16:41:02.8224373Z 
2020-01-26T16:41:02.8224461Z error[E0599]: no method named `dealloc` found for struct `std::alloc::Global` in the current scope
2020-01-26T16:41:02.8224461Z error[E0599]: no method named `dealloc` found for struct `std::alloc::Global` in the current scope
2020-01-26T16:41:02.8224776Z   --> /checkout/src/doc/nomicon/src/destructors.md:82:20
2020-01-26T16:41:02.8224872Z    |
2020-01-26T16:41:02.8224962Z 28 |             Global.dealloc(c.cast::<u8>(), Layout::new::<T>());
2020-01-26T16:41:02.8225159Z    |
2020-01-26T16:41:02.8225253Z    = help: items from traits can only be used if the trait is in scope
2020-01-26T16:41:02.8225253Z    = help: items from traits can only be used if the trait is in scope
2020-01-26T16:41:02.8225359Z help: the following trait is implemented but not in scope; perhaps add a `use` for it:
2020-01-26T16:41:02.8225530Z 4  | use std::alloc::AllocRef;
2020-01-26T16:41:02.8225619Z    |
2020-01-26T16:41:02.8225659Z 
2020-01-26T16:41:02.8225744Z error: aborting due to 3 previous errors
---
2020-01-26T16:41:08.9396641Z ---- /checkout/src/doc/nomicon/src/vec-final.md - The_Final_Code (line 3) stdout ----
2020-01-26T16:41:08.9396748Z error[E0432]: unresolved import `std::alloc::Alloc`
2020-01-26T16:41:08.9397075Z   --> /checkout/src/doc/nomicon/src/vec-final.md:12:18
2020-01-26T16:41:08.9397161Z    |
2020-01-26T16:41:08.9397268Z 10 | use std::alloc::{Alloc, GlobalAlloc, Layout, Global, handle_alloc_error};
2020-01-26T16:41:08.9397457Z    |                  |
2020-01-26T16:41:08.9397553Z    |                  no `Alloc` in `alloc`
2020-01-26T16:41:08.9397649Z    |                  help: a similar name exists in the module: `alloc`
2020-01-26T16:41:08.9397728Z 
2020-01-26T16:41:08.9397728Z 
2020-01-26T16:41:08.9397820Z error[E0599]: no method named `alloc` found for struct `std::alloc::Global` in the current scope
2020-01-26T16:41:08.9398136Z   --> /checkout/src/doc/nomicon/src/vec-final.md:37:34
2020-01-26T16:41:08.9398220Z    |
2020-01-26T16:41:08.9398331Z 35 |                 let ptr = Global.alloc(Layout::array::<T>(1).unwrap());
2020-01-26T16:41:08.9398542Z    |
2020-01-26T16:41:08.9398633Z    = help: items from traits can only be used if the trait is in scope
2020-01-26T16:41:08.9398633Z    = help: items from traits can only be used if the trait is in scope
2020-01-26T16:41:08.9398759Z help: the following trait is implemented but not in scope; perhaps add a `use` for it:
2020-01-26T16:41:08.9398938Z 6  | use std::alloc::AllocRef;
2020-01-26T16:41:08.9399028Z    |
2020-01-26T16:41:08.9399069Z 
2020-01-26T16:41:08.9399159Z error[E0599]: no method named `realloc` found for struct `std::alloc::Global` in the current scope
2020-01-26T16:41:08.9399159Z error[E0599]: no method named `realloc` found for struct `std::alloc::Global` in the current scope
2020-01-26T16:41:08.9399470Z   --> /checkout/src/doc/nomicon/src/vec-final.md:42:34
2020-01-26T16:41:08.9399553Z    |
2020-01-26T16:41:08.9399648Z 40 |                 let ptr = Global.realloc(c.cast(),
2020-01-26T16:41:08.9400022Z    |
2020-01-26T16:41:08.9400120Z    = help: items from traits can only be used if the trait is in scope
2020-01-26T16:41:08.9400120Z    = help: items from traits can only be used if the trait is in scope
2020-01-26T16:41:08.9400510Z help: the following trait is implemented but not in scope; perhaps add a `use` for it:
2020-01-26T16:41:08.9400690Z 6  | use std::alloc::AllocRef;
2020-01-26T16:41:08.9400783Z    |
2020-01-26T16:41:08.9400823Z 
2020-01-26T16:41:08.9400929Z error[E0599]: no method named `dealloc` found for struct `std::alloc::Global` in the current scope
2020-01-26T16:41:08.9400929Z error[E0599]: no method named `dealloc` found for struct `std::alloc::Global` in the current scope
2020-01-26T16:41:08.9401244Z   --> /checkout/src/doc/nomicon/src/vec-final.md:69:24
2020-01-26T16:41:08.9401343Z    |
2020-01-26T16:41:08.9401418Z 67 |                 Global.dealloc(c.cast(),
2020-01-26T16:41:08.9401616Z    |
2020-01-26T16:41:08.9401713Z    = help: items from traits can only be used if the trait is in scope
2020-01-26T16:41:08.9401713Z    = help: items from traits can only be used if the trait is in scope
2020-01-26T16:41:08.9401841Z help: the following trait is implemented but not in scope; perhaps add a `use` for it:
2020-01-26T16:41:08.9402015Z 6  | use std::alloc::AllocRef;
2020-01-26T16:41:08.9402090Z    |
2020-01-26T16:41:08.9402129Z 
2020-01-26T16:41:08.9402224Z error: aborting due to 4 previous errors
---
2020-01-26T16:54:33.4842792Z error: The server responded with 404 Not Found for "https://blog.rust-lang.org/2016/04/19/MIR.html"
2020-01-26T16:54:33.4842963Z 
2020-01-26T16:54:33.4843342Z     ┌── mir/index.md:12:1 ───
2020-01-26T16:54:33.4843695Z     │
2020-01-26T16:54:33.4844105Z  12 │ [rust-lang blog post that introduced MIR][blog].
2020-01-26T16:54:33.4844845Z     │
2020-01-26T16:54:33.4845005Z 
2020-01-26T16:54:33.4851995Z Error: One or more incorrect links
2020-01-26T16:54:33.4852323Z 
---
2020-01-26T17:49:14.7525394Z normalized stderr:
2020-01-26T17:49:14.7533829Z error[E0432]: unresolved import `std::alloc::Alloc`
2020-01-26T17:49:14.7534610Z  --> $DIR/heap_allocator.rs:4:26
2020-01-26T17:49:14.7534736Z   |
2020-01-26T17:49:14.7534818Z 4 | use std::alloc::{Global, Alloc, Layout, System};
2020-01-26T17:49:14.7535001Z   |                          |
2020-01-26T17:49:14.7535114Z   |                          no `Alloc` in `alloc`
2020-01-26T17:49:14.7535227Z   |                          help: a similar name exists in the module: `alloc`
2020-01-26T17:49:14.7535297Z 
---
2020-01-26T17:49:14.7550012Z 
2020-01-26T17:49:14.7550143Z diff of stderr:
2020-01-26T17:49:14.7550219Z 
2020-01-26T17:49:14.7550464Z +error[E0432]: unresolved import `std::alloc::Alloc`
2020-01-26T17:49:14.7551088Z + --> $DIR/heap_allocator.rs:4:26
2020-01-26T17:49:14.7553590Z +  |
2020-01-26T17:49:14.7554647Z +4 | use std::alloc::{Global, Alloc, Layout, System};
2020-01-26T17:49:14.7556875Z +  |                          ^^^^^
2020-01-26T17:49:14.7558318Z +  |                          |
2020-01-26T17:49:14.7560796Z +  |                          no `Alloc` in `alloc`
2020-01-26T17:49:14.7561647Z +  |                          help: a similar name exists in the module: `alloc`
2020-01-26T17:49:14.7568792Z +error: aborting due to previous error
2020-01-26T17:49:14.7568935Z +
2020-01-26T17:49:14.7570026Z +For more information about this error, try `rustc --explain E0432`.
2020-01-26T17:49:14.7578010Z +
2020-01-26T17:49:14.7578010Z +
2020-01-26T17:49:14.7578078Z 
2020-01-26T17:49:14.7578161Z The actual stderr differed from the expected stderr.
2020-01-26T17:49:14.7578945Z Actual stderr saved to /tmp/compiletestQrE0Kv/heap_allocator.stderr
2020-01-26T17:49:14.7579053Z To update references, run this command from build directory:
2020-01-26T17:49:14.7580036Z tests/run-pass/update-references.sh '/tmp/compiletestQrE0Kv' 'heap_allocator.rs'
2020-01-26T17:49:14.7580213Z error: 1 errors occurred comparing output.
2020-01-26T17:49:14.7580292Z status: exit code: 1
2020-01-26T17:49:14.7580292Z status: exit code: 1
2020-01-26T17:49:14.7581118Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/heap_allocator.rs" "-L" "/tmp/compiletestQrE0Kv" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestQrE0Kv/heap_allocator.stage-id" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-L" "/tmp/compiletestQrE0Kv/heap_allocator.stage-id.aux" "-A" "unused"
2020-01-26T17:49:14.7581608Z ------------------------------------------
2020-01-26T17:49:14.7581679Z 
2020-01-26T17:49:14.7581932Z ------------------------------------------
2020-01-26T17:49:14.7582027Z stderr:
2020-01-26T17:49:14.7582027Z stderr:
2020-01-26T17:49:14.7582271Z ------------------------------------------
2020-01-26T17:49:14.7585604Z {"message":"unresolved import `std::alloc::Alloc`","code":{"code":"E0432","explanation":"An import was unresolved.\n\nErroneous code example:\n\n
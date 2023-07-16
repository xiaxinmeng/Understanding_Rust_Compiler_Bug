plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between 337181e07d3cd33c0aec2f17c12279bc9afca88f and 73be59597f8519dd2b109784a4b7cfe054ea486e
Clippy or rustfmt subtrees were updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
   Compiling jsonrpc-derive v18.0.0
warning: redundant field names in struct initialization
   --> src/tools/rls/rls-analysis/src/analysis.rs:117:9
    |
117 |     pub column_end: Column<ZeroIndexed>,
    |         ^^^^^^^^^^ help: replace it with: `column_end`
    = note: `#[warn(redundant_field_initializers)]` on by default

warning: redundant field names in struct initialization
   --> src/tools/rls/rls-analysis/src/analysis.rs:118:9
   --> src/tools/rls/rls-analysis/src/analysis.rs:118:9
    |
118 |     pub id: Id,
    |         ^^ help: replace it with: `id`
warning: redundant field names in struct initialization
   --> src/tools/rls/rls-analysis/src/analysis.rs:119:9
    |
    |
119 |     pub kind: IdentKind,
    |         ^^^^ help: replace it with: `kind`
warning: redundant field names in struct initialization
   --> src/tools/rls/rls-analysis/src/analysis.rs:134:9
    |
    |
134 |     pub span: Span,
    |         ^^^^ help: replace it with: `span`
warning: redundant field names in struct initialization
   --> src/tools/rls/rls-analysis/src/analysis.rs:135:9
    |
    |
135 |     pub id: Id,
    |         ^^ help: replace it with: `id`
warning: redundant field names in struct initialization
   --> src/tools/rls/rls-analysis/src/analysis.rs:136:9
    |
    |
136 |     pub kind: IdentKind,
    |         ^^^^ help: replace it with: `kind`
warning: redundant field names in struct initialization
  --> src/tools/rls/rls-analysis/src/loader.rs:22:9
   |
22 |     pub path: PathBuf,
22 |     pub path: PathBuf,
   |         ^^^^ help: replace it with: `path`

warning: redundant field names in struct initialization
  --> src/tools/rls/rls-analysis/src/loader.rs:27:9
   |
27 |     pub prefix_rewrite: Option<PathBuf>,
   |         ^^^^^^^^^^^^^^ help: replace it with: `prefix_rewrite`
   Compiling jsonrpc-server-utils v18.0.0
   Compiling jsonrpc-pubsub v18.0.0
   Compiling jsonrpc-ipc-server v18.0.0
   Compiling jsonrpc-client-transports v18.0.0
---
   Compiling cargo-util v0.1.0 (/checkout/src/tools/cargo/crates/cargo-util)
warning: redundant field names in struct initialization
 --> src/tools/rustfmt/src/attr/doc_comment.rs:7:5
  |
7 |     literal: &'a str,
  |     ^^^^^^^ help: replace it with: `literal`
  = note: `#[warn(redundant_field_initializers)]` on by default

warning: redundant field names in struct initialization
 --> src/tools/rustfmt/src/attr/doc_comment.rs:8:5
 --> src/tools/rustfmt/src/attr/doc_comment.rs:8:5
  |
8 |     style: CommentStyle<'a>,
  |     ^^^^^ help: replace it with: `style`
warning: redundant field names in struct initialization
   --> src/tools/rustfmt/src/formatting.rs:129:5
    |
    |
129 |     krate: &'a ast::Crate,
    |     ^^^^^ help: replace it with: `krate`
warning: redundant field names in struct initialization
   --> src/tools/rustfmt/src/formatting.rs:130:5
    |
130 |     report: FormatReport,
---

warning: redundant field names in struct initialization
   --> src/tools/rustfmt/src/formatting.rs:133:5
    |
133 |     handler: &'a mut T,
    |     ^^^^^^^ help: replace it with: `handler`
warning: redundant field names in struct initialization
    --> src/tools/rustfmt/src/macros.rs:1241:5
     |
     |
1241 |     toks: Cursor,
     |     ^^^^ help: replace it with: `toks`
warning: redundant field names in struct initialization
  --> src/tools/rustfmt/src/pairs.rs:14:5
   |
   |
14 |     prefix: &'a str,
   |     ^^^^^^ help: replace it with: `prefix`
warning: redundant field names in struct initialization
  --> src/tools/rustfmt/src/pairs.rs:15:5
   |
   |
15 |     infix: &'a str,
   |     ^^^^^ help: replace it with: `infix`
warning: redundant field names in struct initialization
  --> src/tools/rustfmt/src/pairs.rs:16:5
   |
   |
16 |     suffix: &'a str,
   |     ^^^^^^ help: replace it with: `suffix`
   Compiling git2 v0.13.17
   Compiling git2-curl v0.14.1
warning: redundant field names in struct initialization
   --> src/tools/cargo/src/cargo/ops/cargo_package.rs:170:17
---
   Compiling lsp-codec v0.3.0
warning: redundant field names in struct initialization
   --> src/tools/rls/rls-analysis/src/analysis.rs:117:9
    |
117 |     pub column_end: Column<ZeroIndexed>,
    |         ^^^^^^^^^^ help: replace it with: `column_end`
    = note: `#[warn(redundant_field_initializers)]` on by default

warning: redundant field names in struct initialization
   --> src/tools/rls/rls-analysis/src/analysis.rs:118:9
   --> src/tools/rls/rls-analysis/src/analysis.rs:118:9
    |
118 |     pub id: Id,
    |         ^^ help: replace it with: `id`
warning: redundant field names in struct initialization
   --> src/tools/rls/rls-analysis/src/analysis.rs:119:9
    |
    |
119 |     pub kind: IdentKind,
    |         ^^^^ help: replace it with: `kind`
warning: redundant field names in struct initialization
   --> src/tools/rls/rls-analysis/src/analysis.rs:134:9
    |
    |
134 |     pub span: Span,
    |         ^^^^ help: replace it with: `span`
warning: redundant field names in struct initialization
   --> src/tools/rls/rls-analysis/src/analysis.rs:135:9
    |
    |
135 |     pub id: Id,
    |         ^^ help: replace it with: `id`
warning: redundant field names in struct initialization
   --> src/tools/rls/rls-analysis/src/analysis.rs:136:9
    |
    |
136 |     pub kind: IdentKind,
    |         ^^^^ help: replace it with: `kind`
warning: redundant field names in struct initialization
  --> src/tools/rls/rls-analysis/src/loader.rs:22:9
   |
22 |     pub path: PathBuf,
22 |     pub path: PathBuf,
   |         ^^^^ help: replace it with: `path`

warning: redundant field names in struct initialization
  --> src/tools/rls/rls-analysis/src/loader.rs:27:9
   |
27 |     pub prefix_rewrite: Option<PathBuf>,
   |         ^^^^^^^^^^^^^^ help: replace it with: `prefix_rewrite`
warning: 8 warnings emitted

warning: redundant field names in struct initialization
 --> src/tools/rustfmt/src/attr/doc_comment.rs:7:5
 --> src/tools/rustfmt/src/attr/doc_comment.rs:7:5
  |
7 |     literal: &'a str,
  |     ^^^^^^^ help: replace it with: `literal`
  = note: `#[warn(redundant_field_initializers)]` on by default

warning: redundant field names in struct initialization
 --> src/tools/rustfmt/src/attr/doc_comment.rs:8:5
 --> src/tools/rustfmt/src/attr/doc_comment.rs:8:5
  |
8 |     style: CommentStyle<'a>,
  |     ^^^^^ help: replace it with: `style`
warning: redundant field names in struct initialization
   --> src/tools/rustfmt/src/formatting.rs:129:5
    |
    |
129 |     krate: &'a ast::Crate,
    |     ^^^^^ help: replace it with: `krate`
warning: redundant field names in struct initialization
   --> src/tools/rustfmt/src/formatting.rs:130:5
    |
130 |     report: FormatReport,
---

warning: redundant field names in struct initialization
   --> src/tools/rustfmt/src/formatting.rs:133:5
    |
133 |     handler: &'a mut T,
    |     ^^^^^^^ help: replace it with: `handler`
warning: redundant field names in struct initialization
    --> src/tools/rustfmt/src/macros.rs:1241:5
     |
     |
1241 |     toks: Cursor,
     |     ^^^^ help: replace it with: `toks`
warning: redundant field names in struct initialization
  --> src/tools/rustfmt/src/pairs.rs:14:5
   |
   |
14 |     prefix: &'a str,
   |     ^^^^^^ help: replace it with: `prefix`
warning: redundant field names in struct initialization
  --> src/tools/rustfmt/src/pairs.rs:15:5
   |
   |
15 |     infix: &'a str,
   |     ^^^^^ help: replace it with: `infix`
warning: redundant field names in struct initialization
  --> src/tools/rustfmt/src/pairs.rs:16:5
   |
   |
16 |     suffix: &'a str,
   |     ^^^^^^ help: replace it with: `suffix`
warning: 11 warnings emitted

warning: redundant field names in struct initialization
   --> src/tools/cargo/src/cargo/ops/cargo_package.rs:170:17
---
   Compiling miri v0.1.0 (/checkout/src/tools/miri)
warning: redundant field names in struct initialization
   --> src/tools/miri/src/helpers.rs:118:34
    |
118 |         let place = mir::Place { local: local, projection: List::empty() };
    |                                  ^^^^^^^^^^^^ help: replace it with: `local`
    = note: `#[warn(redundant_field_initializers)]` on by default

warning: redundant field names in struct initialization
   --> src/tools/miri/src/thread.rs:238:13
---
   Compiling term v0.7.0
warning: redundant field names in struct initialization
   --> src/tools/miri/src/helpers.rs:118:34
    |
118 |         let place = mir::Place { local: local, projection: List::empty() };
    |                                  ^^^^^^^^^^^^ help: replace it with: `local`
    = note: `#[warn(redundant_field_initializers)]` on by default

warning: redundant field names in struct initialization
   --> src/tools/miri/src/thread.rs:238:13
---
   Compiling tester v0.9.0
warning: redundant field names in struct initialization
   --> src/tools/miri/src/helpers.rs:118:34
    |
118 |         let place = mir::Place { local: local, projection: List::empty() };
    |                                  ^^^^^^^^^^^^ help: replace it with: `local`
    = note: `#[warn(redundant_field_initializers)]` on by default

warning: redundant field names in struct initialization
   --> src/tools/miri/src/thread.rs:238:13
---



The actual stderr differed from the expected stderr.
Actual stderr saved to /tmp/compiletestFps1NX/binops.stage-id.stderr
To only update this specific test, also pass `--test-args binops.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/binops.rs" "-L" "/tmp/compiletestFps1NX" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestFps1NX/binops.stage-id" "-A" "unused" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-L" "/tmp/compiletestFps1NX/binops.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"redundant field names in struct initialization","code":{"code":"redundant_field_initializers","explanation":null},"level":"warning","spans":[{"file_name":"tests/run-pass/binops.rs","byte_start":1385,"byte_end":1389,"line_start":60,"line_end":60,"column_start":9,"column_end":13,"is_primary":true,"text":[{"text":"        x: x,","highlight_start":9,"highlight_end":13}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`#[warn(redundant_field_initializers)]` on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"replace it with","code":null,"level":"help","spans":[{"file_name":"tests/run-pass/binops.rs","byte_start":1385,"byte_end":1389,"line_start":60,"line_end":60,"column_start":9,"column_end":13,"is_primary":true,"text":[{"text":"        x: x,","highlight_start":9,"highlight_end":13}],"label":null,"suggested_replacement":"x","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"warning: redundant field names in struct initialization\n  --> tests/run-pass/binops.rs:60:9\n   |\n60 |         x: x,\n   |         ^^^^ help: replace it with: `x`\n   |\n   = note: `#[warn(redundant_field_initializers)]` on by default\n\n"}
{"message":"redundant field names in struct initialization","code":{"code":"redundant_field_initializers","explanation":null},"level":"warning","spans":[{"file_name":"tests/run-pass/binops.rs","byte_start":1399,"byte_end":1403,"line_start":61,"line_end":61,"column_start":9,"column_end":13,"is_primary":true,"text":[{"text":"        y: y","highlight_start":9,"highlight_end":13}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"replace it with","code":null,"level":"help","spans":[{"file_name":"tests/run-pass/binops.rs","byte_start":1399,"byte_end":1403,"line_start":61,"line_end":61,"column_start":9,"column_end":13,"is_primary":true,"text":[{"text":"        y: y","highlight_start":9,"highlight_end":13}],"label":null,"suggested_replacement":"y","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"warning: redundant field names in struct initialization\n  --> tests/run-pass/binops.rs:61:9\n   |\n61 |         y: y\n   |         ^^^^ help: replace it with: `y`\n\n"}
------------------------------------------

test [ui] run-pass/binops.rs ... FAILED
test [ui] run-pass/arrays.rs ... ok
---
normalized stderr:
warning: redundant field names in struct initialization
  --> $DIR/dyn-traits.rs:61:24
   |
61 |             Whatever { w: w }
   |                        ^^^^ help: replace it with: `w`
   = note: `#[warn(redundant_field_initializers)]` on by default





The actual stderr differed from the expected stderr.
Actual stderr saved to /tmp/compiletestFps1NX/dyn-traits.stage-id.stderr
To only update this specific test, also pass `--test-args dyn-traits.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/dyn-traits.rs" "-L" "/tmp/compiletestFps1NX" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestFps1NX/dyn-traits.stage-id" "-A" "unused" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-L" "/tmp/compiletestFps1NX/dyn-traits.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"redundant field names in struct initialization","code":{"code":"redundant_field_initializers","explanation":null},"level":"warning","spans":[{"file_name":"tests/run-pass/dyn-traits.rs","byte_start":1215,"byte_end":1219,"line_start":61,"line_end":61,"column_start":24,"column_end":28,"is_primary":true,"text":[{"text":"            Whatever { w: w }","highlight_start":24,"highlight_end":28}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`#[warn(redundant_field_initializers)]` on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"replace it with","code":null,"level":"help","spans":[{"file_name":"tests/run-pass/dyn-traits.rs","byte_start":1215,"byte_end":1219,"line_start":61,"line_end":61,"column_start":24,"column_end":28,"is_primary":true,"text":[{"text":"            Whatever { w: w }","highlight_start":24,"highlight_end":28}],"label":null,"suggested_replacement":"w","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"warning: redundant field names in struct initialization\n  --> tests/run-pass/dyn-traits.rs:61:24\n   |\n61 |             Whatever { w: w }\n   |                        ^^^^ help: replace it with: `w`\n   |\n   = note: `#[warn(redundant_field_initializers)]` on by default\n\n"}
------------------------------------------

test [ui] run-pass/dyn-traits.rs ... FAILED
test [ui] run-pass/fat_ptr.rs ... ok
---
normalized stderr:
warning: redundant field names in struct initialization
  --> $DIR/regions-mock-trans.rs:38:21
   |
38 |     let bcx = Bcx { fcx: fcx };
   |                     ^^^^^^^^ help: replace it with: `fcx`
   = note: `#[warn(redundant_field_initializers)]` on by default

warning: redundant field names in struct initialization
  --> $DIR/regions-mock-trans.rs:47:32
  --> $DIR/regions-mock-trans.rs:47:32
   |
47 |     let fcx = Fcx { arena: &a, ccx: ccx };
   |                                ^^^^^^^^ help: replace it with: `ccx`



The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /tmp/compiletestFps1NX/regions-mock-trans.stage-id.stderr
To only update this specific test, also pass `--test-args regions-mock-trans.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/regions-mock-trans.rs" "-L" "/tmp/compiletestFps1NX" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestFps1NX/regions-mock-trans.stage-id" "-A" "unused" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-L" "/tmp/compiletestFps1NX/regions-mock-trans.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"redundant field names in struct initialization","code":{"code":"redundant_field_initializers","explanation":null},"level":"warning","spans":[{"file_name":"tests/run-pass/regions-mock-trans.rs","byte_start":576,"byte_end":584,"line_start":38,"line_end":38,"column_start":21,"column_end":29,"is_primary":true,"text":[{"text":"    let bcx = Bcx { fcx: fcx };","highlight_start":21,"highlight_end":29}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`#[warn(redundant_field_initializers)]` on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"replace it with","code":null,"level":"help","spans":[{"file_name":"tests/run-pass/regions-mock-trans.rs","byte_start":576,"byte_end":584,"line_start":38,"line_end":38,"column_start":21,"column_end":29,"is_primary":true,"text":[{"text":"    let bcx = Bcx { fcx: fcx };","highlight_start":21,"highlight_end":29}],"label":null,"suggested_replacement":"fcx","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"warning: redundant field names in struct initialization\n  --> tests/run-pass/regions-mock-trans.rs:38:21\n   |\n38 |     let bcx = Bcx { fcx: fcx };\n   |                     ^^^^^^^^ help: replace it with: `fcx`\n   |\n   = note: `#[warn(redundant_field_initializers)]` on by default\n\n"}
{"message":"redundant field names in struct initialization","code":{"code":"redundant_field_initializers","explanation":null},"level":"warning","spans":[{"file_name":"tests/run-pass/regions-mock-trans.rs","byte_start":749,"byte_end":757,"line_start":47,"line_end":47,"column_start":32,"column_end":40,"is_primary":true,"text":[{"text":"    let fcx = Fcx { arena: &a, ccx: ccx };","highlight_start":32,"highlight_end":40}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"replace it with","code":null,"level":"help","spans":[{"file_name":"tests/run-pass/regions-mock-trans.rs","byte_start":749,"byte_end":757,"line_start":47,"line_end":47,"column_start":32,"column_end":40,"is_primary":true,"text":[{"text":"    let fcx = Fcx { arena: &a, ccx: ccx };","highlight_start":32,"highlight_end":40}],"label":null,"suggested_replacement":"ccx","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"warning: redundant field names in struct initialization\n  --> tests/run-pass/regions-mock-trans.rs:47:32\n   |\n47 |     let fcx = Fcx { arena: &a, ccx: ccx };\n   |                                ^^^^^^^^ help: replace it with: `ccx`\n\n"}
------------------------------------------

test [ui] run-pass/regions-mock-trans.rs ... FAILED
test [ui] run-pass/rfc1623.rs ... ok
---

test [ui] run-pass/rust-lang-org.rs ... ok

The actual stderr differed from the expected stderr.
Actual stderr saved to /tmp/compiletestFps1NX/sendable-class.stage-id.stderr
To only update this specific test, also pass `--test-args sendable-class.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/sendable-class.rs" "-L" "/tmp/compiletestFps1NX" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestFps1NX/sendable-class.stage-id" "-A" "unused" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-L" "/tmp/compiletestFps1NX/sendable-class.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"redundant field names in struct initialization","code":{"code":"redundant_field_initializers","explanation":null},"level":"warning","spans":[{"file_name":"tests/run-pass/sendable-class.rs","byte_start":206,"byte_end":210,"line_start":13,"line_end":13,"column_start":9,"column_end":13,"is_primary":true,"text":[{"text":"        i: i,","highlight_start":9,"highlight_end":13}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`#[warn(redundant_field_initializers)]` on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"replace it with","code":null,"level":"help","spans":[{"file_name":"tests/run-pass/sendable-class.rs","byte_start":206,"byte_end":210,"line_start":13,"line_end":13,"column_start":9,"column_end":13,"is_primary":true,"text":[{"text":"        i: i,","highlight_start":9,"highlight_end":13}],"label":null,"suggested_replacement":"i","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"warning: redundant field names in struct initialization\n  --> tests/run-pass/sendable-class.rs:13:9\n   |\n13 |         i: i,\n   |         ^^^^ help: replace it with: `i`\n   |\n   = note: `#[warn(redundant_field_initializers)]` on by default\n\n"}
{"message":"redundant field names in struct initialization","code":{"code":"redundant_field_initializers","explanation":null},"level":"warning","spans":[{"file_name":"tests/run-pass/sendable-class.rs","byte_start":220,"byte_end":224,"line_start":14,"line_end":14,"column_start":9,"column_end":13,"is_primary":true,"text":[{"text":"        j: j","highlight_start":9,"highlight_end":13}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"replace it with","code":null,"level":"help","spans":[{"file_name":"tests/run-pass/sendable-class.rs","byte_start":220,"byte_end":224,"line_start":14,"line_end":14,"column_start":9,"column_end":13,"is_primary":true,"text":[{"text":"        j: j","highlight_start":9,"highlight_end":13}],"label":null,"suggested_replacement":"j","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"warning: redundant field names in struct initialization\n  --> tests/run-pass/sendable-class.rs:14:9\n   |\n14 |         j: j\n   |         ^^^^ help: replace it with: `j`\n\n"}
------------------------------------------

test [ui] run-pass/sendable-class.rs ... FAILED
test [ui] run-pass/rc.rs ... ok
---
   Compiling rustfmt-nightly v1.4.37 (/checkout/src/tools/rustfmt)
error: redundant field names in struct initialization
 --> src/tools/rustfmt/src/attr/doc_comment.rs:7:5
  |
7 |     literal: &'a str,
  |     ^^^^^^^ help: replace it with: `literal`
  |
  = note: `-D redundant-field-initializers` implied by `-D warnings`
error: redundant field names in struct initialization
 --> src/tools/rustfmt/src/attr/doc_comment.rs:8:5
  |
  |
8 |     style: CommentStyle<'a>,
  |     ^^^^^ help: replace it with: `style`
error: redundant field names in struct initialization
   --> src/tools/rustfmt/src/formatting.rs:129:5
    |
    |
129 |     krate: &'a ast::Crate,
    |     ^^^^^ help: replace it with: `krate`
error: redundant field names in struct initialization
   --> src/tools/rustfmt/src/formatting.rs:130:5
    |
130 |     report: FormatReport,
---

error: redundant field names in struct initialization
   --> src/tools/rustfmt/src/formatting.rs:133:5
    |
133 |     handler: &'a mut T,
    |     ^^^^^^^ help: replace it with: `handler`
error: redundant field names in struct initialization
    --> src/tools/rustfmt/src/macros.rs:1241:5
     |
     |
1241 |     toks: Cursor,
     |     ^^^^ help: replace it with: `toks`
error: redundant field names in struct initialization
  --> src/tools/rustfmt/src/pairs.rs:14:5
   |
   |
14 |     prefix: &'a str,
   |     ^^^^^^ help: replace it with: `prefix`
error: redundant field names in struct initialization
  --> src/tools/rustfmt/src/pairs.rs:15:5
   |
   |
15 |     infix: &'a str,
   |     ^^^^^ help: replace it with: `infix`
error: redundant field names in struct initialization
  --> src/tools/rustfmt/src/pairs.rs:16:5
   |
   |
16 |     suffix: &'a str,
   |     ^^^^^^ help: replace it with: `suffix`
error: aborting due to 11 previous errors

error: could not compile `rustfmt-nightly`


To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
thread 'main' panicked at 'in-tree tool', src/bootstrap/test.rs:384:14
Build completed unsuccessfully in 0:00:11

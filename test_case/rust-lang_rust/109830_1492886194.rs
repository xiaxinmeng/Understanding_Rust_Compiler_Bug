plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:8f4b7f84864484a7bf31766abe9204da3cbe65b3)
Download action repository 'rust-lang/simpleinfra@master' (SHA:3040325909b538d8ad81ad89a04b7a91b109c313)
Complete job name: PR (x86_64-gnu-tools, false, ubuntu-20.04-16core-64gb)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-tools
---
   Compiling memchr v2.5.0
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.89
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error: attributes starting with `rustc` are reserved for use by the `rustc` compiler
  |
  |
4 | #[rustc_doc_primitive = "bool"]


error: attributes starting with `rustc` are reserved for use by the `rustc` compiler
   |
   |
66 | #[rustc_doc_primitive = "never"]


error: attributes starting with `rustc` are reserved for use by the `rustc` compiler
    |
    |
277 | #[rustc_doc_primitive = "char"]


error: attributes starting with `rustc` are reserved for use by the `rustc` compiler
    |
    |
401 | #[rustc_doc_primitive = "unit"]


error: attributes starting with `rustc` are reserved for use by the `rustc` compiler
    |
    |
463 | #[rustc_doc_primitive = "pointer"]


error: attributes starting with `rustc` are reserved for use by the `rustc` compiler
    |
    |
580 | #[rustc_doc_primitive = "array"]


error: attributes starting with `rustc` are reserved for use by the `rustc` compiler
    |
    |
781 | #[rustc_doc_primitive = "slice"]


error: attributes starting with `rustc` are reserved for use by the `rustc` compiler
    |
    |
873 | #[rustc_doc_primitive = "str"]


error: attributes starting with `rustc` are reserved for use by the `rustc` compiler
    |
    |
940 | #[rustc_doc_primitive = "tuple"]


error: attributes starting with `rustc` are reserved for use by the `rustc` compiler
     |
     |
1084 | #[rustc_doc_primitive = "f32"]


error: attributes starting with `rustc` are reserved for use by the `rustc` compiler
     |
     |
1150 | #[rustc_doc_primitive = "f64"]


error: attributes starting with `rustc` are reserved for use by the `rustc` compiler
     |
     |
1165 | #[rustc_doc_primitive = "i8"]


error: attributes starting with `rustc` are reserved for use by the `rustc` compiler
     |
     |
1171 | #[rustc_doc_primitive = "i16"]


error: attributes starting with `rustc` are reserved for use by the `rustc` compiler
     |
     |
1177 | #[rustc_doc_primitive = "i32"]


error: attributes starting with `rustc` are reserved for use by the `rustc` compiler
     |
     |
1183 | #[rustc_doc_primitive = "i64"]


error: attributes starting with `rustc` are reserved for use by the `rustc` compiler
     |
     |
1189 | #[rustc_doc_primitive = "i128"]


error: attributes starting with `rustc` are reserved for use by the `rustc` compiler
     |
     |
1195 | #[rustc_doc_primitive = "u8"]


error: attributes starting with `rustc` are reserved for use by the `rustc` compiler
     |
     |
1201 | #[rustc_doc_primitive = "u16"]


error: attributes starting with `rustc` are reserved for use by the `rustc` compiler
     |
     |
1207 | #[rustc_doc_primitive = "u32"]


error: attributes starting with `rustc` are reserved for use by the `rustc` compiler
     |
     |
1213 | #[rustc_doc_primitive = "u64"]


error: attributes starting with `rustc` are reserved for use by the `rustc` compiler
     |
     |
1219 | #[rustc_doc_primitive = "u128"]


error: attributes starting with `rustc` are reserved for use by the `rustc` compiler
     |
     |
1225 | #[rustc_doc_primitive = "isize"]


error: attributes starting with `rustc` are reserved for use by the `rustc` compiler
     |
     |
1235 | #[rustc_doc_primitive = "usize"]


error: attributes starting with `rustc` are reserved for use by the `rustc` compiler
     |
     |
1245 | #[rustc_doc_primitive = "reference"]


error: attributes starting with `rustc` are reserved for use by the `rustc` compiler
     |
     |
1376 | #[rustc_doc_primitive = "fn"]

error: cannot find attribute `rustc_doc_primitive` in this scope
 --> library/core/src/primitive_docs.rs:4:3
  |
  |
4 | #[rustc_doc_primitive = "bool"]

error: cannot find attribute `rustc_doc_primitive` in this scope
  --> library/core/src/primitive_docs.rs:66:3
   |
   |
66 | #[rustc_doc_primitive = "never"]

error: cannot find attribute `rustc_doc_primitive` in this scope
   --> library/core/src/primitive_docs.rs:277:3
    |
    |
277 | #[rustc_doc_primitive = "char"]

error: cannot find attribute `rustc_doc_primitive` in this scope
   --> library/core/src/primitive_docs.rs:401:3
    |
    |
401 | #[rustc_doc_primitive = "unit"]

error: cannot find attribute `rustc_doc_primitive` in this scope
   --> library/core/src/primitive_docs.rs:463:3
    |
    |
463 | #[rustc_doc_primitive = "pointer"]

error: cannot find attribute `rustc_doc_primitive` in this scope
   --> library/core/src/primitive_docs.rs:580:3
    |
    |
580 | #[rustc_doc_primitive = "array"]

error: cannot find attribute `rustc_doc_primitive` in this scope
   --> library/core/src/primitive_docs.rs:781:3
    |
    |
781 | #[rustc_doc_primitive = "slice"]

error: cannot find attribute `rustc_doc_primitive` in this scope
   --> library/core/src/primitive_docs.rs:873:3
    |
    |
873 | #[rustc_doc_primitive = "str"]

error: cannot find attribute `rustc_doc_primitive` in this scope
   --> library/core/src/primitive_docs.rs:940:3
    |
    |
940 | #[rustc_doc_primitive = "tuple"]

error: cannot find attribute `rustc_doc_primitive` in this scope
    --> library/core/src/primitive_docs.rs:1084:3
     |
     |
1084 | #[rustc_doc_primitive = "f32"]

error: cannot find attribute `rustc_doc_primitive` in this scope
    --> library/core/src/primitive_docs.rs:1150:3
     |
     |
1150 | #[rustc_doc_primitive = "f64"]

error: cannot find attribute `rustc_doc_primitive` in this scope
    --> library/core/src/primitive_docs.rs:1165:3
     |
     |
1165 | #[rustc_doc_primitive = "i8"]

error: cannot find attribute `rustc_doc_primitive` in this scope
    --> library/core/src/primitive_docs.rs:1171:3
     |
     |
1171 | #[rustc_doc_primitive = "i16"]

error: cannot find attribute `rustc_doc_primitive` in this scope
    --> library/core/src/primitive_docs.rs:1177:3
     |
     |
1177 | #[rustc_doc_primitive = "i32"]

error: cannot find attribute `rustc_doc_primitive` in this scope
    --> library/core/src/primitive_docs.rs:1183:3
     |
     |
1183 | #[rustc_doc_primitive = "i64"]

error: cannot find attribute `rustc_doc_primitive` in this scope
    --> library/core/src/primitive_docs.rs:1189:3
     |
     |
1189 | #[rustc_doc_primitive = "i128"]

error: cannot find attribute `rustc_doc_primitive` in this scope
    --> library/core/src/primitive_docs.rs:1195:3
     |
     |
1195 | #[rustc_doc_primitive = "u8"]

error: cannot find attribute `rustc_doc_primitive` in this scope
    --> library/core/src/primitive_docs.rs:1201:3
     |
     |
1201 | #[rustc_doc_primitive = "u16"]

error: cannot find attribute `rustc_doc_primitive` in this scope
    --> library/core/src/primitive_docs.rs:1207:3
     |
     |
1207 | #[rustc_doc_primitive = "u32"]

error: cannot find attribute `rustc_doc_primitive` in this scope
    --> library/core/src/primitive_docs.rs:1213:3
     |
     |
1213 | #[rustc_doc_primitive = "u64"]

error: cannot find attribute `rustc_doc_primitive` in this scope
    --> library/core/src/primitive_docs.rs:1219:3
     |
     |
1219 | #[rustc_doc_primitive = "u128"]

error: cannot find attribute `rustc_doc_primitive` in this scope
    --> library/core/src/primitive_docs.rs:1225:3
     |
     |
1225 | #[rustc_doc_primitive = "isize"]

error: cannot find attribute `rustc_doc_primitive` in this scope
    --> library/core/src/primitive_docs.rs:1235:3
     |
     |
1235 | #[rustc_doc_primitive = "usize"]

error: cannot find attribute `rustc_doc_primitive` in this scope
    --> library/core/src/primitive_docs.rs:1245:3
     |
     |
1245 | #[rustc_doc_primitive = "reference"]

error: cannot find attribute `rustc_doc_primitive` in this scope
    --> library/core/src/primitive_docs.rs:1376:3
     |
     |
1376 | #[rustc_doc_primitive = "fn"]

error: could not compile `core` due to 50 previous errors
Build completed unsuccessfully in 0:00:06
cat: /tmp/toolstate/toolstates.json: No such file or directory

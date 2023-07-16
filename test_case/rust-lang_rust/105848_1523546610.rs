plain
Successfully built e3cb40a07ffc
Successfully tagged rust-ci:latest
Built container sha256:e3cb40a07ffc5a38bf86edadc6beeeab9fdc1a1100930f762d7f04f95d6899de
Uploading finished image to https://ci-caches.rust-lang.org/docker/015202ffd2b451b3ee398ebe8d49ea4f6c9d53757205c58fdba37c35fe105660c874641165b8b2530058e14cc72b8abbe86932045bbaceef603e3bd22051eee8
upload failed: - to s3://rust-lang-ci-sccache2/docker/015202ffd2b451b3ee398ebe8d49ea4f6c9d53757205c58fdba37c35fe105660c874641165b8b2530058e14cc72b8abbe86932045bbaceef603e3bd22051eee8 Unable to locate credentials
[CI_JOB_NAME=x86_64-gnu-llvm-14]
---
---- [rustdoc] tests/rustdoc/issue-107995.rs stdout ----

error: rustdoc failed!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-107995/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-107995" "--deny" "warnings" "/checkout/tests/rustdoc/issue-107995.rs"
stdout: none
error: unescaped backtick
 --> /checkout/tests/rustdoc/issue-107995.rs:7:26
  |
  |
7 | /// A foo, see also [ bar`]
  |
  |
  = note: `-D rustdoc::unescaped-backticks` implied by `-D warnings`
help: the opening backtick of an inline code may be missing
  |
7 | /// A foo, see also [ `bar`]
  |                       +
help: if you meant to use a literal backtick, escape it
  |
7 | /// A foo, see also [ bar\`]

error: unescaped backtick
  --> /checkout/tests/rustdoc/issue-107995.rs:25:11
   |
   |
25 | /// [ Path`]
   |
help: the opening backtick of an inline code may be missing
   |
   |
25 | /// [ `Path`]
   |       +
help: if you meant to use a literal backtick, escape it
   |
25 | /// [ Path\`]

error: aborting due to 2 previous errors
------------------------------------------


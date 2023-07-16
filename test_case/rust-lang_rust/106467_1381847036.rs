plain
configure: rust.debug-assertions := True
configure: rust.overflow-checks := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
Attempting with retry: make prepare
---
   --> library/alloc/src/collections/btree/node.rs:321:15
    |
321 |           const {
    |  _______________^
322 | |             assert!(BorrowType::TRAVERSAL_PERMIT);
    | |_________^ blocks are not supported in generic constant
    |
    |
    = help: consider moving this anonymous constant into a `const` function

error: overly complex generic constant
    --> library/alloc/src/collections/btree/node.rs:1009:15
     |
     |
1009 |           const {
     |  _______________^
1010 | |             assert!(BorrowType::TRAVERSAL_PERMIT);
     | |_________^ blocks are not supported in generic constant
     |
     |
     = help: consider moving this anonymous constant into a `const` function

error: could not compile `alloc` due to 2 previous errors
Build completed unsuccessfully in 0:00:18

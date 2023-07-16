
PS D:\code\temp> D:\code\rust4\build\x86_64-pc-windows-msvc\llvm\bin\llvm-cov.exe show -Xdemangler=rustfilt .\repro.exe -instr-profile="default.profdata"
lib.rs:
    1|       |mod foo {
    2|       |    #[inline(always)]
    3|      2|    pub fn called() { }
    4|       |
    5|      0|    fn uncalled() { }
    6|       |}
    7|       |
    8|       |pub mod bar {
    9|      1|    pub fn call_me() {
   10|      1|        super::foo::called();
   11|      1|    }
   12|       |}
   13|       |
   14|       |pub mod baz {
   15|      1|    pub fn call_me() {
   16|      1|        super::foo::called();
   17|      1|    }
   18|       |}
   19|       |

repro.rs:
    1|       |use lib::{bar, baz};
    2|       |
    3|      1|fn main() {
    4|      1|    bar::call_me();
    5|      1|    baz::call_me();
    6|      1|}

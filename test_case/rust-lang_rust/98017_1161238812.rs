plain
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 338 tests
ii...i.........i....i..i.................iii........ii.i.......i.................ii..... 88/338
............i..............i................i...iii..F.....i...i.....i........i.......i. 176/338
Some tests failed in compiletest suite=codegen mode=codegen host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
...i.......iii..........i.....................iiiiiii.i...................
failures:


---- [codegen] src/test/codegen/function-arguments.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/usr/lib/llvm-12/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/function-arguments/function-arguments.ll" "/checkout/src/test/codegen/function-arguments.rs" "--check-prefixes" "CHECK,NONMSVC"
stdout: none
--- stderr -------------------------------
/checkout/src/test/codegen/function-arguments.rs:94:11: error: CHECK: expected string not found in input
// CHECK: @unsafe_borrow({{i16\*|ptr}} noundef align 2 dereferenceable(2) %_1)
          ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/function-arguments/function-arguments.ll:822:89: note: scanning from here
define void @named_borrow(i32* noalias noundef readonly align 4 dereferenceable(4) %_1) unnamed_addr #2 {
                                                                                        ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/function-arguments/function-arguments.ll:834:18: note: possible intended match here
define void @mutable_unsafe_borrow(i16* noalias noundef align 2 dereferenceable(2) %_1) unnamed_addr #2 {

Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/function-arguments/function-arguments.ll
Check file: /checkout/src/test/codegen/function-arguments.rs


-dump-input=help explains the following input dump.
Input was:
<<<<<<
            .
            .
            .
            .
          817: start:
          818:  ret void
          819: }
          820: 
          821: ; Function Attrs: nonlazybind uwtable
          822: define void @named_borrow(i32* noalias noundef readonly align 4 dereferenceable(4) %_1) unnamed_addr #2 {
check:94'0                                                                                             X~~~~~~~~~~~~~~~~ error: no match found
          823: start:
check:94'0     ~~~~~~
          824:  ret void
check:94'0     ~~~~~~~~~
          825: }
check:94'0     ~
          826: 
check:94'0     ~
          827: ; Function Attrs: nonlazybind uwtable
check:94'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          828: define void @unsafe_borrow(i16* noundef nonnull align 2 %_1) unnamed_addr #2 {
check:94'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          829: start:
check:94'0     ~~~~~~
          830:  ret void
check:94'0     ~~~~~~~~~
          831: }
check:94'0     ~
          832: 
check:94'0     ~
          833: ; Function Attrs: nonlazybind uwtable
check:94'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          834: define void @mutable_unsafe_borrow(i16* noalias noundef align 2 dereferenceable(2) %_1) unnamed_addr #2 {
check:94'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:94'1                      ?                                                                                        possible intended match
          835: start:
check:94'0     ~~~~~~
          836:  ret void
check:94'0     ~~~~~~~~~
          837: }
check:94'0     ~
          838: 
check:94'0     ~
          839: ; Function Attrs: nonlazybind uwtable
check:94'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            .
            .
>>>>>>
------------------------------------------
